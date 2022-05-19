use std::{
    collections::HashMap,
    ffi::OsString,
    fs::{self, File, FileType, Metadata},
    hash::Hasher,
    io,
    os::unix::prelude::MetadataExt,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use derive_more::{Deref, DerefMut};
use jwalk::WalkDirGeneric;
use seahash::SeaHasher;

use archive_utils::tar;
use buildpack::{
    eyre::{eyre, Result},
    serde::{Deserialize, Serialize},
    serde_json, tracing,
};

/// The set of changes between two snapshots
///
/// This `struct` represents a set of changes between two snapshots and thus represents
/// an OCI [Layer](https://github.com/opencontainers/image-spec/blob/main/layer.md)
#[derive(Deref, DerefMut)]
struct SnapshotChanges(Vec<SnapshotChange>);

impl SnapshotChanges {
    /// Create a new set of snapshot changes
    fn new() -> Self {
        Self(Vec::new())
    }

    /// Create a tar archive from the snapshot changes
    ///
    /// This implements the [Representing Changes](https://github.com/opencontainers/image-spec/blob/main/layer.md#representing-changes)
    /// section of the OCI image spec. `Added` and `Modified` paths are added to the tar archive.
    /// `Removed` paths are represented as "whiteout" files.
    fn write(self, src: &Path, dest: &Path) -> Result<()> {
        let file = File::create(dest).unwrap();
        let mut archive = tar::Builder::new(file);
        for change in self.iter() {
            match change {
                SnapshotChange::Added(path) | SnapshotChange::Modified(path) => {
                    archive.append_path_with_name(src.join(path), path)?;
                }
                SnapshotChange::Removed(path) => {
                    let basename = path
                        .file_name()
                        .ok_or_else(|| eyre!("Path has no file name"))?;
                    let mut whiteout = OsString::from(".wh.".to_string());
                    whiteout.push(basename);
                    let path = match path.parent() {
                        Some(parent) => parent.join(whiteout),
                        None => PathBuf::from(whiteout),
                    };
                    let mut header = tar::Header::new_gnu();
                    header.set_path(path)?;
                    header.set_size(0);
                    header.set_cksum();
                    let data: &[u8] = &[];
                    archive.append(&header, data)?;
                }
            };
        }
        Ok(())
    }
}

/// A change in a path between two snapshots
///
/// This enum represents the [Change Types](https://github.com/opencontainers/image-spec/blob/main/layer.md#change-types)
/// described in the OCI spec.
#[derive(Debug, PartialEq)]
enum SnapshotChange {
    Added(PathBuf),
    Modified(PathBuf),
    Removed(PathBuf),
}

/// A snapshot of the files and directories in a directory
///
/// A snapshot is created at the start of a session and stored to disk. Another snapshot
/// is taken at the end of session. The changes between the snapshots are used to create
/// an image layer.
///
/// Currently this uses `serde_json` for serializing to/from disk. An alternative
/// serialization such as `rkyv` would be a lot more efficient but, at the time of writing,
/// does not support `HashMap` with `PathBuf` as the key.
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(crate = "buildpack::serde")]
struct Snapshot {
    /// The directory to snapshot
    dir: PathBuf,

    /// Entries in the snapshot
    entries: HashMap<PathBuf, SnapshotEntry>,
}

impl Snapshot {
    /// Create a new snapshot of a directory
    fn new<P: AsRef<Path>>(dir: P) -> Self {
        let dir = dir.as_ref().to_path_buf();
        let entries = WalkDirGeneric::<((), SnapshotEntry)>::new(&dir)
            .skip_hidden(false)
            .process_read_dir(|_depth, _path, _read_dir_state, children| {
                children.iter_mut().flatten().for_each(|dir_entry| {
                    if !dir_entry.file_type.is_dir() {
                        dir_entry.client_state = SnapshotEntry::new(
                            &dir_entry.path(),
                            &dir_entry.file_type(),
                            dir_entry.metadata().ok(),
                        );
                    }
                })
            })
            .into_iter()
            .filter_map(|entry_result| match entry_result {
                Ok(entry) => {
                    let relative_path = entry
                        .path()
                        .strip_prefix(&dir)
                        .expect("Should always be able to strip the root dir")
                        .to_path_buf();
                    match relative_path == PathBuf::from("") {
                        true => None, // This is the entry for the dir itself so ignore it
                        false => Some((relative_path, entry.client_state)),
                    }
                }
                Err(error) => {
                    tracing::error!("While snapshotting `{}`: {}", dir.display(), error);
                    None
                }
            })
            .collect();
        Self { dir, entries }
    }

    /// Create a new snapshot by repeating the current one
    fn repeat(&self) -> Self {
        Self::new(&self.dir)
    }

    /// Write a snapshot to a file
    fn write(&self, path: &Path) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }

    /// Read a snapshot from a file
    fn read(path: &Path) -> Result<Self> {
        let json = fs::read_to_string(&path)?;
        let snapshot = serde_json::from_str(&json)?;
        Ok(snapshot)
    }

    /// Create a set of changes by calculating the difference between two snapshots
    fn diff(&self, other: &Snapshot) -> SnapshotChanges {
        let mut changes = SnapshotChanges::new();
        for (path, entry) in self.entries.iter() {
            match other.entries.get(path) {
                Some(other_entry) => {
                    if entry != other_entry {
                        changes.push(SnapshotChange::Modified(path.into()))
                    }
                }
                None => changes.push(SnapshotChange::Removed(path.into())),
            }
        }
        for path in other.entries.keys() {
            if !self.entries.contains_key(path) {
                changes.push(SnapshotChange::Added(path.into()))
            }
        }
        changes
    }

    /// Create a set of changes by repeating the current snapshot
    fn changes(&self) -> SnapshotChanges {
        self.diff(&self.repeat())
    }
}

/// An entry for a file or directory in a snapshot
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(crate = "buildpack::serde")]
struct SnapshotEntry {
    /// Metadata on the file or directory
    ///
    /// Should only be `None` if there was an error getting the metadata
    /// while creating the snapshot.
    metadata: Option<SnapshotEntryMetadata>,

    /// Hash of the content of the file
    ///
    /// Will be `None` if the entry is a directory
    fingerprint: Option<u64>,
}

/// Filesystem metadata for a snapshot entry
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(crate = "buildpack::serde")]
struct SnapshotEntryMetadata {
    created: Option<u64>,
    modified: Option<u64>,
    uid: u32,
    gid: u32,
    readonly: bool,
}

impl SnapshotEntry {
    /// Create a new snapshot entry
    fn new(path: &Path, file_type: &FileType, metadata: Option<Metadata>) -> Self {
        let metadata = metadata.map(|metadata| SnapshotEntryMetadata {
            created: file_timestamp(metadata.created()),
            modified: file_timestamp(metadata.modified()),
            uid: metadata.uid(),
            gid: metadata.gid(),
            readonly: metadata.permissions().readonly(),
        });

        let fingerprint = if file_type.is_file() {
            match file_hash::<SeaHasher>(path) {
                Ok(fingerprint) => Some(fingerprint),
                Err(error) => {
                    tracing::error!("While fingerprinting file `{}`: {}", path.display(), error);
                    None
                }
            }
        } else {
            None
        };

        Self {
            metadata,
            fingerprint,
        }
    }
}

/// Generate a hash of a file's content
///
/// Based on https://github.com/jRimbault/yadf/blob/04205a57882ffa7d6a9ca05016e18214a38079b6/src/fs/hash.rs#L29
fn file_hash<H>(path: &Path) -> io::Result<u64>
where
    H: Hasher + Default,
{
    #[repr(transparent)]
    struct HashWriter<H>(H);
    impl<H: Hasher> io::Write for HashWriter<H> {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.0.write(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut hasher = HashWriter(H::default());
    io::copy(&mut File::open(path)?, &mut hasher)?;
    Ok(hasher.0.finish())
}

/// Get a timestamp from a file's created or modified system time
fn file_timestamp(time: Result<SystemTime, io::Error>) -> Option<u64> {
    time.map(|system_time| {
        system_time
            .duration_since(UNIX_EPOCH)
            .expect("Time should not go backwards")
            .as_secs()
    })
    .ok()
}

#[cfg(test)]
mod tests {
    use test_utils::{print_logs, tempfile::tempdir};

    use super::*;

    #[test]
    fn snapshot_changes() -> Result<()> {
        print_logs();

        // Create a temporary directory as a text fixture and a tar file for writing / reading layers

        let dir = tempdir()?;
        let tars = tempdir()?;
        let tar = tars.path().join("layer.tar");

        // Create an initial snapshot which should be empty and has not changes with self

        let snap1 = Snapshot::new(dir.path());
        assert_eq!(snap1.entries.len(), 0);

        let changes = snap1.diff(&snap1);
        assert_eq!(changes.len(), 0);

        // Add a file, create a new snapshot and check it has one entry and produces a change set
        // with `Added` and tar has entry for it

        let a_txt = PathBuf::from("a.txt");
        fs::write(dir.path().join(&a_txt), "Hello from a.txt")?;

        let snap2 = snap1.repeat();
        assert_eq!(snap2.entries.len(), 1);
        assert_eq!(snap2.entries[&a_txt].fingerprint, Some(3958791156379554752));

        let changes = snap1.diff(&snap2);
        assert_eq!(changes.len(), 1);
        assert_eq!(changes[0], SnapshotChange::Added(a_txt.clone()));

        changes.write(dir.path(), &tar)?;
        let mut layer = tar::Archive::new(File::open(&tar)?);
        let mut entries = layer.entries()?;
        let entry = entries.next().unwrap()?;
        assert_eq!(entry.path()?, a_txt);
        assert_eq!(entry.size(), 16);

        // Repeat

        let b_txt = PathBuf::from("b.txt");
        fs::write(dir.path().join(&b_txt), "Hello from b.txt")?;

        let snap3 = snap1.repeat();
        assert_eq!(snap3.entries.len(), 2);
        assert_eq!(snap2.entries[&a_txt].fingerprint, Some(3958791156379554752));
        assert_eq!(
            snap3.entries[&b_txt].fingerprint,
            Some(15548480638800185371)
        );

        let changes = snap2.diff(&snap3);
        assert_eq!(changes.len(), 1);
        assert_eq!(changes[0], SnapshotChange::Added(b_txt.clone()));

        // Remove a.txt and check that the change set has a `Removed` and tar has
        // a whiteout entry of size 0

        fs::remove_file(dir.path().join(&a_txt))?;

        let snap4 = snap1.repeat();
        assert_eq!(snap4.entries.len(), 1);
        assert_eq!(
            snap4.entries[&b_txt].fingerprint,
            Some(15548480638800185371)
        );

        let changes = snap3.diff(&snap4);
        assert_eq!(changes.len(), 1);
        assert_eq!(changes[0], SnapshotChange::Removed(a_txt));

        changes.write(dir.path(), &tar)?;
        let mut layer = tar::Archive::new(File::open(&tar)?);
        let mut entries = layer.entries()?;
        let entry = entries.next().unwrap()?;
        assert_eq!(entry.path()?, PathBuf::from(".wh.a.txt"));
        assert_eq!(entry.size(), 0);

        // Modify b.txt and check that the change set has a `Modified` and tar has
        // entry with new content

        fs::write(dir.path().join(&b_txt), "Hello")?;

        let snap5 = snap1.repeat();
        assert_eq!(snap5.entries.len(), 1);
        assert_eq!(snap5.entries[&b_txt].fingerprint, Some(3297469917561599766));

        let changes = snap4.diff(&snap5);
        assert_eq!(changes.len(), 1);
        assert_eq!(changes[0], SnapshotChange::Modified(b_txt.clone()));

        changes.write(dir.path(), &tar)?;
        let mut layer = tar::Archive::new(File::open(&tar)?);
        let mut entries = layer.entries()?;
        let entry = entries.next().unwrap()?;
        assert_eq!(entry.path()?, b_txt);
        assert_eq!(entry.size(), 5);

        Ok(())
    }
}

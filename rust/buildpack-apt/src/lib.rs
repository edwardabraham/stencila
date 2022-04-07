use std::{
    env,
    ffi::OsString,
    fs::{copy, create_dir_all, read_to_string, remove_dir_all, write, OpenOptions},
    io::{BufWriter, Write},
    path::{Path, PathBuf},
};

use binary::{http_utils::download_sync, Binary, BinaryTrait};
use buildpack::{
    eyre,
    fs_utils::clear_dir_all,
    hash_utils::str_sha256_hex,
    libcnb::{
        self,
        build::{BuildContext, BuildResult, BuildResultBuilder},
        data::{build_plan::BuildPlan, layer_content_metadata::LayerTypes, layer_name},
        detect::{DetectContext, DetectResult, DetectResultBuilder},
        generic::{GenericMetadata, GenericPlatform},
        layer::{ExistingLayerStrategy, Layer, LayerResult, LayerResultBuilder},
        layer_env::{LayerEnv, ModificationBehavior, Scope},
        Buildpack,
    },
    maplit::hashmap,
    tracing, BuildpackTrait, LayerOptions,
};
use serde::{Deserialize, Serialize};
use utils::vec_string;

pub struct AptBuildpack;

impl BuildpackTrait for AptBuildpack {
    fn toml() -> &'static str {
        include_str!("../buildpack.toml")
    }
}

// The name of the file that is detected in the app dir
const APT_FILE: &str = "Aptfile";

// The name of the layer that the buildpack creates
const APT_PACKAGES: &str = "apt_packages";

impl Buildpack for AptBuildpack {
    type Platform = GenericPlatform;
    type Metadata = GenericMetadata;
    type Error = eyre::Report;

    fn detect(&self, _context: DetectContext<Self>) -> libcnb::Result<DetectResult, Self::Error> {
        // Detect `Aptfile`
        let aptfile = PathBuf::from(APT_FILE);

        // Get the Linux release for reuse below
        let linux_flavour = sys_info::linux_os_release().ok();

        // Fail if no Aptfile, or Aptfile exists but not on Ubuntu Linux
        if !aptfile.exists() {
            return DetectResultBuilder::fail().build();
        } else if env::consts::OS != "linux"
            || linux_flavour
                .as_ref()
                .map_or_else(|| "".to_string(), |rel| rel.id().to_string())
                != "ubuntu"
        {
            tracing::warn!("Aptfile detected but will be ignored because not on Ubuntu Linux");
            return DetectResultBuilder::fail().build();
        }

        let mut build_plan = BuildPlan::new();

        // Require `apt_packages` layer if there is an `Aptfile`
        if aptfile.exists() {
            let version = linux_flavour
                .expect("Should have returned by now if not on Linux")
                .version_codename
                .expect("Should have an Ubuntu version codename");

            let (require, provide) = Self::require_and_provide(
                APT_PACKAGES,
                APT_FILE,
                format!("Install `apt` packages for Ubuntu '{}'", version).trim(),
                Some(hashmap! {
                    "version" => version,
                    "file" => APT_FILE.to_string()
                }),
            );
            build_plan.requires.push(require);
            build_plan.provides.push(provide);
        }

        DetectResultBuilder::pass().build_plan(build_plan).build()
    }

    fn build(&self, context: BuildContext<Self>) -> libcnb::Result<BuildResult, Self::Error> {
        let entries = self.buildpack_plan_entries(&context.buildpack_plan);

        if let Some(options) = entries.get(APT_PACKAGES) {
            context.handle_layer(
                layer_name!("apt_packages"),
                AptPackagesLayer::new(options, Some(&context.app_dir)),
            )?;
        }

        BuildResultBuilder::new().build()
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct AptPackagesLayer {
    /// The version of Ubuntu that packages will be installed for e.g `bionic`, `focal`
    version: String,

    /// The path to the `Aptfile` (or similar name) that specifies packages to be installed
    file: Option<PathBuf>,

    /// A list of package names, or deb URLs to be installed
    ///
    /// Usually instead of an `Aptfile` but can be specified in addition to it
    packages: Vec<String>,

    /// A list of repos to be used
    ///
    /// Usually instead of `:repo:` entries in an `Aptfile` but can be specified in addition to it
    repos: Vec<String>,
}

impl AptPackagesLayer {
    pub fn new(options: &LayerOptions, app_path: Option<&Path>) -> Self {
        let version = match options.get("version") {
            Some(version) => version.to_string(),
            None => sys_info::linux_os_release()
                .ok()
                .and_then(|info| info.version_codename)
                .unwrap_or_default(),
        };

        let file = options.get("file").map(PathBuf::from);

        // Split `Aptfile` into  packages and repos
        let mut repos = Vec::new();
        let mut packages = match (&file, &app_path) {
            (Some(file), Some(path)) => read_to_string(path.join(file))
                .unwrap_or_default()
                .lines()
                .filter_map(|line| {
                    let line = line.trim();
                    if line.is_empty() || line.starts_with('#') {
                        None
                    } else if let Some(repo) = line.strip_prefix(":repo:") {
                        repos.push(repo.to_string());
                        None
                    } else {
                        Some(line.to_string())
                    }
                })
                .collect(),
            _ => Vec::new(),
        };

        // Add any other packages
        if let Some(list) = options.get("packages") {
            packages.append(&mut list.split(',').map(|pkg| pkg.trim().to_string()).collect());
        }

        // Add any other repos
        if let Some(list) = options.get("repos") {
            repos.append(&mut list.split(',').map(|pkg| pkg.trim().to_string()).collect());
        }

        Self {
            version,
            file,
            packages,
            repos,
        }
    }
}

impl Layer for AptPackagesLayer {
    type Buildpack = AptBuildpack;
    type Metadata = AptPackagesLayer;

    fn types(&self) -> LayerTypes {
        LayerTypes {
            build: true,
            launch: true,
            cache: true,
        }
    }

    fn existing_layer_strategy(
        &self,
        _context: &BuildContext<Self::Buildpack>,
        layer_data: &libcnb::layer::LayerData<Self::Metadata>,
    ) -> Result<libcnb::layer::ExistingLayerStrategy, <Self::Buildpack as Buildpack>::Error> {
        let existing = &layer_data.content_metadata.metadata;
        let strategy = if self.version != existing.version {
            tracing::info!(
                "Existing `apt_packages` layer is for different Ubuntu version (`{}` => `{}`); will recreate",
                existing.version,
                self.version,
            );
            ExistingLayerStrategy::Recreate
        } else if self.repos != existing.repos {
            tracing::info!(
                "Existing `apt_packages` layer has different repos (`{}` => `{}`); will recreate",
                existing.repos.join(","),
                self.repos.join(","),
            );
            ExistingLayerStrategy::Recreate
        } else if self.packages != existing.packages {
            tracing::info!(
                "Existing `apt_packages` layer has different packages (`{}` => `{}`); will recreate",
                existing.packages.join(","),
                self.packages.join(",")
            );
            ExistingLayerStrategy::Recreate
        } else {
            tracing::info!("Existing `apt_packages` layer meets requirements; will keep",);
            ExistingLayerStrategy::Keep
        };
        Ok(strategy)
    }

    fn create(
        &self,
        _context: &BuildContext<Self::Buildpack>,
        layer_path: &Path,
    ) -> Result<LayerResult<Self::Metadata>, eyre::Report> {
        tracing::info!("Creating `apt_packages` layer");
        self.install(layer_path)
    }

    fn update(
        &self,
        _context: &BuildContext<Self::Buildpack>,
        layer_data: &libcnb::layer::LayerData<Self::Metadata>,
    ) -> Result<LayerResult<Self::Metadata>, <Self::Buildpack as Buildpack>::Error> {
        tracing::info!("Updating `apt_packages` layer");
        self.install(&layer_data.path)
    }
}

impl AptPackagesLayer {
    pub fn install(
        &self,
        layer_path: &Path,
    ) -> Result<LayerResult<AptPackagesLayer>, eyre::Report> {
        let layer_path = &layer_path.canonicalize()?;

        tracing::info!("Installing apt packages: {}", self.packages.join(", "));

        let apt_cache_dir = layer_path.join("cache");
        let apt_archives_dir = apt_cache_dir.join("archives");
        let apt_state_dir = layer_path.join("state");
        let apt_sources_dir = layer_path.join("sources");
        let apt_downloads_dir = layer_path.join("downloads");

        // Create the directories that `apt-get` expects
        create_dir_all(apt_archives_dir.join("partial"))?;
        create_dir_all(apt_state_dir.join("lists").join("partial"))?;
        create_dir_all(&apt_sources_dir)?;
        create_dir_all(&apt_downloads_dir)?;

        // Copy over the system sources list
        let apt_sources_list = apt_sources_dir.join("sources.list");
        copy(
            PathBuf::from("/")
                .join("etc")
                .join("apt")
                .join("sources.list"),
            &apt_sources_list,
        )?;

        // Add any repositories added in the `Aptfile`
        if !self.repos.is_empty() {
            let mut file = BufWriter::new(
                OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open(&apt_sources_list)
                    .expect("Should be able to open file"),
            );
            for repo in &self.repos {
                writeln!(file, "{}", repo)?;
            }
        }

        // WARN: This function has logic for updating an existing layer (with the view to making updates
        // related to changes in the list of packages faster). However, that seems to be unreliable and
        // so for now at least, the layer is recreated on any detected changes

        // Remove everything in the layer's `state`, `usr` and `etc` dirs so we don't have artifacts
        // of packages that were previous installed but have been removed from the list
        tracing::info!("Removing previous installs");
        clear_dir_all(&apt_state_dir)?;
        let layer_usr_dir = layer_path.join("usr");
        if layer_usr_dir.exists() {
            remove_dir_all(layer_usr_dir)?;
        }
        let layer_etc_dir = layer_path.join("etc");
        if layer_etc_dir.exists() {
            remove_dir_all(layer_etc_dir)?;
        }

        let apt = Binary::named("apt-get").require_sync()?;
        let apt_options: Vec<String> = vec![
            "debug::nolocking=true",
            &format!("dir::cache={}", apt_cache_dir.display()),
            &format!("dir::state={}", apt_state_dir.display()),
            &format!("dir::etc::sourcelist={}", apt_sources_list.display()),
            "dir::etc::sourceparts=/dev/null",
        ]
        .into_iter()
        .map(|option| ["-o", option].concat())
        .collect();

        tracing::info!("Updating apt caches");
        apt.run_sync([apt_options.clone(), vec_string!["update"]].concat())?;

        let dpkg = Binary::named("dpkg").require_sync()?;

        // Get deb files
        for package in &self.packages {
            // Use hash of URL as package name for remote debs
            let package_id = if package.starts_with("http") && package.ends_with(".deb") {
                str_sha256_hex(package)
            } else {
                package.to_string()
            };

            // Use record of downloaded debs for a package or download them
            let downloads_for_package = apt_downloads_dir.join(&package_id);
            let downloaded_debs = if downloads_for_package.exists() {
                tracing::info!("Package `{}` already downloaded", package);

                read_to_string(downloads_for_package)?
                    .lines()
                    .map(OsString::from)
                    .collect()
            } else {
                // Get a list of the debs in archive
                let get_debs = || -> Vec<OsString> {
                    apt_archives_dir
                        .read_dir()
                        .expect("Archives directory should be readable")
                        .flatten()
                        .filter_map(|entry| {
                            let path = entry.path();
                            if path.extension() == Some(&OsString::from("deb")) {
                                path.file_name().map(|name| name.to_os_string())
                            } else {
                                None
                            }
                        })
                        .collect()
                };

                // Record debs before
                let debs_before = get_debs();

                if package.starts_with("http") && package.ends_with(".deb") {
                    tracing::info!("Downloading `{}`", package);

                    let path = apt_archives_dir.join(format!("{}.deb", package_id));
                    download_sync(package, &path)?;
                } else {
                    tracing::info!("Fetching deb files for package `{}`", package);

                    // Assumes using `apt-get` >= 1.1 which replaced `--force-yes` with
                    // `--allow-*` options
                    apt.run_sync(
                        [
                            apt_options.clone(),
                            vec_string![
                                "-y",
                                "--allow-downgrades",
                                "--allow-remove-essential",
                                "--allow-change-held-packages",
                                "-d",
                                "install",
                                "--reinstall",
                                package
                            ],
                        ]
                        .concat(),
                    )?;
                }

                let debs_after = get_debs();
                let debs_downloaded: Vec<OsString> = debs_after
                    .into_iter()
                    .filter(|item| !debs_before.contains(item))
                    .collect();

                // Record the debs that were downloaded for the package
                write(
                    apt_downloads_dir.join(package_id),
                    debs_downloaded
                        .iter()
                        .map(|deb| deb.to_string_lossy().to_string())
                        .collect::<Vec<String>>()
                        .join("\n"),
                )?;

                debs_downloaded
            };

            // Install the downloaded deb files into layer using `dpkg`
            tracing::info!("Installing debs for package `{}`", package);
            for deb in downloaded_debs {
                let deb_path = apt_archives_dir.join(deb);
                dpkg.run_sync([
                    "-x",
                    &deb_path.display().to_string(),
                    &layer_path.display().to_string(),
                ])?;
            }
        }

        // Prepend a lot of env vars

        let prefix_paths = |paths: &[&str]| {
            // The trailing colon here is important to separate what we prepend
            // from the existing path
            env::join_paths(paths.iter().map(|path| layer_path.join(path)))
                .map(|joined| format!("{}:", joined.to_string_lossy()))
        };

        let mut layer_env = LayerEnv::new().chainable_insert(
            Scope::All,
            ModificationBehavior::Prepend,
            "PATH",
            prefix_paths(&["usr/bin", "usr/local/bin"])?,
        );

        let include_path_prepend = prefix_paths(&[
            "usr/include/x86_64-linux-gnu",
            "usr/include/i386-linux-gnu",
            "usr/include",
        ])?;
        for var in ["INCLUDE_PATH", "CPATH", "CPPPATH"] {
            layer_env.insert(
                Scope::All,
                ModificationBehavior::Prepend,
                var,
                &include_path_prepend,
            );
        }

        let library_paths = prefix_paths(&[
            "usr/lib/x86_64-linux-gnu",
            "usr/lib/i386-linux-gnu",
            "usr/lib",
            "lib/x86_64-linux-gnu",
            "lib/i386-linux-gnu",
            "lib",
        ])?;
        for var in ["LD_LIBRARY_PATH", "LIBRARY_PATH"] {
            layer_env.insert(
                Scope::All,
                ModificationBehavior::Prepend,
                var,
                &library_paths,
            );
        }

        layer_env.insert(
            Scope::All,
            ModificationBehavior::Prepend,
            "PKG_CONFIG_PATH",
            prefix_paths(&[
                "usr/lib/x86_64-linux-gnu/pkgconfig",
                "usr/lib/i386-linux-gnu/pkgconfig",
                "usr/lib/pkgconfig",
            ])?,
        );

        LayerResultBuilder::new(self.clone()).env(layer_env).build()
    }
}
use std::{
    fs::{metadata, read_to_string, remove_file, write},
    path::{Path, PathBuf},
    str::FromStr,
    time::UNIX_EPOCH,
};

use provider::{
    common::{
        async_trait::async_trait,
        chrono::{DateTime, Utc},
        eyre::{bail, eyre, Result},
        futures::future,
        once_cell::sync::Lazy,
        regex::{Captures, Regex},
        serde_json::{self, json},
        strum::{AsRefStr, Display, EnumString},
        tokio::{self, fs::read},
        tracing,
    },
    http_utils::{
        download_with,
        http::{
            header::{self, HeaderName},
            Request, Response, StatusCode,
        },
        reqwest::{self, multipart},
        response,
    },
    resolve_token,
    stencila_schema::{Article, CreativeWork, Node},
    ExportOptions, ImportOptions, ParseItem, Provider, SyncOptions, WatchMode,
};

pub use provider::ProviderTrait;

/// Base URLs for APIs used
const GOOGLE_DRIVE_API: &str = "https://www.googleapis.com/drive/v3";
const GOOGLE_DOCS_API: &str = "https://docs.googleapis.com/v1";

/// The default name for the token used to authenticate with the API
const TOKEN_NAME: &str = "GOOGLE_TOKEN";

// Regex targeting short identifiers
static SIMPLE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"gdrive:(folder|file|doc|sheet)/([a-zA-Z0-9-_]+)").expect("Unable to create regex")
});

// Regex targeting URL copied from the browser address bar
static URL_REGEX_1: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
                r"(?:https?://)?(?:drive|docs)\.google\.com/(drive|file|document|spreadsheets)(?:.*?)/(?:folders|d)/([a-zA-Z0-9-_]+)(?:/[^\s]*)?",
            )
            .expect("Unable to create regex")
});

// Regex for "open" URLs
static URL_REGEX_2: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?:https?://)?docs\.google\.com/open\?id=([a-zA-Z0-9-_]+)")
        .expect("Unable to create regex")
});

/// The kind of entity on Google Drive
///
/// Used to determine which API requests to make, default import formats etc
///
/// Note: the final serialization variants below allow parsing from the URL_REGEX above
#[derive(Display, Clone, AsRefStr, EnumString)]
#[strum(serialize_all = "lowercase", crate = "provider::common::strum")]
pub enum FileKind {
    #[strum(serialize = "folder", serialize = "drive")]
    Folder,
    File,
    #[strum(serialize = "doc", serialize = "docs", serialize = "document")]
    Doc,
    #[strum(serialize = "sheet", serialize = "spreadsheets")]
    Sheet,
}

impl FileKind {
    fn to_media_type(&self) -> &str {
        match self {
            FileKind::Doc => "application/vnd.google-apps.document",
            FileKind::Sheet => "application/vnd.google-apps.spreadsheet",
            _ => "application/octet",
        }
    }
}

#[derive(Clone)]
struct GoogleDriveClient {
    /// A Google API token
    token: String,
}

impl GoogleDriveClient {
    /// Create a new Google Drive API client
    async fn new(token: Option<String>) -> Result<Self> {
        let token = token.as_deref().unwrap_or(TOKEN_NAME);
        let token = match resolve_token(token) {
            Some(token) => token,
            None => cloud::auth::provider_token("google").await?.access_token,
        };
        Ok(Self { token })
    }

    /// Get a `google_drive` create API client
    async fn api(&self) -> Result<google_drive::Client> {
        Ok(google_drive::Client::new(
            String::new(),
            String::new(),
            String::new(),
            self.token.clone(),
            String::new(),
        ))
    }

    /// Get additional headers required for a request
    fn headers(&self) -> Vec<(HeaderName, String)> {
        vec![(header::AUTHORIZATION, ["Bearer ", &self.token].concat())]
    }

    /// Download a file/folder
    ///
    /// The `google_drive` crate provides a `files().export()` method. However, for some reason (probably
    /// because it is autogenerated from the OpenAPI spec) it returns void. So instead, we use the
    /// `http_utils::download_with` function which saves direct to the path for us too.
    async fn download(&self, file_kind: &FileKind, file_id: &str, dest: &Path) -> Result<()> {
        let headers = self.headers();

        if matches!(file_kind, FileKind::Folder) {
            // Get all the children of the folder
            // See https://developers.google.com/drive/api/v3/search-files
            let query = format!("'{file_id}' in parents");
            let children = self
                .api()
                .await?
                .files()
                .list_all("", "", false, "", false, "", &query, "", false, false, "")
                .await
                .map_err(enhance_error)?;

            // Download in parallel
            let futures = children.into_iter().map(|child| {
                let dest_clone = dest.to_path_buf();
                let headers_clone = headers.clone();
                tokio::spawn(async move {
                    let file_id = child.id;
                    // TODO mime type
                    let url =
                        format!("{GOOGLE_DRIVE_API}/files/{file_id}/export?mimeType=text/html");
                    let filename = PathBuf::from(child.name);
                    let path = dest_clone.join(filename);
                    tracing::debug!(
                        "Downloading child `{file_id}` to `{path}`",
                        path = path.display()
                    );
                    download_with(&url, &path, &headers_clone).await
                })
            });
            future::try_join_all(futures).await?;
        } else {
            // The destination path does not have an extension so use the default for the kind
            // The list of available export formats: https://developers.google.com/drive/api/v3/ref-export-formats
            let (ext, mime_type) = match file_kind {
                FileKind::Doc => (".gdoc", ""),
                FileKind::Sheet => (".csv", "text/csv"),
                FileKind::File => ("", ""),
                _ => bail!("Unhandled Google Drive file kind `{}`", file_kind),
            };

            // Make sure the destination is a file path
            let dest = if dest.is_dir() || dest.extension().is_none() {
                dest.join(format!("{}{}", file_id, ext))
            } else {
                dest.to_path_buf()
            };

            let url = match file_kind {
                FileKind::Doc => {
                    format!("{GOOGLE_DOCS_API}/documents/{file_id}")
                }
                FileKind::Sheet => {
                    format!("{GOOGLE_DRIVE_API}/files/{file_id}/export?mimeType={mime_type}")
                }
                FileKind::File => format!("{GOOGLE_DRIVE_API}/files/{file_id}?alt=media"),
                _ => bail!("Unhandled Google Drive file kind `{}`", file_kind),
            };

            download_with(&url, &dest, &headers)
                .await
                .map_err(enhance_error)?;

            // If the file is a doc, then we need to fetch the Google hosted images in it as well since
            // those are ephemeral (last for ~30 minutes?)
            if matches!(file_kind, FileKind::Doc) && dest.exists() {
                static REGEX: Lazy<Regex> = Lazy::new(|| {
                    Regex::new("\"(https://[a-z0-9]+\\.googleusercontent\\.com/[A-Za-z0-9_-]+)\"")
                        .expect("Unable to create regex")
                });

                let media_dir = dest
                    .file_name()
                    .map(|name| PathBuf::from(format!("{}.media", name.to_string_lossy())))
                    .unwrap_or_else(|| PathBuf::from("media"));

                let gdoc = read_to_string(&dest)?;

                // Replace temporary image URL with file paths in media dir
                let mut index = 1;
                let mut images = Vec::new();
                let gdoc = REGEX
                    .replace_all(&gdoc, |captures: &Captures| -> String {
                        let image_url = match captures.get(1) {
                            Some(mat) => mat.as_str().to_string(),
                            None => return "".to_string(),
                        };

                        let image_file = media_dir.join(format!("{:05}.png", index));

                        images.push((image_url, image_file.clone()));
                        index += 1;

                        let image_file = image_file.to_string_lossy().to_string();
                        ["\"", &image_file, "\""].concat()
                    })
                    .to_string();

                // Download in parallel
                let futures = images.into_iter().map(|(url, path)| {
                    let headers_clone = headers.clone();
                    tokio::spawn(async move { download_with(&url, &path, &headers_clone).await })
                });
                future::try_join_all(futures).await?;

                write(dest, gdoc)?;
            }
        }

        Ok(())
    }

    /// Upload a file
    ///
    /// The `google_drive` crate provides a `files().create()` method. However, for some reason (probably
    /// because it is autogenerated from the OpenAPI spec) it does seem to include the upload functionality
    /// described [here](https://developers.google.com/drive/api/guides/manage-uploads) and [here](https://developers.google.com/drive/api/v3/reference/files/create)
    /// which uses the `https://www.googleapis.com/upload/drive/v3/files` endpoint.
    ///
    /// This implements multipart uploads but note that there is a resumable upload API which may be preferable.
    async fn upload(&self, file_kind: &FileKind, path: &Path) -> Result<google_drive::types::File> {
        let filename = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        let mime_type = file_kind.to_media_type();
        let metadata = serde_json::to_string(&json!({
            "originalFilename": filename,
            "mimeType": mime_type
        }))?;
        let metadata =
            multipart::Part::text(metadata).mime_str("application/json; charset=UTF-8")?;

        let bytes = read(path).await?;
        let media = multipart::Part::stream(bytes);

        // The form parts must be in the correct order
        // See https://developers.google.com/drive/api/guides/manage-uploads#http_1
        let form = multipart::Form::new()
            .part("metadata", metadata)
            .part("media", media);

        // Note: using a raw reqwest client because streaming body interferes with middleware
        let client = reqwest::Client::new();
        let response = client
            .post("https://www.googleapis.com/upload/drive/v3/files?uploadType=multipart")
            .bearer_auth(self.token.clone())
            .multipart(form)
            .send()
            .await?;
        if !response.status().is_success() {
            let text = response.text().await?;
            let message = match serde_json::from_str::<serde_json::Value>(&text) {
                Ok(value) => value
                    .get("error")
                    .and_then(|error| error.get("message"))
                    .and_then(|msg| msg.as_str())
                    .map(|msg| msg.to_string())
                    .unwrap_or_else(|| {
                        serde_json::to_string(&text).unwrap_or_else(|_| "Unknown error".to_string())
                    }),
                Err(..) => text,
            };
            bail!("While uploading file: {}", message)
        }

        let file = response.json().await?;
        Ok(file)
    }
}

pub struct GoogleDriveProvider;

impl GoogleDriveProvider {
    /// Create an URL for a Google Drive resource (usually to store in a [`CreativeWork`] node)
    pub fn create_url(kind: &FileKind, id: &str) -> String {
        let prefix = match kind {
            FileKind::Folder => "https://drive.google.com/drive/folders/",
            FileKind::File => "https://drive.google.com/file/d/",
            FileKind::Doc => "https://docs.google.com/document/d/",
            FileKind::Sheet => "https://docs.google.com/spreadsheets/d/",
        };
        [prefix, id].concat()
    }

    /// Parse a URL for a Google Drive resources (usually to retrieve the kind and id from the URL of a [`CreativeWork`])
    fn parse_url(url: &str) -> Result<(FileKind, String)> {
        let captures = URL_REGEX_1
            .captures(url)
            .ok_or_else(|| eyre!("Unable to parse as a Google Drive URL"))?;
        let kind = FileKind::from_str(&captures[1])?;
        let id = captures[2].to_string();
        Ok((kind, id))
    }

    /// Map a [`Node`] type to a Google Drive [`FileKind`]
    fn file_kind(node: &Node) -> FileKind {
        match node {
            Node::Article(..) => FileKind::Doc,
            _ => FileKind::File,
        }
    }

    /// Extract the kind and id of a Google Drive resource from the URL of a [`CreativeWork`]
    fn file_kind_id(node: &Node) -> Result<(FileKind, String)> {
        match node {
            Node::Article(Article { url, .. }) | Node::CreativeWork(CreativeWork { url, .. }) => {
                match &url {
                    Some(url) => Self::parse_url(url),
                    None => bail!("Creative work has no `url` property"),
                }
            }
            _ => bail!("Unrecognized node type"),
        }
    }
}

#[async_trait]
impl ProviderTrait for GoogleDriveProvider {
    fn spec() -> Provider {
        Provider::new("gdrive")
    }

    fn parse(string: &str) -> Vec<ParseItem> {
        SIMPLE_REGEX
            .captures_iter(string)
            .into_iter()
            .map(|captures| {
                let capture = captures.get(0).unwrap();
                (
                    capture.start(),
                    capture.end(),
                    FileKind::from_str(&captures[1]).unwrap_or(FileKind::File),
                    captures[2].to_string(),
                )
            })
            .chain(
                URL_REGEX_1
                    .captures_iter(string)
                    .into_iter()
                    .map(|captures| {
                        let capture = captures.get(0).unwrap();
                        (
                            capture.start(),
                            capture.end(),
                            FileKind::from_str(&captures[1]).unwrap_or(FileKind::File),
                            captures[2].to_string(),
                        )
                    }),
            )
            .chain(
                URL_REGEX_2
                    .captures_iter(string)
                    .into_iter()
                    .map(|captures| {
                        let capture = captures.get(0).unwrap();
                        (
                            capture.start(),
                            capture.end(),
                            FileKind::Doc,
                            captures[1].to_string(),
                        )
                    }),
            )
            .map(|(begin, end, kind, id)| ParseItem {
                begin,
                end,
                node: Node::CreativeWork(CreativeWork {
                    url: Some(Box::new(Self::create_url(&kind, &id))),
                    ..Default::default()
                }),
            })
            .collect()
    }

    fn recognize(node: &Node) -> bool {
        Self::file_kind_id(node).is_ok()
    }

    async fn import(node: &Node, dest: &Path, options: Option<ImportOptions>) -> Result<()> {
        let (file_kind, file_id) = Self::file_kind_id(node)?;

        let options = options.unwrap_or_default();
        let client = GoogleDriveClient::new(options.token).await?;

        tracing::info!(
            "Downloading {} to {}",
            Self::create_url(&file_kind, &file_id),
            dest.display()
        );
        client.download(&file_kind, &file_id, dest).await
    }

    async fn export(node: &Node, path: &Path, options: Option<ExportOptions>) -> Result<Node> {
        let file_kind = Self::file_kind(node);

        let options = options.unwrap_or_default();
        let client = GoogleDriveClient::new(options.token).await?;

        tracing::info!("Uploading {} as {}", path.display(), file_kind);
        let file = client.upload(&file_kind, path).await?;

        let url = Some(Box::new(Self::create_url(&file_kind, &file.id)));

        let node = match file_kind {
            FileKind::Doc => Node::Article(Article {
                url,
                ..Default::default()
            }),
            _ => Node::CreativeWork(CreativeWork {
                url,
                ..Default::default()
            }),
        };
        Ok(node)
    }

    /// Synchronize from a Google Drive resource to a local file or folder
    ///
    /// This does not yet handle syncing of folders. For those we will probably need to set up
    /// individual watches for each file in the folder. Note that the the user still needs to give
    /// Stencila permission to access each individual file in a folder.
    #[tracing::instrument(skip(request))]
    async fn sync(
        node: &Node,
        dest: &Path,
        request: &Request<serde_json::Value>,
        options: Option<SyncOptions>,
    ) -> Result<Response<String>> {
        tracing::trace!("Received a Google Drive sync event");

        let (file_kind, file_id) = Self::file_kind_id(node)?;

        let options = options.unwrap_or_default();
        let client = GoogleDriveClient::new(options.token).await?;

        let headers = request.headers();

        let dest = dest.to_path_buf();
        let watch_mode = options.mode.unwrap_or_default();

        let state = match headers.get("X-Goog-Resource-State") {
            Some(state) => state.to_str()?,
            None => {
                let msg = "Rejected notification without `X-Goog-Resource-State`";
                tracing::warn!("{}", msg);
                bail!("{}", msg)
            }
        };

        // For a list of states see https://developers.google.com/drive/api/v3/push#understanding-drive-api-notification-events
        match state {
            "sync" => {
                // Initial sync message
                // See https://developers.google.com/drive/api/v3/push#sync-message
                let msg = "Channel sync notification";
                tracing::debug!("{}", msg);
                return Ok(response(StatusCode::ACCEPTED, msg));
            }
            "add" | "untrash" => {
                tracing::info!(
                    "Google Drive file `{}` was added or untrashed, adding `{}`",
                    file_id,
                    dest.display()
                );
                client.download(&file_kind, &file_id, &dest).await?;
            }
            "update" => {
                // File was updated so get the type of change (so we can ignore changes to `parents`
                // `children` and `permissions`; at least for now)
                let changed = headers
                    .get("X-Goog-Changed")
                    .map_or("", |val| val.to_str().unwrap_or_default());
                if changed.contains("content") || changed.contains("properties") {
                    match watch_mode {
                        WatchMode::Changed | WatchMode::Committed => {
                            // Change in content so download
                            // Experiments showed that notifications for changes to content
                            // are throttled to once every three minutes (at least for sheets). So "live"
                            // means with up to three minute delay (but will be immediate if the last change
                            // was more than three minutes ago)
                            tracing::info!(
                                "Google Drive file `{}` changed, updating `{}`",
                                file_id,
                                dest.display()
                            );
                            client.download(&file_kind, &file_id, &dest).await?;
                        }
                        WatchMode::Tagged => {
                            // See if there is a new revision since the local file was modified
                            if let Some(revision) = client
                                .api()
                                .await?
                                .revisions()
                                .list_all(&file_id)
                                .await
                                .map_err(enhance_error)?
                                .last()
                            {
                                if let Some(time) = revision.modified_time {
                                    let local_modified_time = metadata(&dest)
                                        .and_then(|metadata| metadata.modified())
                                        .map(DateTime::<Utc>::from)
                                        .unwrap_or_else(|_| DateTime::<Utc>::from(UNIX_EPOCH));
                                    if time > local_modified_time {
                                        tracing::info!(
                                            "Google Drive file `{}` has a new revision, updating `{}`",
                                            file_id,
                                            dest.display()
                                        );
                                        client.download(&file_kind, &file_id, &dest).await?;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            "remove" | "trash" => {
                tracing::info!(
                    "Google Drive file `{}` was removed or trashed, deleting `{}`",
                    file_id,
                    dest.display()
                );
                if dest.exists() {
                    remove_file(dest)?;
                }
            }
            _ => {
                let msg = format!("Ignoring notification for state `{}`", state);
                tracing::warn!("{}", msg);
                return Ok(response(StatusCode::ACCEPTED, &msg));
            }
        }

        Ok(response(StatusCode::OK, "OK"))
    }
}

/// Convert an `anyhow::Error` to an `eyre::Report` and if the error is
/// a 404 provide the user with some hints as to what to do.
fn enhance_error<E: std::fmt::Debug>(error: E) -> provider::common::eyre::Report {
    let mut message = format!("{:?}", error);
    if message.contains("404 Not Found") {
        message = "Could not access the Google Drive file or folder. Please check that it exists, that you have given Stencila permission to access it, and a Google access token is available (you may need to connect Google to your Stencila account)".to_string();
    }
    eyre!(message)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_utils::assert_json_is;

    #[test]
    fn parse() {
        for string in [
            "gdrive:folder/1OcB7VTWb3lc0u8FJX2LXc5GraKpn-r_m",
            "drive.google.com/drive/folders/1OcB7VTWb3lc0u8FJX2LXc5GraKpn-r_m",
            "https://drive.google.com/drive/folders/1OcB7VTWb3lc0u8FJX2LXc5GraKpn-r_m",
            "https://drive.google.com/drive/u/1/folders/1OcB7VTWb3lc0u8FJX2LXc5GraKpn-r_m",
        ] {
            assert_json_is!(
                GoogleDriveProvider::parse(string)[0].node,
                {
                    "type": "CreativeWork",
                    "url": "https://drive.google.com/drive/folders/1OcB7VTWb3lc0u8FJX2LXc5GraKpn-r_m",
                }
            );
        }

        for string in [
            "gdrive:file/1BW6MubIyDirCGW9Wq-tSqCma8pioxBI6VpeLyXn5mZA",
            "drive.google.com/file/d/1BW6MubIyDirCGW9Wq-tSqCma8pioxBI6VpeLyXn5mZA",
            "https://drive.google.com/file/d/1BW6MubIyDirCGW9Wq-tSqCma8pioxBI6VpeLyXn5mZA",
            "https://drive.google.com/file/d/1BW6MubIyDirCGW9Wq-tSqCma8pioxBI6VpeLyXn5mZA/view?usp=sharing",
            ] {
                assert_json_is!(
                    GoogleDriveProvider::parse(string)[0].node,
                    {
                        "type": "CreativeWork",
                        "url": "https://drive.google.com/file/d/1BW6MubIyDirCGW9Wq-tSqCma8pioxBI6VpeLyXn5mZA",
                    }
                );
            }

        for string in [
                "docs.google.com/document/d/1BW6MubIyDirCGW9Wq-tSqCma8pioxBI6VpeLyXn5mZA",
                "https://docs.google.com/document/d/1BW6MubIyDirCGW9Wq-tSqCma8pioxBI6VpeLyXn5mZA/",
                "https://docs.google.com/document/d/1BW6MubIyDirCGW9Wq-tSqCma8pioxBI6VpeLyXn5mZA/edit",
                "https://docs.google.com/document/u/1/d/1BW6MubIyDirCGW9Wq-tSqCma8pioxBI6VpeLyXn5mZA/edit",
                "https://docs.google.com/open?id=1BW6MubIyDirCGW9Wq-tSqCma8pioxBI6VpeLyXn5mZA"
            ] {
                assert_json_is!(
                    GoogleDriveProvider::parse(string)[0].node,
                    {
                        "type": "CreativeWork",
                        "url": "https://docs.google.com/document/d/1BW6MubIyDirCGW9Wq-tSqCma8pioxBI6VpeLyXn5mZA",
                    }
                );
            }

        for string in [
            "docs.google.com/spreadsheets/d/1BW6MubIyDirCGW9Wq-tSqCma8pioxBI6VpeLyXn5mZA",
            "https://docs.google.com/spreadsheets/d/1BW6MubIyDirCGW9Wq-tSqCma8pioxBI6VpeLyXn5mZA/",
            "https://docs.google.com/spreadsheets/d/1BW6MubIyDirCGW9Wq-tSqCma8pioxBI6VpeLyXn5mZA/edit",
            "https://docs.google.com/spreadsheets/u/0/d/1BW6MubIyDirCGW9Wq-tSqCma8pioxBI6VpeLyXn5mZA/edit",
        ] {
            assert_json_is!(
                GoogleDriveProvider::parse(string)[0].node,
                {
                    "type": "CreativeWork",
                    "url": "https://docs.google.com/spreadsheets/d/1BW6MubIyDirCGW9Wq-tSqCma8pioxBI6VpeLyXn5mZA",
                }
            );
        }

        // Multiple items in a string
        let parse_items = GoogleDriveProvider::parse(
            "
            gdrive:file/17Fw92iZgjD9dEcE8N2m08m-CRa3g-6_Ar24TLumjVV0 som word to be ignored
            and then another url https://docs.google.com/spreadsheets/d/1STkgekwd0Vqo9wj8huU2ps9RaPRvfAWDF7GoR5Vb3GY
            and https://docs.google.com/open?id=1BW6MubIyDirCGW9Wq-tSqCma8pioxBI6VpeLyXn5mZA too
        ",
        );
        assert_eq!(parse_items.len(), 3);
        assert_json_is!(parse_items[0].node, {
            "type": "CreativeWork",
            "url": "https://drive.google.com/file/d/17Fw92iZgjD9dEcE8N2m08m-CRa3g-6_Ar24TLumjVV0",
        });
        assert_json_is!(parse_items[1].node, {
            "type": "CreativeWork",
            "url": "https://docs.google.com/spreadsheets/d/1STkgekwd0Vqo9wj8huU2ps9RaPRvfAWDF7GoR5Vb3GY",
        });
        assert_json_is!(parse_items[2].node, {
            "type": "CreativeWork",
            "url": "https://docs.google.com/document/d/1BW6MubIyDirCGW9Wq-tSqCma8pioxBI6VpeLyXn5mZA",
        });
    }
}

use std::{collections::BTreeMap, path::Path};

use common::{
    async_trait::async_trait,
    eyre::{bail, Result},
    serde::Serialize,
    smart_default::SmartDefault,
    strum::{Display, IntoEnumIterator},
    tokio::{
        fs::{create_dir_all, File},
        io::{AsyncReadExt, AsyncWriteExt},
    },
    tracing,
};
use format::Format;
use node_strip::StripScope;
use schema::{Node, NodeType};
use status::Status;

// Re-exports for the convenience of internal crates implementing `Codec`
pub use codec_losses::{Losses, LossesResponse};
pub use common;
pub use format;
pub use schema;
pub use status;

/// The direction of conversion
pub enum CodecDirection {
    Decode,
    Encode,
}

/// A codec for decoding/encoding between Stencila Schema nodes and alternative formats
#[async_trait]
pub trait Codec: Sync + Send {
    /// The name of the codec
    ///
    /// Used when listing codecs and to select a codec when the user specifies
    /// the relevant options on the command line e.g. `--to jats-pandoc`.
    /// Should be kebab-cased.
    fn name(&self) -> &str;

    /// The status of the codec
    ///
    /// Used when listing codecs and to warn users when using a codec that
    /// is not stable.
    fn status(&self) -> Status;

    /// The level of support that the codec provides for decoding from a format
    #[allow(unused)]
    fn supports_from_format(&self, format: Format) -> CodecSupport {
        CodecSupport::None
    }

    /// The level of support that the codec provides for decoding from each format
    fn supports_from_formats(&self) -> BTreeMap<Format, CodecSupport> {
        Format::iter()
            .filter_map(|format| {
                let support = self.supports_from_format(format);
                support.is_supported().then_some((format, support))
            })
            .collect()
    }

    /// The level of support that the codec provides for decoding for a [`NodeType`]
    #[allow(unused)]
    fn supports_from_type(&self, node_type: NodeType) -> CodecSupport {
        CodecSupport::None
    }

    /// The level of support that the codec provides for decoding for each [`NodeType`]
    fn supports_from_types(&self) -> BTreeMap<String, CodecSupport> {
        NodeType::iter()
            .filter_map(|node_type| {
                let support = self.supports_from_type(node_type);
                support
                    .is_supported()
                    .then_some((node_type.to_string(), support))
            })
            .collect()
    }

    /// Whether the codec supports decoding from string content
    fn supports_from_string(&self) -> bool {
        true
    }

    /// Whether the codec supports decoding from a file system path
    fn supports_from_path(&self) -> bool {
        true
    }

    /// The level of support that the codec provides for encoding to a format
    #[allow(unused)]
    fn supports_to_format(&self, format: Format) -> CodecSupport {
        CodecSupport::None
    }

    /// The level of support that the codec provides for encoding to each format
    fn supports_to_formats(&self) -> BTreeMap<Format, CodecSupport> {
        Format::iter()
            .filter_map(|format| {
                let support = self.supports_to_format(format);
                support.is_supported().then_some((format, support))
            })
            .collect()
    }

    /// The level of support that the codec provides for encoding for a [`NodeType`]
    #[allow(unused)]
    fn supports_to_type(&self, node_type: NodeType) -> CodecSupport {
        CodecSupport::None
    }

    /// The level of support that the codec provides for encoding for each [`NodeType`]
    fn supports_to_types(&self) -> BTreeMap<String, CodecSupport> {
        NodeType::iter()
            .filter_map(|node_type| {
                let support = self.supports_to_type(node_type);
                support
                    .is_supported()
                    .then_some((node_type.to_string(), support))
            })
            .collect()
    }

    /// Whether the codec supports encoding to string content
    fn supports_to_string(&self) -> bool {
        true
    }

    /// Whether the codec supports encoding to a file system path
    fn supports_to_path(&self) -> bool {
        true
    }

    /// Get a list of types that the codec has either lossy decoding, or encoding, or both
    fn lossy_types(&self, direction: Option<CodecDirection>) -> Vec<NodeType> {
        let mut types = Vec::new();

        for node_type in NodeType::iter() {
            if (direction.is_none() || matches!(direction, Some(CodecDirection::Decode)))
                && self.supports_from_type(node_type).is_lossy()
                && !types.contains(&node_type)
            {
                types.push(node_type)
            }

            if (direction.is_none() || matches!(direction, Some(CodecDirection::Encode)))
                && self.supports_to_type(node_type).is_lossy()
                && !types.contains(&node_type)
            {
                types.push(node_type)
            }
        }

        types
    }

    /// Whether the codec uses remote state
    ///
    /// Some formats (e.g. Google Docs) have their canonical state in a remote
    /// location (e.g. Google's servers) and Stencila only "mirrors" them in a local
    /// file containing an id or URL linking the file to the remote state.
    fn has_remote_state(&self) -> bool {
        false
    }

    /// Decode a Stencila Schema node from a string
    #[allow(clippy::wrong_self_convention)]
    async fn from_str(
        &self,
        _str: &str,
        _options: Option<DecodeOptions>,
    ) -> Result<(Node, Losses)> {
        bail!(
            "Decoding from string is not implemented for codec `{}`",
            self.name()
        )
    }

    /// Decode a Stencila Schema node from a file
    ///
    /// This function reads the file as a string and passes that on to `from_str`
    /// for decoding. If working with binary formats, you should override this function
    /// to read the file as bytes instead.
    #[tracing::instrument(skip(self))]
    async fn from_file(
        &self,
        file: &mut File,
        options: Option<DecodeOptions>,
    ) -> Result<(Node, Losses)> {
        let mut content = String::new();
        file.read_to_string(&mut content).await?;
        self.from_str(&content, options).await
    }

    /// Decode a Stencila Schema node from a file system path
    #[tracing::instrument(skip(self))]
    async fn from_path(
        &self,
        path: &Path,
        options: Option<DecodeOptions>,
    ) -> Result<(Node, Losses)> {
        if !path.exists() {
            bail!("Path `{}` does not exist", path.display());
        }

        let mut file = File::open(path).await?;
        self.from_file(&mut file, options).await
    }

    /// Encode a Stencila Schema node to a string
    async fn to_string(
        &self,
        _node: &Node,
        _options: Option<EncodeOptions>,
    ) -> Result<(String, Losses)> {
        bail!(
            "Encoding to a string is not implemented for codec `{}`",
            self.name()
        )
    }

    /// Encode a Stencila Schema to a file
    #[tracing::instrument(skip(self))]
    async fn to_file(
        &self,
        node: &Node,
        file: &mut File,
        options: Option<EncodeOptions>,
    ) -> Result<Losses> {
        let mut options = options.unwrap_or_default();
        if options.standalone.is_none() {
            options.standalone = Some(true);
        }

        let (content, losses) = self.to_string(node, Some(options)).await?;
        file.write_all(content.as_bytes()).await?;
        Ok(losses)
    }

    /// Encode a Stencila Schema to a file system path
    #[tracing::instrument(skip(self))]
    async fn to_path(
        &self,
        node: &Node,
        path: &Path,
        options: Option<EncodeOptions>,
    ) -> Result<Losses> {
        if let Some(parent) = path.parent() {
            create_dir_all(parent).await?;
        }
        let mut file = File::create(path).await?;
        self.to_file(node, &mut file, options).await
    }
}

/// The level of support that a codec provides for a format or node type
#[derive(Debug, Default, Display, Serialize)]
#[serde(crate = "common::serde")]
pub enum CodecSupport {
    #[default]
    None,
    HighLoss,
    LowLoss,
    NoLoss,
}

impl CodecSupport {
    /// Whether a format or node type is supported
    pub fn is_supported(&self) -> bool {
        !matches!(self, CodecSupport::None)
    }

    /// Whether there is any loss for a format or node type
    pub fn is_lossy(&self) -> bool {
        !matches!(self, CodecSupport::NoLoss)
    }
}

/// Decoding options
#[derive(Debug, SmartDefault, Clone)]
pub struct DecodeOptions {
    /// The name of the codec to use for decoding
    ///
    /// If not supplied then the format will be used to choose a codec.
    pub codec: Option<String>,

    /// The format to be decode from
    ///
    /// Most codecs only decode one format. However, for those that handle multiple
    /// format it may be necessary to specify this option.
    pub format: Option<Format>,

    /// The response to take when there are losses in the decoding
    #[default(_code = "LossesResponse::Warn")]
    pub losses: LossesResponse,
}

/// Encoding options
#[derive(Debug, SmartDefault, Clone)]
pub struct EncodeOptions {
    /// The name of the codec to use for encoding
    ///
    /// If not supplied then the format will be used to choose a codec.
    pub codec: Option<String>,

    /// The format to encode to
    ///
    /// Most codecs only encode to one format. However, for those that handle multiple
    /// formats it may be necessary to specify this option.
    pub format: Option<Format>,

    /// Whether to encode as a standalone document
    ///
    /// Unless specified otherwise, this is the default when encoding to a file
    /// (as opposed to a string).
    pub standalone: Option<bool>,

    /// Whether to encode in compact form
    ///
    /// Some formats (e.g HTML and JSON) can be encoded in either compact
    /// or "pretty-printed" (e.g. indented) forms.
    #[default = false]
    pub compact: bool,

    /// Scopes defining which properties of nodes should be stripped when encoding
    pub strip_scopes: Vec<StripScope>,

    /// A list of node types to strip when encoding
    pub strip_types: Vec<String>,

    /// A list of node properties to strip when encoding
    pub strip_props: Vec<String>,

    /// The response to take when there are losses in the encoding
    #[default(_code = "LossesResponse::Warn")]
    pub losses: LossesResponse,
}

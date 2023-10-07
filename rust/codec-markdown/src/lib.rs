use codec::{
    common::{async_trait::async_trait, eyre::Result},
    format::Format,
    schema::{Node, NodeType},
    status::Status,
    Codec, CodecSupport, EncodeOptions, Losses,
};
use codec_markdown_trait::{MarkdownCodec as _, MarkdownEncodeContext};

/// A codec for Markdown
pub struct MarkdownCodec;

#[async_trait]
impl Codec for MarkdownCodec {
    fn name(&self) -> &str {
        "markdown"
    }

    fn status(&self) -> Status {
        Status::UnderDevelopment
    }

    fn supports_to_format(&self, format: Format) -> CodecSupport {
        match format {
            Format::Markdown => CodecSupport::LowLoss,
            _ => CodecSupport::None,
        }
    }

    fn supports_to_type(&self, node_type: NodeType) -> CodecSupport {
        use CodecSupport::*;
        use NodeType::*;
        match node_type {
            // Data
            String | Cord => NoLoss,
            Null | Boolean | Integer | UnsignedInteger | Number => LowLoss,
            // Prose Inlines
            Text | Emphasis | Strong | Subscript | Superscript | Underline => NoLoss,
            Link | Parameter | AudioObject | ImageObject | MediaObject => LowLoss,
            // Prose Blocks
            Section | Heading | Paragraph | ThematicBreak => NoLoss,
            List | ListItem | Table | TableRow | TableCell => LowLoss,
            // Code
            CodeFragment | CodeBlock => NoLoss,
            CodeExpression | CodeChunk => LowLoss,
            // Math
            MathFragment | MathBlock => NoLoss,
            // Works,
            Article => LowLoss,
            // Because `to_markdown` is implemented for all types, defaulting to
            // `to_text`, fallback to high loss
            _ => HighLoss,
        }
    }

    async fn to_string(
        &self,
        node: &Node,
        _options: Option<EncodeOptions>,
    ) -> Result<(String, Losses)> {
        let context = MarkdownEncodeContext::default();
        let (markdown, losses) = node.to_markdown(&context);

        let markdown = markdown.trim().to_string();

        Ok((markdown, losses))
    }
}

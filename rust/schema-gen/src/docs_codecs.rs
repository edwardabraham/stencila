//! Generation of reference documentation from Stencila Schema

use std::{
    fs::{self},
    path::PathBuf,
    str::FromStr,
};

use codecs::{CodecSupport, EncodeOptions, Format};
use common::{
    eyre::Result, glob::glob, inflector::Inflector, itertools::Itertools, strum::IntoEnumIterator,
};
use schema::{shortcuts::*, Article, Inline, Node, NodeType, TableCell};

use crate::{
    schema::{Category, HtmlOptions, JatsOptions, MarkdownOptions, Schema},
    schemas::Schemas,
};

impl Schemas {
    /// Generate documentation for codecs
    ///
    /// Rather than create new documentation files for each codec
    /// this is designed to augment existing documentation. Add the
    /// following to a Markdown file in the `reference/formats`
    /// folder:
    ///
    /// <!-- prettier-ignore-start -->
    /// <!-- CODEC-DOCS:START -->
    ///
    /// <!-- CODEC-DOCS:STOP -->
    /// <!-- prettier-ignore-end -->
    pub async fn docs_codecs(&self) -> Result<()> {
        eprintln!("Generating documentation for codecs");

        let dest = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../docs/reference/formats");

        const START: &str = "<!-- CODEC-DOCS:START -->";
        const STOP: &str = "<!-- CODEC-DOCS:STOP -->";

        for file in glob(dest.join("*.md").as_os_str().to_str().unwrap())?.flatten() {
            let name = file.file_stem().unwrap().to_string_lossy().to_string();
            let format = Format::from_name(&name)?;

            let Ok(codec) = codecs::get(None, Some(format), None) else {
                continue;
            };

            let mut content = fs::read_to_string(&file)?;

            let start = content.find(START);
            let stop = content.rfind(STOP);

            let (Some(start), Some(stop)) = (start, stop) else {
                continue
            };

            let mut items = Vec::new();
            if codec.supports_from_path() {
                items.push(li([text("decoding from a file")]))
            }
            if codec.supports_from_string() {
                items.push(li([text("decoding from a string")]))
            }
            if codec.supports_to_path() {
                items.push(li([text("encoding to a file")]))
            }
            if codec.supports_to_string() {
                items.push(li([text("encoding to a string")]))
            }

            let mut rows = vec![tr([
                th([text("Node type")]),
                th([text("Encoding")]),
                th([text("Decoding")]),
                th([text("Notes")]),
            ])];
            for category in Category::iter() {
                rows.push(tr([td([strong([text(
                    category.to_string().to_title_case(),
                )])])]));

                for (title, schema) in self
                    .schemas
                    .iter()
                    .filter(|(_, schema)| schema.category == category)
                    .sorted_by(|(a, ..), (b, ..)| a.cmp(b))
                {
                    let Ok(node_type) = NodeType::from_str(title) else {
                        continue
                    };

                    let title = td([link(
                        [text(title)],
                        format!(
                            "https://github.com/stencila/stencila/blob/main/docs/reference/schema/{category}/{title}.md",
                            title = title.to_snake_case()
                        ),
                    )]);

                    fn codec_support(support: CodecSupport) -> TableCell {
                        match support {
                            CodecSupport::None => td([]),
                            support => td([text(format!(
                                "{icon} {desc}",
                                icon = match support {
                                    CodecSupport::NoLoss => "🟢",
                                    CodecSupport::LowLoss => "🔷",
                                    CodecSupport::HighLoss => "⚠️",
                                    CodecSupport::None => "",
                                },
                                desc = support.to_string().to_sentence_case()
                            ))]),
                        }
                    }

                    let encoding = codec_support(codec.supports_to_type(node_type));
                    let decoding = codec_support(codec.supports_from_type(node_type));
                    let notes = td(Schemas::docs_format_notes(schema, format));

                    rows.push(tr([title, encoding, decoding, notes]));
                }
            }

            let article = Article {
                content: vec![
                    h2([text("Codec")]),
                    p([
                        text("The codec (en"),
                        strong([text("co")]),
                        text("der/"),
                        strong([text("dec")]),
                        text("oder) for "),
                        text(format.name()),
                        text(" supports:"),
                    ]),
                    ul(items),
                    p([text("Support and degree of loss for node types:")]),
                    table(rows),
                ],
                ..Default::default()
            };

            let md = codecs::to_string(
                &Node::Article(article),
                Some(EncodeOptions {
                    format: Some(Format::Markdown),
                    ..Default::default()
                }),
            )
            .await?;
            let md = ["\n\n", &md, "\n\n"].concat();

            content.replace_range(start.saturating_add(START.len())..stop, &md);

            fs::write(file, content)?;
        }

        Ok(())
    }

    /// Generates notes for a schema and format
    pub fn docs_format_notes(schema: &Schema, format: Format) -> Vec<Inline> {
        if let (Format::Html, Some(HtmlOptions { special, elem, .. })) = (format, &schema.html) {
            if *special {
                if let Some(elem) = elem {
                    vec![
                        text("Encoded to tag "),
                        link(
                            [cf(format!("<{elem}>"))],
                            format!(
                                "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/{elem}"
                            ),
                        ),
                        text(" using special function"),
                    ]
                } else {
                    vec![text("Encoded using special function")]
                }
            } else if let Some(elem) = elem {
                vec![
                    text("Encoded to tag "),
                    link(
                        [cf(format!("<{elem}>"))],
                        format!("https://developer.mozilla.org/en-US/docs/Web/HTML/Element/{elem}"),
                    ),
                ]
            } else {
                vec![text("Encoded using derived function")]
            }
        } else if let (Format::Jats, Some(JatsOptions { elem, special, .. })) =
            (format, &schema.jats)
        {
            if *special {
                if let Some(elem) = elem {
                    vec![
                        text("Encoded to tag "),
                        link(
                            [cf(format!("<{elem}>"))],
                            format!("https://jats.nlm.nih.gov/articleauthoring/tag-library/1.3/element/{elem}.html"),
                        ),
                        text(" using special function")
                    ]
                } else {
                    vec![text("Encoded using special function")]
                }
            } else if let Some(elem) = elem {
                vec![
                    text("Encoded to tag "),
                    link(
                        [cf(format!("<{elem}>"))],
                        format!("https://jats.nlm.nih.gov/articleauthoring/tag-library/1.3/element/{elem}.html"),
                    ),
                ]
            } else {
                vec![text("Encoded using derived function")]
            }
        } else if let (
            Format::Markdown,
            Some(MarkdownOptions {
                special, format, ..
            }),
        ) = (format, &schema.markdown)
        {
            if *special {
                vec![text("Encoded using special function")]
            } else if let Some(format) = format {
                vec![text("Encoded using template "), cf(format)]
            } else {
                vec![text("Encoded using derived function")]
            }
        } else {
            vec![]
        }
    }
}

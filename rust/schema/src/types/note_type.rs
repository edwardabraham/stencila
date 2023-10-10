// Generated file; do not edit. See `schema-gen` crate.

use crate::prelude::*;

/// The type of a `Note` which determines where the note content is displayed within the document..
#[derive(Debug, Display, Clone, PartialEq, Serialize, Deserialize, StripNode, HtmlCodec, JatsCodec, MarkdownCodec, TextCodec, SmartDefault, ReadNode, WriteNode)]
#[serde(crate = "common::serde")]
#[cfg_attr(feature = "proptest", derive(Arbitrary))]
pub enum NoteType {
    #[default]
    Footnote,

    Endnote,

    Sidenote,
}

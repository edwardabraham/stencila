// Generated file; do not edit. See `schema-gen` crate.

use crate::prelude::*;

use super::number::Number;
use super::string::String;

/// A validator specifying the constraints on a numeric node.
#[skip_serializing_none]
#[serde_as]
#[derive(Debug, SmartDefault, Clone, PartialEq, Serialize, Deserialize, StripNode, HtmlCodec, JatsCodec, MarkdownCodec, TextCodec, WriteNode, ReadNode)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
#[derive(derive_more::Display)]
#[display(fmt = "NumberValidator")]
pub struct NumberValidator {
    /// The type of this item.
    pub r#type: MustBe!("NumberValidator"),

    /// The identifier for this item.
    #[strip(metadata)]
    #[html(attr = "id")]
    pub id: Option<String>,

    /// The inclusive lower limit for a numeric node.
    pub minimum: Option<Number>,

    /// The exclusive lower limit for a numeric node.
    #[serde(alias = "exclusive-minimum", alias = "exclusive_minimum")]
    pub exclusive_minimum: Option<Number>,

    /// The inclusive upper limit for a numeric node.
    pub maximum: Option<Number>,

    /// The exclusive upper limit for a numeric node.
    #[serde(alias = "exclusive-maximum", alias = "exclusive_maximum")]
    pub exclusive_maximum: Option<Number>,

    /// A number that a numeric node must be a multiple of.
    #[serde(alias = "multiple-of", alias = "multiple_of")]
    pub multiple_of: Option<Number>,
}

impl NumberValidator {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

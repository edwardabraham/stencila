// Generated file; do not edit. See `schema-gen` crate.

use crate::prelude::*;

use super::image_object::ImageObject;
use super::property_value_or_string::PropertyValueOrString;
use super::string::String;
use super::text::Text;

/// A word, name, acronym, phrase, etc. with a formal definition.
#[skip_serializing_none]
#[serde_as]
#[derive(Debug, SmartDefault, Clone, PartialEq, Serialize, Deserialize, StripNode, HtmlCodec, JatsCodec, MarkdownCodec, TextCodec, WriteNode, ReadNode)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
#[derive(derive_more::Display)]
#[display(fmt = "DefinedTerm")]
pub struct DefinedTerm {
    /// The type of this item.
    pub r#type: MustBe!("DefinedTerm"),

    /// The identifier for this item.
    #[strip(metadata)]
    #[html(attr = "id")]
    pub id: Option<String>,

    /// The name of the item.
    #[strip(metadata)]
    pub name: String,

    /// Non-core optional fields
    #[serde(flatten)]
    #[html(flatten)]
    #[jats(flatten)]
    #[markdown(flatten)]
    pub options: Box<DefinedTermOptions>,
}

#[skip_serializing_none]
#[serde_as]
#[derive(Debug, SmartDefault, Clone, PartialEq, Serialize, Deserialize, StripNode, HtmlCodec, JatsCodec, MarkdownCodec, TextCodec, WriteNode, ReadNode)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct DefinedTermOptions {
    /// Alternate names (aliases) for the item.
    #[serde(alias = "alternate-names", alias = "alternate_names", alias = "alternateName", alias = "alternate-name", alias = "alternate_name")]
    #[serde(default, deserialize_with = "option_csv_or_array")]
    #[strip(metadata)]
    pub alternate_names: Option<Vec<String>>,

    /// A description of the item.
    #[strip(metadata)]
    pub description: Option<Text>,

    /// Any kind of identifier for any kind of Thing.
    #[serde(alias = "identifier")]
    #[serde(default, deserialize_with = "option_one_or_many")]
    #[strip(metadata)]
    pub identifiers: Option<Vec<PropertyValueOrString>>,

    /// Images of the item.
    #[serde(alias = "image")]
    #[serde(default, deserialize_with = "option_one_or_many")]
    #[strip(metadata)]
    pub images: Option<Vec<ImageObject>>,

    /// The URL of the item.
    #[strip(metadata)]
    pub url: Option<String>,

    /// A code that identifies this DefinedTerm within a DefinedTermSet
    #[serde(alias = "term-code", alias = "term_code")]
    pub term_code: Option<String>,
}

impl DefinedTerm {
    pub fn new(name: String) -> Self {
        Self {
            name,
            ..Default::default()
        }
    }
}

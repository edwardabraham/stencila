// Generated file; do not edit. See `schema-gen` crate.

use crate::prelude::*;

use super::cord::Cord;
use super::string::String;

/// Textual content.
#[skip_serializing_none]
#[serde_as]
#[derive(Debug, SmartDefault, Clone, PartialEq, Serialize, Deserialize, StripNode, HtmlCodec, JatsCodec, MarkdownCodec, TextCodec, WriteNode, ReadNode)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
#[cfg_attr(feature = "proptest", derive(Arbitrary))]
#[derive(derive_more::Display)]
#[display(fmt = "Text")]
#[html(elem = "span")]
#[jats(special)]
#[markdown(format = "{value}")]
pub struct Text {
    /// The type of this item.
    #[cfg_attr(feature = "proptest", proptest(value = "Default::default()"))]
    pub r#type: MustBe!("Text"),

    /// The identifier for this item.
    #[strip(metadata)]
    #[cfg_attr(feature = "proptest", proptest(value = "None"))]
    #[html(attr = "id")]
    pub id: Option<String>,

    /// The value of the text content
    #[cfg_attr(feature = "proptest-min", proptest(value = r#"Cord::new("text")"#))]
    #[cfg_attr(feature = "proptest-low", proptest(strategy = r#"r"[a-zA-Z0-9]{1,10}".prop_map(Cord::new)"#))]
    #[cfg_attr(feature = "proptest-high", proptest(strategy = r#"r"[a-zA-Z0-9 \t\-_.!?*+-/()'<>=]{1,100}".prop_map(Cord::new)"#))]
    #[cfg_attr(feature = "proptest-max", proptest(strategy = r#"String::arbitrary().prop_map(Cord::new)"#))]
    #[html(content)]
    pub value: Cord,
}

impl Text {
    pub fn new(value: Cord) -> Self {
        Self {
            value,
            ..Default::default()
        }
    }
}

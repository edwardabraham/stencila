// Generated file; do not edit. See `schema-gen` crate.

use crate::prelude::*;

use super::list_item::ListItem;
use super::list_order::ListOrder;
use super::string::String;

/// A list of items.
#[skip_serializing_none]
#[serde_as]
#[derive(Debug, SmartDefault, Clone, PartialEq, Serialize, Deserialize, StripNode, HtmlCodec, JatsCodec, MarkdownCodec, TextCodec, WriteNode, ReadNode)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
#[cfg_attr(feature = "proptest", derive(Arbitrary))]
#[derive(derive_more::Display)]
#[display(fmt = "List")]
#[html(special)]
#[jats(elem = "list")]
#[markdown(special)]
pub struct List {
    /// The type of this item.
    #[cfg_attr(feature = "proptest", proptest(value = "Default::default()"))]
    pub r#type: MustBe!("List"),

    /// The identifier for this item.
    #[strip(metadata)]
    #[cfg_attr(feature = "proptest", proptest(value = "None"))]
    #[html(attr = "id")]
    pub id: Option<String>,

    /// The items in the list.
    #[serde(alias = "item")]
    #[serde(deserialize_with = "one_or_many")]
    #[cfg_attr(feature = "proptest-min", proptest(strategy = r#"vec(ListItem::arbitrary(), size_range(1..=1))"#))]
    #[cfg_attr(feature = "proptest-low", proptest(strategy = r#"vec(ListItem::arbitrary(), size_range(1..=2))"#))]
    #[cfg_attr(feature = "proptest-high", proptest(strategy = r#"vec(ListItem::arbitrary(), size_range(1..=4))"#))]
    #[cfg_attr(feature = "proptest-max", proptest(strategy = r#"vec(ListItem::arbitrary(), size_range(1..=8))"#))]
    #[jats(content)]
    pub items: Vec<ListItem>,

    /// The ordering of the list.
    #[cfg_attr(feature = "proptest-min", proptest(value = r#"ListOrder::Unordered"#))]
    #[cfg_attr(feature = "proptest-low", proptest(strategy = r#"prop_oneof![Just(ListOrder::Unordered),Just(ListOrder::Ascending)]"#))]
    #[cfg_attr(feature = "proptest-high", proptest(strategy = r#"ListOrder::arbitrary()"#))]
    #[cfg_attr(feature = "proptest-max", proptest(strategy = r#"ListOrder::arbitrary()"#))]
    #[jats(attr = "list-type")]
    pub order: ListOrder,
}

impl List {
    pub fn new(items: Vec<ListItem>, order: ListOrder) -> Self {
        Self {
            items,
            order,
            ..Default::default()
        }
    }
}

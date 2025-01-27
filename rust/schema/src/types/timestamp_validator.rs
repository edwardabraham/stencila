// Generated file; do not edit. See `schema-gen` crate.

use crate::prelude::*;

use super::string::String;
use super::time_unit::TimeUnit;
use super::timestamp::Timestamp;

/// A validator specifying the constraints on a timestamp.
#[skip_serializing_none]
#[serde_as]
#[derive(Debug, SmartDefault, Clone, PartialEq, Serialize, Deserialize, StripNode, HtmlCodec, JatsCodec, MarkdownCodec, TextCodec, WriteNode, ReadNode)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
#[derive(derive_more::Display)]
#[display(fmt = "TimestampValidator")]
pub struct TimestampValidator {
    /// The type of this item.
    pub r#type: MustBe!("TimestampValidator"),

    /// The identifier for this item.
    #[strip(metadata)]
    #[html(attr = "id")]
    pub id: Option<String>,

    /// The time units that the timestamp can have.
    #[serde(alias = "time-units", alias = "time_units", alias = "timeUnit", alias = "time-unit", alias = "time_unit")]
    #[serde(default, deserialize_with = "option_one_or_many")]
    pub time_units: Option<Vec<TimeUnit>>,

    /// The inclusive lower limit for a timestamp.
    pub minimum: Option<Timestamp>,

    /// The inclusive upper limit for a timestamp.
    pub maximum: Option<Timestamp>,
}

impl TimestampValidator {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

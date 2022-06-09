///! A module for functions and configuration related to telemetry.
///!
///! Currently this module just has Sentry related settings but
///! in the future may also include functionality around OpenTelemetry
///! or similar for monitoring of worker processes.

pub mod config {
    use common::{
        defaults::Defaults,
        serde::{Deserialize, Serialize},
    };
    use schemars::JsonSchema;
    use validator::Validate;

    /// Telemetry settings for Stencila CLI
    #[derive(Debug, Defaults, PartialEq, Clone, JsonSchema, Deserialize, Serialize, Validate)]
    #[serde(default, crate = "common::serde")]
    #[schemars(deny_unknown_fields)]
    pub struct TelemetryCliConfig {
        /// Whether to send error reports. Default is false.
        #[def = "false"]
        pub error_reports: bool,
    }

    /// Telemetry settings for Stencila Desktop
    #[derive(Debug, Defaults, PartialEq, Clone, JsonSchema, Deserialize, Serialize, Validate)]
    #[serde(default, crate = "common::serde")]
    #[schemars(deny_unknown_fields)]
    pub struct TelemetryDesktopConfig {
        /// Whether to send error reports. Default is false.
        #[def = "false"]
        pub error_reports: bool,
    }

    /// Telemetry
    ///
    /// Configuration settings for telemetry
    #[derive(Debug, Default, PartialEq, Clone, JsonSchema, Deserialize, Serialize, Validate)]
    #[serde(default, crate = "common::serde")]
    #[schemars(deny_unknown_fields)]
    pub struct TelemetryConfig {
        pub cli: TelemetryCliConfig,
        pub desktop: TelemetryDesktopConfig,
    }
}

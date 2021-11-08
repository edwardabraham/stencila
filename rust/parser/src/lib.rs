use eyre::Result;
use graph_triples::Pairs;
use serde::{Deserialize, Serialize};
use std::path::Path;

// Export and re-export for the convenience of crates that implement a parser
pub mod utils;
pub use eyre;
pub use graph_triples;

/// A specification for parsers
///
/// All parsers, including those implemented in plugins, should provide this
/// specification. Rust implementations return a `Parser` instance from the
/// `spec` function of `ParserTrait`. Plugins provide a JSON or YAML serialization
/// as part of their manifest.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Parser {
    /// The language that the parser parses
    pub language: String,
}

/// A trait for parsers
///
/// This trait can be used by Rust implementations of parsers, allowing them to
/// be compiled into the Stencila binaries.
///
/// It defines similar functions to `serde_json` (and other `serde_` crates) for
/// converting nodes to/from strings, files, readers etc.
pub trait ParserTrait {
    /// Get the [`Parser`] specification
    fn spec() -> Parser;

    /// Parse some code and return a set of graph pairs
    fn parse(path: &Path, code: &str) -> Result<Pairs>;
}

/// Parsing options
///
/// Parsing functions (including those in plugins) are encouraged to respect these options
/// but are not required to.
#[derive(Clone)]
pub struct ParseOptions {}

impl Default for ParseOptions {
    fn default() -> Self {
        Self {}
    }
}
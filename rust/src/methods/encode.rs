use crate::nodes::Node;
use eyre::Result;

// Allow these for when no features are enabled
#[allow(unused_variables, unreachable_code)]
pub fn encode(node: Node, format: &str) -> Result<String> {
    let content = match format {
        #[cfg(feature = "format-json")]
        "json" => serde_json::to_string(&node)?,
        #[cfg(feature = "format-yaml")]
        "yaml" => serde_yaml::to_string(&node)?,
        _ => {
            #[cfg(feature = "request")]
            {
                let node = super::delegate::delegate(
                    super::Method::Encode,
                    serde_json::json!({
                        "node": node,
                        "format": format,
                    }),
                )?;
                // Delegate returns a node so always convert it to a string
                return Ok(node.to_string());
            };

            #[cfg(not(feature = "request"))]
            eyre::bail!("Unable to encode a node to format \"{}\"", format)
        }
    };
    Ok(content)
}

#[cfg(any(feature = "request", feature = "serve"))]
pub mod rpc {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Params {
        pub node: Node,

        pub format: Option<String>,
    }

    pub fn encode(params: Params) -> Result<String> {
        let Params { node, format } = params;
        super::encode(node, &format.unwrap_or_else(|| "json".to_string()))
    }
}
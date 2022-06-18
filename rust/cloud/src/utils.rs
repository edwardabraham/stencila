use std::{env, fs::read_to_string, path::PathBuf};

use common::{
    dirs,
    eyre::{bail, Result},
    serde_json,
};

use crate::types::ApiToken;

/// The base URL for Stencila Cloud
pub(crate) const BASE_URL: &str = if cfg!(debug_assertions) {
    "http://localhost:3000/api/v1"
} else {
    "https://stencila.fly.dev/api/v1"
};

/// Get the path used to store `token.json`, `user.json`, and other files
/// associated with this crate
pub(crate) fn config_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| env::current_dir().unwrap())
        .join("stencila")
}

/// Get the path of `token.json`
pub(crate) fn token_path() -> PathBuf {
    config_dir().join("token.json")
}

/// Read the current Stencila access token
pub(crate) fn token_read() -> Result<String> {
    let path = token_path();
    if path.exists() {
        let json = read_to_string(token_path())?;
        let token: ApiToken = serde_json::from_str(&json)?;
        Ok(token.token)
    } else {
        bail!("You are not logged in; try using `stencila login` first");
    }
}

/// Get the path of `user.json`
pub(crate) fn user_path() -> PathBuf {
    config_dir().join("user.json")
}

use std::net::IpAddr;
use std::path::{Path, PathBuf};

use serde::Deserialize;
use thiserror::Error;
use url::Url;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Read error: {0}")]
    ReadError(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    ParseError(#[from] toml::de::Error),
}

#[derive(Deserialize)]
pub struct InstanceConfigSection {
    pub name: String,
    pub location: String,

    #[serde(default = "bool::default")]
    pub hidden: bool,

    #[serde(default = "bool::default")]
    pub randomize_secret: bool,
}

#[derive(Deserialize)]
pub struct ServerConfigSection {
    pub address: Option<IpAddr>,
    pub port: Option<u16>,

    pub public_url: Option<Url>,

    pub serve_ui: Option<bool>,
    pub static_files: Option<PathBuf>,
    pub index_file: Option<String>,
}

#[derive(Deserialize)]
pub struct ServerConfigFile {
    pub server: Option<ServerConfigSection>,
    pub buckets: Vec<InstanceConfigSection>,
}

impl ServerConfigFile {
    pub async fn from_file(path: &Path) -> Result<Self, ConfigError> {
        let toml = tokio::fs::read_to_string(path).await?;
        Ok(toml::de::from_str(toml.as_str())?)
    }
}

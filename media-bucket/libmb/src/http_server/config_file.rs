use std::net::IpAddr;
use std::path::Path;

use serde::Deserialize;
use thiserror::Error;

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
}

#[derive(Deserialize)]
pub struct ServerConfigSection {
    pub address: Option<IpAddr>,
    pub port: Option<u16>,
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

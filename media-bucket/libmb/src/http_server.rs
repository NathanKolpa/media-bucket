use std::net::{IpAddr, Ipv4Addr};
use std::path::Path;
use std::sync::Arc;

use actix_cors::Cors;
use actix_web::middleware::NormalizePath;
use actix_web::{web, App, HttpServer};

pub use config_file::ConfigError;

use crate::http_server::instance::{InstanceDataSource, ServerBucketInstance};
use crate::http_server::routes::routes;

mod config_file;
mod instance;
mod middleware;
mod routes;
mod stream_file;
mod web_error;

struct InstanceConfig {
    id: u64,
    name: String,
    location: String,
}

#[derive(Default)]
pub struct ServerConfig {
    address: Option<IpAddr>,
    port: Option<u16>,
    instances: Vec<InstanceConfig>,
}

impl ServerConfig {
    pub async fn from_file(path: &Path) -> Result<Self, ConfigError> {
        let config = config_file::ServerConfigFile::from_file(path).await?;

        Ok(Self {
            port: config.server.as_ref().and_then(|s| s.port),
            address: config.server.as_ref().and_then(|a| a.address),
            instances: config
                .buckets
                .into_iter()
                .enumerate()
                .map(|(i, instance)| InstanceConfig {
                    id: (i + 1) as u64,
                    location: instance.location,
                    name: instance.name,
                })
                .collect(),
        })
    }

    pub fn address(&mut self, addr: IpAddr) {
        self.address = Some(addr);
    }

    pub fn port(&mut self, port: u16) {
        self.port = Some(port);
    }

    fn get_address(&self) -> IpAddr {
        self.address
            .unwrap_or(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)))
    }

    fn get_port(&self) -> u16 {
        self.port.unwrap_or(3434)
    }

    async fn load_instance_data_source(&self) -> std::io::Result<InstanceDataSource> {
        let mut instances = Vec::with_capacity(self.instances.len());

        for instance_config in self.instances.iter() {
            instances.push(Arc::new(
                ServerBucketInstance::load(
                    instance_config.id,
                    instance_config.name.clone(),
                    instance_config.location.clone(),
                )
                .await?,
            ))
        }

        Ok(InstanceDataSource::new(instances))
    }
}

pub async fn start_server(config: ServerConfig) -> std::io::Result<()> {
    let instance_data_source = web::Data::new(config.load_instance_data_source().await?);

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .app_data(instance_data_source.clone())
            .wrap(NormalizePath::trim())
            .wrap(cors)
            .service(routes())
    })
    .bind((config.get_address(), config.get_port()))?
    .run()
    .await
}
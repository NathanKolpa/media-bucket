use std::net::{IpAddr, Ipv4Addr};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use actix_cors::Cors;
use actix_web::middleware::NormalizePath;
use actix_web::{web, App, HttpServer};

use chrono::{Duration, Utc};
pub use config_file::ConfigError;
use log::info;
use tokio::{select, time::sleep};
use url::Url;

use crate::http_server::instance::{InstanceDataSource, ServerBucketInstance};
use crate::http_server::routes::{routes, routes_with_static};

mod config_file;
mod instance;
mod middleware;
mod routes;
mod stream_file;
mod stream_playlist;
mod token;
mod web_error;

struct InstanceConfig {
    id: u64,
    name: String,
    location: String,
    hidden: bool,
    randomize_secret: bool,
    session_lifetime: Duration,
}

struct StaticFilesConfig {
    file_root: Option<PathBuf>,
    index_file: Option<String>,
}

impl StaticFilesConfig {
    pub fn file_root(&self) -> &Path {
        self.file_root
            .as_deref()
            .unwrap_or_else(|| Path::new("/var/www/html"))
    }
    pub fn index_file(&self) -> &str {
        self.index_file.as_deref().unwrap_or("index.html")
    }
}

#[derive(Default)]
pub struct ServerConfig {
    address: Option<IpAddr>,
    port: Option<u16>,
    instances: Vec<InstanceConfig>,

    public_url: Option<Url>,

    static_files: Option<StaticFilesConfig>,
}

impl ServerConfig {
    pub async fn from_file(path: &Path) -> Result<Self, ConfigError> {
        let config = config_file::ServerConfigFile::from_file(path).await?;

        Ok(Self {
            port: config.server.as_ref().and_then(|s| s.port),
            address: config.server.as_ref().and_then(|a| a.address),
            public_url: config.server.as_ref().and_then(|a| a.public_url.clone()),
            static_files: config.server.as_ref().and_then(|s| {
                if !s.serve_ui.unwrap_or(false) {
                    None
                } else {
                    Some(StaticFilesConfig {
                        index_file: s.index_file.clone(),
                        file_root: s.static_files.clone(),
                    })
                }
            }),
            instances: config
                .buckets
                .into_iter()
                .enumerate()
                .map(|(i, instance)| InstanceConfig {
                    id: (i + 1) as u64,
                    location: instance.location,
                    name: instance.name,
                    hidden: instance.hidden,
                    randomize_secret: instance.randomize_secret,
                    session_lifetime: instance
                        .session_lifetime
                        .map(|seconds| Duration::seconds(seconds as i64))
                        .unwrap_or(Duration::days(14)),
                })
                .collect(),
        })
    }

    pub fn static_files(&mut self, file_root: Option<PathBuf>, index_file: Option<String>) {
        self.static_files = Some(StaticFilesConfig {
            file_root,
            index_file,
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
        let mut base_url = self.public_url.clone();

        if self.static_files.is_some() {
            base_url = base_url.map(|url| url.join("api/").unwrap());
        }

        let base_url = base_url.map(Arc::new);

        for instance_config in self.instances.iter() {
            instances.push(Arc::new(
                ServerBucketInstance::load(
                    instance_config.id,
                    instance_config.name.clone(),
                    base_url.clone(),
                    instance_config.location.clone(),
                    instance_config.hidden,
                    instance_config.randomize_secret,
                    instance_config.session_lifetime,
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

    let config = Arc::new(config);

    let factory_config = config.clone();

    let unload_future = watch_bucket_unload(instance_data_source.clone());

    let server_future = HttpServer::new(move || {
        let app_routes = if let Some(files) = &factory_config.static_files {
            routes_with_static(
                files.file_root().to_path_buf(),
                files.index_file().to_string(),
            )
        } else {
            routes()
        };

        let mut cors = Cors::default();

        // use a lax cors policy in debug mode
        if cfg!(debug_assertions) {
            cors = cors
                .allow_any_origin()
                .allow_any_method()
                .allow_any_header()
                .supports_credentials()
                .max_age(3600);
        }

        App::new()
            .app_data(instance_data_source.clone())
            .wrap(NormalizePath::trim())
            .wrap(cors)
            .service(app_routes)
    })
    .bind((config.get_address(), config.get_port()))?
    .run();

    select! {
        result = server_future => {
            result
        },
        _ = unload_future => {
            Ok(())
        }
    }
}

async fn watch_bucket_unload(instances: web::Data<InstanceDataSource>) {
    loop {
        sleep(std::time::Duration::from_secs(20)).await;
        let now = Utc::now();

        for instance in instances.all() {
            let Some(unload_time) = instance.should_unload_at() else {
                continue;
            };

            if now >= unload_time {
                info!("Unloading instance {instance}");
                instance.unload();
            }
        }
    }
}

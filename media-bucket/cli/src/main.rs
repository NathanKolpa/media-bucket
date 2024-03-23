use std::fmt::Debug;
use std::sync::atomic::AtomicUsize;
use std::{net::IpAddr, sync::atomic::Ordering};
use std::{path::PathBuf, process::ExitCode};

use clap::{Parser, Subcommand, ValueEnum};
use libmb::{
    data_source::DataSourceError, http_server::ConfigError, model::Post, Bucket, BucketError,
    SyncMatchStategy,
};

use libmb::http_server::{start_server, ServerConfig};

enum CliError {
    ConfigError(ConfigError),
    StartServerError(std::io::Error),
    CreateDirectoryError(std::io::Error),
    PasswordsDoNotMatch,
    CreateBucketError(BucketError),

    OpenError(BucketError),
    GcError(DataSourceError),
    OpenSourceError(BucketError),
    OpenDestError(BucketError),
    SyncBucketError(BucketError),
}

impl Debug for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CliError::ConfigError(err) => match err {
                ConfigError::ReadError(e) => write!(f, "Error while opening config file: {e}"),
                ConfigError::ParseError(e) => {
                    write!(
                        f,
                        "Cannot read config file because it contains syntax errors: {e}"
                    )
                }
            },
            CliError::StartServerError(err) => write!(f, "Error while starting HTTP server: {err}"),
            CliError::CreateDirectoryError(err) => {
                write!(f, "Error while creating bucket directory: {err}")
            }
            CliError::PasswordsDoNotMatch => write!(f, "Passwords do not match"),
            CliError::CreateBucketError(err) => {
                write!(f, "Error while creating bucket: {err:?}")
            }
            CliError::OpenSourceError(err) => {
                write!(f, "Error while opening source bucket: {err:?}")
            }
            CliError::OpenDestError(err) => {
                write!(f, "Error while opening destination bucket: {err:?}")
            }
            CliError::SyncBucketError(err) => write!(f, "Error while syncing: {err:?}"),
            CliError::OpenError(err) => write!(f, "Error while opening bucket: {err:?}"),
            CliError::GcError(err) => write!(f, "Error while running garbage collection: {err:?}"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum CliSyncMatchStrat {
    /// Sync all posts regardless if any post already exists on the dest bucket.
    /// This will effectivly copy all posts.
    None,

    /// Sync all posts based on the url as unique identifier. Posts without url will be ignored.
    Url,
}

impl Into<SyncMatchStategy> for CliSyncMatchStrat {
    fn into(self) -> SyncMatchStategy {
        match self {
            CliSyncMatchStrat::None => SyncMatchStategy::None,
            CliSyncMatchStrat::Url => SyncMatchStategy::Url,
        }
    }
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new bucket.
    Init {
        /// The file path where to create the new bucket.
        #[clap(value_parser, value_name = "PATH")]
        path: PathBuf,
    },

    /// Run garbage collection on a bucket
    Gc {
        /// The bucket location.
        #[clap(value_parser, value_name = "LOCAITON")]
        location: String,
    },

    /// Move data from one bucket across another in bulk.
    Sync {
        /// The bucket location where to copy from.
        #[clap(value_parser, value_name = "SOURCE")]
        source: String,

        /// The bucket location where to write to.
        #[clap(value_parser, value_name = "DESTINATION")]
        destination: String,

        /// Specify if the posts should be removed from the source bucket when sucessfully synced.
        #[clap(value_parser, short, long, default_value_t = false)]
        remove: bool,

        /// Specify how posts should be matched across the source and destination.
        #[clap(value_parser, short = 'm', long = "match")]
        strategy: CliSyncMatchStrat,
    },

    /// Start the REST API.
    Server {
        /// The path to the configuration file.
        #[clap(
            value_parser,
            short,
            long,
            value_name = "config",
            default_value = "/etc/media-bucket/config.toml"
        )]
        config: PathBuf,

        /// The address to bind to.
        /// Overrides the value in the config file.
        #[clap(value_parser, short, long, default_value = None)]
        address: Option<IpAddr>,

        /// The port number.
        /// Overrides the value in the config file.
        #[clap(value_parser, short, long, default_value = None)]
        port: Option<u16>,

        /// Serve static files and mount the api at "/api"
        #[clap(value_parser, short, long, default_value_t = false)]
        ui: bool,

        /// The location of the static files.
        /// The "ui" flag must be set for this to have effect.
        #[clap(value_parser, long, default_value = None)]
        static_files: Option<PathBuf>,

        /// The name of the default file to serve
        /// The "ui" flag must be set for this to have effect.
        #[clap(value_parser, long, default_value = None)]
        index_file: Option<String>,
    },
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

async fn run(cli: Cli) -> Result<(), CliError> {
    match cli.command {
        Commands::Server {
            config,
            address,
            port,
            ui,
            static_files,
            index_file,
        } => {
            let mut config = ServerConfig::from_file(&config)
                .await
                .map_err(CliError::ConfigError)?;

            if let Some(addr) = address {
                config.address(addr);
            }

            if let Some(port) = port {
                config.port(port);
            }

            if ui {
                config.static_files(static_files, index_file);
            }

            start_server(config)
                .await
                .map_err(CliError::StartServerError)?;
        }
        Commands::Init { path } => {
            if !path.exists() {
                tokio::fs::create_dir(&path)
                    .await
                    .map_err(CliError::CreateDirectoryError)?;
            }

            let password = rpassword::prompt_password("Enter your password: ").unwrap();
            if rpassword::prompt_password("Enter your password again: ").unwrap() != password {
                return Err(CliError::PasswordsDoNotMatch);
            }

            Bucket::create_encrypted(&path, &password)
                .await
                .map_err(CliError::CreateBucketError)?;

            println!("Successfully created bucket at {}", path.display());
        }
        Commands::Sync {
            source,
            destination,
            remove,
            strategy,
        } => {
            let src = open_bucket(
                None,
                &source,
                Some(&format!("Enter your password for \"{source}\": ")),
            )
            .await
            .map_err(CliError::OpenSourceError)?;

            let dest = open_bucket(
                None,
                &destination,
                Some(&format!("Enter your password for \"{destination}\": ")),
            )
            .await
            .map_err(CliError::OpenDestError)?;

            let total = AtomicUsize::new(0);
            let on_sync = |post: &Post| {
                println!("Created post id: {}", post.id);
                total.fetch_add(1, Ordering::SeqCst);
            };

            dest.sync_from(&src, strategy.into(), remove, &on_sync)
                .await
                .map_err(CliError::SyncBucketError)?;

            println!("Synced a total of {} post(s)", total.load(Ordering::SeqCst));
        }
        Commands::Gc { location } => {
            let bucket = open_bucket(None, &location, None)
                .await
                .map_err(CliError::OpenError)?;

            let rows_affected = bucket
                .data_source()
                .cross()
                .gc()
                .await
                .map_err(CliError::GcError)?;

            println!("{rows_affected} row(s) affected");
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> ExitCode {
    let cli: Cli = Cli::parse();

    match run(cli).await {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err:?}");
            ExitCode::FAILURE
        }
    }
}

async fn open_bucket(
    password: Option<String>,
    location: &str,
    prompt_override: Option<&str>,
) -> Result<Bucket, BucketError> {
    let password = if Bucket::password_protected(location)
        .await
        .map_err(|_| BucketError::InvalidLocation)?
    {
        password.or_else(|| {
            Some(
                rpassword::prompt_password(prompt_override.unwrap_or("Enter your password: "))
                    .unwrap(),
            )
        })
    } else {
        None
    };

    Ok(Bucket::open(location, password.as_deref()).await?)
}

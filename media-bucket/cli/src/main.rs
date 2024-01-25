use std::error::Error;
use std::net::IpAddr;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use libmb::Bucket;

use libmb::http_server::{start_server, ServerConfig};

#[derive(Subcommand)]
enum Commands {
    Init {
        /// The path where to create the new bucket.
        #[clap(value_parser, value_name = "PATH")]
        path: PathBuf,

        #[clap(value_parser, short, long)]
        password: Option<String>,
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

    Open {
        location: String,

        #[clap(value_parser, short, long, default_value = None)]
        password: Option<String>,
    },
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli: Cli = Cli::parse();

    match cli.command {
        Commands::Server {
            config,
            address,
            port,
            ui,
            static_files,
            index_file,
        } => {
            let mut config = ServerConfig::from_file(&config).await?;

            if let Some(addr) = address {
                config.address(addr);
            }

            if let Some(port) = port {
                config.port(port);
            }

            if ui {
                config.static_files(static_files, index_file);
            }

            start_server(config).await?;
        }
        Commands::Init { path, password } => {
            if !path.exists() {
                tokio::fs::create_dir(&path).await?;
            }

            let should_verify_password = password.is_none();
            let password = password
                .unwrap_or_else(|| rpassword::prompt_password("Enter your password: ").unwrap());

            if should_verify_password
                && rpassword::prompt_password("Enter your password again: ").unwrap() != password
            {
                return Err("Passwords do not match".into());
            }

            Bucket::create_encrypted(&path, &password).await?;

            println!("Successfully created bucket at {}", path.display());
        }
        Commands::Open { location, password } => {
            let _bucket = Bucket::open(&location, password.as_deref()).await?;
        }
    }

    Ok(())
}

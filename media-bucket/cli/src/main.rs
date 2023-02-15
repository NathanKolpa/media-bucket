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
        password: String,
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
    },

    Open {
        location: String,

        #[clap(value_parser, short, long, default_value = None)]
        password: Option<String>
    }
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
        } => {
            let mut config = ServerConfig::from_file(&config).await?;

            if let Some(addr) = address {
                config.address(addr);
            }

            if let Some(port) = port {
                config.port(port);
            }

            start_server(config).await?;
        }
        Commands::Init { path, password } => {
            if !path.exists() {
                tokio::fs::create_dir(&path).await?;
            }

            Bucket::create_encrypted(&path, &password).await?;
        },
        Commands::Open { location, password } => {
            let _bucket = Bucket::open(&location, password.as_deref())
                .await?;
        }
    }

    Ok(())
}

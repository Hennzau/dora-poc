use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    about = env!("CARGO_PKG_DESCRIPTION"),
)]

struct NarrCLI {
    #[command(subcommand)]
    command: NarrCommands,
}

#[derive(Subcommand)]
enum NarrCommands {
    #[command(
        about = "Open a new daemon that will listen for incoming connections on specified interface."
    )]
    Open {
        #[arg(value_name = "Daemon ID", required = true)]
        daemon_id: String,

        #[arg(
            value_name = "Interface to listen on (default: udp/0.0.0.0:0)",
            default_value = "udp/0.0.0.0:0"
        )]
        listen: Vec<String>,
    },

    #[command(
        about = "Start a new application. This will read the Cargo.toml workspace, start a Daemon, connect to remote Daemons, distribute the application, and start the application."
    )]
    Start {
        #[arg(value_name = "Path to Cargo.toml", default_value = "Cargo.toml")]
        manifest_path: PathBuf,
    },

    #[command(about = "Distribute the application to remote Daemons.")]
    Distribute {
        #[arg(value_name = "Path to Cargo.toml", default_value = "Cargo.toml")]
        manifest_path: PathBuf,
    },
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let cli = NarrCLI::parse();

    match cli.command {
        NarrCommands::Open { daemon_id, listen } => {
            let daemon =
                narr_rs::prelude::Daemon::new_without_application(daemon_id, listen).await?;

            daemon.run().await?;
        }
        NarrCommands::Start { manifest_path } => {
            let application =
                narr_rs::prelude::read_toml_and_parse_to_application(manifest_path).await?;

            let daemon_id = format!("DAEMON_{}", application.id);

            let daemon =
                narr_rs::prelude::Daemon::new_with_application(daemon_id, application).await?;

            daemon.run().await?;
        }
        NarrCommands::Distribute { manifest_path } => {
            let application =
                narr_rs::prelude::read_toml_and_parse_to_application(manifest_path).await?;

            let daemon_id = format!("DAEMON_{}", application.id);

            let daemon =
                narr_rs::prelude::Daemon::new_with_application(daemon_id, application).await?;

            daemon.distribute().await?;
        }
    }

    Ok(())
}

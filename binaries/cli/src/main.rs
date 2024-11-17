use narr_cli::{daemon_check, daemon_list, daemon_spawn};
use narr_rs::prelude::DaemonAddress;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    about = env!("CARGO_PKG_DESCRIPTION"),
)]

struct CLI {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Interact with a daemon.")]
    Daemon {
        #[command(subcommand)]
        command: DaemonCommands,
    },
}

#[derive(Subcommand)]
enum DaemonCommands {
    #[command(
        about = "Start a new daemon that will listen for incoming connections on specified interface and connect to a network of daemons."
    )]
    Start {
        #[arg(value_name = "Daemon ID", required = true)]
        id: String,

        #[arg(
            value_name = "Listen Address: this is the address you will use to connect to this daemon.",
            required = true,
            long = "listen"
        )]
        listen: Vec<String>,

        #[arg(
            value_name = "Connect Address: pass the address of the daemon/router of the network you want to connect to.",
            required = false,
            long = "connect"
        )]
        connect: Vec<String>,
    },

    #[command(about = "Check if a daemon is running.")]
    Check {
        #[arg(value_name = "Daemon ID", required = true)]
        id: String,

        #[arg(
            value_name = "Connect Address: pass the address of the daemon/router of the network you want to connect to.",
            required = true,
            long = "connect"
        )]
        connect: String,
    },

    #[command(about = "List all running daemons.")]
    List {
        #[arg(
            value_name = "Connect Address: pass the address of the daemon/router of the network you want to connect to.",
            required = true,
            long = "connect"
        )]
        connect: String,
    },
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .parse("")?,
        )
        .init();

    let cli = CLI::parse();

    match cli.command {
        Commands::Daemon { command } => match command {
            DaemonCommands::Start {
                id,
                listen,
                connect,
            } => {
                let listen = listen
                    .iter()
                    .map(|l| DaemonAddress::from_string(l.to_string()))
                    .collect::<Result<Vec<_>, _>>()?;

                let connect = connect
                    .iter()
                    .map(|c| DaemonAddress::from_string(c.to_string()))
                    .collect::<Result<Vec<_>, _>>()?;

                daemon_spawn(id, listen, connect).await?;
            }
            DaemonCommands::Check { id, connect } => {
                let connect = DaemonAddress::from_string(connect)?;

                daemon_check(id, connect).await?;
            }
            DaemonCommands::List { connect } => {
                let connect = DaemonAddress::from_string(connect)?;

                daemon_list(connect).await?;
            }
        },
    }

    Ok(())
}

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
        #[arg(
            value_name = "Interface to listen on (default: 0.0.0.0:0)",
            default_value = "0.0.0.0:0"
        )]
        listen: Vec<String>,
    },

    #[command(
        about = "Start a new application. This will read the Cargo.toml workspace, start a Daemon, connect to remote Daemons, distribute the application, and start the application."
    )]
    Start {
        #[arg(value_name = "Path to Cargo.toml", default_value = "Cargo.toml")]
        manifest_path: Option<String>,
    },

    #[command(about = "Distribute the application to remote Daemons.")]
    Distribute {
        #[arg(value_name = "Path to Cargo.toml", default_value = "Cargo.toml")]
        manifest_path: Option<String>,
    },
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let cli = NarrCLI::parse();

    match cli.command {
        NarrCommands::Open { listen } => {
            println!("Opening daemon on {:?}", listen);
        }
        NarrCommands::Start { manifest_path } => {
            println!("Starting application with manifest {:?}", manifest_path);
        }
        NarrCommands::Distribute { manifest_path } => {
            println!("Distributing application with manifest {:?}", manifest_path);
        }
    }

    Ok(())
}

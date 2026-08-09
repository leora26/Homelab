mod commands;
mod helpers;
mod output;
mod storage_client;

use crate::commands::ResizeCommand;
use crate::helpers::parse_size;
use crate::output::{print_volume_status_human, print_volume_json, print_resize};
use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use storage_client::Client;

#[derive(Parser)]
#[command(name = "pvk", version, about = "Pavuk admin CLI")]
struct Cli {
    /// Address of the admin-console gRPC server
    #[arg(
        long,
        env = "PVK_ADDR",
        default_value = "http://[::1]:50053",
        global = true
    )]
    server: String,

    #[command(subcommand)]
    command: Command,

    /// Output format for results
    #[arg(long, short = 'o', value_enum, default_value = "human", global = true)]
    output: OutputFormat,
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum)]
enum OutputFormat {
    Human,
    Json,
}

#[derive(Subcommand)]
enum Command {
    /// Manage the NAS storage volume (ZFS-backed)
    Volume {
        #[command(subcommand)]
        action: VolumeAction,
    },
}

#[derive(Subcommand)]
enum VolumeAction {
    /// Show the volume's current size, usage, and free space
    Status,

    /// Grow or shrink the volume's reserved size
    Resize {
        /// New size with an optional unit suffix (e.g. 10G, 512M, 2T)
        size: String,

        /// Allow shrinking the volume below its current size
        #[arg(long)]
        force: bool,
    },
}

#[tokio::main]
async fn main() -> std::process::ExitCode {
    let cli = Cli::parse();
    match run(cli).await {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e:#}");
            std::process::ExitCode::FAILURE
        }
    }
}

async fn run(cli: Cli) -> Result<()> {
    let format = cli.output;
    let client = Client::connect(cli.server).await?;

    match cli.command {
        Command::Volume { action } => match action {
            VolumeAction::Status => {
                let status = client.get_status().await?;
                match format {
                    OutputFormat::Human => print_volume_status_human(&status),
                    OutputFormat::Json => print_volume_json(&status)?,
                }
                Ok(())
            }
            VolumeAction::Resize { size, force } => {
                let size = parse_size(&size)?;
                let command = ResizeCommand {
                    requested_bytes: size,
                    force_shrink: force,
                };

                let resize_response = client
                    .resize(command)
                    .await?;

                print_resize(&resize_response);
                Ok(())
            }
        },
    }
}

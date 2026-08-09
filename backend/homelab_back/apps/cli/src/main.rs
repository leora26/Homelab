mod storage_client;
mod output;

use anyhow::Result;
use clap::{Args, Parser, Subcommand, ValueEnum};
use storage_client::Client;
use crate::output::{print_volume_human, print_volume_json};

#[derive(Parser)]
#[command(name = "pvk", version, about = "Pavuk admin CLI")]
struct Cli {
    #[arg(long, env = "PVK_ADDR", default_value = "http://[::1]:50053", global = true)]
    server: String,

    #[command(subcommand)]
    command: Command,
    #[arg(long, short = 'o', value_enum, default_value = "human", global = true)]
    output: OutputFormat
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum)]
enum OutputFormat {
    Human,
    Json
}

#[derive(Subcommand)]
enum Command {
    Volume {
        #[command(subcommand)]
        action: VolumeAction,
    },
}

#[derive(Subcommand)]
enum VolumeAction {
    Status,
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
                let status = client.get_status()
                    .await
                    .map_err(|e| anyhow::anyhow!(e))?;
                match format {
                    OutputFormat::Human => print_volume_human(&status),
                    OutputFormat::Json => print_volume_json(&status)?,
                }
                Ok(())
            }
        },
    }
}

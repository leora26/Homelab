mod args;
mod client;
mod commands;
mod helpers;
mod output;

use crate::args::FileTypeArg;
use crate::client::config::connect;
use crate::client::file_client::FileClient;
use crate::commands::{FindFileCommand, GetVersionsCommand, ListFileCommand, ResizeCommand};
use crate::helpers::parse_size;
use crate::output::{print_file_table, print_resize, print_volume_json, print_volume_status_human};
use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use client::storage_client::StorageClient;

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
    /// Inspect the files tracked by the system (log, current state, history)
    File {
        #[command(subcommand)]
        action: FileAction,
    },
}

#[derive(Subcommand)]
enum FileAction {
    /// Show the raw file event log, newest first
    Log {
        /// Max number of records to return
        #[arg(long, short = 'm', default_value = "10")]
        max: i64,
        /// Only show files of this type
        #[arg(long = "type", value_enum)]
        file_type: Option<FileTypeArg>,
    },

    /// List the latest state of each file
    List {
        /// Max number of files to return
        #[arg(long, short = 'm', default_value = "10")]
        max: i64,
        /// Only show files of this type
        #[arg(long = "type", value_enum)]
        file_type: Option<FileTypeArg>,
    },

    /// Show details for a single file by id (or id prefix)
    Show {
        /// Full file id, or a leading prefix (e.g. the first few chars)
        id: String,
    },

    /// Show the version history of a single file (by id prefix)
    Versions {
        /// Full file id, or a leading prefix (e.g. the first few chars)
        id: String,
        /// Max number of versions to return
        #[arg(long, short = 'm', default_value = "10")]
        max: i64,
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
    let channel = connect(cli.server).await?;
    let storage_client = StorageClient::new(channel.clone());
    let file_client = FileClient::new(channel);

    match cli.command {
        Command::Volume { action } => match action {
            VolumeAction::Status => {
                let status = storage_client.get_status().await?;
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

                let resize_response = storage_client.resize(command).await?;

                print_resize(&resize_response);
                Ok(())
            }
        },
        Command::File { action } => match action {
            FileAction::Log { max, file_type } => {
                let req = ListFileCommand {
                    limit: max,
                    file_type: file_type.map(FileTypeArg::to_proto),
                };

                let res = file_client.get_log(req).await?;
                print_file_table(&res);
                Ok(())
            }
            FileAction::List { max, file_type } => {
                let req = ListFileCommand {
                    limit: max,
                    file_type: file_type.map(FileTypeArg::to_proto),
                };

                let res = file_client.get_latest(req).await?;
                print_file_table(&res);
                Ok(())
            }
            FileAction::Show { id } => {
                let req = FindFileCommand { prefix: id };

                let res = file_client.find_files(req).await?;
                print_file_table(&res);
                Ok(())
            }
            FileAction::Versions { max, id } => {
                let req = GetVersionsCommand {
                    prefix: id,
                    limit: max,
                };

                let res = file_client.get_versions(req).await?;
                print_file_table(&res);
                Ok(())
            }
        },
    }
}

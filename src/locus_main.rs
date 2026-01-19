use clap::Parser;
use std::io;

mod locus_cli;

use crate::locus_cli::LocusCommands;

#[derive(Parser)]
#[command(name = "locus")]
#[command(about = "Locus - Human TUI Interface for Engram System", long_about = None)]
#[command(version = env!("CARGO_PKG_VERSION"))]
struct Cli {
    #[command(subcommand)]
    command: LocusCommands,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli.command {
        LocusCommands::Workflow { subcommand } => {
            locus_cli::workflow::handle_workflow_command(subcommand).await
        }
        LocusCommands::Template { subcommand } => {
            locus_cli::template::handle_template_command(subcommand).await
        }
        LocusCommands::Visualize { subcommand } => {
            locus_cli::visualize::handle_visualize_command(subcommand).await
        }
        LocusCommands::Govern { subcommand } => {
            locus_cli::govern::handle_govern_command(subcommand).await
        }
        LocusCommands::Override { subcommand } => {
            locus_cli::override_cmd::handle_override_command(subcommand).await
        }
    }
}

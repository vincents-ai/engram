use clap::Parser;
use engram::locus_cli::LocusCommands;
use engram::locus_handlers::handle_locus_command;
use engram::locus_integration::LocusIntegration;
use engram::locus_tui::LocusTuiApp;
use engram::storage::GitRefsStorage;

mod locus_cli;

#[derive(Parser)]
#[command(name = "locus")]
#[command(about = "Locus - Human TUI Interface for Engram System", long_about = None)]
#[command(version = env!("CARGO_PKG_VERSION"))]
struct Cli {
    /// Run in CLI mode instead of TUI
    #[arg(long)]
    cli: bool,

    #[command(subcommand)]
    command: Option<LocusCommands>,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    // Default to TUI mode when no arguments provided
    let in_cli_mode = cli.cli || cli.command.is_some();

    if in_cli_mode {
        // CLI mode
        let storage = GitRefsStorage::new(".", "default")
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        let mut integration = LocusIntegration::new(storage);

        if let Some(command) = cli.command {
            handle_locus_command(&mut integration, command).await?;
        } else {
            println!("No command provided. Use --help to see available commands.");
        }
    } else {
        // TUI mode (default)
        let storage = GitRefsStorage::new(".", "default")
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        // Load workspace config to get the refresh interval.
        // Fall back to the default (30s) if the config file is absent or invalid.
        let workspace_cfg = engram::config::workspace_config::WorkspaceConfig::default();
        let backend: Box<dyn engram::locus_tui::backend::LocusTuiBackend> =
            match engram::locus_tui::backend::GitEngramBackend::new() {
                Ok(b) => Box::new(b),
                Err(_) => {
                    let mem = engram::storage::memory_only_storage::MemoryStorage::new("locus-tui");
                    Box::new(engram::locus_tui::backend::EngramBackend::from_storage(mem))
                }
            };
        let mut app = LocusTuiApp::new_with_refresh_interval(
            storage,
            backend,
            workspace_cfg.refresh_interval_secs,
        );
        app.run()?;
    }
    Ok(())
}

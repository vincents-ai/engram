use clap::Subcommand;

/// Convert commands
#[derive(Subcommand)]
pub enum ConvertCommands {
    /// Convert from external format
    Convert {
        /// Source format (openspec, beads, github)
        #[arg(long, short)]
        from: String,

        /// Source file path
        #[arg(long, short)]
        file: String,
    },
}

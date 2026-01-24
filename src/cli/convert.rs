use clap::Subcommand;

/// Convert commands
#[derive(Subcommand)]
pub enum ConvertCommands {
    /// Convert from external format
    Convert {
        /// Source format (openspec, beads, github)
        #[arg(long, short = 'o')]
        from: String,

        /// Source file path
        #[arg(long, short = 'f')]
        file: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[derive(Parser)]
    struct Cli {
        #[command(subcommand)]
        command: ConvertCommands,
    }

    #[test]
    fn test_convert_command_parsing() {
        let args = vec![
            "engram",
            "convert",
            "--from",
            "github",
            "--file",
            "issues.json",
        ];
        let cli = Cli::try_parse_from(args).unwrap();

        match cli.command {
            ConvertCommands::Convert { from, file } => {
                assert_eq!(from, "github");
                assert_eq!(file, "issues.json");
            }
        }
    }
}

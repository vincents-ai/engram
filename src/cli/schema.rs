//! Schema generation commands

use crate::entities::workflow::Workflow;
use clap::Subcommand;
use schemars::schema_for;

#[derive(Subcommand)]
pub enum SchemaCommands {
    /// Generate JSON Schema for workflow entity
    Workflow {
        /// Output file path (prints to stdout if not specified)
        #[arg(long, short)]
        output: Option<String>,
    },
}

pub fn handle_schema_command(command: SchemaCommands) -> crate::Result<()> {
    match command {
        SchemaCommands::Workflow { output } => {
            let schema = schema_for!(Workflow);
            let schema_json = serde_json::to_string_pretty(&schema)
                .map_err(|e| crate::EngramError::Serialization(e))?;

            if let Some(output_path) = output {
                std::fs::write(&output_path, schema_json).map_err(|e| crate::EngramError::Io(e))?;
                println!("✅ Schema written to: {}", output_path);
            } else {
                println!("{}", schema_json);
            }

            Ok(())
        }
    }
}

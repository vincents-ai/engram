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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;
    use tempfile::NamedTempFile;

    #[test]
    fn test_handle_schema_command_workflow_stdout() {
        // Since handle_schema_command prints to stdout when no output is provided,
        // we can check if it returns Ok. Capturing stdout in Rust tests is tricky without external crates,
        // but checking for execution success is a good baseline.
        // For unit tests, we mainly care that the logic doesn't crash.
        let result = handle_schema_command(SchemaCommands::Workflow { output: None });
        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_schema_command_workflow_file() {
        // Test writing to a temporary file
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_str().unwrap().to_string();

        let result = handle_schema_command(SchemaCommands::Workflow {
            output: Some(path.clone()),
        });
        assert!(result.is_ok());

        // Verify file content is valid JSON
        let mut file = std::fs::File::open(&path).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();

        let json: serde_json::Value = serde_json::from_str(&content).expect("Should be valid JSON");

        // Basic check to see if it looks like a schema
        assert!(json.get("$schema").is_some());
        assert_eq!(json.get("title").and_then(|v| v.as_str()), Some("Workflow"));
    }

    #[test]
    fn test_handle_schema_command_invalid_path() {
        // Test writing to an invalid path (e.g., directory that doesn't exist)
        let path = "/non/existent/directory/schema.json".to_string();

        let result = handle_schema_command(SchemaCommands::Workflow { output: Some(path) });

        // This should fail with an IO error
        assert!(matches!(result, Err(crate::EngramError::Io(_))));
    }
}

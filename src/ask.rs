//! Natural Language Query Interface for Engram
//!
//! Provides conversational access to Engram's memory system through pattern matching
//! and entity extraction. Maps natural language queries to structured Engram operations.

use crate::error::EngramError;
use crate::nlq::NLQEngine;
use crate::storage::GitStorage;
use clap::Subcommand;
use serde_json;

/// Natural language query commands
#[derive(Subcommand)]
pub enum AskCommands {
    /// Ask a natural language query about your Engram data
    Query {
        /// Natural language query to execute
        #[arg(help = "Natural language query about your Engram data")]
        query: String,

        /// Context for the query (task ID, agent, etc.)
        #[arg(
            long,
            short = 'c',
            help = "Context for the query (task ID, agent, etc.)"
        )]
        context: Option<String>,

        /// Verbose output with detailed explanation
        #[arg(long, short = 'v', help = "Verbose output with detailed explanation")]
        verbose: bool,

        /// Output in JSON format for programmatic use
        #[arg(long, short = 'j', help = "Output in JSON format for programmatic use")]
        json: bool,
    },
}

/// Handle natural language query commands
pub async fn handle_ask_command(command: AskCommands) -> Result<(), EngramError> {
    let AskCommands::Query {
        query,
        context,
        verbose,
        json,
    } = command;

    let nlq_engine = NLQEngine::new();
    let storage = GitStorage::new(".", "default")?;

    match nlq_engine.process_query(&query, context, &storage).await {
        Ok(result) => {
            if json {
                let json_output = serde_json::json!({
                    "success": result.success,
                    "query": query,
                    "response": result.formatted_response,
                    "data": result.data,
                    "execution_time_ms": result.execution_time_ms
                });
                println!("{}", serde_json::to_string_pretty(&json_output)?);
            } else {
                println!("{}", result.formatted_response);

                if verbose {
                    println!("\n--- Debug Information ---");
                    println!("Execution time: {}ms", result.execution_time_ms);
                    println!("Raw data: {}", serde_json::to_string_pretty(&result.data)?);
                }
            }
        }
        Err(e) => {
            if json {
                let error_output = serde_json::json!({
                    "success": false,
                    "query": query,
                    "error": e.to_string()
                });
                println!("{}", serde_json::to_string_pretty(&error_output)?);
            } else {
                println!("Error processing query: {}", e);
            }
        }
    }

    Ok(())
}

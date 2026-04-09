//! Natural Language Query Interface for Engram
//!
//! Provides conversational access to Engram's memory system through pattern matching
//! and entity extraction. Maps natural language queries to structured Engram operations.

use crate::error::EngramError;
use crate::nlq::NLQEngine;
use crate::storage::GitRefsStorage;
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

        /// Filter knowledge results by type (fact, pattern, rule, concept, procedure, heuristic, skill, technique)
        #[arg(
            long,
            short = 'k',
            help = "Filter knowledge results by type (fact, pattern, rule, concept, procedure, heuristic, skill, technique)"
        )]
        knowledge_type: Option<String>,

        /// Enable deep relationship graph walking from matched entities
        #[arg(
            long,
            short = 'd',
            help = "Enable deep relationship graph walking from matched entities"
        )]
        deep: bool,

        /// Maximum traversal depth for deep walk (default: 2)
        #[arg(
            long,
            short = 'D',
            help = "Maximum traversal depth for deep walk (default: 2)"
        )]
        max_depth: Option<usize>,

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
        knowledge_type,
        deep,
        max_depth,
        verbose,
        json,
    } = command;

    let nlq_engine = NLQEngine::new();
    let storage = GitRefsStorage::new(".", "default")?;

    let query_context = match (&context, &knowledge_type) {
        (Some(ctx), Some(kt)) => Some(format!("{} [knowledge-type:{}]", ctx, kt)),
        (Some(ctx), None) => Some(ctx.clone()),
        (None, Some(kt)) => Some(format!("knowledge-type:{}", kt)),
        (None, None) => None,
    };

    match nlq_engine
        .process_query_with_deep(&query, query_context, &storage, deep, max_depth)
        .await
    {
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

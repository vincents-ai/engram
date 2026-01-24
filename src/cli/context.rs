//! Context command implementations

use crate::entities::{Context, ContextRelevance, Entity};
use crate::error::EngramError;
use crate::storage::Storage;
use clap::Subcommand;
use serde::Deserialize;
use std::fs;
use std::io::{self, Read};

/// Context input structure for JSON
#[derive(Debug, Deserialize)]
pub struct ContextInput {
    pub title: String,
    pub content: Option<String>,
    pub source: Option<String>,
    pub relevance: Option<String>,
    pub source_id: Option<String>,
    pub agent: Option<String>,
    pub tags: Option<Vec<String>>,
}

/// Context commands
#[derive(Debug, Subcommand)]
pub enum ContextCommands {
    /// Create a new context
    Create {
        /// Context title
        #[arg(long, short, conflicts_with_all = ["title_stdin", "title_file"])]
        title: Option<String>,

        /// Context content
        #[arg(long, short, conflicts_with_all = ["content_stdin", "content_file"])]
        content: Option<String>,

        /// Context source
        #[arg(long, short)]
        source: Option<String>,

        /// Relevance level (low, medium, high, critical)
        #[arg(long, short, default_value = "medium")]
        relevance: String,

        /// Source ID (URL, file path, etc.)
        #[arg(long)]
        source_id: Option<String>,

        /// Assigned agent
        #[arg(long, short)]
        agent: Option<String>,

        /// Tags (comma-separated)
        #[arg(long)]
        tags: Option<String>,

        /// Read title from stdin
        #[arg(long, conflicts_with_all = ["title", "title_file"])]
        title_stdin: bool,

        /// Read title from file
        #[arg(long, conflicts_with_all = ["title", "title_stdin"])]
        title_file: Option<String>,

        /// Read content from stdin
        #[arg(long, conflicts_with_all = ["content", "content_file"])]
        content_stdin: bool,

        /// Read content from file
        #[arg(long, conflicts_with_all = ["content", "content_stdin"])]
        content_file: Option<String>,

        /// Create context from JSON input (stdin or file)
        #[arg(long, conflicts_with_all = ["title", "content", "title_stdin", "title_file", "content_stdin", "content_file"])]
        json: bool,

        /// JSON file path (requires --json)
        #[arg(long, requires = "json")]
        json_file: Option<String>,
    },
    /// List contexts
    List {
        /// Filter by agent
        #[arg(long, short)]
        agent: Option<String>,

        /// Filter by relevance
        #[arg(long, short)]
        relevance: Option<String>,

        /// Limit number of results
        #[arg(long, short)]
        limit: Option<usize>,
    },
    /// Show context details
    Show {
        /// Context ID
        #[arg(help = "Context ID to show")]
        id: String,
    },
    /// Update context content
    Update {
        /// Context ID
        #[arg(help = "Context ID to update")]
        id: String,

        /// New content
        #[arg(long, short)]
        content: String,
    },
    /// Delete a context
    Delete {
        /// Context ID
        #[arg(help = "Context ID to delete")]
        id: String,
    },
}

/// Helper function to read from stdin
fn read_stdin() -> Result<String, EngramError> {
    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .map_err(EngramError::Io)?;
    Ok(buffer.trim().to_string())
}

/// Helper function to read from file
fn read_file(path: &str) -> Result<String, EngramError> {
    fs::read_to_string(path).map_err(EngramError::Io)
}

/// Create context from JSON input
fn create_context_from_input<S: Storage>(
    storage: &mut S,
    input: ContextInput,
) -> Result<(), EngramError> {
    // Parse relevance level
    let relevance = match input.relevance.as_deref().unwrap_or("medium") {
        "low" => ContextRelevance::Low,
        "medium" => ContextRelevance::Medium,
        "high" => ContextRelevance::High,
        "critical" => ContextRelevance::Critical,
        _ => {
            return Err(EngramError::Validation(
                "Invalid relevance level. Use: low, medium, high, critical".to_string(),
            ))
        }
    };

    let agent = input.agent.unwrap_or_else(|| "default".to_string());

    let mut context = Context::new(
        input.title,
        input.content.unwrap_or_default(),
        input.source.unwrap_or_default(),
        relevance,
        agent.clone(),
    );

    context.source_id = input.source_id;

    // Convert to generic entity
    let generic_entity = context.to_generic();

    // Store
    storage.store(&generic_entity)?;

    println!("Context '{}' created successfully", context.id);
    println!("ID: {}", context.id);
    println!("Agent: {}", agent);

    Ok(())
}

/// Create a new context with flexible input
pub fn create_context<S: Storage>(
    storage: &mut S,
    title: Option<String>,
    content: Option<String>,
    source: Option<String>,
    relevance: &str,
    source_id: Option<String>,
    agent: Option<String>,
    _tags: Option<String>,
    // Flexible input parameters
    title_stdin: bool,
    title_file: Option<String>,
    content_stdin: bool,
    content_file: Option<String>,
    json: bool,
    json_file: Option<String>,
) -> Result<(), EngramError> {
    // Handle JSON input first (overrides all other inputs)
    if json {
        let json_content = if let Some(ref file_path) = json_file {
            read_file(file_path)?
        } else {
            read_stdin()?
        };

        let context_input: ContextInput = serde_json::from_str(&json_content)
            .map_err(|e| EngramError::Validation(format!("Invalid JSON: {}", e)))?;

        return create_context_from_input(storage, context_input);
    }

    // Resolve title from various sources
    let final_title = if title_stdin {
        read_stdin()?
    } else if let Some(ref file_path) = title_file {
        read_file(file_path)?
    } else if let Some(ref t) = title {
        t.clone()
    } else {
        return Err(EngramError::Validation(
            "Title required: use --title, --title-stdin, or --title-file".to_string(),
        ));
    };

    // Resolve content from various sources
    let final_content = if content_stdin {
        read_stdin()?
    } else if let Some(ref file_path) = content_file {
        read_file(file_path)?
    } else if let Some(ref c) = content {
        c.clone()
    } else {
        String::new() // Content is optional
    };

    // Parse relevance level
    let relevance_level = match relevance {
        "low" => ContextRelevance::Low,
        "medium" => ContextRelevance::Medium,
        "high" => ContextRelevance::High,
        "critical" => ContextRelevance::Critical,
        _ => {
            return Err(EngramError::Validation(
                "Invalid relevance level. Use: low, medium, high, critical".to_string(),
            ))
        }
    };

    let final_agent = agent.unwrap_or_else(|| "default".to_string());

    let mut context = Context::new(
        final_title,
        final_content,
        source.unwrap_or_default(),
        relevance_level,
        final_agent.clone(),
    );

    context.source_id = source_id;

    // Convert to generic entity
    let generic_entity = context.to_generic();

    // Store
    storage.store(&generic_entity)?;

    println!("Context '{}' created successfully", context.id);
    println!("ID: {}", context.id);
    println!("Title: {}", context.title);
    println!("Agent: {}", final_agent);
    println!("Relevance: {:?}", context.relevance);

    Ok(())
}

/// List contexts
pub fn list_contexts<S: Storage>(
    storage: &S,
    agent: Option<&str>,
    relevance: Option<&str>,
    limit: Option<usize>,
) -> Result<(), EngramError> {
    // Query contexts from storage
    let mut filter = crate::storage::QueryFilter {
        entity_type: Some("context".to_string()),
        agent: agent.map(|s| s.to_string()),
        limit,
        ..Default::default()
    };

    // Add relevance filter if specified
    if let Some(rel) = relevance {
        filter.field_filters.insert(
            "relevance".to_string(),
            serde_json::Value::String(rel.to_string()),
        );
    }

    let result = storage.query(&filter)?;

    if result.entities.is_empty() {
        println!("No contexts found");
        return Ok(());
    }

    println!("Found {} context(s)", result.entities.len());
    println!();

    for entity in result.entities {
        if let Ok(context) = Context::from_generic(entity) {
            println!("ID: {}", context.id);
            println!("Title: {}", context.title);
            println!("Agent: {}", context.agent);
            println!("Relevance: {:?}", context.relevance);
            if !context.source.is_empty() {
                println!("Source: {}", context.source);
            }
            if context.content.len() > 100 {
                println!("Content: {}...", &context.content[..97]);
            } else {
                println!("Content: {}", context.content);
            }
            println!(
                "Created: {}",
                context.timestamp().format("%Y-%m-%d %H:%M:%S")
            );
            println!("---");
        }
    }

    if result.has_more {
        println!("(More results available - use --limit to see more)");
    }

    Ok(())
}

/// Show context details
pub fn show_context<S: Storage>(storage: &S, id: &str) -> Result<(), EngramError> {
    let entity = storage.get(id, "context")?;

    match entity {
        Some(generic_entity) => {
            let context = Context::from_generic(generic_entity)?;

            println!("Context Details:");
            println!("================");
            println!("ID: {}", context.id);
            println!("Title: {}", context.title);
            println!("Agent: {}", context.agent);
            println!("Relevance: {:?}", context.relevance);
            println!(
                "Source: {}",
                if context.source.is_empty() {
                    "N/A"
                } else {
                    &context.source
                }
            );
            if let Some(ref source_id) = context.source_id {
                println!("Source ID: {}", source_id);
            }
            println!(
                "Created: {}",
                context.timestamp().format("%Y-%m-%d %H:%M:%S UTC")
            );
            println!();
            println!("Content:");
            println!("--------");
            println!("{}", context.content);
        }
        None => {
            return Err(EngramError::NotFound(format!(
                "Context with ID '{}' not found",
                id
            )));
        }
    }

    Ok(())
}

/// Update context
pub fn update_context<S: Storage>(
    storage: &mut S,
    id: &str,
    content: &str,
) -> Result<(), EngramError> {
    let entity = storage.get(id, "context")?;

    match entity {
        Some(generic_entity) => {
            let mut context = Context::from_generic(generic_entity)?;

            context.content = content.to_string();
            context.updated_at = chrono::Utc::now();

            let updated_entity = context.to_generic();
            storage.store(&updated_entity)?;

            println!("Context '{}' updated successfully", context.id);
            println!("Title: {}", context.title);
            println!(
                "Updated: {}",
                context.updated_at.format("%Y-%m-%d %H:%M:%S UTC")
            );
        }
        None => {
            return Err(EngramError::NotFound(format!(
                "Context with ID '{}' not found",
                id
            )));
        }
    }

    Ok(())
}

/// Delete context
pub fn delete_context<S: Storage>(storage: &mut S, id: &str) -> Result<(), EngramError> {
    let entity = storage.get(id, "context")?;

    match entity {
        Some(generic_entity) => {
            let context = Context::from_generic(generic_entity)?;

            storage.delete(id, "context")?;

            println!("Context '{}' deleted successfully", context.title);
            println!("ID: {}", context.id);
        }
        None => {
            return Err(EngramError::NotFound(format!(
                "Context with ID '{}' not found",
                id
            )));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::MemoryStorage;

    fn create_test_storage() -> MemoryStorage {
        MemoryStorage::new("default")
    }

    #[test]
    fn test_create_context_basic() {
        let mut storage = create_test_storage();
        let result = create_context(
            &mut storage,
            Some("Test Context".to_string()),
            Some("Content body".to_string()),
            Some("manual".to_string()),
            "medium",
            None,
            None,
            None,
            false,
            None,
            false,
            None,
            false,
            None,
        );
        assert!(result.is_ok());

        let contexts = storage.query_by_agent("default", Some("context")).unwrap();
        assert_eq!(contexts.len(), 1);

        let context = Context::from_generic(contexts[0].clone()).unwrap();
        assert_eq!(context.title, "Test Context");
        assert_eq!(context.content, "Content body");
        assert_eq!(context.relevance, ContextRelevance::Medium);
    }

    #[test]
    fn test_create_context_validation() {
        let mut storage = create_test_storage();

        // Missing title
        let result = create_context(
            &mut storage,
            None,
            None,
            None,
            "medium",
            None,
            None,
            None,
            false,
            None,
            false,
            None,
            false,
            None,
        );
        assert!(matches!(result, Err(EngramError::Validation(_))));

        // Invalid relevance
        let result = create_context(
            &mut storage,
            Some("Title".to_string()),
            None,
            None,
            "invalid_relevance",
            None,
            None,
            None,
            false,
            None,
            false,
            None,
            false,
            None,
        );
        assert!(matches!(result, Err(EngramError::Validation(_))));
    }

    #[test]
    fn test_update_context() {
        let mut storage = create_test_storage();
        create_context(
            &mut storage,
            Some("Test Context".to_string()),
            Some("Initial content".to_string()),
            None,
            "medium",
            None,
            None,
            None,
            false,
            None,
            false,
            None,
            false,
            None,
        )
        .unwrap();

        let contexts = storage.query_by_agent("default", Some("context")).unwrap();
        let id = &contexts[0].id;

        update_context(&mut storage, id, "Updated content").unwrap();

        let updated_entity = storage.get(id, "context").unwrap().unwrap();
        let context = Context::from_generic(updated_entity).unwrap();
        assert_eq!(context.content, "Updated content");
    }

    #[test]
    fn test_delete_context() {
        let mut storage = create_test_storage();
        create_context(
            &mut storage,
            Some("Delete Me".to_string()),
            None,
            None,
            "medium",
            None,
            None,
            None,
            false,
            None,
            false,
            None,
            false,
            None,
        )
        .unwrap();

        let contexts = storage.query_by_agent("default", Some("context")).unwrap();
        let id = &contexts[0].id;

        delete_context(&mut storage, id).unwrap();

        let result = storage.get(id, "context").unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_show_context_not_found() {
        let storage = create_test_storage();
        let result = show_context(&storage, "non-existent-id");
        assert!(matches!(result, Err(EngramError::NotFound(_))));
    }

    #[test]
    fn test_list_contexts() {
        let mut storage = create_test_storage();
        create_context(
            &mut storage,
            Some("C1".to_string()),
            None,
            None,
            "medium",
            None,
            None,
            None,
            false,
            None,
            false,
            None,
            false,
            None,
        )
        .unwrap();
        create_context(
            &mut storage,
            Some("C2".to_string()),
            None,
            None,
            "high",
            None,
            None,
            None,
            false,
            None,
            false,
            None,
            false,
            None,
        )
        .unwrap();

        // Test listing all
        list_contexts(&storage, None, None, None).unwrap();

        // Test filtering by relevance
        list_contexts(&storage, None, Some("high"), None).unwrap();
    }

    #[test]
    fn test_show_context_success() {
        let mut storage = create_test_storage();
        create_context(
            &mut storage,
            Some("Show Me".to_string()),
            Some("Content".to_string()),
            None,
            "medium",
            None,
            None,
            None,
            false,
            None,
            false,
            None,
            false,
            None,
        )
        .unwrap();

        let contexts = storage.query_by_agent("default", Some("context")).unwrap();
        let id = &contexts[0].id;

        assert!(show_context(&storage, id).is_ok());
    }

    #[test]
    fn test_create_context_from_json_struct() {
        let mut storage = create_test_storage();
        let input = ContextInput {
            title: "JSON Context".to_string(),
            content: Some("JSON Content".to_string()),
            source: Some("api".to_string()),
            relevance: Some("high".to_string()),
            source_id: Some("123".to_string()),
            agent: Some("bot".to_string()),
            tags: None,
        };

        create_context_from_input(&mut storage, input).unwrap();

        let contexts = storage.query_by_agent("bot", Some("context")).unwrap();
        assert_eq!(contexts.len(), 1);
        let ctx = Context::from_generic(contexts[0].clone()).unwrap();
        assert_eq!(ctx.title, "JSON Context");
        assert_eq!(ctx.relevance, ContextRelevance::High);
    }
}

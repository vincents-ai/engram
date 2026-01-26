//! Knowledge command implementations

use crate::entities::{Entity, Knowledge, KnowledgeType};
use crate::error::EngramError;
use crate::storage::Storage;
use clap::Subcommand;
use serde::Deserialize;
use std::fs;
use std::io::{self, Read};

/// Knowledge input structure for JSON
#[derive(Debug, Deserialize)]
pub struct KnowledgeInput {
    pub title: String,
    pub content: Option<String>,
    pub knowledge_type: Option<String>,
    pub confidence: Option<f64>,
    pub source: Option<String>,
    pub agent: Option<String>,
    pub tags: Option<Vec<String>>,
}

/// Knowledge commands
#[derive(Debug, Subcommand)]
pub enum KnowledgeCommands {
    /// Create a new knowledge item
    Create {
        /// Knowledge title
        #[arg(long, short, conflicts_with_all = ["title_stdin", "title_file"])]
        title: Option<String>,

        /// Knowledge content
        #[arg(long, short, conflicts_with_all = ["content_stdin", "content_file"])]
        content: Option<String>,

        /// Knowledge type (fact, pattern, rule, concept, procedure, heuristic)
        #[arg(long, short = 'k', default_value = "fact")]
        knowledge_type: String,

        /// Confidence level (0.0 to 1.0)
        #[arg(long, short = 'f', default_value = "0.8")]
        confidence: f64,

        /// Source of this knowledge
        #[arg(long, short)]
        source: Option<String>,

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

        /// Create knowledge from JSON input (stdin or file)
        #[arg(long, conflicts_with_all = ["title", "content", "title_stdin", "title_file", "content_stdin", "content_file"])]
        json: bool,

        /// JSON file path (requires --json)
        #[arg(long, requires = "json")]
        json_file: Option<String>,
    },
    /// List knowledge items
    List {
        /// Agent filter
        #[arg(long, short)]
        agent: Option<String>,

        /// Type filter (fact, pattern, rule, concept, procedure, heuristic)
        #[arg(long, short)]
        kind: Option<String>,

        /// Limit results
        #[arg(long, short)]
        limit: Option<usize>,
    },
    /// Show knowledge details
    Show {
        /// Knowledge item ID
        #[arg(long, short)]
        id: String,
    },
    /// Update knowledge item
    Update {
        /// Knowledge item ID
        #[arg(long, short)]
        id: String,

        /// Field to update (content, confidence, type)
        #[arg(long, short)]
        field: String,

        /// New value
        #[arg(long, short)]
        value: String,
    },
    /// Delete knowledge item
    Delete {
        /// Knowledge item ID
        #[arg(long, short)]
        id: String,
    },
}

/// Read from stdin
fn read_stdin() -> Result<String, EngramError> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer.trim().to_string())
}

fn read_file(path: &str) -> Result<String, EngramError> {
    fs::read_to_string(path)
        .map(|s| s.trim().to_string())
        .map_err(|e| EngramError::Io(e))
}

/// Parse knowledge type string to KnowledgeType enum
fn parse_knowledge_type(type_str: &str) -> Result<KnowledgeType, EngramError> {
    match type_str.to_lowercase().as_str() {
        "fact" => Ok(KnowledgeType::Fact),
        "pattern" => Ok(KnowledgeType::Pattern),
        "rule" => Ok(KnowledgeType::Rule),
        "concept" => Ok(KnowledgeType::Concept),
        "procedure" => Ok(KnowledgeType::Procedure),
        "heuristic" => Ok(KnowledgeType::Heuristic),
        _ => Err(EngramError::Validation(
            format!("Invalid knowledge type '{}'. Must be one of: fact, pattern, rule, concept, procedure, heuristic", type_str)
        )),
    }
}

/// Create knowledge from JSON input
fn create_knowledge_from_input<S: Storage>(
    storage: &mut S,
    input: KnowledgeInput,
) -> Result<(), EngramError> {
    let agent = input.agent.unwrap_or_else(|| "default".to_string());
    let content = input.content.unwrap_or_default();
    let confidence = input.confidence.unwrap_or(0.8);
    let knowledge_type_str = input.knowledge_type.unwrap_or_else(|| "fact".to_string());
    let knowledge_type = parse_knowledge_type(&knowledge_type_str)?;

    // Validate confidence
    if confidence < 0.0 || confidence > 1.0 {
        return Err(EngramError::Validation(
            "Confidence must be between 0.0 and 1.0".to_string(),
        ));
    }

    let mut knowledge = Knowledge::new(input.title, content, knowledge_type, confidence, agent);

    // Set optional fields
    if let Some(source) = input.source {
        knowledge.set_source(source);
    }

    if let Some(tags) = input.tags {
        for tag in tags {
            knowledge.add_tag(tag);
        }
    }

    let generic = knowledge.to_generic();
    storage.store(&generic)?;

    println!("Knowledge created successfully with ID: {}", knowledge.id);
    Ok(())
}

/// Create a new knowledge item
pub fn create_knowledge<S: Storage>(
    storage: &mut S,
    title: Option<String>,
    content: Option<String>,
    knowledge_type: String,
    confidence: f64,
    source: Option<String>,
    agent: Option<String>,
    tags: Option<String>,
    title_stdin: bool,
    title_file: Option<String>,
    content_stdin: bool,
    content_file: Option<String>,
    json: bool,
    json_file: Option<String>,
) -> Result<(), EngramError> {
    // Handle JSON input first
    if json {
        let json_str = if let Some(file) = json_file {
            read_file(&file)?
        } else {
            read_stdin()?
        };

        let input: KnowledgeInput = serde_json::from_str(&json_str)
            .map_err(|e| EngramError::Validation(format!("Invalid JSON: {}", e)))?;

        return create_knowledge_from_input(storage, input);
    }

    // Resolve title
    let final_title = if title_stdin {
        read_stdin()?
    } else if let Some(file) = title_file {
        read_file(&file)?
    } else if let Some(t) = title {
        t
    } else {
        return Err(EngramError::Validation(
            "Title is required (use --title, --title-stdin, or --title-file)".to_string(),
        ));
    };

    // Resolve content (optional)
    let final_content = if content_stdin {
        read_stdin()?
    } else if let Some(file) = content_file {
        read_file(&file)?
    } else {
        content.unwrap_or_default()
    };

    // Parse knowledge type
    let knowledge_type_enum = parse_knowledge_type(&knowledge_type)?;

    // Validate confidence
    if confidence < 0.0 || confidence > 1.0 {
        return Err(EngramError::Validation(
            "Confidence must be between 0.0 and 1.0".to_string(),
        ));
    }

    let agent_name = agent.unwrap_or_else(|| "default".to_string());

    let mut knowledge = Knowledge::new(
        final_title,
        final_content,
        knowledge_type_enum,
        confidence,
        agent_name,
    );

    // Set optional fields
    if let Some(src) = source {
        knowledge.set_source(src);
    }

    if let Some(tags_str) = tags {
        for tag in tags_str.split(',') {
            knowledge.add_tag(tag.trim().to_string());
        }
    }

    let generic = knowledge.to_generic();
    storage.store(&generic)?;

    println!("Knowledge created successfully with ID: {}", knowledge.id);
    Ok(())
}

use crate::cli::utils::{create_table, truncate};
use prettytable::{cell, row};

/// List knowledge items
pub fn list_knowledge<S: Storage>(
    storage: &S,
    agent: Option<String>,
    kind: Option<String>,
    limit: Option<usize>,
) -> Result<(), EngramError> {
    let ids = storage.list_ids(Knowledge::entity_type())?;

    let mut items: Vec<Knowledge> = Vec::new();

    for id in ids {
        if let Some(entity) = storage.get(&id, Knowledge::entity_type())? {
            if let Ok(knowledge) = Knowledge::from_generic(entity) {
                if let Some(ref agent_filter) = agent {
                    if knowledge.agent != *agent_filter {
                        continue;
                    }
                }

                if let Some(ref type_filter) = kind {
                    let type_str = format!("{:?}", knowledge.knowledge_type).to_lowercase();
                    if type_str != type_filter.to_lowercase() {
                        continue;
                    }
                }

                items.push(knowledge);
            }
        }
    }

    if let Some(limit_val) = limit {
        items.truncate(limit_val);
    }

    if items.is_empty() {
        println!("No knowledge items found matching the criteria.");
        return Ok(());
    }

    let mut table = create_table();
    table.set_titles(row![
        "ID", "Title", "Type", "Conf", "Agent", "Source", "Updated"
    ]);

    for knowledge in items {
        let type_str = format!("{:?}", knowledge.knowledge_type);
        let source_str = knowledge.source.unwrap_or_else(|| "-".to_string());

        table.add_row(row![
            &knowledge.id[..8],
            truncate(&knowledge.title, 40),
            type_str,
            format!("{:.2}", knowledge.confidence),
            truncate(&knowledge.agent, 15),
            truncate(&source_str, 20),
            knowledge.updated_at.format("%Y-%m-%d")
        ]);
    }

    table.printstd();
    Ok(())
}

/// Show knowledge details
pub fn show_knowledge<S: Storage>(storage: &S, id: &str) -> Result<(), EngramError> {
    let entity = storage
        .get(id, Knowledge::entity_type())?
        .ok_or_else(|| EngramError::NotFound(format!("Knowledge not found: {}", id)))?;

    let knowledge =
        Knowledge::from_generic(entity).map_err(|e| EngramError::Validation(e.to_string()))?;

    println!("Knowledge Details:");
    println!("==================");
    println!("ID: {}", knowledge.id);
    println!("Title: {}", knowledge.title);
    println!("Content: {}", knowledge.content);
    println!("Type: {:?}", knowledge.knowledge_type);
    println!("Confidence: {:.2}", knowledge.confidence);
    println!("Agent: {}", knowledge.agent);
    println!("Created: {}", knowledge.created_at);
    println!("Updated: {}", knowledge.updated_at);

    if let Some(source) = &knowledge.source {
        println!("Source: {}", source);
    }

    if !knowledge.tags.is_empty() {
        println!("Tags: {}", knowledge.tags.join(", "));
    }

    if !knowledge.related_knowledge.is_empty() {
        println!(
            "Related Knowledge: {}",
            knowledge.related_knowledge.join(", ")
        );
    }

    if !knowledge.contexts.is_empty() {
        println!("Contexts: {}", knowledge.contexts.join(", "));
    }

    println!("Usage Count: {}", knowledge.usage_count);

    if let Some(last_used) = knowledge.last_used {
        println!("Last Used: {}", last_used);
    }

    Ok(())
}

/// Update knowledge item
pub fn update_knowledge<S: Storage>(
    storage: &mut S,
    id: &str,
    field: &str,
    value: &str,
) -> Result<(), EngramError> {
    let entity = storage
        .get(id, Knowledge::entity_type())?
        .ok_or_else(|| EngramError::NotFound(format!("Knowledge not found: {}", id)))?;

    let mut knowledge =
        Knowledge::from_generic(entity).map_err(|e| EngramError::Validation(e.to_string()))?;

    match field.to_lowercase().as_str() {
        "content" => {
            knowledge.update_content(value.to_string(), knowledge.confidence);
        }
        "confidence" => {
            let confidence: f64 = value
                .parse()
                .map_err(|_| EngramError::Validation("Confidence must be a number".to_string()))?;
            if confidence < 0.0 || confidence > 1.0 {
                return Err(EngramError::Validation(
                    "Confidence must be between 0.0 and 1.0".to_string(),
                ));
            }
            knowledge.update_content(knowledge.content.clone(), confidence);
        }
        "type" => {
            let new_type = parse_knowledge_type(value)?;
            knowledge.knowledge_type = new_type;
            knowledge.updated_at = chrono::Utc::now();
        }
        "source" => {
            knowledge.set_source(value.to_string());
        }
        _ => {
            return Err(EngramError::Validation(format!(
                "Unknown field '{}'. Supported fields: content, confidence, type, source",
                field
            )));
        }
    }

    let generic = knowledge.to_generic();
    storage.store(&generic)?;

    println!("Knowledge updated successfully: {}", id);
    Ok(())
}

/// Delete knowledge item
pub fn delete_knowledge<S: Storage>(storage: &mut S, id: &str) -> Result<(), EngramError> {
    storage.delete(id, Knowledge::entity_type())?;
    println!("Knowledge deleted successfully: {}", id);
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
    fn test_create_knowledge_basic() {
        let mut storage = create_test_storage();
        let result = create_knowledge(
            &mut storage,
            Some("Test Fact".to_string()),
            Some("Water is wet".to_string()),
            "fact".to_string(),
            0.9,
            Some("Observation".to_string()),
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

        let ids = storage.list_ids("knowledge").unwrap();
        assert_eq!(ids.len(), 1);

        let entity = storage.get(&ids[0], "knowledge").unwrap().unwrap();
        let knowledge = Knowledge::from_generic(entity).unwrap();
        assert_eq!(knowledge.title, "Test Fact");
        assert_eq!(knowledge.content, "Water is wet");
        assert!(matches!(knowledge.knowledge_type, KnowledgeType::Fact));
    }

    #[test]
    fn test_create_knowledge_validation() {
        let mut storage = create_test_storage();

        // Missing title
        let result = create_knowledge(
            &mut storage,
            None,
            None,
            "fact".to_string(),
            0.8,
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

        // Invalid type
        let result = create_knowledge(
            &mut storage,
            Some("Title".to_string()),
            None,
            "invalid_type".to_string(),
            0.8,
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

        // Invalid confidence
        let result = create_knowledge(
            &mut storage,
            Some("Title".to_string()),
            None,
            "fact".to_string(),
            1.5,
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
    fn test_update_knowledge() {
        let mut storage = create_test_storage();
        create_knowledge(
            &mut storage,
            Some("Test Fact".to_string()),
            None,
            "fact".to_string(),
            0.8,
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

        let ids = storage.list_ids("knowledge").unwrap();
        let id = &ids[0];

        // Update content
        update_knowledge(&mut storage, id, "content", "New content").unwrap();
        let entity = storage.get(id, "knowledge").unwrap().unwrap();
        let knowledge = Knowledge::from_generic(entity).unwrap();
        assert_eq!(knowledge.content, "New content");

        // Update confidence
        update_knowledge(&mut storage, id, "confidence", "0.95").unwrap();
        let entity = storage.get(id, "knowledge").unwrap().unwrap();
        let knowledge = Knowledge::from_generic(entity).unwrap();
        assert_eq!(knowledge.confidence, 0.95);
    }

    #[test]
    fn test_delete_knowledge() {
        let mut storage = create_test_storage();
        create_knowledge(
            &mut storage,
            Some("Delete Me".to_string()),
            None,
            "fact".to_string(),
            0.8,
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

        let ids = storage.list_ids("knowledge").unwrap();
        let id = &ids[0];

        delete_knowledge(&mut storage, id).unwrap();

        let result = storage.get(id, "knowledge").unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_list_knowledge() {
        let mut storage = create_test_storage();
        create_knowledge(
            &mut storage,
            Some("Fact 1".to_string()),
            None,
            "fact".to_string(),
            0.8,
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

        create_knowledge(
            &mut storage,
            Some("Rule 1".to_string()),
            None,
            "rule".to_string(),
            0.8,
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

        // Just verify it runs without error (output is to stdout)
        assert!(list_knowledge(&storage, None, Some("fact".to_string()), None).is_ok());
    }

    #[test]
    fn test_show_knowledge() {
        let mut storage = create_test_storage();
        create_knowledge(
            &mut storage,
            Some("Show Me".to_string()),
            Some("Content to show".to_string()),
            "fact".to_string(),
            0.8,
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

        let ids = storage.list_ids("knowledge").unwrap();
        let id = &ids[0];

        assert!(show_knowledge(&storage, id).is_ok());
    }

    #[test]
    fn test_show_knowledge_not_found() {
        let storage = create_test_storage();
        let result = show_knowledge(&storage, "missing-id");
        assert!(matches!(result, Err(EngramError::NotFound(_))));
    }

    #[test]
    fn test_update_knowledge_not_found() {
        let mut storage = create_test_storage();
        let result = update_knowledge(&mut storage, "missing-id", "content", "new content");
        assert!(matches!(result, Err(EngramError::NotFound(_))));
    }

    #[test]
    fn test_update_knowledge_invalid_field() {
        let mut storage = create_test_storage();
        create_knowledge(
            &mut storage,
            Some("Test Fact".to_string()),
            None,
            "fact".to_string(),
            0.8,
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

        let ids = storage.list_ids("knowledge").unwrap();
        let id = &ids[0];

        let result = update_knowledge(&mut storage, id, "invalid_field", "value");
        assert!(matches!(result, Err(EngramError::Validation(_))));
    }

    #[test]
    fn test_update_knowledge_invalid_confidence() {
        let mut storage = create_test_storage();
        create_knowledge(
            &mut storage,
            Some("Test Fact".to_string()),
            None,
            "fact".to_string(),
            0.8,
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

        let ids = storage.list_ids("knowledge").unwrap();
        let id = &ids[0];

        let result = update_knowledge(&mut storage, id, "confidence", "2.0");
        assert!(matches!(result, Err(EngramError::Validation(_))));
    }

    #[test]
    fn test_delete_knowledge_not_found() {
        let mut storage = create_test_storage();
        // MemoryStorage delete usually returns Ok even if not found, but delete_knowledge wraps it.
        // Actually storage.delete signature: fn delete(&mut self, id: &str, entity_type: &str) -> Result<(), EngramError>
        // MemoryStorage implementation: returns Ok(()) regardless or NotFound?
        // Let's assume it might return NotFound. If it returns Ok, the test will fail if we assert Error.
        // Let's check other tests. test_delete_reasoning_not_found asserted Err(NotFound).
        // So MemoryStorage probably returns NotFound.
        let result = delete_knowledge(&mut storage, "missing-id");
        // Adjust expectation based on storage implementation if needed.
        // If storage.delete returns NotFound error, then this is correct.
        // If storage.delete returns Ok, then we should assert Ok.
        // Based on previous files, let's assume NotFound.
        // But wait, the MemoryStorage delete implementation:
        //      if self.data.remove(key).is_none() { return Err(EngramError::NotFound(...)); }
        // Yes, it returns NotFound.
        assert!(matches!(result, Err(EngramError::NotFound(_))));
    }

    #[test]
    fn test_update_knowledge_invalid_type() {
        let mut storage = create_test_storage();
        create_knowledge(
            &mut storage,
            Some("Test Fact".to_string()),
            None,
            "fact".to_string(),
            0.8,
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

        let ids = storage.list_ids("knowledge").unwrap();
        let id = &ids[0];

        let result = update_knowledge(&mut storage, id, "type", "invalid_type");
        assert!(matches!(result, Err(EngramError::Validation(_))));
    }

    #[test]
    fn test_update_knowledge_source() {
        let mut storage = create_test_storage();
        create_knowledge(
            &mut storage,
            Some("Test Fact".to_string()),
            None,
            "fact".to_string(),
            0.8,
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

        let ids = storage.list_ids("knowledge").unwrap();
        let id = &ids[0];

        update_knowledge(&mut storage, id, "source", "New Source").unwrap();
        let entity = storage.get(id, "knowledge").unwrap().unwrap();
        let knowledge = Knowledge::from_generic(entity).unwrap();
        assert_eq!(knowledge.source, Some("New Source".to_string()));
    }

    #[test]
    fn test_update_knowledge_confidence_nan() {
        let mut storage = create_test_storage();
        create_knowledge(
            &mut storage,
            Some("Test Fact".to_string()),
            None,
            "fact".to_string(),
            0.8,
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

        let ids = storage.list_ids("knowledge").unwrap();
        let id = &ids[0];

        let result = update_knowledge(&mut storage, id, "confidence", "not_a_number");
        assert!(matches!(result, Err(EngramError::Validation(_))));
    }
}

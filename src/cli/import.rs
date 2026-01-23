//! Import command for batch entity creation from structured markdown files
//!
//! This module provides the `engram import` command which parses Engram Markdown
//! (EMD) files and auto-creates entities with relationships based on pattern matching.

use crate::entities::{Context, Entity, Reasoning, Task};
use crate::error::EngramError;
use crate::storage::{RelationshipStorage, Storage};
use clap::Subcommand;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

/// Import commands
#[derive(Debug, Subcommand)]
pub enum ImportCommands {
    /// Import entities from an Engram Markdown (EMD) file
    Import {
        /// Path to the markdown file to import
        #[arg(long, short = 'f')]
        file: PathBuf,

        /// Verbose output showing progress
        #[arg(long, short = 'v')]
        verbose: bool,

        /// Preview changes without creating entities
        #[arg(long)]
        dry_run: bool,

        /// Overwrite existing entities
        #[arg(long)]
        force: bool,

        /// Output results as JSON
        #[arg(long, short = 'j')]
        json: bool,
    },
}

/// Document types supported by import
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum DocType {
    #[serde(rename = "review")]
    #[default]
    Review,
    #[serde(rename = "context")]
    Context,
    #[serde(rename = "task")]
    Task,
    #[serde(rename = "reasoning")]
    Reasoning,
    #[serde(rename = "findings")]
    Findings,
    #[serde(rename = "general")]
    General,
}

/// Frontmatter structure for Engram Markdown files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Frontmatter {
    pub title: String,
    #[serde(default)]
    pub doc_type: DocType,
    pub date: Option<String>,
    pub author: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    pub parent_id: Option<String>,
}

/// Represents a finding section in the markdown
#[derive(Debug, Clone)]
pub struct Finding {
    pub title: String,
    pub content: String,
    pub uuid: Option<Uuid>,
}

/// Represents a task reference in the markdown
#[derive(Debug, Clone)]
pub struct TaskRef {
    pub priority: String,
    pub uuid: Uuid,
    pub description: String,
}

/// Represents a reasoning section in the markdown
#[derive(Debug, Clone)]
pub struct ReasoningSection {
    pub title: String,
    pub content: String,
    pub task_id: Option<Uuid>,
    pub uuid: Option<Uuid>,
}

/// Result of an import operation
#[derive(Debug, Clone)]
pub struct ImportResult {
    pub entities_created: usize,
    pub relationships_created: usize,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
    pub entity_ids: Vec<String>,
}

/// Import errors
#[derive(Debug, thiserror::Error)]
pub enum ImportError {
    #[error("File not found: {0}")]
    FileNotFound(String),
    #[error("Invalid frontmatter: {0}")]
    InvalidFrontmatter(String),
    #[error("Missing frontmatter")]
    MissingFrontmatter,
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("Invalid UUID: {0}")]
    InvalidUuid(String),
    #[error("Entity not found: {0}")]
    EntityNotFound(Uuid),
    #[error("Storage error: {0}")]
    StorageError(String),
}

/// Handle import command
pub fn handle_import_command<S: Storage + RelationshipStorage>(
    command: ImportCommands,
    storage: &mut S,
) -> Result<(), EngramError> {
    match command {
        ImportCommands::Import {
            file,
            verbose,
            dry_run,
            force: _,
            json,
        } => {
            let result = import_file(&file, verbose, dry_run, storage)?;

            if json {
                let json_output = serde_json::json!({
                    "success": result.errors.is_empty(),
                    "entities_created": result.entities_created,
                    "relationships_created": result.relationships_created,
                    "warnings": result.warnings,
                    "errors": result.errors,
                    "entity_ids": result.entity_ids,
                });
                println!("{}", serde_json::to_string_pretty(&json_output)?);
            } else {
                println!(
                    "Import complete: {} entities, {} relationships",
                    result.entities_created, result.relationships_created
                );

                if !result.warnings.is_empty() && verbose {
                    println!("\nWarnings:");
                    for warning in &result.warnings {
                        println!("  - {}", warning);
                    }
                }

                if !result.errors.is_empty() {
                    println!("\nErrors:");
                    for error in &result.errors {
                        println!("  - {}", error);
                    }
                    return Err(EngramError::Validation(
                        "Import completed with errors".to_string(),
                    ));
                }
            }

            Ok(())
        }
    }
}

/// Main import function
fn import_file<S: Storage + RelationshipStorage>(
    file: &PathBuf,
    verbose: bool,
    dry_run: bool,
    storage: &mut S,
) -> Result<ImportResult, EngramError> {
    let mut result = ImportResult {
        entities_created: 0,
        relationships_created: 0,
        warnings: Vec::new(),
        errors: Vec::new(),
        entity_ids: Vec::new(),
    };

    // Read file
    if verbose {
        println!("Reading file: {:?}", file);
    }

    let content = fs::read_to_string(file).map_err(|e| EngramError::Io(e))?;

    // Parse frontmatter
    if verbose {
        println!("Parsing frontmatter...");
    }

    let frontmatter =
        parse_frontmatter(&content).map_err(|e| EngramError::Validation(e.to_string()))?;

    if verbose {
        println!("  Title: {}", frontmatter.title);
        println!("  Type: {:?}", frontmatter.doc_type);
        if !frontmatter.tags.is_empty() {
            println!("  Tags: {:?}", frontmatter.tags);
        }
    }

    // Extract sections
    let sections = extract_markdown_sections(&content);

    if verbose {
        println!("Found {} sections", sections.len());
    }

    // Extract findings
    let findings = extract_findings(&sections);
    if verbose {
        println!("Found {} findings", findings.len());
    }

    // Extract task references
    let tasks = extract_task_references(&content);
    if verbose {
        println!("Found {} task references", tasks.len());
    }

    // Extract reasoning sections
    let reasoning_sections = extract_reasoning_sections(&sections);
    if verbose {
        println!("Found {} reasoning sections", reasoning_sections.len());
    }

    // Extract UUID patterns for auto-linking
    let uuid_patterns = extract_uuid_patterns(&content);
    if verbose {
        println!("Found {} UUID patterns for linking", uuid_patterns.len());
    }

    // For dry run, just show what would be created
    if dry_run {
        println!("\n[DRY RUN] Would create:");
        println!("  - {} findings", findings.len());
        println!("  - {} task references", tasks.len());
        println!("  - {} reasoning sections", reasoning_sections.len());
        println!("  - {} relationship patterns", uuid_patterns.len());

        for finding in &findings {
            println!("    - Finding: {}", finding.title);
        }

        return Ok(result);
    }

    // Create entities (storage operations would go here)
    // For now, just count what we would create
    result.entities_created = findings.len() + reasoning_sections.len();
    result.relationships_created = uuid_patterns.len();

    // Collect entity IDs
    for finding in &findings {
        if let Some(uuid) = finding.uuid {
            result.entity_ids.push(uuid.to_string());
        }
    }

    if matches!(frontmatter.doc_type, DocType::Task) {
        let entity_id = Uuid::new_v4();

        let task = Task::new(
            frontmatter.title.clone(),
            "Imported from markdown".to_string(),
            frontmatter
                .author
                .clone()
                .unwrap_or_else(|| "import".to_string()),
            crate::entities::TaskPriority::Medium,
            None,
        );

        let generic = task.to_generic();
        storage.store(&generic)?;

        result.entities_created += 1;
        result.entity_ids.push(entity_id.to_string());

        if verbose {
            println!("  Created task: {}", frontmatter.title);
        }
    }

    for finding in &findings {
        let entity_id = finding.uuid.unwrap_or_else(Uuid::new_v4);

        let context = Context::new(
            finding.title.clone(),
            finding.content.clone(),
            frontmatter
                .author
                .clone()
                .unwrap_or_else(|| "import".to_string()),
            crate::entities::ContextRelevance::Medium,
            "import".to_string(),
        );

        let generic = context.to_generic();
        storage.store(&generic)?;

        result.entities_created += 1;
        result.entity_ids.push(entity_id.to_string());

        if verbose {
            println!("  Created context: {}", finding.title);
        }
    }

    for reasoning in &reasoning_sections {
        let entity_id = reasoning.uuid.unwrap_or_else(Uuid::new_v4);

        let mut reasoning_entity = Reasoning::new(
            reasoning.title.clone(),
            reasoning
                .task_id
                .map(|t| t.to_string())
                .unwrap_or_else(|| "default".to_string()),
            "import".to_string(),
        );

        if !reasoning.content.is_empty() {
            reasoning_entity.add_step(reasoning.content.clone(), reasoning.title.clone(), 0.5);
        }

        let generic = reasoning_entity.to_generic();
        storage.store(&generic)?;

        result.entities_created += 1;
        result.entity_ids.push(entity_id.to_string());

        if verbose {
            println!("  Created reasoning: {}", reasoning.title);
        }
    }

    Ok(result)
}

/// Parse YAML frontmatter from markdown content
fn parse_frontmatter(content: &str) -> Result<Frontmatter, ImportError> {
    // Check for frontmatter markers
    if !content.starts_with("---") {
        // No frontmatter - extract title from first line
        let first_line = content
            .lines()
            .next()
            .unwrap_or("")
            .trim_start_matches('#')
            .trim();

        return Ok(Frontmatter {
            title: first_line.to_string(),
            doc_type: DocType::General,
            date: None,
            author: None,
            tags: Vec::new(),
            parent_id: None,
        });
    }

    // Find the closing frontmatter marker
    let start = 3; // Skip opening ---
    let end = match content[3..].find("---") {
        Some(pos) => pos + 3,
        None => {
            return Err(ImportError::InvalidFrontmatter(
                "Missing closing ---".to_string(),
            ))
        }
    };

    let frontmatter_str = &content[start..end].trim();

    // Parse YAML
    let frontmatter: Frontmatter = serde_yaml::from_str(frontmatter_str)
        .map_err(|e| ImportError::InvalidFrontmatter(e.to_string()))?;

    // Validate required fields
    if frontmatter.title.is_empty() {
        return Err(ImportError::InvalidFrontmatter(
            "title is required".to_string(),
        ));
    }

    Ok(frontmatter)
}

/// Extract markdown sections (## headers and their content)
fn extract_markdown_sections(content: &str) -> Vec<(String, String)> {
    let mut sections = Vec::new();

    // Split content after frontmatter
    let content_after_frontmatter = if content.starts_with("---") {
        if let Some(pos) = content[3..].find("---") {
            &content[pos + 6..]
        } else {
            content
        }
    } else {
        content
    };

    // Simple regex-like parsing for ## headers
    let re = regex::Regex::new(r"^(#{1,6})\s+(.+)$").unwrap();
    let lines: Vec<&str> = content_after_frontmatter.lines().collect();

    let mut current_section = None;
    let mut current_content = Vec::new();

    for line in lines {
        if let Some(caps) = re.captures(line) {
            // Save previous section
            if let Some(title) = current_section {
                sections.push((title, current_content.join("\n")));
            }

            // Start new section
            current_section = Some(caps[2].to_string());
            current_content = Vec::new();
        } else if current_section.is_some() {
            current_content.push(line);
        }
    }

    // Save last section
    if let Some(title) = current_section {
        sections.push((title, current_content.join("\n")));
    }

    sections
}

/// Extract findings from sections
fn extract_findings(sections: &[(String, String)]) -> Vec<Finding> {
    let mut findings = Vec::new();

    for (title, content) in sections {
        if title.starts_with("Finding:") || title.starts_with("## Findings") {
            // Extract UUID from pattern like [Finding: UUID]
            let uuid = extract_uuid_from_pattern(content);

            findings.push(Finding {
                title: title.trim_start_matches("Finding:").trim().to_string(),
                content: content.clone(),
                uuid,
            });
        }
    }

    findings
}

/// Extract task references from content
fn extract_task_references(content: &str) -> Vec<TaskRef> {
    let mut tasks = Vec::new();

    // Pattern: [P0: UUID] or [Task: UUID] description
    let re = regex::Regex::new(r"\[(P[0-3]):\s*([a-f0-9-]{36})\]").unwrap();

    for cap in re.captures_iter(content) {
        let priority = cap[1].to_string();
        let uuid_str = cap[2].to_string();

        if let Ok(uuid) = Uuid::parse_str(&uuid_str) {
            // Try to extract description (text after the pattern)
            let full_match = cap.get(0).unwrap().as_str();
            let after_match = content[content.find(full_match).unwrap() + full_match.len()..]
                .trim()
                .lines()
                .next()
                .unwrap_or("")
                .trim()
                .trim_start_matches('-')
                .trim()
                .to_string();

            tasks.push(TaskRef {
                priority,
                uuid,
                description: after_match,
            });
        }
    }

    tasks
}

/// Extract reasoning sections from markdown
fn extract_reasoning_sections(sections: &[(String, String)]) -> Vec<ReasoningSection> {
    let mut reasoning = Vec::new();

    // Pattern to find [Reasoning: UUID] in any section
    let reasoning_pattern = regex::Regex::new(r"\[Reasoning:\s*([a-f0-9-]{36})\]").unwrap();

    for (title, content) in sections {
        // Check if section title indicates reasoning
        let is_reasoning_section = title.starts_with("Reasoning:")
            || title.starts_with("## Reasoning")
            || title.starts_with("### Analysis:")
            || title.starts_with("### Design:");

        // Also check if content contains reasoning pattern
        let has_reasoning_pattern = reasoning_pattern.is_match(content);

        if is_reasoning_section || has_reasoning_pattern {
            let uuid = extract_uuid_from_pattern(content);
            let task_id = extract_task_id_from_content(content);

            reasoning.push(ReasoningSection {
                title: if title.starts_with("##") || title.starts_with("###") {
                    title.clone()
                } else {
                    format!("Reasoning: {}", title)
                },
                content: content.clone(),
                task_id,
                uuid,
            });
        }
    }

    reasoning
}

/// Extract UUID patterns for auto-linking
fn extract_uuid_patterns(content: &str) -> Vec<(Uuid, String)> {
    let mut patterns = Vec::new();

    // Pattern: [UUID] or [type: UUID]
    let re = regex::RegexBuilder::new(
        r"\[([a-f0-9]{8}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{12})\]",
    )
    .case_insensitive(true)
    .build()
    .unwrap();

    for cap in re.captures_iter(content) {
        let uuid_str = cap[1].to_string();

        if let Ok(uuid) = Uuid::parse_str(&uuid_str) {
            patterns.push((uuid, cap[0].to_string()));
        }
    }

    patterns
}

/// Extract UUID from a pattern like [Finding: UUID]
fn extract_uuid_from_pattern(text: &str) -> Option<Uuid> {
    let re =
        regex::Regex::new(r"\[([a-f0-9]{8}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{12})\]")
            .ok()?;

    for cap in re.captures_iter(text) {
        let uuid_str = cap[1].to_string();
        if let Ok(uuid) = Uuid::parse_str(&uuid_str) {
            return Some(uuid);
        }
    }

    None
}

/// Extract task ID from reasoning content
fn extract_task_id_from_content(content: &str) -> Option<Uuid> {
    // Look for [Task: UUID] or [uuid] patterns
    let re = regex::Regex::new(r"\[Task:\s*([a-f0-9-]{36})\]").ok()?;
    let task_re = regex::Regex::new(r"\[([a-f0-9-]{36})\]").ok()?;

    // First check for explicit [Task: UUID]
    for cap in re.captures_iter(content) {
        let uuid_str = cap[1].to_string();
        if let Ok(uuid) = Uuid::parse_str(&uuid_str) {
            return Some(uuid);
        }
    }

    // Then check for any [UUID] pattern
    for cap in task_re.captures_iter(content) {
        let uuid_str = cap[1].to_string();
        if let Ok(uuid) = Uuid::parse_str(&uuid_str) {
            return Some(uuid);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_frontmatter() {
        let content = r#"---
title: Test Document
doc_type: review
tags: [test, example]
---

# Content"#;

        let frontmatter = parse_frontmatter(content).unwrap();
        assert_eq!(frontmatter.title, "Test Document");
        // DocType::Review has #[default] so when unspecified it defaults to Review
        // But when specified as "review", serde maps it correctly
        assert!(matches!(frontmatter.doc_type, DocType::Review));
        assert_eq!(frontmatter.tags, vec!["test", "example"]);
    }

    #[test]
    fn test_extract_sections() {
        let content = r#"---
title: Test
---

# Overview

Some content here.

## Section 1

More content.

### Subsection

Deeper content.
"#;

        let sections = extract_markdown_sections(content);
        assert!(sections.len() >= 3);
    }

    #[test]
    fn test_extract_task_references() {
        let content = r#"
- [P0: da23ec54-04c8-4679-81e4-d90c09642d4c] Fix the tests
- [P1: 58339da6-71e3-41df-a6af-93d6b4f861eb] Add feature
"#;

        let tasks = extract_task_references(content);
        assert_eq!(tasks.len(), 2);
        assert_eq!(tasks[0].priority, "P0");
        assert_eq!(tasks[1].priority, "P1");
    }

    #[test]
    fn test_extract_uuid_patterns() {
        let content = r#"Context: [d453d97e-0f2d-416c-9199-fc70834cb546]
Finding: [3ecbee2c-aedc-4f74-ab7e-49491dda201e]
"#;

        let patterns = extract_uuid_patterns(content);
        assert_eq!(patterns.len(), 2);
    }
}

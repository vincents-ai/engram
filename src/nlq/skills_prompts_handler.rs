//! Skills and Prompts Query Handler for NLQ Engine
//!
//! Provides natural language querying of skills and prompts from
//! ENGRAM_SKILLS_PATH and ENGRAM_PROMPTS_PATH.

use crate::error::EngramError;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Get skills path from environment or default
pub fn get_skills_path() -> PathBuf {
    std::env::var("ENGRAM_SKILLS_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("./engram/skills"))
}

/// Get prompts path from environment or default
pub fn get_prompts_path() -> PathBuf {
    std::env::var("ENGRAM_PROMPTS_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("./engram/prompts"))
}

/// Skill query result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillInfo {
    pub name: String,
    pub path: String,
    pub description: Option<String>,
    pub category: String,
}

/// Prompt query result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptInfo {
    pub name: String,
    pub path: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub category: String,
}

/// Query for skills
#[derive(Debug, Clone)]
pub struct SkillsQuery {
    pub category: Option<String>,
    pub search_term: Option<String>,
    pub format: String,
}

/// Query for prompts
#[derive(Debug, Clone)]
pub struct PromptsQuery {
    pub category: Option<String>,
    pub search_term: Option<String>,
    pub format: String,
}

/// List all available skills
pub fn list_skills(query: &SkillsQuery) -> Result<Vec<SkillInfo>, EngramError> {
    let skills_path = get_skills_path();

    if !skills_path.exists() {
        return Ok(Vec::new());
    }

    let mut skills = Vec::new();
    let entries = fs::read_dir(&skills_path)?;

    for entry in entries.flatten() {
        if entry.path().is_dir() {
            let category = entry.file_name().to_string_lossy().to_string();

            // Filter by category if specified
            if let Some(ref cat) = query.category {
                if category.to_lowercase() != cat.to_lowercase() {
                    continue;
                }
            }

            // Search in skill name
            if let Some(ref term) = query.search_term {
                if !category.to_lowercase().contains(&term.to_lowercase()) {
                    continue;
                }
            }

            // Try to read description from skill.md
            let skill_file = entry.path().join("skill.md");
            let description = if skill_file.exists() {
                let content = fs::read_to_string(&skill_file).ok();
                content.and_then(|c| {
                    c.lines().next().map(|s| s.to_string()).filter(|s| !s.is_empty())
                })
            } else {
                None
            };

            skills.push(SkillInfo {
                name: category.clone(),
                path: entry.path().to_string_lossy().to_string(),
                description,
                category,
            });
        }
    }

    Ok(skills)
}

/// List all available prompts
pub fn list_prompts(query: &PromptsQuery) -> Result<Vec<PromptInfo>, EngramError> {
    let prompts_path = get_prompts_path();

    if !prompts_path.exists() {
        return Ok(Vec::new());
    }

    let mut prompts = Vec::new();
    let entries = fs::read_dir(&prompts_path)?;

    for entry in entries.flatten() {
        if entry.path().is_dir() {
            let category = entry.file_name().to_string_lossy().to_string();

            // Filter by category if specified
            if let Some(ref cat) = query.category {
                if category.to_lowercase() != cat.to_lowercase() {
                    continue;
                }
            }

            // Search in subdirectory
            let subentries = fs::read_dir(&entry.path())?;
            for subentry in subentries.flatten() {
                if subentry.path().is_file() {
                    let name = subentry.file_name().to_string_lossy().to_string();

                    // Search in prompt name
                    if let Some(ref term) = query.search_term {
                        if !name.to_lowercase().contains(&term.to_lowercase()) {
                            continue;
                        }
                    }

                    // Try to extract title/description from YAML
                    let content = fs::read_to_string(&subentry.path()).ok();
                    let (title, description) = extract_yaml_metadata(&content.unwrap_or_default());

                    prompts.push(PromptInfo {
                        name: name.clone(),
                        path: format!("{}/{}", category, name),
                        title,
                        description,
                        category: category.clone(),
                    });
                }
            }
        }
    }

    Ok(prompts)
}

/// Extract title and description from YAML frontmatter
fn extract_yaml_metadata(content: &str) -> (Option<String>, Option<String>) {
    if !content.starts_with("---") {
        return (None, None);
    }

    // Find closing frontmatter
    let start = 3;
    let end = match content[3..].find("---") {
        Some(pos) => pos + 3,
        None => return (None, None),
    };

    let frontmatter = &content[start..end];

    // Simple extraction
    let title = extract_yaml_field(frontmatter, "title");
    let desc = extract_yaml_field(frontmatter, "description");

    (title, desc)
}

/// Extract a field value from YAML
fn extract_yaml_field(yaml: &str, field: &str) -> Option<String> {
    let pattern = format!(r#"{}\s*:\s*"([^"]+)""#, field);
    let re = regex::RegexBuilder::new(&pattern)
        .case_insensitive(true)
        .build()
        .ok()?;

    if let Some(cap) = re.captures(yaml) {
        return Some(cap[1].to_string());
    }

    // Try without quotes
    let pattern = format!(r#"{}\s*:\s*(\w+)"#, field);
    let re = regex::RegexBuilder::new(&pattern)
        .case_insensitive(true)
        .build()
        .ok()?;

    if let Some(cap) = re.captures(yaml) {
        return Some(cap[1].to_string());
    }

    None
}

/// Search for skills matching a natural language query
pub fn search_skills(query: &str) -> Result<Vec<SkillInfo>, EngramError> {
    // Parse natural language to extract search terms
    let search_term = if query.contains("for") {
        query.split("for").last().map(|s| s.trim().to_string())
    } else if query.contains("related to") {
        query.split("related to").last().map(|s| s.trim().to_string())
    } else {
        Some(query.trim().to_string())
    };

    list_skills(&SkillsQuery {
        category: None,
        search_term,
        format: "full".to_string(),
    })
}

/// Search for prompts matching a natural language query
pub fn search_prompts(query: &str) -> Result<Vec<PromptInfo>, EngramError> {
    // Parse natural language to extract search terms
    let search_term = if query.contains("for") {
        query.split("for").last().map(|s| s.trim().to_string())
    } else if query.contains("related to") {
        query.split("related to").last().map(|s| s.trim().to_string())
    } else {
        Some(query.trim().to_string())
    };

    // Try to extract category
    let category = if query.contains("agent") {
        Some("agents".to_string())
    } else if query.contains("pipeline") {
        Some("ai/pipelines".to_string())
    } else if query.contains("compliance") {
        Some("compliance_and_certification".to_string())
    } else {
        None
    };

    list_prompts(&PromptsQuery {
        category,
        search_term,
        format: "full".to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yaml_extraction() {
        let yaml = r#"---
title: "Test Skill"
description: "A test skill for testing"
---"#;

        let (title, desc) = extract_yaml_metadata(yaml);
        assert_eq!(title, Some("Test Skill".to_string()));
        assert_eq!(desc, Some("A test skill for testing".to_string()));
    }

    #[test]
    fn test_skills_path_from_env() {
        std::env::set_var("ENGRAM_SKILLS_PATH", "/custom/skills");
        assert_eq!(get_skills_path(), PathBuf::from("/custom/skills"));
        std::env::remove_var("ENGRAM_SKILLS_PATH");
    }
}

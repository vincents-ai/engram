//! Commit message parsing for task ID extraction

use crate::error::EngramError;
use crate::validation::{config::ValidationConfig, ParsedTaskInfo, TaskIdFormat};
use regex::Regex;

/// Parser for extracting task IDs from commit messages
pub struct CommitMessageParser {
    task_id_patterns: Vec<Regex>,
    config: ValidationConfig,
}

impl CommitMessageParser {
    /// Create a new parser with default configuration
    pub fn new() -> Result<Self, EngramError> {
        let config = ValidationConfig::default();
        Self::with_config(config)
    }

    /// Create a new parser with custom configuration
    pub fn with_config(config: ValidationConfig) -> Result<Self, EngramError> {
        let task_id_patterns = config.get_task_regexes()?;
        Ok(Self {
            task_id_patterns,
            config,
        })
    }

    /// Parse task ID from commit message
    pub fn parse_task_id(&self, message: &str) -> Result<Option<ParsedTaskInfo>, EngramError> {
        // Check for exemptions first
        if self.config.should_exempt(message, "require_task_reference") {
            return Ok(None);
        }

        // Try each pattern in order
        for (pattern_index, pattern) in self.task_id_patterns.iter().enumerate() {
            if let Some(captures) = pattern.captures(message) {
                if let Some(task_id_match) = captures.get(1) {
                    let task_id = task_id_match.as_str().to_string();
                    let format = match pattern_index {
                        0 => TaskIdFormat::Brackets,
                        1 => TaskIdFormat::Colon,
                        2 => TaskIdFormat::Refs,
                        _ => TaskIdFormat::Custom(
                            self.config.task_id_patterns[pattern_index].name.clone(),
                        ),
                    };
                    return Ok(Some(ParsedTaskInfo { task_id, format }));
                }
            }
        }

        Ok(None)
    }

    /// Extract all task IDs from a message (multiple tasks per commit)
    pub fn parse_all_task_ids(&self, message: &str) -> Result<Vec<ParsedTaskInfo>, EngramError> {
        // Check for exemptions first
        if self.config.should_exempt(message, "require_task_reference") {
            return Ok(vec![]);
        }

        let mut task_ids = Vec::new();
        let mut used_positions: Vec<std::ops::Range<usize>> = Vec::new();

        // Try each pattern in order
        for (pattern_index, pattern) in self.task_id_patterns.iter().enumerate() {
            for capture in pattern.captures_iter(message) {
                if let Some(task_id_match) = capture.get(1) {
                    let position = task_id_match.range();

                    // Check if this range overlaps with already used positions
                    let overlaps = used_positions
                        .iter()
                        .any(|used| position.start < used.end && position.end > used.start);

                    if overlaps {
                        continue;
                    }

                    let task_id = task_id_match.as_str().to_string();
                    let format = match pattern_index {
                        0 => TaskIdFormat::Brackets,
                        1 => TaskIdFormat::Colon,
                        2 => TaskIdFormat::Refs,
                        _ => TaskIdFormat::Custom(
                            self.config.task_id_patterns[pattern_index].name.clone(),
                        ),
                    };

                    task_ids.push(ParsedTaskInfo { task_id, format });
                    used_positions.push(position);
                }
            }
        }

        Ok(task_ids)
    }

    /// Validate commit message format
    pub fn validate_message(&self, message: &str) -> Result<Vec<String>, EngramError> {
        let mut errors = Vec::new();

        // Check for basic commit message requirements
        if message.trim().is_empty() {
            errors.push("Commit message cannot be empty".to_string());
            return Ok(errors);
        }

        // Check line length (git convention)
        let lines: Vec<&str> = message.lines().collect();
        if let Some(first_line) = lines.first() {
            if first_line.len() > 72 {
                errors.push("First line should be 72 characters or less".to_string());
            }
        }

        // Check for task ID requirement
        if !self.config.should_exempt(message, "require_task_reference") {
            let has_task_id = self
                .task_id_patterns
                .iter()
                .any(|pattern| pattern.is_match(message));

            if !has_task_id {
                errors.push(format!(
                    "Commit message must reference a task. Supported formats:\n{}",
                    self.config.get_help_examples()
                ));
            }
        }

        // Check for common commit message issues
        if message.ends_with('.') && message.lines().count() == 1 {
            errors.push("First line should not end with a period".to_string());
        }

        if message.starts_with(' ') {
            errors.push("Commit message should not start with whitespace".to_string());
        }

        Ok(errors)
    }

    /// Get help text for supported task ID formats
    pub fn get_help_text(&self) -> String {
        self.config.get_help_examples()
    }

    /// Check if a message matches any exemption pattern
    pub fn is_exempt(&self, message: &str) -> bool {
        self.config.should_exempt(message, "all")
    }

    /// Extract commit type (feat, fix, docs, etc.)
    pub fn extract_commit_type(&self, message: &str) -> Option<String> {
        let first_line = message.lines().next()?.trim();

        // Common conventional commit types
        let types = [
            "feat", "fix", "docs", "style", "refactor", "test", "chore", "perf", "ci", "build",
        ];

        for commit_type in types {
            if first_line.starts_with(commit_type) {
                return Some(commit_type.to_string());
            }
        }

        None
    }

    /// Extract commit scope from conventional commit format
    pub fn extract_commit_scope(&self, message: &str) -> Option<String> {
        let first_line = message.lines().next()?.trim();

        // Look for scope in format: type(scope): description
        let scope_regex = Regex::new(r"^[a-z]+\(([^)]+)\):").ok()?;
        if let Some(captures) = scope_regex.captures(first_line) {
            captures.get(1).map(|m| m.as_str().to_string())
        } else {
            None
        }
    }

    /// Parse conventional commit structure
    pub fn parse_conventional_commit(&self, message: &str) -> Option<ConventionalCommit> {
        let first_line = message.lines().next()?.trim();

        // Regex for conventional commits: type(scope)!: description
        let regex = Regex::new(r"^([a-z]+)(?:\(([^)]+)\))?(!)?:\s+(.+)$").ok()?;
        let captures = regex.captures(first_line)?;

        let commit_type = captures.get(1)?.as_str().to_string();
        let scope = captures.get(2).map(|m| m.as_str().to_string());
        let breaking_change = captures.get(3).is_some();
        let description = captures.get(4)?.as_str().to_string();

        // Extract task IDs
        let task_ids = self.parse_all_task_ids(message).ok().unwrap_or_default();

        Some(ConventionalCommit {
            commit_type,
            scope,
            breaking_change,
            description,
            task_ids,
            body: self.extract_body(message),
        })
    }

    /// Extract commit body (everything after first line)
    fn extract_body(&self, message: &str) -> Option<String> {
        let lines: Vec<&str> = message.lines().collect();
        if lines.len() <= 1 {
            return None;
        }

        let body_lines = lines[1..].join("\n");
        if body_lines.trim().is_empty() {
            None
        } else {
            Some(body_lines.trim().to_string())
        }
    }
}

/// Parsed conventional commit structure
#[derive(Debug, Clone)]
pub struct ConventionalCommit {
    pub commit_type: String,
    pub scope: Option<String>,
    pub breaking_change: bool,
    pub description: String,
    pub task_ids: Vec<ParsedTaskInfo>,
    pub body: Option<String>,
}

impl ConventionalCommit {
    /// Get formatted representation
    pub fn format(&self) -> String {
        let mut result = String::new();

        result.push_str(&self.commit_type);

        if let Some(scope) = &self.scope {
            result.push('(');
            result.push_str(scope);
            result.push(')');
        }

        if self.breaking_change {
            result.push('!');
        }

        result.push_str(": ");
        result.push_str(&self.description);

        // Add task IDs to description if not already included
        if !self.task_ids.is_empty() && !result.contains('[') {
            result.push(' ');
            result.push_str(&self.task_ids[0].task_id);
        }

        result
    }
}

impl Default for CommitMessageParser {
    fn default() -> Self {
        Self::new().expect("Failed to create default parser")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_brackets_format() {
        let parser = CommitMessageParser::new().unwrap();
        let result = parser
            .parse_task_id("feat: implement user authentication [TASK-123]")
            .unwrap();

        assert!(result.is_some());
        let parsed = result.unwrap();
        assert_eq!(parsed.task_id, "TASK-123");
        assert!(matches!(parsed.format, TaskIdFormat::Brackets));
    }

    #[test]
    fn test_parse_colon_format() {
        let parser = CommitMessageParser::new().unwrap();
        let result = parser
            .parse_task_id("fix: resolve database issue [task:auth-impl-001]")
            .unwrap();

        assert!(result.is_some());
        let parsed = result.unwrap();
        assert_eq!(parsed.task_id, "auth-impl-001");
        assert!(matches!(parsed.format, TaskIdFormat::Colon));
    }

    #[test]
    fn test_parse_refs_format() {
        let parser = CommitMessageParser::new().unwrap();
        let result = parser
            .parse_task_id("feat: add new feature\n\nRefs: #456")
            .unwrap();

        assert!(result.is_some());
        let parsed = result.unwrap();
        assert_eq!(parsed.task_id, "456");
        assert!(matches!(parsed.format, TaskIdFormat::Refs));
    }

    #[test]
    fn test_no_task_id() {
        let parser = CommitMessageParser::new().unwrap();
        let result = parser.parse_task_id("chore: update dependencies").unwrap();

        assert!(result.is_none());
    }

    #[test]
    fn test_exempt_commit() {
        let parser = CommitMessageParser::new().unwrap();
        let result = parser.parse_task_id("chore: update dependencies").unwrap();

        // Should be exempt from task requirement but still return None
        assert!(result.is_none());
    }

    #[test]
    fn test_conventional_commit_parsing() {
        let parser = CommitMessageParser::new().unwrap();
        let commit = parser.parse_conventional_commit(
            "feat(auth): add login endpoint [TASK-123]\n\nImplementation details",
        );

        assert!(commit.is_some());
        let parsed = commit.unwrap();
        assert_eq!(parsed.commit_type, "feat");
        assert_eq!(parsed.scope, Some("auth".to_string()));
        assert_eq!(parsed.description, "add login endpoint [TASK-123]");
        assert!(parsed.body.is_some());
        assert_eq!(parsed.task_ids.len(), 1);
    }

    #[test]
    fn test_multiple_task_ids() {
        let parser = CommitMessageParser::new().unwrap();
        let result = parser
            .parse_all_task_ids("feat: implement [TASK-123] and related to [TASK-456]")
            .unwrap();

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].task_id, "TASK-123");
        assert_eq!(result[1].task_id, "TASK-456");
    }
}

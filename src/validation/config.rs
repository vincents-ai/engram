//! Configuration for pre-commit hook validation

use crate::error::EngramError;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Configuration for validation rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationConfig {
    /// Whether validation is enabled
    pub enabled: bool,

    /// Require task reference in commit message
    pub require_task_reference: bool,

    /// Require reasoning relationship for tasks
    pub require_reasoning_relationship: bool,

    /// Require context relationship for tasks
    pub require_context_relationship: bool,

    /// Require file scope to match task memories
    pub require_file_scope_match: bool,

    /// Supported task ID patterns
    pub task_id_patterns: Vec<TaskIdPattern>,

    /// Exemptions from validation
    pub exemptions: Vec<ValidationExemption>,

    /// Performance settings
    pub performance: PerformanceConfig,
}

/// Pattern for matching task IDs in commit messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskIdPattern {
    /// Regex pattern to match task ID
    pub pattern: String,
    /// Pattern name for error messages
    pub name: String,
    /// Example of the format
    pub example: String,
}

/// Exemption rules for validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationExemption {
    /// Pattern to match commit messages
    pub message_pattern: String,
    /// Whether to skip all validation
    pub skip_validation: bool,
    /// Specific validations to skip
    pub skip_specific: Vec<String>,
}

/// Performance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// Cache TTL in seconds
    pub cache_ttl_seconds: u64,

    /// Maximum cache size
    pub max_cache_entries: usize,

    /// Enable parallel validation
    pub enable_parallel_validation: bool,

    /// Timeout for validation in seconds
    pub validation_timeout_seconds: u64,
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            require_task_reference: true,
            require_reasoning_relationship: true,
            require_context_relationship: true,
            require_file_scope_match: true,
            task_id_patterns: vec![
                TaskIdPattern {
                    pattern: r"\[([0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12})\]"
                        .to_string(),
                    name: "UUID format".to_string(),
                    example: "[69190cf0-243a-4979-b4c1-604ba48f72eb]".to_string(),
                },
                TaskIdPattern {
                    pattern: r"\[([A-Z]+-\d+)\]".to_string(),
                    name: "Brackets format".to_string(),
                    example: "[TASK-123]".to_string(),
                },
                TaskIdPattern {
                    pattern: r"\[task:([a-z0-9-]+)\]".to_string(),
                    name: "Colon format".to_string(),
                    example: "[task:auth-impl-001]".to_string(),
                },
                TaskIdPattern {
                    pattern: r"Refs:\s*#(\d+)".to_string(),
                    name: "Refs format".to_string(),
                    example: "Refs: #456".to_string(),
                },
            ],
            exemptions: vec![
                ValidationExemption {
                    message_pattern: r"^(chore|docs):".to_string(),
                    skip_validation: false,
                    skip_specific: vec!["require_task_reference".to_string()],
                },
                ValidationExemption {
                    message_pattern: r"^fixup!".to_string(),
                    skip_validation: true,
                    skip_specific: vec![],
                },
                ValidationExemption {
                    message_pattern: r"^amend!".to_string(),
                    skip_validation: true,
                    skip_specific: vec![],
                },
            ],
            performance: PerformanceConfig::default(),
        }
    }
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            cache_ttl_seconds: 300, // 5 minutes
            max_cache_entries: 1000,
            enable_parallel_validation: true,
            validation_timeout_seconds: 30,
        }
    }
}

impl ValidationConfig {
    /// Load configuration from file
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, EngramError> {
        let content = std::fs::read_to_string(path).map_err(|e| EngramError::Io(e))?;

        let config: Self = serde_yaml::from_str(&content).map_err(EngramError::Yaml)?;

        Ok(config)
    }

    /// Save configuration to file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), EngramError> {
        let content = serde_yaml::to_string(self)
            .map_err(|e| EngramError::Validation(format!("Failed to serialize config: {}", e)))?;

        std::fs::write(path, content).map_err(|e| EngramError::Io(e))?;

        Ok(())
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), EngramError> {
        // Validate task ID patterns
        for pattern in &self.task_id_patterns {
            if let Err(e) = regex::Regex::new(&pattern.pattern) {
                return Err(EngramError::Validation(format!(
                    "Invalid task ID pattern '{}': {}",
                    pattern.name, e
                )));
            }
        }

        // Validate exemption patterns
        for exemption in &self.exemptions {
            if let Err(e) = regex::Regex::new(&exemption.message_pattern) {
                return Err(EngramError::Validation(format!(
                    "Invalid exemption pattern: {}",
                    e
                )));
            }
        }

        // Validate performance settings
        if self.performance.cache_ttl_seconds == 0 {
            return Err(EngramError::Validation(
                "Cache TTL must be greater than 0".to_string(),
            ));
        }

        if self.performance.max_cache_entries == 0 {
            return Err(EngramError::Validation(
                "Max cache entries must be greater than 0".to_string(),
            ));
        }

        if self.performance.validation_timeout_seconds == 0 {
            return Err(EngramError::Validation(
                "Validation timeout must be greater than 0".to_string(),
            ));
        }

        Ok(())
    }

    /// Check if a commit message should be exempted from validation
    pub fn should_exempt(&self, message: &str, validation_type: &str) -> bool {
        for exemption in &self.exemptions {
            if let Ok(regex) = regex::Regex::new(&exemption.message_pattern) {
                if regex.is_match(message) {
                    if exemption.skip_validation {
                        return true;
                    }
                    if exemption
                        .skip_specific
                        .contains(&validation_type.to_string())
                    {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Get all task ID regex patterns
    pub fn get_task_regexes(&self) -> Result<Vec<regex::Regex>, EngramError> {
        self.task_id_patterns
            .iter()
            .map(|p| {
                regex::Regex::new(&p.pattern).map_err(|e| {
                    EngramError::Validation(format!("Invalid pattern '{}': {}", p.name, e))
                })
            })
            .collect()
    }

    /// Get a sample commit message format for help
    pub fn get_help_examples(&self) -> String {
        let mut examples = vec!["Supported task ID formats:".to_string()];

        for pattern in &self.task_id_patterns {
            examples.push(format!("  - {}: {}", pattern.name, pattern.example));
        }

        examples.push("\nExemptions:".to_string());
        for exemption in &self.exemptions {
            if exemption.skip_validation {
                examples.push(format!(
                    "  - '{}' (skip all validation)",
                    exemption.message_pattern
                ));
            } else if !exemption.skip_specific.is_empty() {
                examples.push(format!(
                    "  - '{}' (skip: {})",
                    exemption.message_pattern,
                    exemption.skip_specific.join(", ")
                ));
            }
        }

        examples.join("\n")
    }
}

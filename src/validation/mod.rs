//! Pre-commit hook validation system for Engram
//!
//! Provides comprehensive validation of git commits to ensure they follow
//! disciplined development practices with proper task referencing and
//! relationship requirements.

pub mod config;
pub mod hook;
pub mod parser;
pub mod quality_gates;
pub mod stage_transitions;
pub mod validator;
pub mod workflow_validator;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use config::ValidationConfig;
pub use hook::HookManager;
pub use parser::{CommitMessageParser, ConventionalCommit};
pub use quality_gates::{BuiltinValidators, QualityGate, QualityGatesExecutor};
pub use stage_transitions::{
    StageTransitionManager, StageTransitionRule, TransitionCondition, TransitionEligibility,
};
pub use validator::CommitValidator;
pub use workflow_validator::{StagePolicy, WorkflowValidator};

/// Result of commit validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub valid: bool,
    pub errors: Vec<ValidationError>,
    pub task_id: Option<String>,
    pub validated_relationships: Vec<String>,
    pub validated_files: Vec<String>,
    pub validation_time_ms: u64,
}

/// Individual validation error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    pub error_type: ValidationErrorType,
    pub message: String,
    pub suggestion: Option<String>,
}

/// Types of validation errors
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ValidationErrorType {
    NoTaskReference,
    TaskNotFound,
    MissingRequiredRelationship,
    FileScopeMismatch,
    InvalidTaskIdFormat,
    HookNotInstalled,
    ConfigurationError,
    QualityGateFailed,
    PolicyViolation,
    Other,
}

/// Parsed task information from commit message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedTaskInfo {
    pub task_id: String,
    pub format: TaskIdFormat,
}

/// Supported task ID formats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskIdFormat {
    Brackets, // [TASK-123]
    Colon,    // task:auth-impl-001
    Refs,     // Refs: #456
    Custom(String),
}

/// Performance cache for validation results
#[derive(Debug, Default)]
pub struct ValidationCache {
    task_cache: HashMap<String, CachedTaskInfo>,
    file_cache: HashMap<String, Vec<String>>,
}

/// Cached task information for performance
#[derive(Debug, Clone)]
pub struct CachedTaskInfo {
    pub relationships: Vec<String>,
    pub allowed_files: Vec<String>,
    pub cached_at: std::time::Instant,
    pub ttl: std::time::Duration,
}

impl ValidationResult {
    /// Create a successful validation result
    pub fn success(
        task_id: String,
        validated_relationships: Vec<String>,
        validated_files: Vec<String>,
        validation_time_ms: u64,
    ) -> Self {
        Self {
            valid: true,
            errors: Vec::new(),
            task_id: Some(task_id),
            validated_relationships,
            validated_files,
            validation_time_ms,
        }
    }

    /// Create a failed validation result
    pub fn failure(errors: Vec<ValidationError>, validation_time_ms: u64) -> Self {
        Self {
            valid: false,
            errors,
            task_id: None,
            validated_relationships: Vec::new(),
            validated_files: Vec::new(),
            validation_time_ms,
        }
    }

    /// Add an error to an existing result
    pub fn with_error(mut self, error: ValidationError) -> Self {
        self.errors.push(error);
        self.valid = false;
        self
    }

    /// Get a formatted error message
    pub fn error_summary(&self) -> String {
        if self.errors.is_empty() {
            return "✅ Validation passed".to_string();
        }

        let mut messages = vec!["❌ Validation failed:".to_string()];
        for (i, error) in self.errors.iter().enumerate() {
            messages.push(format!("  {}. {}", i + 1, error.message));
            if let Some(suggestion) = &error.suggestion {
                messages.push(format!("     💡 {}", suggestion));
            }
        }
        messages.join("\n")
    }
}

impl ValidationError {
    /// Create a new validation error
    pub fn new(error_type: ValidationErrorType, message: String) -> Self {
        Self {
            error_type,
            message,
            suggestion: None,
        }
    }

    /// Add a suggestion to the error
    pub fn with_suggestion(mut self, suggestion: String) -> Self {
        self.suggestion = Some(suggestion);
        self
    }
}

impl ValidationCache {
    /// Create a new validation cache
    pub fn new() -> Self {
        Self::default()
    }

    /// Get cached task information
    pub fn get_task_info(&self, task_id: &str) -> Option<&CachedTaskInfo> {
        self.task_cache.get(task_id).and_then(|info| {
            if info.cached_at.elapsed() < info.ttl {
                Some(info)
            } else {
                None
            }
        })
    }

    /// Cache task information
    pub fn cache_task_info(&mut self, task_id: String, info: CachedTaskInfo) {
        self.task_cache.insert(task_id, info);
    }

    /// Get cached file list for a directory
    pub fn get_files(&self, dir_hash: &str) -> Option<&Vec<String>> {
        self.file_cache.get(dir_hash)
    }

    /// Cache file list for a directory
    pub fn cache_files(&mut self, dir_hash: String, files: Vec<String>) {
        self.file_cache.insert(dir_hash, files);
    }

    /// Clear expired cache entries
    pub fn cleanup_expired(&mut self) {
        self.task_cache
            .retain(|_, info| info.cached_at.elapsed() < info.ttl);
        // Note: file cache doesn't expire as often since file changes are handled differently
    }
}

impl CachedTaskInfo {
    /// Create new cached task info with default TTL
    pub fn new(relationships: Vec<String>, allowed_files: Vec<String>) -> Self {
        Self {
            relationships,
            allowed_files,
            cached_at: std::time::Instant::now(),
            ttl: std::time::Duration::from_secs(300), // 5 minutes
        }
    }

    /// Create new cached task info with custom TTL
    pub fn with_ttl(
        relationships: Vec<String>,
        allowed_files: Vec<String>,
        ttl: std::time::Duration,
    ) -> Self {
        Self {
            relationships,
            allowed_files,
            cached_at: std::time::Instant::now(),
            ttl,
        }
    }

    /// Check if the cache entry is still valid
    pub fn is_valid(&self) -> bool {
        self.cached_at.elapsed() < self.ttl
    }
}

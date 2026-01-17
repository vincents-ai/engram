//! Core validation engine for commit validation

use crate::error::EngramError;
use crate::storage::{RelationshipStorage, Storage};
use crate::validation::{
    config::ValidationConfig, parser::CommitMessageParser, CachedTaskInfo, ValidationCache,
    ValidationError, ValidationErrorType, ValidationResult,
};
use std::time::Instant;

/// Main commit validator
pub struct CommitValidator<S: Storage + RelationshipStorage> {
    storage: S,
    config: ValidationConfig,
    parser: CommitMessageParser,
    cache: ValidationCache,
}

impl<S: Storage + RelationshipStorage> CommitValidator<S> {
    /// Create a new validator with default configuration
    pub fn new(storage: S) -> Result<Self, EngramError> {
        let config = ValidationConfig::default();
        Self::with_config(storage, config)
    }

    /// Create a new validator with custom configuration
    pub fn with_config(storage: S, config: ValidationConfig) -> Result<Self, EngramError> {
        let parser = CommitMessageParser::with_config(config.clone())?;
        Ok(Self {
            storage,
            config,
            parser,
            cache: ValidationCache::new(),
        })
    }

    /// Validate a commit with staged changes
    pub fn validate_commit(
        &mut self,
        commit_message: &str,
        staged_files: &[String],
    ) -> ValidationResult {
        let start_time = Instant::now();

        // Parse task ID from commit message
        let task_info = match self.parser.parse_task_id(commit_message) {
            Ok(Some(info)) => info,
            Ok(None) => {
                if self.config.require_task_reference && !self.parser.is_exempt(commit_message) {
                    return ValidationResult::failure(
                        vec![ValidationError::new(
                            ValidationErrorType::NoTaskReference,
                            "Commit message must reference a task".to_string(),
                        )
                        .with_suggestion(
                            "Use formats like [TASK-123], [task:auth-impl-001], or Refs: #456"
                                .to_string(),
                        )],
                        start_time.elapsed().as_millis() as u64,
                    );
                } else {
                    // Exempt commit - pass validation
                    return ValidationResult::success(
                        "exempt".to_string(),
                        vec![],
                        vec![],
                        start_time.elapsed().as_millis() as u64,
                    );
                }
            }
            Err(e) => {
                return ValidationResult::failure(
                    vec![ValidationError::new(
                        ValidationErrorType::InvalidTaskIdFormat,
                        format!("Failed to parse task ID: {}", e),
                    )],
                    start_time.elapsed().as_millis() as u64,
                );
            }
        };

        // Validate task exists and has required relationships
        let (validated_relationships, errors) =
            self.validate_task_relationships(&task_info.task_id);
        if !errors.is_empty() {
            return ValidationResult::failure(errors, start_time.elapsed().as_millis() as u64);
        }

        // Validate file scope matches task context
        let (validated_files, errors) = if self.config.require_file_scope_match {
            self.validate_file_scope(&task_info.task_id, staged_files)
        } else {
            (staged_files.to_vec(), vec![])
        };

        if !errors.is_empty() {
            return ValidationResult::failure(errors, start_time.elapsed().as_millis() as u64);
        }

        ValidationResult::success(
            task_info.task_id,
            validated_relationships,
            validated_files,
            start_time.elapsed().as_millis() as u64,
        )
    }

    /// Validate task exists and has required relationships
    fn validate_task_relationships(
        &mut self,
        task_id: &str,
    ) -> (Vec<String>, Vec<ValidationError>) {
        let mut validated_relationships = Vec::new();
        let mut errors = Vec::new();

        // Check cache first
        if let Some(cached_info) = self.cache.get_task_info(task_id) {
            if self.config.require_reasoning_relationship
                && !cached_info.relationships.contains(&"reasoning".to_string())
            {
                errors.push(
                    ValidationError::new(
                        ValidationErrorType::MissingRequiredRelationship,
                        "Task must have a reasoning relationship".to_string(),
                    )
                    .with_suggestion("Create a reasoning entity linked to this task".to_string()),
                );
            }

            if self.config.require_context_relationship
                && !cached_info.relationships.contains(&"context".to_string())
            {
                errors.push(
                    ValidationError::new(
                        ValidationErrorType::MissingRequiredRelationship,
                        "Task must have a context relationship".to_string(),
                    )
                    .with_suggestion("Create a context entity linked to this task".to_string()),
                );
            }

            if errors.is_empty() {
                validated_relationships = cached_info.relationships.clone();
            }

            return (validated_relationships, errors);
        }

        // Check if task exists in storage
        let _task = match self.storage.get(task_id, "task") {
            Ok(Some(entity)) => entity,
            Ok(None) => {
                errors.push(
                    ValidationError::new(
                        ValidationErrorType::TaskNotFound,
                        format!("Task '{}' not found in Engram", task_id),
                    )
                    .with_suggestion("Create the task in Engram before committing".to_string()),
                );
                return (validated_relationships, errors);
            }
            Err(_) => {
                errors.push(ValidationError::new(
                    ValidationErrorType::ConfigurationError,
                    "Failed to access Engram storage".to_string(),
                ));
                return (validated_relationships, errors);
            }
        };

        // Get task relationships
        let relationships = match self.storage.get_entity_relationships(task_id) {
            Ok(rels) => rels,
            Err(_) => {
                errors.push(ValidationError::new(
                    ValidationErrorType::ConfigurationError,
                    "Failed to access task relationships".to_string(),
                ));
                return (validated_relationships, errors);
            }
        };

        let mut relationship_types = Vec::new();
        for rel in &relationships {
            let target_type = rel.target_type.clone();
            relationship_types.push(target_type.clone());
            validated_relationships.push(format!("{}:{}", rel.relationship_type, target_type));
        }

        // Check required relationships
        if self.config.require_reasoning_relationship
            && !relationship_types.iter().any(|t| t == "reasoning")
        {
            errors.push(
                ValidationError::new(
                    ValidationErrorType::MissingRequiredRelationship,
                    "Task must have a reasoning relationship".to_string(),
                )
                .with_suggestion("Create a reasoning entity linked to this task".to_string()),
            );
        }

        if self.config.require_context_relationship
            && !relationship_types.iter().any(|t| t == "context")
        {
            errors.push(
                ValidationError::new(
                    ValidationErrorType::MissingRequiredRelationship,
                    "Task must have a context relationship".to_string(),
                )
                .with_suggestion("Create a context entity linked to this task".to_string()),
            );
        }

        // Cache the results
        let cached_info = CachedTaskInfo::new(relationship_types, vec![]);
        self.cache.cache_task_info(task_id.to_string(), cached_info);

        (validated_relationships, errors)
    }

    /// Validate that changed files are within task scope
    fn validate_file_scope(
        &mut self,
        _task_id: &str,
        staged_files: &[String],
    ) -> (Vec<String>, Vec<ValidationError>) {
        let mut validated_files = Vec::new();
        let mut errors = Vec::new();

        // For now, accept all files. In a full implementation, this would:
        // 1. Get the task's context and reasoning entities
        // 2. Extract file references from those entities
        // 3. Validate that staged files are in the allowed scope
        // 4. Check for unexpected files

        // Basic validation: check for file types that should have documentation
        for file in staged_files {
            // Allow all files for now, but could add rules like:
            // - .rs files should have tests
            // - API changes should have documentation
            // - Database changes should have migration files

            validated_files.push(file.clone());
        }

        (validated_files, errors)
    }

    /// Get staged files from git
    pub fn get_staged_files(&self) -> Result<Vec<String>, EngramError> {
        // Temporarily return empty list to avoid git2 compilation issues
        Ok(vec![])
    }

    /// Check if validation is enabled
    pub fn is_enabled(&self) -> bool {
        self.config.enabled
    }

    /// Get configuration
    pub fn get_config(&self) -> &ValidationConfig {
        &self.config
    }

    /// Update configuration
    pub fn update_config(&mut self, config: ValidationConfig) -> Result<(), EngramError> {
        config.validate()?;
        self.parser = CommitMessageParser::with_config(config.clone())?;
        self.config = config;
        self.cache.cleanup_expired();
        Ok(())
    }

    /// Clear cache
    pub fn clear_cache(&mut self) {
        self.cache = ValidationCache::new();
    }

    /// Get cache statistics
    pub fn get_cache_stats(&self) -> CacheStats {
        CacheStats {
            task_cache_size: self.cache.task_cache.len(),
            file_cache_size: self.cache.file_cache.len(),
        }
    }

    /// Warm up cache with common task IDs
    pub fn warm_cache(&mut self, task_ids: &[String]) -> Result<(), EngramError> {
        for task_id in task_ids {
            // Check if already cached
            if self.cache.get_task_info(task_id).is_none() {
                // Cache the task info
                let _task_info = self.validate_task_relationships(task_id);
            }
        }
        Ok(())
    }
}

/// Cache statistics
#[derive(Debug)]
pub struct CacheStats {
    pub task_cache_size: usize,
    pub file_cache_size: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::MemoryStorage;

    #[test]
    fn test_validate_commit_with_task() {
        // This test would require setting up storage with test data
        // For now, we'll test the basic structure
        let storage = MemoryStorage::new("test");
        let mut validator = CommitValidator::new(storage).unwrap();

        assert!(validator.is_enabled());
    }

    #[test]
    fn test_validate_commit_without_task() {
        let storage = MemoryStorage::new("test");
        let mut validator = CommitValidator::new(storage).unwrap();

        let result = validator.validate_commit("chore: update dependencies", &vec![]);
        assert!(result.valid); // Should be exempt
    }
}

//! ExecutionResult entity implementation
//!
//! Stores results from quality gate execution including command output,
//! timing, environment context, and validation status.

use super::{Entity, EntityResult, GenericEntity};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use validator::Validate;

/// Validation status for quality gate execution
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ValidationStatus {
    /// Quality gate passed successfully
    Passed,
    /// Quality gate failed with reason
    Failed { reason: String },
    /// Quality gate was skipped with reason
    Skipped { reason: String },
}

/// Expected result type for quality gates
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ExpectedResult {
    /// Command should succeed (exit code 0)
    Success,
    /// Command should fail (non-zero exit code) - for BDD RED phase
    Failure,
    /// Any result is acceptable
    Any,
}

/// Execution result entity for quality gate output
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ExecutionResult {
    /// Unique identifier
    #[serde(rename = "id")]
    pub id: String,

    /// Associated task ID
    #[serde(rename = "task_id")]
    pub task_id: String,

    /// Workflow stage when executed
    #[serde(rename = "workflow_stage")]
    pub workflow_stage: String,

    /// Command that was executed
    #[serde(rename = "command")]
    pub command: String,

    /// Command exit code
    #[serde(rename = "exit_code")]
    pub exit_code: i32,

    /// Standard output
    #[serde(rename = "stdout")]
    pub stdout: String,

    /// Standard error
    #[serde(rename = "stderr")]
    pub stderr: String,

    /// Execution timestamp
    #[serde(rename = "timestamp")]
    pub timestamp: DateTime<Utc>,

    /// Execution duration in milliseconds
    #[serde(rename = "duration_ms")]
    pub duration_ms: u64,

    /// Environment variables at time of execution
    #[serde(
        rename = "environment",
        skip_serializing_if = "HashMap::is_empty",
        default
    )]
    pub environment: HashMap<String, String>,

    /// Files that were changed during execution
    #[serde(
        rename = "file_changes",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub file_changes: Vec<String>,

    /// Expected result type
    #[serde(rename = "expected_result")]
    pub expected_result: Option<ExpectedResult>,

    /// Validation status
    #[serde(rename = "validation_status")]
    pub validation_status: ValidationStatus,

    /// Associated agent
    #[serde(rename = "agent")]
    pub agent: String,

    /// Working directory
    #[serde(rename = "working_directory", skip_serializing_if = "Option::is_none")]
    pub working_directory: Option<String>,

    /// Additional metadata
    #[serde(
        rename = "metadata",
        skip_serializing_if = "HashMap::is_empty",
        default
    )]
    pub metadata: HashMap<String, serde_json::Value>,

    /// Tags for categorization
    #[serde(rename = "tags", skip_serializing_if = "Vec::is_empty", default)]
    pub tags: Vec<String>,

    /// Quality gate name/identifier
    #[serde(rename = "quality_gate")]
    pub quality_gate: String,

    /// Retry count if this execution was retried
    #[serde(rename = "retry_count", default)]
    pub retry_count: u32,

    /// Previous execution result ID if this is a retry
    #[serde(
        rename = "previous_execution_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub previous_execution_id: Option<String>,
}

impl ExecutionResult {
    /// Create a new execution result
    pub fn new(
        task_id: String,
        workflow_stage: String,
        quality_gate: String,
        command: String,
        agent: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            task_id,
            workflow_stage,
            command,
            exit_code: 0,
            stdout: String::new(),
            stderr: String::new(),
            timestamp: Utc::now(),
            duration_ms: 0,
            environment: HashMap::new(),
            file_changes: Vec::new(),
            expected_result: Some(ExpectedResult::Success),
            validation_status: ValidationStatus::Passed,
            agent,
            working_directory: None,
            metadata: HashMap::new(),
            tags: Vec::new(),
            quality_gate,
            retry_count: 0,
            previous_execution_id: None,
        }
    }

    /// Set execution results
    pub fn set_results(
        &mut self,
        exit_code: i32,
        stdout: String,
        stderr: String,
        duration_ms: u64,
    ) {
        self.exit_code = exit_code;
        self.stdout = stdout;
        self.stderr = stderr;
        self.duration_ms = duration_ms;
        self.timestamp = Utc::now();

        self.update_validation_status();
    }

    /// Update validation status based on exit code and expected result
    pub fn update_validation_status(&mut self) {
        match &self.expected_result {
            Some(ExpectedResult::Success) => {
                if self.exit_code == 0 {
                    self.validation_status = ValidationStatus::Passed;
                } else {
                    self.validation_status = ValidationStatus::Failed {
                        reason: format!(
                            "Command failed with exit code {} (expected success)",
                            self.exit_code
                        ),
                    };
                }
            }
            Some(ExpectedResult::Failure) => {
                if self.exit_code != 0 {
                    self.validation_status = ValidationStatus::Passed;
                } else {
                    self.validation_status = ValidationStatus::Failed {
                        reason: "Command succeeded but failure was expected (BDD RED phase)"
                            .to_string(),
                    };
                }
            }
            Some(ExpectedResult::Any) => {
                self.validation_status = ValidationStatus::Passed;
            }
            None => {
                if self.exit_code == 0 {
                    self.validation_status = ValidationStatus::Passed;
                } else {
                    self.validation_status = ValidationStatus::Failed {
                        reason: format!("Command failed with exit code {}", self.exit_code),
                    };
                }
            }
        }
    }

    /// Set environment variables
    pub fn set_environment(&mut self, env: HashMap<String, String>) {
        self.environment = env;
    }

    /// Add file change
    pub fn add_file_change(&mut self, file_path: String) {
        if !self.file_changes.contains(&file_path) {
            self.file_changes.push(file_path);
        }
    }

    /// Set expected result
    pub fn set_expected_result(&mut self, expected: ExpectedResult) {
        self.expected_result = Some(expected);
        self.update_validation_status();
    }

    /// Skip execution with reason
    pub fn skip_execution(&mut self, reason: String) {
        self.validation_status = ValidationStatus::Skipped { reason };
        self.timestamp = Utc::now();
    }

    /// Mark as retry
    pub fn mark_as_retry(&mut self, previous_id: String, retry_count: u32) {
        self.previous_execution_id = Some(previous_id);
        self.retry_count = retry_count;
    }

    /// Add metadata
    pub fn add_metadata(&mut self, key: String, value: serde_json::Value) {
        self.metadata.insert(key, value);
    }

    /// Add tag
    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
    }

    /// Check if execution passed
    pub fn passed(&self) -> bool {
        matches!(self.validation_status, ValidationStatus::Passed)
    }

    /// Check if execution failed
    pub fn failed(&self) -> bool {
        matches!(self.validation_status, ValidationStatus::Failed { .. })
    }

    /// Check if execution was skipped
    pub fn skipped(&self) -> bool {
        matches!(self.validation_status, ValidationStatus::Skipped { .. })
    }

    /// Get failure reason if failed
    pub fn failure_reason(&self) -> Option<&str> {
        match &self.validation_status {
            ValidationStatus::Failed { reason } => Some(reason),
            _ => None,
        }
    }

    /// Get skip reason if skipped
    pub fn skip_reason(&self) -> Option<&str> {
        match &self.validation_status {
            ValidationStatus::Skipped { reason } => Some(reason),
            _ => None,
        }
    }
}

impl Entity for ExecutionResult {
    fn entity_type() -> &'static str {
        "execution_result"
    }

    fn id(&self) -> &str {
        &self.id
    }

    fn agent(&self) -> &str {
        &self.agent
    }

    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }

    fn validate_entity(&self) -> super::EntityResult<()> {
        if let Err(errors) = <ExecutionResult as validator::Validate>::validate(self) {
            let error_messages: Vec<String> = errors
                .field_errors()
                .values()
                .flat_map(|field_errors| field_errors.iter())
                .map(|error| {
                    error
                        .message
                        .clone()
                        .map(|s| s.to_string())
                        .unwrap_or_default()
                })
                .collect();
            return Err(error_messages.join(", "));
        }

        if self.task_id.is_empty() {
            return Err("ExecutionResult must have a task_id".to_string());
        }

        if self.workflow_stage.is_empty() {
            return Err("ExecutionResult must have a workflow_stage".to_string());
        }

        if self.command.is_empty() {
            return Err("ExecutionResult must have a command".to_string());
        }

        if self.quality_gate.is_empty() {
            return Err("ExecutionResult must have a quality_gate identifier".to_string());
        }

        if self.agent.is_empty() {
            return Err("ExecutionResult must have an agent".to_string());
        }

        Ok(())
    }

    fn to_generic(&self) -> GenericEntity {
        GenericEntity {
            id: self.id.clone(),
            entity_type: Self::entity_type().to_string(),
            agent: self.agent.clone(),
            timestamp: self.timestamp,
            data: serde_json::to_value(self).unwrap_or_default(),
        }
    }

    fn from_generic(entity: GenericEntity) -> EntityResult<Self> {
        serde_json::from_value(entity.data)
            .map_err(|e| format!("Failed to deserialize ExecutionResult: {}", e))
    }

    fn as_any(&self) -> &dyn std::any::Any
    where
        Self: Sized,
    {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_execution_result() {
        let result = ExecutionResult::new(
            "task-123".to_string(),
            "development".to_string(),
            "cargo-test".to_string(),
            "cargo test".to_string(),
            "test-agent".to_string(),
        );

        assert_eq!(result.task_id, "task-123");
        assert_eq!(result.workflow_stage, "development");
        assert_eq!(result.quality_gate, "cargo-test");
        assert_eq!(result.command, "cargo test");
        assert_eq!(result.agent, "test-agent");
        assert_eq!(result.exit_code, 0);
        assert!(result.passed());
    }

    #[test]
    fn test_set_results_success() {
        let mut result = ExecutionResult::new(
            "task-123".to_string(),
            "development".to_string(),
            "cargo-test".to_string(),
            "cargo test".to_string(),
            "test-agent".to_string(),
        );

        result.set_results(0, "All tests passed".to_string(), "".to_string(), 5000);

        assert_eq!(result.exit_code, 0);
        assert_eq!(result.stdout, "All tests passed");
        assert_eq!(result.duration_ms, 5000);
        assert!(result.passed());
    }

    #[test]
    fn test_set_results_failure() {
        let mut result = ExecutionResult::new(
            "task-123".to_string(),
            "development".to_string(),
            "cargo-test".to_string(),
            "cargo test".to_string(),
            "test-agent".to_string(),
        );

        result.set_results(1, "".to_string(), "Test failed".to_string(), 3000);

        assert_eq!(result.exit_code, 1);
        assert_eq!(result.stderr, "Test failed");
        assert!(result.failed());
    }

    #[test]
    fn test_bdd_red_phase_validation() {
        let mut result = ExecutionResult::new(
            "task-123".to_string(),
            "bdd".to_string(),
            "cargo-test".to_string(),
            "cargo test".to_string(),
            "test-agent".to_string(),
        );

        result.set_expected_result(ExpectedResult::Failure);
        result.set_results(
            1,
            "".to_string(),
            "Tests failed as expected".to_string(),
            2000,
        );

        assert_eq!(result.exit_code, 1);
        assert!(result.passed());
    }

    #[test]
    fn test_bdd_red_phase_unexpected_success() {
        let mut result = ExecutionResult::new(
            "task-123".to_string(),
            "bdd".to_string(),
            "cargo-test".to_string(),
            "cargo test".to_string(),
            "test-agent".to_string(),
        );

        result.set_expected_result(ExpectedResult::Failure);
        result.set_results(0, "All tests passed".to_string(), "".to_string(), 2000);

        assert_eq!(result.exit_code, 0);
        assert!(result.failed());
        assert_eq!(
            result.failure_reason().unwrap(),
            "Command succeeded but failure was expected (BDD RED phase)"
        );
    }

    #[test]
    fn test_skip_execution() {
        let mut result = ExecutionResult::new(
            "task-123".to_string(),
            "development".to_string(),
            "cargo-test".to_string(),
            "cargo test".to_string(),
            "test-agent".to_string(),
        );

        result.skip_execution("No tests in repository".to_string());

        assert!(result.skipped());
        assert_eq!(result.skip_reason().unwrap(), "No tests in repository");
    }

    #[test]
    fn test_entity_trait() {
        let result = ExecutionResult::new(
            "task-123".to_string(),
            "development".to_string(),
            "cargo-test".to_string(),
            "cargo test".to_string(),
            "test-agent".to_string(),
        );

        assert_eq!(ExecutionResult::entity_type(), "execution_result");
        assert_eq!(result.agent(), "test-agent");
        assert!(result.validate_entity().is_ok());

        let generic = result.to_generic();
        assert_eq!(generic.entity_type, "execution_result");

        let restored = ExecutionResult::from_generic(generic).unwrap();
        assert_eq!(restored.task_id, "task-123");
        assert_eq!(restored.quality_gate, "cargo-test");
    }

    #[test]
    fn test_retry_functionality() {
        let mut result = ExecutionResult::new(
            "task-123".to_string(),
            "development".to_string(),
            "cargo-test".to_string(),
            "cargo test".to_string(),
            "test-agent".to_string(),
        );

        result.mark_as_retry("previous-exec-456".to_string(), 2);

        assert_eq!(result.retry_count, 2);
        assert_eq!(
            result.previous_execution_id.as_ref().unwrap(),
            "previous-exec-456"
        );
    }
}

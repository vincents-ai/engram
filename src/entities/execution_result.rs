//! Execution result entity for storing workflow stage execution outcomes
//!
//! This module provides the `ExecutionResult` entity which captures the output of quality
//! gate executions, including command output, validation status, and execution metadata.
//! This enables workflow engines to track command execution history and share results
//! between agents for collaborative debugging and quality assurance.
//!
//! # Usage Example
//!
//! ```rust
//! use engram::entities::{ExecutionResult, ValidationStatus};
//!
//! // Create a successful test result
//! let result = ExecutionResult::new(
//!     "task-123".to_string(),
//!     "testing".to_string(),
//!     "cargo test".to_string(),
//!     0,
//!     "All tests passed".to_string(),
//!     "".to_string(),
//! );
//! assert_eq!(result.validation_status, ValidationStatus::Passed);
//!
//! // Create with expected failure
//! let failing_result = ExecutionResult::new(
//!     "task-456".to_string(),
//!     "red-phase".to_string(),
//!     "cargo test".to_string(),
//!     1,
//!     "".to_string(),
//!     "Test failed".to_string(),
//! ).with_expected_result("failure".to_string());
//! assert_eq!(failing_result.validation_status, ValidationStatus::Passed);
//! ```

use super::{Entity, GenericEntity, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Execution result entity capturing quality gate execution outcomes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExecutionResult {
    /// Unique identifier for this execution result
    #[serde(rename = "id")]
    pub id: String,

    /// Task ID associated with this execution
    #[serde(rename = "task_id")]
    pub task_id: String,

    /// Workflow stage where this execution occurred
    #[serde(rename = "workflow_stage")]
    pub workflow_stage: String,

    /// Command that was executed
    #[serde(rename = "command")]
    pub command: String,

    /// Exit code from command execution
    #[serde(rename = "exit_code")]
    pub exit_code: i32,

    /// Standard output from command execution
    #[serde(rename = "stdout")]
    pub stdout: String,

    /// Standard error from command execution
    #[serde(rename = "stderr")]
    pub stderr: String,

    /// Timestamp when execution occurred
    #[serde(rename = "timestamp")]
    pub timestamp: DateTime<Utc>,

    /// Duration of execution in milliseconds
    #[serde(rename = "duration_ms")]
    pub duration_ms: u64,

    /// Environment variables during execution
    #[serde(
        rename = "environment",
        skip_serializing_if = "HashMap::is_empty",
        default
    )]
    pub environment: HashMap<String, String>,

    /// Files changed during execution
    #[serde(
        rename = "file_changes",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub file_changes: Vec<String>,

    /// Expected result for this execution ("success", "failure", "any")
    #[serde(rename = "expected_result", skip_serializing_if = "Option::is_none")]
    pub expected_result: Option<String>,

    /// Validation status based on exit code and expected result
    #[serde(rename = "validation_status")]
    pub validation_status: ValidationStatus,

    /// Agent associated with this execution result
    #[serde(rename = "agent")]
    pub agent: String,
}

/// Validation status for execution results
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ValidationStatus {
    /// Execution passed validation
    Passed,
    /// Execution failed validation with reason
    Failed {
        #[serde(rename = "reason")]
        reason: String,
    },
    /// Execution was skipped with reason
    Skipped {
        #[serde(rename = "reason")]
        reason: String,
    },
}

impl ExecutionResult {
    /// Create a new execution result
    ///
    /// # Arguments
    ///
    /// * `task_id` - Task ID associated with this execution
    /// * `workflow_stage` - Workflow stage where execution occurred
    /// * `command` - Command that was executed
    /// * `exit_code` - Exit code from command execution
    /// * `stdout` - Standard output from command
    /// * `stderr` - Standard error from command
    ///
    /// # Returns
    ///
    /// New `ExecutionResult` instance with validation status determined by exit code
    pub fn new(
        task_id: String,
        workflow_stage: String,
        command: String,
        exit_code: i32,
        stdout: String,
        stderr: String,
    ) -> Self {
        let validation_status = if exit_code == 0 {
            ValidationStatus::Passed
        } else {
            ValidationStatus::Failed {
                reason: format!("Command failed with exit code {}", exit_code),
            }
        };

        Self {
            id: Uuid::new_v4().to_string(),
            task_id,
            workflow_stage,
            command,
            exit_code,
            stdout,
            stderr,
            timestamp: Utc::now(),
            duration_ms: 0,
            environment: HashMap::new(),
            file_changes: Vec::new(),
            expected_result: None,
            validation_status,
            agent: "default".to_string(),
        }
    }

    /// Set execution duration
    pub fn with_duration(mut self, duration_ms: u64) -> Self {
        self.duration_ms = duration_ms;
        self
    }

    /// Set environment variables
    pub fn with_environment(mut self, env: HashMap<String, String>) -> Self {
        self.environment = env;
        self
    }

    /// Set expected result and re-evaluate validation status
    pub fn with_expected_result(mut self, expected: String) -> Self {
        self.expected_result = Some(expected.clone());

        // Re-evaluate validation status based on expected result
        self.validation_status = match expected.as_str() {
            "failure" => {
                if self.exit_code != 0 {
                    ValidationStatus::Passed // Expected failure, got failure
                } else {
                    ValidationStatus::Failed {
                        reason: "Expected failure but command succeeded".to_string(),
                    }
                }
            }
            "success" => {
                if self.exit_code == 0 {
                    ValidationStatus::Passed
                } else {
                    ValidationStatus::Failed {
                        reason: format!("Expected success but got exit code {}", self.exit_code),
                    }
                }
            }
            _ => self.validation_status, // Keep existing status for "any" or unknown
        };

        self
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

    fn validate_entity(&self) -> Result<()> {
        if self.task_id.is_empty() {
            return Err("Execution result must have a task_id".to_string());
        }

        if self.command.is_empty() {
            return Err("Execution result must have a command".to_string());
        }

        if self.workflow_stage.is_empty() {
            return Err("Execution result must have a workflow_stage".to_string());
        }

        if self.agent.is_empty() {
            return Err("Execution result must have an agent".to_string());
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

    fn from_generic(entity: GenericEntity) -> Result<Self> {
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
    use serde_json;

    #[test]
    fn test_execution_result_creation() {
        let result = ExecutionResult::new(
            "task-123".to_string(),
            "development".to_string(),
            "cargo test".to_string(),
            0,
            "test passed".to_string(),
            "".to_string(),
        );

        assert_eq!(result.task_id, "task-123");
        assert_eq!(result.workflow_stage, "development");
        assert_eq!(result.command, "cargo test");
        assert_eq!(result.exit_code, 0);
        assert_eq!(result.stdout, "test passed");
        assert_eq!(result.validation_status, ValidationStatus::Passed);
        assert_eq!(result.agent, "default");
    }

    #[test]
    fn test_execution_result_failure() {
        let result = ExecutionResult::new(
            "task-456".to_string(),
            "testing".to_string(),
            "cargo test".to_string(),
            1,
            "".to_string(),
            "test failed".to_string(),
        );

        assert_eq!(result.exit_code, 1);
        assert_eq!(result.stderr, "test failed");
        match result.validation_status {
            ValidationStatus::Failed { reason } => {
                assert!(reason.contains("exit code 1"));
            }
            _ => panic!("Expected failed status"),
        }
    }

    #[test]
    fn test_execution_result_with_expected_failure() {
        let result = ExecutionResult::new(
            "task-789".to_string(),
            "red-phase".to_string(),
            "cargo test failing_test".to_string(),
            1,
            "".to_string(),
            "test failed as expected".to_string(),
        )
        .with_expected_result("failure".to_string());

        assert_eq!(result.validation_status, ValidationStatus::Passed);
        assert_eq!(result.expected_result, Some("failure".to_string()));
    }

    #[test]
    fn test_execution_result_expected_success_but_failed() {
        let result = ExecutionResult::new(
            "task-012".to_string(),
            "testing".to_string(),
            "cargo test".to_string(),
            1,
            "".to_string(),
            "unexpected failure".to_string(),
        )
        .with_expected_result("success".to_string());

        match result.validation_status {
            ValidationStatus::Failed { reason } => {
                assert!(reason.contains("Expected success but got exit code 1"));
            }
            _ => panic!("Expected failed status"),
        }
    }

    #[test]
    fn test_execution_result_with_duration() {
        let result = ExecutionResult::new(
            "task-345".to_string(),
            "testing".to_string(),
            "cargo test".to_string(),
            0,
            "tests passed".to_string(),
            "".to_string(),
        )
        .with_duration(5000);

        assert_eq!(result.duration_ms, 5000);
    }

    #[test]
    fn test_execution_result_with_environment() {
        let mut env = HashMap::new();
        env.insert("RUST_ENV".to_string(), "test".to_string());
        env.insert("CI".to_string(), "true".to_string());

        let result = ExecutionResult::new(
            "task-678".to_string(),
            "testing".to_string(),
            "cargo test".to_string(),
            0,
            "tests passed".to_string(),
            "".to_string(),
        )
        .with_environment(env.clone());

        assert_eq!(result.environment, env);
    }

    #[test]
    fn test_execution_result_serialization() {
        let result = ExecutionResult::new(
            "task-901".to_string(),
            "testing".to_string(),
            "cargo test".to_string(),
            0,
            "tests passed".to_string(),
            "".to_string(),
        );

        let json = serde_json::to_string(&result).unwrap();
        let deserialized: ExecutionResult = serde_json::from_str(&json).unwrap();

        assert_eq!(result.task_id, deserialized.task_id);
        assert_eq!(result.command, deserialized.command);
        assert_eq!(result.validation_status, deserialized.validation_status);
    }

    #[test]
    fn test_execution_result_validation() {
        let mut result = ExecutionResult::new(
            "task-234".to_string(),
            "testing".to_string(),
            "cargo test".to_string(),
            0,
            "tests passed".to_string(),
            "".to_string(),
        );

        // Valid result
        assert!(result.validate_entity().is_ok());

        // Invalid task_id
        result.task_id = "".to_string();
        assert!(result.validate_entity().is_err());
        assert!(result.validate_entity().unwrap_err().contains("task_id"));

        // Reset and test invalid command
        result.task_id = "task-234".to_string();
        result.command = "".to_string();
        assert!(result.validate_entity().is_err());
        assert!(result.validate_entity().unwrap_err().contains("command"));

        // Reset and test invalid workflow_stage
        result.command = "cargo test".to_string();
        result.workflow_stage = "".to_string();
        assert!(result.validate_entity().is_err());
        assert!(result
            .validate_entity()
            .unwrap_err()
            .contains("workflow_stage"));

        // Reset and test invalid agent
        result.workflow_stage = "testing".to_string();
        result.agent = "".to_string();
        assert!(result.validate_entity().is_err());
        assert!(result.validate_entity().unwrap_err().contains("agent"));
    }
}

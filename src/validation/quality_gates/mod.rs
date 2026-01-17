//! Quality Gates Framework
//!
//! Provides automated execution of quality gates (tests, builds, linting)
//! with configurable validators and BDD Red-Green-Refactor cycle support.

use crate::entities::{Entity, ExecutionResult, ExpectedResult, ValidationStatus};
use crate::error::EngramError;
use crate::storage::Storage;
use std::collections::HashMap;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

pub mod validators;

pub use validators::*;

/// Quality gate definition
#[derive(Debug, Clone)]
pub struct QualityGate {
    pub name: String,
    pub command: String,
    pub required: bool,
    pub expected_result: ExpectedResult,
    pub timeout_seconds: Option<u64>,
    pub working_directory: Option<String>,
    pub environment: HashMap<String, String>,
    pub retry_count: u32,
    pub failure_message: Option<String>,
}

impl QualityGate {
    pub fn new(name: String, command: String) -> Self {
        Self {
            name,
            command,
            required: true,
            expected_result: ExpectedResult::Success,
            timeout_seconds: Some(300),
            working_directory: None,
            environment: HashMap::new(),
            retry_count: 0,
            failure_message: None,
        }
    }

    pub fn with_expected_result(mut self, expected: ExpectedResult) -> Self {
        self.expected_result = expected;
        self
    }

    pub fn with_timeout(mut self, seconds: u64) -> Self {
        self.timeout_seconds = Some(seconds);
        self
    }

    pub fn optional(mut self) -> Self {
        self.required = false;
        self
    }

    pub fn with_working_directory(mut self, dir: String) -> Self {
        self.working_directory = Some(dir);
        self
    }

    pub fn with_environment(mut self, env: HashMap<String, String>) -> Self {
        self.environment = env;
        self
    }

    pub fn with_retry_count(mut self, retries: u32) -> Self {
        self.retry_count = retries;
        self
    }

    pub fn with_failure_message(mut self, message: String) -> Self {
        self.failure_message = Some(message);
        self
    }
}

/// Quality gates executor
pub struct QualityGatesExecutor<S: Storage> {
    storage: S,
}

impl<S: Storage> QualityGatesExecutor<S> {
    pub fn new(storage: S) -> Self {
        Self { storage }
    }

    /// Execute a single quality gate
    pub fn execute_gate(
        &mut self,
        task_id: &str,
        workflow_stage: &str,
        gate: &QualityGate,
        agent: &str,
    ) -> Result<ExecutionResult, EngramError> {
        let mut execution_result = ExecutionResult::new(
            task_id.to_string(),
            workflow_stage.to_string(),
            gate.name.clone(),
            gate.command.clone(),
            agent.to_string(),
        );

        execution_result.set_expected_result(gate.expected_result.clone());

        if let Some(working_dir) = &gate.working_directory {
            execution_result.working_directory = Some(working_dir.clone());
        }

        execution_result.set_environment(gate.environment.clone());

        let start_time = Instant::now();
        let mut attempts = 0;
        let max_attempts = gate.retry_count + 1;

        loop {
            attempts += 1;

            let result = self.execute_command_with_timeout(gate);

            match result {
                Ok((exit_code, stdout, stderr)) => {
                    let duration = start_time.elapsed().as_millis() as u64;
                    execution_result.set_results(exit_code, stdout, stderr, duration);

                    if attempts > 1 {
                        execution_result.retry_count = attempts - 1;
                    }

                    break;
                }
                Err(e) if attempts < max_attempts => {
                    continue;
                }
                Err(e) => {
                    let duration = start_time.elapsed().as_millis() as u64;
                    execution_result.set_results(-1, String::new(), e.to_string(), duration);
                    execution_result.validation_status = ValidationStatus::Failed {
                        reason: format!(
                            "Failed to execute command after {} attempts: {}",
                            attempts, e
                        ),
                    };
                    break;
                }
            }
        }

        if execution_result.failed() && !gate.required {
            execution_result.validation_status = ValidationStatus::Skipped {
                reason: "Gate failed but is not required".to_string(),
            };
        }

        if execution_result.failed() && gate.failure_message.is_some() {
            execution_result.add_metadata(
                "custom_failure_message".to_string(),
                serde_json::Value::String(gate.failure_message.as_ref().unwrap().clone()),
            );
        }

        let generic = execution_result.to_generic();
        self.storage.store(&generic)?;

        Ok(execution_result)
    }

    /// Execute multiple quality gates
    pub fn execute_gates(
        &mut self,
        task_id: &str,
        workflow_stage: &str,
        gates: &[QualityGate],
        agent: &str,
    ) -> Result<Vec<ExecutionResult>, EngramError> {
        let mut results = Vec::new();
        let mut has_required_failure = false;

        for gate in gates {
            let result = self.execute_gate(task_id, workflow_stage, gate, agent)?;

            if result.failed() && gate.required {
                has_required_failure = true;
            }

            results.push(result);

            if has_required_failure && gate.required {
                break;
            }
        }

        Ok(results)
    }

    fn execute_command_with_timeout(
        &self,
        gate: &QualityGate,
    ) -> Result<(i32, String, String), EngramError> {
        let parts: Vec<&str> = gate.command.split_whitespace().collect();
        if parts.is_empty() {
            return Err(EngramError::Validation("Empty command".to_string()));
        }

        let mut cmd = Command::new(parts[0]);
        if parts.len() > 1 {
            cmd.args(&parts[1..]);
        }

        if let Some(working_dir) = &gate.working_directory {
            cmd.current_dir(working_dir);
        }

        for (key, value) in &gate.environment {
            cmd.env(key, value);
        }

        cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

        let child = cmd.spawn().map_err(|e| {
            EngramError::Validation(format!("Failed to spawn command '{}': {}", gate.command, e))
        })?;

        let timeout_duration = gate
            .timeout_seconds
            .map(Duration::from_secs)
            .unwrap_or(Duration::from_secs(300));

        let output = match self.wait_for_output_with_timeout(child, timeout_duration) {
            Ok(output) => output,
            Err(e) => return Err(e),
        };

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let exit_code = output.status.code().unwrap_or(-1);

        Ok((exit_code, stdout, stderr))
    }

    fn wait_for_output_with_timeout(
        &self,
        mut child: std::process::Child,
        timeout: Duration,
    ) -> Result<std::process::Output, EngramError> {
        use std::sync::mpsc;
        use std::thread;

        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let result = child.wait_with_output();
            let _ = tx.send(result);
        });

        match rx.recv_timeout(timeout) {
            Ok(Ok(output)) => Ok(output),
            Ok(Err(e)) => Err(EngramError::Validation(format!(
                "Command execution failed: {}",
                e
            ))),
            Err(_) => Err(EngramError::Validation("Command timed out".to_string())),
        }
    }

    /// Get execution results for a task
    pub fn get_execution_results(
        &self,
        task_id: &str,
        workflow_stage: Option<&str>,
    ) -> Result<Vec<ExecutionResult>, EngramError> {
        use crate::storage::QueryFilter;

        let mut filter = QueryFilter {
            entity_type: Some("execution_result".to_string()),
            limit: Some(100),
            ..Default::default()
        };

        let mut field_filters = HashMap::new();
        field_filters.insert(
            "task_id".to_string(),
            serde_json::Value::String(task_id.to_string()),
        );

        if let Some(stage) = workflow_stage {
            field_filters.insert(
                "workflow_stage".to_string(),
                serde_json::Value::String(stage.to_string()),
            );
        }

        filter.field_filters = field_filters;

        let query_result = self.storage.query(&filter)?;
        let mut results = Vec::new();

        for entity in query_result.entities {
            if let Ok(execution_result) = ExecutionResult::from_generic(entity) {
                results.push(execution_result);
            }
        }

        results.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        Ok(results)
    }

    /// Check if all required gates passed for a workflow stage
    pub fn stage_gates_passed(
        &self,
        task_id: &str,
        workflow_stage: &str,
    ) -> Result<bool, EngramError> {
        let results = self.get_execution_results(task_id, Some(workflow_stage))?;

        if results.is_empty() {
            return Ok(false);
        }

        let mut required_gates: HashMap<String, bool> = HashMap::new();

        for result in results {
            if result.passed() || result.skipped() {
                required_gates.insert(result.quality_gate, true);
            } else {
                required_gates.insert(result.quality_gate, false);
            }
        }

        Ok(required_gates.values().all(|&passed| passed))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::MemoryStorage;

    #[test]
    fn test_quality_gate_creation() {
        let gate = QualityGate::new("test".to_string(), "echo hello".to_string())
            .with_expected_result(ExpectedResult::Success)
            .with_timeout(60)
            .optional();

        assert_eq!(gate.name, "test");
        assert_eq!(gate.command, "echo hello");
        assert!(!gate.required);
        assert_eq!(gate.timeout_seconds, Some(60));
    }

    #[test]
    fn test_execute_successful_gate() {
        let storage = MemoryStorage::new("test-agent");
        let mut executor = QualityGatesExecutor::new(storage);

        let gate = QualityGate::new("echo-test".to_string(), "echo hello world".to_string());

        let result = executor
            .execute_gate("task-123", "test", &gate, "test-agent")
            .unwrap();

        assert_eq!(result.task_id, "task-123");
        assert_eq!(result.workflow_stage, "test");
        assert_eq!(result.quality_gate, "echo-test");
        assert_eq!(result.exit_code, 0);
        assert!(result.passed());
        assert!(result.stdout.contains("hello world"));
    }

    #[test]
    fn test_execute_failing_gate() {
        let storage = MemoryStorage::new("test-agent");
        let mut executor = QualityGatesExecutor::new(storage);

        let gate = QualityGate::new("false-test".to_string(), "false".to_string());

        let result = executor
            .execute_gate("task-123", "test", &gate, "test-agent")
            .unwrap();

        assert_ne!(result.exit_code, 0);
        assert!(result.failed());
    }

    #[test]
    fn test_bdd_red_phase_gate() {
        let storage = MemoryStorage::new("test-agent");
        let mut executor = QualityGatesExecutor::new(storage);

        let gate = QualityGate::new("test-should-fail".to_string(), "false".to_string())
            .with_expected_result(ExpectedResult::Failure);

        let result = executor
            .execute_gate("task-123", "bdd", &gate, "test-agent")
            .unwrap();

        assert_ne!(result.exit_code, 0);
        assert!(result.passed());
    }

    #[test]
    fn test_multiple_gates_execution() {
        let storage = MemoryStorage::new("test-agent");
        let mut executor = QualityGatesExecutor::new(storage);

        let gates = vec![
            QualityGate::new("echo1".to_string(), "echo test1".to_string()),
            QualityGate::new("echo2".to_string(), "echo test2".to_string()),
        ];

        let results = executor
            .execute_gates("task-123", "test", &gates, "test-agent")
            .unwrap();

        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|r| r.passed()));
    }

    #[test]
    fn test_optional_gate_failure() {
        let storage = MemoryStorage::new("test-agent");
        let mut executor = QualityGatesExecutor::new(storage);

        let gate = QualityGate::new("optional-fail".to_string(), "false".to_string()).optional();

        let result = executor
            .execute_gate("task-123", "test", &gate, "test-agent")
            .unwrap();

        assert!(result.skipped());
    }
}

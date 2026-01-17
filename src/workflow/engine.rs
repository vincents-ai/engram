use crate::entities::{Entity, ExecutionResult, TransitionTrigger, Workflow};
use crate::error::EngramError;
use crate::storage::{RelationshipStorage, Storage};
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::time::Instant;

pub struct WorkflowEngine<S: Storage + RelationshipStorage> {
    storage: Arc<S>,
}

impl<S: Storage + RelationshipStorage + Clone + 'static> WorkflowEngine<S> {
    pub fn new(storage: Arc<S>) -> Result<Self, EngramError> {
        Ok(Self { storage })
    }

    pub fn can_advance(&self, _task_id: &str, _stage: &str) -> Result<bool, EngramError> {
        Ok(false)
    }

    pub fn execute_quality_gates(
        &self,
        task_id: &str,
        stage_name: &str,
        workflow: &Workflow,
    ) -> Result<Vec<ExecutionResult>, EngramError> {
        let stage = workflow
            .stages
            .iter()
            .find(|s| s.name == stage_name)
            .ok_or_else(|| {
                EngramError::Validation(format!("Stage '{}' not found in workflow", stage_name))
            })?;

        let mut results = Vec::new();

        for gate in &stage.quality_gates {
            let start_time = Instant::now();

            let output = Command::new("sh")
                .arg("-c")
                .arg(&gate.command)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()
                .map_err(|e| {
                    EngramError::Validation(format!(
                        "Failed to execute command '{}': {}",
                        gate.command, e
                    ))
                })?;

            let duration = start_time.elapsed().as_millis() as u64;
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();

            let mut result = ExecutionResult::new(
                task_id.to_string(),
                stage_name.to_string(),
                gate.command.clone(),
                output.status.code().unwrap_or(-1),
                stdout,
                stderr,
            )
            .with_duration(duration);

            if let Some(ref expected) = gate.expected_result {
                result = result.with_expected_result(expected.clone());
            }

            let generic_result = result.to_generic();
            self.storage.store(&generic_result)?;
            results.push(result);
        }

        Ok(results)
    }

    pub fn advance_task(
        &self,
        _task_id: &str,
        _trigger: TransitionTrigger,
    ) -> Result<(), EngramError> {
        println!("🔄 Task advancement logic not yet implemented - workflow engine needs relationship system integration");
        Ok(())
    }

    fn get_task_workflow(&self, _task_id: &str) -> Result<Option<Workflow>, EngramError> {
        Ok(None)
    }

    fn get_task_current_stage(&self, _task_id: &str) -> Result<Option<String>, EngramError> {
        Ok(Some("planning".to_string()))
    }
}

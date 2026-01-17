use crate::entities::{ExecutionResult, TransitionTrigger, Workflow};
use crate::error::EngramError;
use crate::storage::Storage;
use crate::validation::CommitValidator;
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::time::Instant;

pub struct WorkflowEngine {
    storage: Arc<dyn Storage>,
    validator: Arc<CommitValidator>,
}

impl WorkflowEngine {
    pub fn new(storage: Arc<dyn Storage>) -> Result<Self, EngramError> {
        let validator = Arc::new(CommitValidator::new(storage.clone())?);

        Ok(Self { storage, validator })
    }

    /// Check if a task can advance to the target stage
    pub fn can_advance(&self, task_id: &str, target_stage: &str) -> Result<bool, EngramError> {
        let workflow = self.get_task_workflow(task_id)?;
        let workflow = match workflow {
            Some(w) => w,
            None => return Ok(false),
        };

        let current_stage = self.get_task_current_stage(task_id)?;
        let current_stage = match current_stage {
            Some(stage) => stage,
            None => return Ok(true),
        };

        let transition_exists = workflow
            .transitions
            .iter()
            .any(|t| t.from == current_stage && t.to == target_stage);

        if !transition_exists {
            return Ok(false);
        }

        let gates_passed = self.check_quality_gates(task_id, &current_stage)?;

        Ok(gates_passed)
    }

    /// Execute quality gates for a task's current stage
    pub fn run_quality_gates(&self, task_id: &str) -> Result<Vec<ExecutionResult>, EngramError> {
        let workflow = self
            .get_task_workflow(task_id)?
            .ok_or_else(|| EngramError::NotFound("No workflow assigned to task".to_string()))?;

        let current_stage = self
            .get_task_current_stage(task_id)?
            .ok_or_else(|| EngramError::NotFound("Task has no current stage".to_string()))?;

        let stage = workflow
            .stages
            .iter()
            .find(|s| s.name == current_stage)
            .ok_or_else(|| EngramError::NotFound("Stage not found in workflow".to_string()))?;

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
                    EngramError::InvalidOperation(format!("Failed to execute command: {}", e))
                })?;

            let duration = start_time.elapsed().as_millis() as u64;

            let mut result = ExecutionResult::new(
                task_id.to_string(),
                current_stage.clone(),
                gate.command.clone(),
                output.status.code().unwrap_or(-1),
                String::from_utf8_lossy(&output.stdout).to_string(),
                String::from_utf8_lossy(&output.stderr).to_string(),
            )
            .with_duration(duration);

            if let Some(ref expected) = gate.expected_result {
                result = result.with_expected_result(expected.clone());
            }

            self.storage.store(&result)?;
            results.push(result);
        }

        Ok(results)
    }

    /// Advance task to next stage
    pub fn advance_task(
        &self,
        task_id: &str,
        trigger: TransitionTrigger,
    ) -> Result<(), EngramError> {
        // Implementation placeholder - will be completed in subsequent tasks
        todo!("Implement task advancement logic")
    }

    fn get_task_workflow(&self, _task_id: &str) -> Result<Option<Workflow>, EngramError> {
        // Placeholder - will be implemented when relationship system is integrated
        Ok(None)
    }

    fn get_task_current_stage(&self, _task_id: &str) -> Result<Option<String>, EngramError> {
        // Placeholder implementation
        Ok(Some("planning".to_string()))
    }

    fn check_quality_gates(&self, _task_id: &str, _stage: &str) -> Result<bool, EngramError> {
        // Placeholder - will query recent ExecutionResult entities
        Ok(true)
    }
}

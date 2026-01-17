use crate::entities::{CommitPolicy, Workflow};
use crate::error::EngramError;
use crate::storage::Storage;
use crate::workflow::WorkflowEngine;
use std::sync::Arc;

/// Validator that enforces workflow-based commit policies
///
/// This validator integrates with the workflow engine to enforce commit policies
/// based on the current workflow stage a task is in.
pub struct WorkflowValidator {
    storage: Arc<dyn Storage>,
    engine: WorkflowEngine,
}

impl WorkflowValidator {
    /// Create a new workflow validator with storage and engine
    pub fn new(storage: Arc<dyn Storage>) -> Result<Self, EngramError> {
        let engine = WorkflowEngine::new(storage.clone())?;

        Ok(Self { storage, engine })
    }

    /// Validate a commit against the current workflow stage policies
    ///
    /// Returns true if the commit is allowed, false if blocked by policy
    pub fn validate_commit_against_workflow(
        &self,
        task_id: &str,
        changed_files: &[String],
    ) -> Result<bool, EngramError> {
        let workflow = self.get_task_workflow(task_id)?;
        let workflow = match workflow {
            Some(w) => w,
            None => return Ok(true), // No workflow assigned, allow commit
        };

        let current_stage = self.get_task_current_stage(task_id)?;
        let current_stage = match current_stage {
            Some(stage) => stage,
            None => return Ok(true), // No current stage, allow commit
        };

        // Find stage definition
        let stage = workflow
            .stages
            .iter()
            .find(|s| s.name == current_stage)
            .ok_or_else(|| EngramError::NotFound("Stage not found".to_string()))?;

        // Check commit policy
        match &stage.commit_policy {
            CommitPolicy::EngramOnly => self.validate_engram_only_policy(changed_files),
            CommitPolicy::ResearchArtifacts => {
                self.validate_research_artifacts_policy(changed_files)
            }
            CommitPolicy::TestsOnly => self.validate_tests_only_policy(changed_files),
            CommitPolicy::CodeWithTests => self.validate_code_with_tests_policy(changed_files),
            CommitPolicy::FullValidation => {
                // Run quality gates and check they pass
                let results = self.engine.run_quality_gates(task_id)?;
                let all_passed = results.iter().all(|r| {
                    matches!(
                        r.validation_status,
                        crate::entities::ValidationStatus::Passed
                    )
                });
                Ok(all_passed)
            }
        }
    }

    /// Validate engram only policy - only .engram/ and docs/plans/ files allowed
    fn validate_engram_only_policy(&self, changed_files: &[String]) -> Result<bool, EngramError> {
        let allowed_patterns = [".engram/", "docs/plans/"];

        for file in changed_files {
            let is_allowed = allowed_patterns
                .iter()
                .any(|pattern| file.starts_with(pattern));
            if !is_allowed {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Validate research artifacts policy - docs, examples, research files allowed
    fn validate_research_artifacts_policy(
        &self,
        changed_files: &[String],
    ) -> Result<bool, EngramError> {
        let allowed_patterns = [".engram/", "docs/", "examples/", "research/", ".md"];

        for file in changed_files {
            let is_allowed = allowed_patterns
                .iter()
                .any(|pattern| file.starts_with(pattern) || file.ends_with(pattern));
            if !is_allowed {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Validate tests only policy - only test files allowed
    fn validate_tests_only_policy(&self, changed_files: &[String]) -> Result<bool, EngramError> {
        let test_patterns = ["tests/", "_test.rs", ".test."];

        for file in changed_files {
            // Skip engram files
            if file.starts_with(".engram/") {
                continue;
            }

            let is_test = test_patterns.iter().any(|pattern| file.contains(pattern));
            if !is_test {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Validate code with tests policy - allow any code changes
    /// Quality gates will enforce test passing requirement
    fn validate_code_with_tests_policy(
        &self,
        _changed_files: &[String],
    ) -> Result<bool, EngramError> {
        Ok(true)
    }

    /// Get the workflow assigned to a task (placeholder for relationship system)
    fn get_task_workflow(&self, _task_id: &str) -> Result<Option<Workflow>, EngramError> {
        // TODO: Implementation will use relationship system to find workflow assigned to task
        // For now, return None to allow all commits during development
        Ok(None)
    }

    /// Get the current stage of a task (placeholder for task metadata)
    fn get_task_current_stage(&self, _task_id: &str) -> Result<Option<String>, EngramError> {
        // TODO: Implementation will get current stage from task metadata or workflow state
        // For now, return None to allow all commits during development
        Ok(None)
    }
}

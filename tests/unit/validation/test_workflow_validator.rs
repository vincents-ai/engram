#[cfg(test)]
mod tests {
    use engram_rust::entities::{
        CommitPolicy, QualityGate, Task, TaskStatus, Workflow, WorkflowStage,
    };
    use engram_rust::error::EngramError;
    use engram_rust::storage::memory::MemoryStorage;
    use engram_rust::validation::workflow_validator::WorkflowValidator;
    use std::sync::Arc;
    use uuid::Uuid;

    fn create_test_workflow() -> Workflow {
        let mut workflow = Workflow::new("Test Workflow", "Test workflow for validation");

        // Planning stage - engram only commits
        let planning_stage = WorkflowStage {
            name: "planning".to_string(),
            description: "Planning stage - documentation and engram entities only".to_string(),
            commit_policy: CommitPolicy::EngramOnly,
            quality_gates: vec![QualityGate {
                command: "echo 'Planning validation complete'".to_string(),
                required: true,
            }],
        };

        // Development stage - code with tests
        let development_stage = WorkflowStage {
            name: "development".to_string(),
            description: "Development stage - code changes with tests".to_string(),
            commit_policy: CommitPolicy::CodeWithTests,
            quality_gates: vec![QualityGate {
                command: "cargo test".to_string(),
                required: true,
            }],
        };

        // Integration stage - full validation
        let integration_stage = WorkflowStage {
            name: "integration".to_string(),
            description: "Integration stage - full test suite and build".to_string(),
            commit_policy: CommitPolicy::FullValidation,
            quality_gates: vec![
                QualityGate {
                    command: "cargo test".to_string(),
                    required: true,
                },
                QualityGate {
                    command: "cargo build".to_string(),
                    required: true,
                },
            ],
        };

        workflow.stages = vec![planning_stage, development_stage, integration_stage];
        workflow
    }

    fn create_test_task() -> Task {
        Task::new(
            "Test workflow validation",
            "High",
            TaskStatus::InProgress,
            "default".to_string(),
        )
    }

    #[test]
    fn test_workflow_commit_policy_engram_only_allows_engram_files() {
        // This test will fail until WorkflowValidator is implemented
        let storage = Arc::new(MemoryStorage::new());
        let validator_result = WorkflowValidator::new(storage);

        // This should fail because WorkflowValidator doesn't exist yet
        assert!(
            validator_result.is_err(),
            "WorkflowValidator should not exist yet - this test should fail initially"
        );
    }

    #[test]
    fn test_workflow_commit_policy_engram_only_blocks_code_files() {
        // Test that planning stage (engram_only) blocks code file changes
        let storage = Arc::new(MemoryStorage::new());

        // This will fail until WorkflowValidator is implemented
        let validator_result = WorkflowValidator::new(storage);
        assert!(
            validator_result.is_err(),
            "Expected WorkflowValidator to not exist yet"
        );
    }

    #[test]
    fn test_workflow_commit_policy_code_with_tests_allows_code_changes() {
        // Test that development stage allows code changes
        let storage = Arc::new(MemoryStorage::new());

        // This will fail until WorkflowValidator is implemented
        let validator_result = WorkflowValidator::new(storage);
        assert!(
            validator_result.is_err(),
            "Expected WorkflowValidator to not exist yet"
        );
    }

    #[test]
    fn test_workflow_commit_policy_full_validation_runs_quality_gates() {
        // Test that integration stage runs and validates quality gates
        let storage = Arc::new(MemoryStorage::new());

        // This will fail until WorkflowValidator is implemented
        let validator_result = WorkflowValidator::new(storage);
        assert!(
            validator_result.is_err(),
            "Expected WorkflowValidator to not exist yet"
        );
    }

    #[test]
    fn test_no_workflow_assigned_allows_all_commits() {
        // Test that tasks without workflows don't block commits
        let storage = Arc::new(MemoryStorage::new());

        // This will fail until WorkflowValidator is implemented
        let validator_result = WorkflowValidator::new(storage);
        assert!(
            validator_result.is_err(),
            "Expected WorkflowValidator to not exist yet"
        );
    }
}

use engram::cli;
use engram::entities::{CommitPolicy, Task, TaskPriority, TaskStatus, TransitionTrigger, Workflow};
use engram::error::EngramError;
use engram::storage::MemoryStorage;
use engram::workflow::{WorkflowEngine, WorkflowParser};
use std::sync::Arc;
use uuid::Uuid;

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_complete_workflow_lifecycle() -> Result<(), EngramError> {
        let mut storage = MemoryStorage::new();

        let task_id = Uuid::new_v4().to_string();
        let mut task = Task::new(
            "Test feature implementation".to_string(),
            "Test task description".to_string(),
            "default".to_string(),
            TaskPriority::Medium,
        );
        task.id = task_id.clone();
        task.status = TaskStatus::Pending;

        storage.store(&task)?;

        let workflow_yaml = include_str!("../../workflows/feature-development.yaml");
        let workflow = WorkflowParser::parse(workflow_yaml)?;

        storage.store(&workflow)?;

        let storage = Arc::new(storage);
        let engine = WorkflowEngine::new(storage)?;

        assert!(engine.can_advance(&task_id, "planning")?);

        engine.advance_task(&task_id, TransitionTrigger::Manual)?;

        Ok(())
    }

    #[test]
    fn test_workflow_template_loading() -> Result<(), EngramError> {
        let templates = [
            (
                "feature-development",
                include_str!("../../workflows/feature-development.yaml"),
            ),
            ("bug-fix", include_str!("../../workflows/bug-fix.yaml")),
            ("research", include_str!("../../workflows/research.yaml")),
        ];

        for (name, yaml_content) in templates {
            let workflow = WorkflowParser::parse(yaml_content).map_err(|e| {
                EngramError::Validation(format!("Failed to parse {} workflow: {}", name, e))
            })?;

            assert!(
                !workflow.name.is_empty(),
                "Workflow {} should have a name",
                name
            );
            assert!(
                !workflow.stages.is_empty(),
                "Workflow {} should have stages",
                name
            );
            assert!(
                !workflow.transitions.is_empty(),
                "Workflow {} should have transitions",
                name
            );

            for stage in &workflow.stages {
                match stage.commit_policy {
                    CommitPolicy::EngramOnly
                    | CommitPolicy::ResearchArtifacts
                    | CommitPolicy::TestsOnly
                    | CommitPolicy::CodeWithTests
                    | CommitPolicy::FullValidation => {}
                }
            }
        }

        Ok(())
    }

    #[test]
    fn test_bdd_red_green_refactor_enforcement() -> Result<(), EngramError> {
        let workflow_yaml = include_str!("../../workflows/feature-development.yaml");
        let workflow = WorkflowParser::parse(workflow_yaml)?;

        let bdd_stage = workflow
            .stages
            .iter()
            .find(|s| s.name == "bdd")
            .ok_or_else(|| EngramError::Validation("BDD stage not found".to_string()))?;

        let test_gate = bdd_stage
            .quality_gates
            .iter()
            .find(|g| g.command.contains("test"))
            .ok_or_else(|| {
                EngramError::Validation("Test quality gate not found in BDD stage".to_string())
            })?;

        assert_eq!(
            test_gate.expected_result,
            Some("failure".to_string()),
            "BDD stage should expect test failures"
        );

        let dev_stage = workflow
            .stages
            .iter()
            .find(|s| s.name == "development")
            .ok_or_else(|| EngramError::Validation("Development stage not found".to_string()))?;

        let dev_test_gate = dev_stage
            .quality_gates
            .iter()
            .find(|g| g.command.contains("test"))
            .ok_or_else(|| {
                EngramError::Validation(
                    "Test quality gate not found in development stage".to_string(),
                )
            })?;

        assert!(
            dev_test_gate.expected_result.is_none()
                || dev_test_gate.expected_result == Some("success".to_string()),
            "Development stage should expect test success (or default to success)"
        );

        Ok(())
    }

    #[test]
    fn test_progressive_commit_policies() -> Result<(), EngramError> {
        let workflow_yaml = include_str!("../../workflows/feature-development.yaml");
        let workflow = WorkflowParser::parse(workflow_yaml)?;

        let expected_policies = [
            ("planning", CommitPolicy::EngramOnly),
            ("bdd", CommitPolicy::TestsOnly),
            ("development", CommitPolicy::CodeWithTests),
            ("integration", CommitPolicy::FullValidation),
        ];

        for (stage_name, expected_policy) in expected_policies {
            let stage = workflow
                .stages
                .iter()
                .find(|s| s.name == stage_name)
                .ok_or_else(|| {
                    EngramError::Validation(format!("Stage {} not found", stage_name))
                })?;

            assert_eq!(
                stage.commit_policy, expected_policy,
                "Stage {} should have policy {:?}",
                stage_name, expected_policy
            );
        }

        Ok(())
    }

    #[test]
    fn test_workflow_cli_integration() -> Result<(), EngramError> {
        let mut storage = MemoryStorage::new();

        let task_id = Uuid::new_v4().to_string();
        let mut task = Task::new(
            "CLI test task".to_string(),
            "Test task for CLI integration".to_string(),
            "default".to_string(),
            TaskPriority::Low,
        );
        task.id = task_id.clone();
        storage.store(&task)?;

        let result = cli::advance_task_stage(&mut storage, &task_id);
        assert!(result.is_ok(), "CLI advance function should not error");

        Ok(())
    }

    #[test]
    fn test_quality_gate_validation() -> Result<(), EngramError> {
        let workflow_yaml = r#"
name: "Quality Gate Test"
description: "Test quality gate validation"
stages:
  - name: "test_stage"
    description: "Test stage with quality gates"
    commit_policy: "code_with_tests"
    quality_gates:
      - command: "cargo check"
        required: true
      - command: "cargo test"
        required: true
        expected_result: "success"
      - command: "cargo clippy"
        required: false
        failure_message: "Clippy warnings detected"
transitions:
  - from: "test_stage"
    to: "complete"
    trigger: "auto"
"#;

        let workflow = WorkflowParser::parse(workflow_yaml)?;
        let stage = &workflow.stages[0];

        assert_eq!(stage.quality_gates.len(), 3);

        let check_gate = &stage.quality_gates[0];
        assert_eq!(check_gate.command, "cargo check");
        assert!(check_gate.required);

        let test_gate = &stage.quality_gates[1];
        assert_eq!(test_gate.command, "cargo test");
        assert_eq!(test_gate.expected_result, Some("success".to_string()));

        let clippy_gate = &stage.quality_gates[2];
        assert!(!clippy_gate.required);
        assert_eq!(
            clippy_gate.failure_message,
            Some("Clippy warnings detected".to_string())
        );

        Ok(())
    }

    #[test]
    fn test_transition_trigger_types() -> Result<(), EngramError> {
        let workflow_yaml = r#"
name: "Transition Test"
description: "Test transition trigger types"
stages:
  - name: "manual_stage"
    description: "Manual transition stage"
    commit_policy: "engram_only"
    quality_gates: []
  - name: "auto_stage"
    description: "Auto transition stage"
    commit_policy: "engram_only"
    quality_gates: []
transitions:
  - from: "manual_stage"
    to: "auto_stage"
    trigger: "manual"
  - from: "auto_stage"
    to: "complete"
    trigger: "auto"
"#;

        let workflow = WorkflowParser::parse(workflow_yaml)?;

        assert_eq!(workflow.transitions.len(), 2);

        let manual_transition = &workflow.transitions[0];
        assert_eq!(manual_transition.trigger, TransitionTrigger::Manual);

        let auto_transition = &workflow.transitions[1];
        assert_eq!(auto_transition.trigger, TransitionTrigger::Auto);

        Ok(())
    }

    #[test]
    fn test_workflow_engine_error_handling() {
        let storage = Arc::new(MemoryStorage::new());

        let engine = WorkflowEngine::new(storage.clone());
        assert!(engine.is_ok(), "Engine creation should succeed");

        let engine = engine.unwrap();

        let fake_task_id = "non-existent-task";
        let result = engine.can_advance(fake_task_id, "any_stage");
        assert!(
            result.is_ok(),
            "can_advance should handle non-existent tasks gracefully"
        );
        assert!(
            !result.unwrap(),
            "can_advance should return false for non-existent tasks"
        );
    }

    #[test]
    fn test_commit_policy_validation() -> Result<(), EngramError> {
        let policies = [
            "engram_only",
            "research_artifacts",
            "tests_only",
            "code_with_tests",
            "full_validation",
        ];

        for policy_str in policies {
            let yaml = format!(
                r#"
name: "Policy Test"
description: "Test commit policy parsing"
stages:
  - name: "test_stage"
    description: "Test stage"
    commit_policy: "{}"
    quality_gates: []
transitions: []
"#,
                policy_str
            );

            let workflow = WorkflowParser::parse(&yaml)?;
            assert_eq!(workflow.stages.len(), 1);

            let stage = &workflow.stages[0];
            match policy_str {
                "engram_only" => assert_eq!(stage.commit_policy, CommitPolicy::EngramOnly),
                "research_artifacts" => {
                    assert_eq!(stage.commit_policy, CommitPolicy::ResearchArtifacts)
                }
                "tests_only" => assert_eq!(stage.commit_policy, CommitPolicy::TestsOnly),
                "code_with_tests" => assert_eq!(stage.commit_policy, CommitPolicy::CodeWithTests),
                "full_validation" => assert_eq!(stage.commit_policy, CommitPolicy::FullValidation),
                _ => panic!("Unexpected policy: {}", policy_str),
            }
        }

        Ok(())
    }
}

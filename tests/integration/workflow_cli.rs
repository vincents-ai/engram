use engram::cli::{task as task_cli, workflow as workflow_cli};
use engram::entities::{Task, TaskStatus, Workflow};
use engram::error::EngramError;
use engram::storage::MemoryStorage;
use engram::workflow::{WorkflowEngine, WorkflowParser};
use std::sync::Arc;
use uuid::Uuid;

#[cfg(test)]
mod workflow_cli_tests {
    use super::*;

    #[test]
    fn test_workflow_create_command() -> Result<(), EngramError> {
        let mut storage = MemoryStorage::new();

        let result = workflow_cli::create_workflow(
            &mut storage,
            "Test Workflow".to_string(),
            "Test workflow description".to_string(),
        );

        assert!(result.is_ok(), "Workflow creation should succeed");
        Ok(())
    }

    #[test]
    fn test_workflow_list_command() -> Result<(), EngramError> {
        let mut storage = MemoryStorage::new();

        workflow_cli::create_workflow(
            &mut storage,
            "Workflow 1".to_string(),
            "First workflow".to_string(),
        )?;

        workflow_cli::create_workflow(
            &mut storage,
            "Workflow 2".to_string(),
            "Second workflow".to_string(),
        )?;

        let result = workflow_cli::list_workflows(&storage, "default");
        assert!(result.is_ok(), "Workflow listing should succeed");

        Ok(())
    }

    #[test]
    fn test_workflow_show_command() -> Result<(), EngramError> {
        let mut storage = MemoryStorage::new();

        let workflow_id = workflow_cli::create_workflow(
            &mut storage,
            "Show Test Workflow".to_string(),
            "Workflow for show command test".to_string(),
        )?;

        let result = workflow_cli::show_workflow(&storage, &workflow_id);
        assert!(result.is_ok(), "Workflow show should succeed");

        Ok(())
    }

    #[test]
    fn test_task_advance_integration() -> Result<(), EngramError> {
        let mut storage = MemoryStorage::new();

        let task_id = Uuid::new_v4().to_string();
        let mut task = Task::new("Advance test task".to_string(), "default".to_string());
        task.id = task_id.clone();
        task.status = TaskStatus::Pending;
        storage.store(&task)?;

        let result = task_cli::advance_task_stage(&mut storage, &task_id);
        assert!(
            result.is_ok(),
            "Task advance should not error with placeholder implementation"
        );

        Ok(())
    }

    #[test]
    fn test_workflow_engine_with_real_templates() -> Result<(), EngramError> {
        let storage = Arc::new(MemoryStorage::new());

        let feature_workflow_yaml = include_str!("../../workflows/feature-development.yaml");
        let feature_workflow = WorkflowParser::parse(feature_workflow_yaml)?;

        let bug_workflow_yaml = include_str!("../../workflows/bug-fix.yaml");
        let bug_workflow = WorkflowParser::parse(bug_workflow_yaml)?;

        let research_workflow_yaml = include_str!("../../workflows/research.yaml");
        let research_workflow = WorkflowParser::parse(research_workflow_yaml)?;

        let engine = WorkflowEngine::new(storage.clone())?;

        assert_eq!(feature_workflow.name, "Feature Development");
        assert_eq!(bug_workflow.name, "Bug Fix");
        assert_eq!(research_workflow.name, "Research");

        Ok(())
    }

    #[test]
    fn test_end_to_end_workflow_cycle() -> Result<(), EngramError> {
        let mut storage = MemoryStorage::new();

        let workflow_yaml = r#"
name: "E2E Test Workflow"
description: "End-to-end test workflow"
stages:
  - name: "start"
    description: "Starting stage"
    commit_policy: "engram_only"
    quality_gates: []
  - name: "middle"
    description: "Middle stage"
    commit_policy: "tests_only"
    quality_gates:
      - command: "echo test"
        required: false
  - name: "end"
    description: "End stage"  
    commit_policy: "full_validation"
    quality_gates:
      - command: "echo complete"
        required: true
transitions:
  - from: "start"
    to: "middle"
    trigger: "manual"
  - from: "middle"
    to: "end"
    trigger: "auto"
"#;

        let workflow = WorkflowParser::parse(workflow_yaml)?;
        storage.store(&workflow)?;

        let task_id = Uuid::new_v4().to_string();
        let mut task = Task::new("E2E test task".to_string(), "default".to_string());
        task.id = task_id.clone();
        storage.store(&task)?;

        let engine = WorkflowEngine::new(Arc::new(storage))?;

        assert!(engine.can_advance(&task_id, "start").unwrap_or(false));

        Ok(())
    }

    #[test]
    fn test_workflow_validation_edge_cases() -> Result<(), EngramError> {
        let invalid_yaml = r#"
name: "Invalid Workflow"
description: "This workflow has invalid structure"
stages: []
transitions: []
"#;

        let result = WorkflowParser::parse(invalid_yaml);
        assert!(
            result.is_ok(),
            "Empty stages/transitions should be parseable"
        );

        let invalid_commit_policy = r#"
name: "Invalid Policy Workflow"  
description: "This workflow has invalid commit policy"
stages:
  - name: "test"
    description: "Test stage"
    commit_policy: "invalid_policy"
    quality_gates: []
transitions: []
"#;

        let result = WorkflowParser::parse(invalid_commit_policy);
        assert!(result.is_err(), "Invalid commit policy should be rejected");

        Ok(())
    }

    #[test]
    fn test_quality_gate_execution_simulation() -> Result<(), EngramError> {
        let workflow_yaml = r#"
name: "Quality Gate Test"
description: "Test quality gate execution"
stages:
  - name: "test_stage"
    description: "Stage with quality gates"
    commit_policy: "code_with_tests"
    quality_gates:
      - command: "echo success"
        required: true
        expected_result: "success"
      - command: "echo optional"
        required: false
transitions: []
"#;

        let workflow = WorkflowParser::parse(workflow_yaml)?;

        let test_stage = &workflow.stages[0];
        assert_eq!(test_stage.quality_gates.len(), 2);

        let required_gate = &test_stage.quality_gates[0];
        assert!(required_gate.required);
        assert_eq!(required_gate.expected_result, Some("success".to_string()));

        let optional_gate = &test_stage.quality_gates[1];
        assert!(!optional_gate.required);

        Ok(())
    }
}

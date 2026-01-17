use engram::entities::workflow::{
    CommitPolicy, QualityGate, TransitionTrigger, Workflow, WorkflowStage, WorkflowTransition,
};
use serde_json;

#[test]
fn test_workflow_creation() {
    let workflow = Workflow::new(
        "feature-development".to_string(),
        "Complete BDD workflow for features".to_string(),
    );

    assert_eq!(workflow.name, "feature-development");
    assert_eq!(workflow.description, "Complete BDD workflow for features");
    assert!(workflow.stages.is_empty());
}

#[test]
fn test_workflow_serialization() {
    let workflow = Workflow::new("test-workflow".to_string(), "Test workflow".to_string());

    let json = serde_json::to_string(&workflow).unwrap();
    let deserialized: Workflow = serde_json::from_str(&json).unwrap();

    assert_eq!(workflow.name, deserialized.name);
}

#[test]
fn test_workflow_add_stage() {
    let mut workflow = Workflow::new("test-workflow".to_string(), "Test workflow".to_string());

    let stage = WorkflowStage {
        name: "development".to_string(),
        description: "Development phase".to_string(),
        commit_policy: CommitPolicy::CodeWithTests,
        quality_gates: vec![],
    };

    workflow.add_stage(stage);
    assert_eq!(workflow.stages.len(), 1);
    assert_eq!(workflow.stages[0].name, "development");
}

#[test]
fn test_workflow_with_quality_gates() {
    let mut workflow = Workflow::new("test-workflow".to_string(), "Test workflow".to_string());

    let quality_gate = QualityGate {
        command: "cargo test".to_string(),
        required: true,
        expected_result: Some("success".to_string()),
        failure_message: Some("Tests failed".to_string()),
    };

    let stage = WorkflowStage {
        name: "testing".to_string(),
        description: "Testing phase".to_string(),
        commit_policy: CommitPolicy::TestsOnly,
        quality_gates: vec![quality_gate],
    };

    workflow.add_stage(stage);

    assert_eq!(workflow.stages[0].quality_gates.len(), 1);
    assert_eq!(workflow.stages[0].quality_gates[0].command, "cargo test");
    assert!(workflow.stages[0].quality_gates[0].required);
}

#[test]
fn test_workflow_validation() {
    let mut workflow = Workflow::new("test-workflow".to_string(), "Test workflow".to_string());

    let stage1 = WorkflowStage {
        name: "planning".to_string(),
        description: "Planning phase".to_string(),
        commit_policy: CommitPolicy::EngramOnly,
        quality_gates: vec![],
    };

    let stage2 = WorkflowStage {
        name: "planning".to_string(),
        description: "Planning phase 2".to_string(),
        commit_policy: CommitPolicy::CodeWithTests,
        quality_gates: vec![],
    };

    workflow.add_stage(stage1);
    workflow.add_stage(stage2);

    let result = workflow.validate_entity();
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .contains("Duplicate stage name: planning"));
}

#[test]
fn test_workflow_stage_retrieval() {
    let mut workflow = Workflow::new("test-workflow".to_string(), "Test workflow".to_string());

    let stage1 = WorkflowStage {
        name: "planning".to_string(),
        description: "Planning phase".to_string(),
        commit_policy: CommitPolicy::EngramOnly,
        quality_gates: vec![],
    };

    let stage2 = WorkflowStage {
        name: "implementation".to_string(),
        description: "Implementation phase".to_string(),
        commit_policy: CommitPolicy::CodeWithTests,
        quality_gates: vec![],
    };

    workflow.add_stage(stage1);
    workflow.add_stage(stage2);

    // Since we removed the get_stage method, let's verify stages exist by iterating
    let stage_names: Vec<String> = workflow.stages.iter().map(|s| s.name.clone()).collect();
    assert!(stage_names.contains(&"planning".to_string()));
    assert!(stage_names.contains(&"implementation".to_string()));
    assert!(!stage_names.contains(&"nonexistent".to_string()));
}

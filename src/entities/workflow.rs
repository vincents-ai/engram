use crate::entities::Entity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    pub id: String,
    pub name: String,
    pub description: String,
    pub stages: Vec<WorkflowStage>,
    pub transitions: Vec<WorkflowTransition>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub agent: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStage {
    pub name: String,
    pub description: String,
    pub commit_policy: CommitPolicy,
    pub quality_gates: Vec<QualityGate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityGate {
    pub command: String,
    pub required: bool,
    pub expected_result: Option<String>, // "success", "failure", "any"
    pub failure_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommitPolicy {
    EngramOnly,        // Only engram entity changes
    ResearchArtifacts, // Docs, examples, spikes
    TestsOnly,         // Only test files
    CodeWithTests,     // Code + tests
    FullValidation,    // All quality gates
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowTransition {
    pub from: String,
    pub to: String,
    pub trigger: TransitionTrigger,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransitionTrigger {
    Manual, // Requires explicit command
    Auto,   // Triggered by quality gate success
}

impl Workflow {
    pub fn new(name: String, description: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            description,
            stages: Vec::new(),
            transitions: Vec::new(),
            created_at: now,
            updated_at: now,
            agent: "default".to_string(),
        }
    }

    pub fn add_stage(&mut self, stage: WorkflowStage) {
        self.stages.push(stage);
        self.updated_at = Utc::now();
    }

    pub fn add_transition(&mut self, transition: WorkflowTransition) {
        self.transitions.push(transition);
        self.updated_at = Utc::now();
    }
}

impl Entity for Workflow {
    fn entity_type() -> &'static str {
        "workflow"
    }

    fn id(&self) -> &str {
        &self.id
    }

    fn agent(&self) -> &str {
        &self.agent
    }

    fn timestamp(&self) -> DateTime<Utc> {
        self.created_at
    }

    fn validate_entity(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("Workflow name cannot be empty".to_string());
        }

        let mut stage_names = std::collections::HashSet::new();
        for stage in &self.stages {
            if !stage_names.insert(&stage.name) {
                return Err(format!("Duplicate stage name: {}", stage.name));
            }
        }

        Ok(())
    }

    fn to_generic(&self) -> super::GenericEntity {
        super::GenericEntity {
            id: self.id.clone(),
            entity_type: Self::entity_type().to_string(),
            agent: self.agent.clone(),
            timestamp: self.created_at,
            data: serde_json::to_value(self).unwrap_or_default(),
        }
    }

    fn from_generic(entity: super::GenericEntity) -> Result<Self, String> {
        serde_json::from_value(entity.data)
            .map_err(|e| format!("Failed to deserialize Workflow: {}", e))
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

        let testing_stage = &workflow.stages[0];
        assert_eq!(testing_stage.quality_gates.len(), 1);
        assert_eq!(testing_stage.quality_gates[0].command, "cargo test");
        assert!(testing_stage.quality_gates[0].required);
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
            name: "planning".to_string(), // Duplicate name
            description: "Planning phase 2".to_string(),
            commit_policy: CommitPolicy::CodeWithTests,
            quality_gates: vec![],
        };

        workflow.add_stage(stage1);
        workflow.add_stage(stage2);

        // Should fail validation due to duplicate stage names
        let result = workflow.validate_entity();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("Duplicate stage name: planning"));
    }
}

//! Workflow entity implementation
//!
//! This module provides workflow entities for managing state machines with transitions,
//! quality gates, and commit policies. Workflows define stages that tasks can progress
//! through with specific validation rules and permission schemes.
//!
//! # Example
//!
//! ```rust
//! use engram::entities::workflow::{Workflow, WorkflowStage, CommitPolicy, QualityGate};
//!
//! let mut workflow = Workflow::new(
//!     "feature-development".to_string(),
//!     "Complete BDD workflow for features".to_string(),
//! );
//!
//! let stage = WorkflowStage {
//!     name: "development".to_string(),
//!     description: "Development phase".to_string(),
//!     commit_policy: CommitPolicy::CodeWithTests,
//!     quality_gates: vec![],
//! };
//!
//! workflow.add_stage(stage);
//! ```

use crate::entities::Entity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use uuid::Uuid;

/// Workflow entity representing a state machine with transitions and quality gates
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Workflow {
    /// Unique identifier
    #[serde(rename = "id")]
    pub id: String,

    /// Workflow name
    #[serde(rename = "name")]
    pub name: String,

    /// Detailed description
    #[serde(rename = "description")]
    pub description: String,

    /// Workflow stages
    #[serde(rename = "stages", skip_serializing_if = "Vec::is_empty", default)]
    pub stages: Vec<WorkflowStage>,

    /// Stage transitions
    #[serde(rename = "transitions", skip_serializing_if = "Vec::is_empty", default)]
    pub transitions: Vec<WorkflowTransition>,

    /// Creation timestamp
    #[serde(rename = "created_at")]
    pub created_at: DateTime<Utc>,

    /// Last updated timestamp
    #[serde(rename = "updated_at")]
    pub updated_at: DateTime<Utc>,

    /// Associated agent
    #[serde(rename = "agent")]
    pub agent: String,
}

/// Workflow stage defining rules and policies for a phase of work
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkflowStage {
    /// Stage name
    #[serde(rename = "name")]
    pub name: String,

    /// Stage description  
    #[serde(rename = "description")]
    pub description: String,

    /// Commit policy for this stage
    #[serde(rename = "commit_policy")]
    pub commit_policy: CommitPolicy,

    /// Quality gates that must pass
    #[serde(
        rename = "quality_gates",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub quality_gates: Vec<QualityGate>,
}

/// Quality gate for validating stage completion
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QualityGate {
    /// Command to execute for validation
    #[serde(rename = "command")]
    pub command: String,

    /// Whether this gate is required for progression
    #[serde(rename = "required")]
    pub required: bool,

    /// Expected result for success
    #[serde(rename = "expected_result", skip_serializing_if = "Option::is_none")]
    pub expected_result: Option<String>, // "success", "failure", "any"

    /// Message to display on failure
    #[serde(rename = "failure_message", skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
}

/// Commit policy defining what types of changes are allowed in a stage
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CommitPolicy {
    /// Only engram entity changes
    EngramOnly,
    /// Documentation, examples, and research spikes
    ResearchArtifacts,
    /// Only test file changes
    TestsOnly,
    /// Code changes with accompanying tests
    CodeWithTests,
    /// All changes with full quality validation
    FullValidation,
}

/// Workflow transition defining movement between stages
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkflowTransition {
    /// Source stage name
    #[serde(rename = "from")]
    pub from: String,

    /// Target stage name  
    #[serde(rename = "to")]
    pub to: String,

    /// Transition trigger mechanism
    #[serde(rename = "trigger")]
    pub trigger: TransitionTrigger,
}

/// Trigger mechanism for workflow transitions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TransitionTrigger {
    /// Requires explicit command
    Manual,
    /// Triggered by quality gate success
    Auto,
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

    /// Add a stage transition with validation
    pub fn add_transition(&mut self, transition: WorkflowTransition) -> Result<(), String> {
        let stage_names: HashSet<&String> = self.stages.iter().map(|s| &s.name).collect();

        if !stage_names.contains(&transition.from) {
            return Err(format!("Source stage '{}' does not exist", transition.from));
        }

        if !stage_names.contains(&transition.to) {
            return Err(format!("Target stage '{}' does not exist", transition.to));
        }

        for existing in &self.transitions {
            if existing.from == transition.from && existing.to == transition.to {
                return Err(format!(
                    "Transition from '{}' to '{}' already exists",
                    transition.from, transition.to
                ));
            }
        }

        self.transitions.push(transition);
        self.updated_at = Utc::now();
        Ok(())
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

        if self.description.is_empty() {
            return Err("Workflow description cannot be empty".to_string());
        }

        if self.agent.is_empty() {
            return Err("Workflow agent cannot be empty".to_string());
        }

        let mut stage_names = HashSet::new();
        for stage in &self.stages {
            if stage.name.is_empty() {
                return Err("Stage name cannot be empty".to_string());
            }
            if !stage_names.insert(&stage.name) {
                return Err(format!("Duplicate stage name: {}", stage.name));
            }
        }

        for transition in &self.transitions {
            if !stage_names.contains(&transition.from) {
                return Err(format!(
                    "Transition references unknown stage: {}",
                    transition.from
                ));
            }
            if !stage_names.contains(&transition.to) {
                return Err(format!(
                    "Transition references unknown stage: {}",
                    transition.to
                ));
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
            data: serde_json::to_value(self).unwrap_or_else(|_| serde_json::Value::Null),
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
    fn test_workflow_add_transition_validation() {
        let mut workflow = Workflow::new("test-workflow".to_string(), "Test workflow".to_string());

        let planning_stage = WorkflowStage {
            name: "planning".to_string(),
            description: "Planning phase".to_string(),
            commit_policy: CommitPolicy::EngramOnly,
            quality_gates: vec![],
        };

        let development_stage = WorkflowStage {
            name: "development".to_string(),
            description: "Development phase".to_string(),
            commit_policy: CommitPolicy::CodeWithTests,
            quality_gates: vec![],
        };

        workflow.add_stage(planning_stage);
        workflow.add_stage(development_stage);

        let transition = WorkflowTransition {
            from: "planning".to_string(),
            to: "development".to_string(),
            trigger: TransitionTrigger::Manual,
        };

        let result = workflow.add_transition(transition);
        assert!(result.is_ok());
        assert_eq!(workflow.transitions.len(), 1);
    }

    #[test]
    fn test_workflow_transition_invalid_stage_reference() {
        let mut workflow = Workflow::new("test-workflow".to_string(), "Test workflow".to_string());

        let transition = WorkflowTransition {
            from: "nonexistent".to_string(),
            to: "development".to_string(),
            trigger: TransitionTrigger::Manual,
        };

        let result = workflow.add_transition(transition);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("Source stage 'nonexistent' does not exist"));
    }

    #[test]
    fn test_workflow_duplicate_transition() {
        let mut workflow = Workflow::new("test-workflow".to_string(), "Test workflow".to_string());

        let planning_stage = WorkflowStage {
            name: "planning".to_string(),
            description: "Planning phase".to_string(),
            commit_policy: CommitPolicy::EngramOnly,
            quality_gates: vec![],
        };

        let development_stage = WorkflowStage {
            name: "development".to_string(),
            description: "Development phase".to_string(),
            commit_policy: CommitPolicy::CodeWithTests,
            quality_gates: vec![],
        };

        workflow.add_stage(planning_stage);
        workflow.add_stage(development_stage);

        let transition1 = WorkflowTransition {
            from: "planning".to_string(),
            to: "development".to_string(),
            trigger: TransitionTrigger::Manual,
        };

        let transition2 = WorkflowTransition {
            from: "planning".to_string(),
            to: "development".to_string(),
            trigger: TransitionTrigger::Auto,
        };

        assert!(workflow.add_transition(transition1).is_ok());
        let result = workflow.add_transition(transition2);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("Transition from 'planning' to 'development' already exists"));
    }

    #[test]
    fn test_workflow_enhanced_validation() {
        let mut workflow = Workflow::new("".to_string(), "".to_string());

        let result = workflow.validate_entity();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("Workflow name cannot be empty"));

        workflow.name = "test".to_string();
        let result = workflow.validate_entity();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("Workflow description cannot be empty"));

        workflow.description = "test desc".to_string();
        workflow.agent = "".to_string();
        let result = workflow.validate_entity();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("Workflow agent cannot be empty"));
    }
}

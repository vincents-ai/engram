//! Workflow entity implementation

use super::{Entity, GenericEntity, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use validator::Validate;

/// Workflow status variants
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum WorkflowStatus {
    Active,
    Inactive,
    Draft,
    Archived,
}

/// State type variants
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum StateType {
    Start,
    InProgress,
    Review,
    Done,
    Blocked,
}

/// Transition type variants
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TransitionType {
    Automatic,
    Manual,
    Conditional,
    Scheduled,
}

/// Workflow entity
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Workflow {
    /// Unique identifier
    #[serde(rename = "id")]
    pub id: String,

    /// Workflow title
    #[serde(rename = "title")]
    pub title: String,

    /// Workflow name (alias for title for compatibility)
    #[serde(skip)]
    _name_alias: Option<String>,

    /// Workflow description
    #[serde(rename = "description")]
    pub description: String,

    /// Current status
    #[serde(rename = "status")]
    pub status: WorkflowStatus,

    /// Associated agent
    #[serde(rename = "agent")]
    pub agent: String,

    /// Creation timestamp
    #[serde(rename = "created_at")]
    pub created_at: DateTime<Utc>,

    /// Last updated timestamp
    #[serde(rename = "updated_at")]
    pub updated_at: DateTime<Utc>,

    /// Workflow states
    #[serde(rename = "states")]
    pub states: Vec<WorkflowState>,

    /// Workflow transitions
    #[serde(rename = "transitions")]
    pub transitions: Vec<WorkflowTransition>,

    /// Workflow stages (simplified interface for quality gates)
    #[serde(rename = "stages", skip_serializing_if = "Vec::is_empty", default)]
    pub stages: Vec<WorkflowStage>,

    /// Initial state
    #[serde(rename = "initial_state")]
    pub initial_state: String,

    /// Final states
    #[serde(
        rename = "final_states",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub final_states: Vec<String>,

    /// Entity types this workflow applies to
    #[serde(rename = "entity_types")]
    pub entity_types: Vec<String>,

    /// Permission schemes
    #[serde(
        rename = "permission_schemes",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub permission_schemes: Vec<PermissionScheme>,

    /// Event handlers
    #[serde(
        rename = "event_handlers",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub event_handlers: Vec<EventHandler>,

    /// Tags for categorization
    #[serde(rename = "tags", skip_serializing_if = "Vec::is_empty", default)]
    pub tags: Vec<String>,

    /// Additional metadata
    #[serde(
        rename = "metadata",
        skip_serializing_if = "HashMap::is_empty",
        default
    )]
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Workflow state
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct WorkflowState {
    /// State identifier
    #[serde(rename = "id")]
    pub id: String,

    /// State name
    #[serde(rename = "name")]
    pub name: String,

    /// State type
    #[serde(rename = "state_type")]
    pub state_type: StateType,

    /// State description
    #[serde(rename = "description")]
    pub description: String,

    /// Whether this is a final state
    #[serde(rename = "is_final")]
    pub is_final: bool,

    /// Guards (conditions for entering/leaving state)
    #[serde(rename = "guards", skip_serializing_if = "Vec::is_empty", default)]
    pub guards: Vec<StateGuard>,

    /// Post-functions (actions when entering state)
    #[serde(
        rename = "post_functions",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub post_functions: Vec<StateFunction>,
}

/// Workflow transition
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct WorkflowTransition {
    /// Transition identifier
    #[serde(rename = "id")]
    pub id: String,

    /// Transition name
    #[serde(rename = "name")]
    pub name: String,

    /// Source state
    #[serde(rename = "from_state")]
    pub from_state: String,

    /// Target state
    #[serde(rename = "to_state")]
    pub to_state: String,

    /// Transition type
    #[serde(rename = "transition_type")]
    pub transition_type: TransitionType,

    /// Transition description
    #[serde(rename = "description")]
    pub description: String,

    /// Conditions for transition
    #[serde(rename = "conditions", skip_serializing_if = "Vec::is_empty", default)]
    pub conditions: Vec<TransitionCondition>,

    /// Actions to execute during transition
    #[serde(rename = "actions", skip_serializing_if = "Vec::is_empty", default)]
    pub actions: Vec<TransitionAction>,
}

/// State guard condition
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct StateGuard {
    /// Guard identifier
    #[serde(rename = "id")]
    pub id: String,

    /// Guard type (permission, field, custom)
    #[serde(rename = "guard_type")]
    pub guard_type: String,

    /// Guard condition (JSON logic)
    #[serde(rename = "condition")]
    pub condition: serde_json::Value,

    /// Error message if guard fails
    #[serde(rename = "error_message")]
    pub error_message: String,
}

/// State function (post-function)
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct StateFunction {
    /// Function identifier
    #[serde(rename = "id")]
    pub id: String,

    /// Function name
    #[serde(rename = "name")]
    pub name: String,

    /// Function type (notification, validation, custom)
    #[serde(rename = "function_type")]
    pub function_type: String,

    /// Function parameters
    #[serde(rename = "parameters")]
    pub parameters: HashMap<String, serde_json::Value>,
}

/// Transition condition
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct TransitionCondition {
    /// Condition identifier
    #[serde(rename = "id")]
    pub id: String,

    /// Condition type
    #[serde(rename = "condition_type")]
    pub condition_type: String,

    /// Condition logic
    #[serde(rename = "logic")]
    pub logic: serde_json::Value,
}

/// Transition action
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct TransitionAction {
    /// Action identifier
    #[serde(rename = "id")]
    pub id: String,

    /// Action name
    #[serde(rename = "name")]
    pub name: String,

    /// Action type
    #[serde(rename = "action_type")]
    pub action_type: String,

    /// Action parameters
    #[serde(rename = "parameters")]
    pub parameters: HashMap<String, serde_json::Value>,
}

/// Permission scheme
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct PermissionScheme {
    /// Scheme identifier
    #[serde(rename = "id")]
    pub id: String,

    /// Scheme name
    #[serde(rename = "name")]
    pub name: String,

    /// User filter (who can perform actions)
    #[serde(rename = "user_filter")]
    pub user_filter: String,

    /// Permissions granted
    #[serde(rename = "permissions")]
    pub permissions: Vec<String>,
}

/// Event handler
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct EventHandler {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "event_type")]
    pub event_type: String,
    #[serde(rename = "event_name")]
    pub event_name: String,
    #[serde(rename = "handler")]
    pub handler: serde_json::Value,
    #[serde(rename = "active")]
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStage {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "commit_policy")]
    pub commit_policy: CommitPolicy,
    #[serde(
        rename = "quality_gates",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub quality_gates: Vec<QualityGate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityGate {
    #[serde(rename = "command")]
    pub command: String,
    #[serde(rename = "required")]
    pub required: bool,
    #[serde(rename = "expected_result", skip_serializing_if = "Option::is_none")]
    pub expected_result: Option<String>,
    #[serde(rename = "failure_message", skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommitPolicy {
    EngramOnly,
    ResearchArtifacts,
    TestsOnly,
    CodeWithTests,
    FullValidation,
}

impl Workflow {
    /// Create a new workflow
    pub fn new(title: String, description: String, agent: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            _name_alias: None,
            description,
            status: WorkflowStatus::Draft,
            agent,
            created_at: now,
            updated_at: now,
            states: Vec::new(),
            transitions: Vec::new(),
            stages: Vec::new(),
            initial_state: String::new(),
            final_states: Vec::new(),
            entity_types: Vec::new(),
            permission_schemes: Vec::new(),
            event_handlers: Vec::new(),
            tags: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Create a new workflow with simple interface (for compatibility)
    pub fn new_simple(name: String, description: String) -> Self {
        Self::new(name.clone(), description, "default".to_string())
    }

    /// Get workflow name (compatibility property)
    pub fn name(&self) -> &str {
        &self.title
    }

    /// Activate workflow
    pub fn activate(&mut self) {
        self.status = WorkflowStatus::Active;
        self.updated_at = Utc::now();
    }

    /// Deactivate workflow
    pub fn deactivate(&mut self) {
        self.status = WorkflowStatus::Inactive;
        self.updated_at = Utc::now();
    }

    /// Add a state
    pub fn add_state(&mut self, state: WorkflowState) {
        self.states.push(state);
        self.updated_at = Utc::now();
    }

    /// Add a transition
    pub fn add_transition(&mut self, transition: WorkflowTransition) {
        self.transitions.push(transition);
        self.updated_at = Utc::now();
    }

    /// Add a stage (simplified interface)
    pub fn add_stage(&mut self, stage: WorkflowStage) {
        self.stages.push(stage);
        self.updated_at = Utc::now();
    }

    /// Set initial state
    pub fn set_initial_state(&mut self, state_id: String) {
        self.initial_state = state_id;
        self.updated_at = Utc::now();
    }

    /// Add a final state
    pub fn add_final_state(&mut self, state_id: String) {
        if !self.final_states.contains(&state_id) {
            self.final_states.push(state_id);
        }
        self.updated_at = Utc::now();
    }

    /// Add entity type
    pub fn add_entity_type(&mut self, entity_type: String) {
        if !self.entity_types.contains(&entity_type) {
            self.entity_types.push(entity_type);
        }
        self.updated_at = Utc::now();
    }

    /// Get stage by name
    pub fn get_stage(&self, name: &str) -> Option<&WorkflowStage> {
        self.stages.iter().find(|stage| stage.name == name)
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

    fn validate_entity(&self) -> super::Result<()> {
        if let Err(errors) = <Workflow as validator::Validate>::validate(self) {
            let error_messages: Vec<String> = errors
                .field_errors()
                .values()
                .flat_map(|field_errors| field_errors.iter())
                .map(|error| {
                    error
                        .message
                        .clone()
                        .map(|s| s.to_string())
                        .unwrap_or_default()
                })
                .collect();
            return Err(error_messages.join(", "));
        }

        if self.title.is_empty() {
            return Err("Workflow title cannot be empty".to_string());
        }

        if self.description.is_empty() {
            return Err("Workflow description cannot be empty".to_string());
        }

        // Validate stage names are unique
        let mut stage_names = std::collections::HashSet::new();
        for stage in &self.stages {
            if !stage_names.insert(&stage.name) {
                return Err(format!("Duplicate stage name: {}", stage.name));
            }
        }

        Ok(())
    }

    fn to_generic(&self) -> GenericEntity {
        GenericEntity {
            id: self.id.clone(),
            entity_type: Self::entity_type().to_string(),
            agent: self.agent.clone(),
            timestamp: self.created_at,
            data: serde_json::to_value(self).unwrap_or_default(),
        }
    }

    fn from_generic(entity: GenericEntity) -> Result<Self> {
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
        let workflow = Workflow::new_simple(
            "feature-development".to_string(),
            "Complete BDD workflow for features".to_string(),
        );

        assert_eq!(workflow.name(), "feature-development");
        assert_eq!(workflow.description, "Complete BDD workflow for features");
        assert!(workflow.stages.is_empty());
    }

    #[test]
    fn test_workflow_serialization() {
        let workflow =
            Workflow::new_simple("test-workflow".to_string(), "Test workflow".to_string());

        let json = serde_json::to_string(&workflow).unwrap();
        let deserialized: Workflow = serde_json::from_str(&json).unwrap();

        assert_eq!(workflow.name(), deserialized.name());
    }

    #[test]
    fn test_workflow_add_stage() {
        let mut workflow =
            Workflow::new_simple("test-workflow".to_string(), "Test workflow".to_string());

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
        let mut workflow =
            Workflow::new_simple("test-workflow".to_string(), "Test workflow".to_string());

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

        let testing_stage = workflow.get_stage("testing").unwrap();
        assert_eq!(testing_stage.quality_gates.len(), 1);
        assert_eq!(testing_stage.quality_gates[0].command, "cargo test");
        assert!(testing_stage.quality_gates[0].required);
    }

    #[test]
    fn test_workflow_validation() {
        let mut workflow =
            Workflow::new_simple("test-workflow".to_string(), "Test workflow".to_string());

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

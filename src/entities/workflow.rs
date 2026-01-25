//! Workflow entity implementation

use super::{Entity, GenericEntity};
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use validator::Validate;

/// Workflow status variants
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum WorkflowStatus {
    Active,
    Inactive,
    Draft,
    Archived,
}

/// State type variants
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum StateType {
    Start,
    InProgress,
    Review,
    Done,
    Blocked,
}

/// Transition type variants
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum TransitionType {
    Automatic,
    Manual,
    Conditional,
    Scheduled,
}

/// Workflow entity
#[derive(Debug, Clone, Serialize, Deserialize, Validate, JsonSchema)]
pub struct Workflow {
    /// Unique identifier
    #[serde(rename = "id")]
    pub id: String,

    /// Workflow title
    #[serde(rename = "title")]
    pub title: String,

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

/// Prompt template for agent instructions
#[derive(Debug, Clone, Serialize, Deserialize, Validate, PartialEq, JsonSchema)]
pub struct PromptTemplate {
    /// System prompt template (sets behavior/role)
    #[serde(rename = "system", skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    /// User prompt template (specific task instructions)
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

/// Workflow state
#[derive(Debug, Clone, Serialize, Deserialize, Validate, JsonSchema)]
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

    /// Prompt templates for this state
    #[serde(rename = "prompts", skip_serializing_if = "Option::is_none")]
    pub prompts: Option<PromptTemplate>,

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
#[derive(Debug, Clone, Serialize, Deserialize, Validate, JsonSchema)]
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
#[derive(Debug, Clone, Serialize, Deserialize, Validate, JsonSchema)]
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
#[derive(Debug, Clone, Serialize, Deserialize, Validate, JsonSchema)]
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
#[derive(Debug, Clone, Serialize, Deserialize, Validate, JsonSchema)]
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
#[derive(Debug, Clone, Serialize, Deserialize, Validate, JsonSchema)]
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
#[derive(Debug, Clone, Serialize, Deserialize, Validate, JsonSchema)]
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
#[derive(Debug, Clone, Serialize, Deserialize, Validate, JsonSchema)]
pub struct EventHandler {
    /// Handler identifier
    #[serde(rename = "id")]
    pub id: String,

    /// Event type
    #[serde(rename = "event_type")]
    pub event_type: String,

    /// Event name
    #[serde(rename = "event_name")]
    pub event_name: String,

    /// Handler logic
    #[serde(rename = "handler")]
    pub handler: serde_json::Value,

    /// Whether this handler is active
    #[serde(rename = "active")]
    pub active: bool,
}

impl Workflow {
    /// Create a new workflow
    pub fn new(title: String, description: String, agent: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            description,
            status: WorkflowStatus::Draft,
            agent,
            created_at: now,
            updated_at: now,
            states: Vec::new(),
            transitions: Vec::new(),
            initial_state: String::new(),
            final_states: Vec::new(),
            entity_types: Vec::new(),
            permission_schemes: Vec::new(),
            event_handlers: Vec::new(),
            tags: Vec::new(),
            metadata: HashMap::new(),
        }
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

    fn validate_entity(&self) -> crate::Result<()> {
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
            return Err(crate::EngramError::Validation(error_messages.join(", ")));
        }

        if self.title.is_empty() {
            return Err(crate::EngramError::Validation(
                "Workflow title cannot be empty".to_string(),
            ));
        }

        if self.description.is_empty() {
            return Err(crate::EngramError::Validation(
                "Workflow description cannot be empty".to_string(),
            ));
        }

        if self.initial_state.is_empty() {
            return Err(crate::EngramError::Validation(
                "Workflow must have an initial state".to_string(),
            ));
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

    fn from_generic(entity: GenericEntity) -> crate::Result<Self> {
        serde_json::from_value(entity.data).map_err(|e| {
            crate::EngramError::Deserialization(format!("Failed to deserialize Workflow: {}", e))
        })
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

    fn create_test_state(id: &str, name: &str, is_final: bool) -> WorkflowState {
        WorkflowState {
            id: id.to_string(),
            name: name.to_string(),
            state_type: if is_final {
                StateType::Done
            } else {
                StateType::Start
            },
            description: format!("Description for {}", name),
            is_final,
            prompts: None,
            guards: vec![],
            post_functions: vec![],
        }
    }

    #[test]
    fn test_workflow_creation() {
        let workflow = Workflow::new(
            "Code Review".to_string(),
            "Standard code review workflow".to_string(),
            "agent-1".to_string(),
        );

        assert_eq!(workflow.title, "Code Review");
        assert_eq!(workflow.status, WorkflowStatus::Draft);
        assert!(workflow.states.is_empty());
        assert!(workflow.initial_state.is_empty());
    }

    #[test]
    fn test_workflow_lifecycle() {
        let mut workflow = Workflow::new(
            "Test Workflow".to_string(),
            "Description".to_string(),
            "agent-1".to_string(),
        );

        // Define states
        let start = create_test_state("start", "Start", false);
        let end = create_test_state("end", "End", true);

        workflow.add_state(start.clone());
        workflow.add_state(end.clone());
        workflow.set_initial_state(start.id.clone());
        workflow.add_final_state(end.id.clone());

        assert_eq!(workflow.states.len(), 2);
        assert_eq!(workflow.initial_state, "start");
        assert!(workflow.final_states.contains(&"end".to_string()));

        // Activation
        workflow.activate();
        assert_eq!(workflow.status, WorkflowStatus::Active);

        // Deactivation
        workflow.deactivate();
        assert_eq!(workflow.status, WorkflowStatus::Inactive);
    }

    #[test]
    fn test_workflow_validation() {
        let mut workflow = Workflow::new(
            "".to_string(), // Empty title
            "Desc".to_string(),
            "agent".to_string(),
        );
        workflow.set_initial_state("start".to_string());
        assert!(workflow.validate_entity().is_err());

        let mut workflow = Workflow::new(
            "Title".to_string(),
            "".to_string(), // Empty description
            "agent".to_string(),
        );
        workflow.set_initial_state("start".to_string());
        assert!(workflow.validate_entity().is_err());

        let workflow = Workflow::new("Title".to_string(), "Desc".to_string(), "agent".to_string());
        // Missing initial state
        assert!(workflow.validate_entity().is_err());
    }

    #[test]
    fn test_transitions() {
        let mut workflow =
            Workflow::new("Title".to_string(), "Desc".to_string(), "agent".to_string());

        let transition = WorkflowTransition {
            id: "t1".to_string(),
            name: "Submit".to_string(),
            from_state: "draft".to_string(),
            to_state: "review".to_string(),
            transition_type: TransitionType::Manual,
            description: "Submit for review".to_string(),
            conditions: vec![],
            actions: vec![],
        };

        workflow.add_transition(transition);
        assert_eq!(workflow.transitions.len(), 1);
        assert_eq!(workflow.transitions[0].name, "Submit");
    }
}

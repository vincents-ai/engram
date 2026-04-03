//! Workflow Automation Engine
//!
//! Provides state machine-based workflow automation for business processes,
//! multi-agent coordination, and automated task orchestration.

use crate::engines::action_executor::{ActionExecutor, ActionResult};
use crate::engines::rule_engine::{RuleExecutionEngine, RuleValue};
use crate::entities::{Entity, Workflow, WorkflowInstance};
use crate::error::EngramError;
use crate::storage::Storage;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::fmt;
use uuid::Uuid;

/// Workflow state definition
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkflowState {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub is_initial: bool,
    pub is_final: bool,
    pub metadata: HashMap<String, String>,
}

/// Workflow transition between states
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowTransition {
    pub id: String,
    pub name: String,
    pub from_state: String,
    pub to_state: String,
    pub condition: Option<String>, // Rule expression
    pub action: Option<String>,    // Action to execute on transition
    pub description: Option<String>,
    pub required_permissions: Vec<String>,
    pub metadata: HashMap<String, String>,
}

/// Workflow definition containing states and transitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowDefinition {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub states: Vec<WorkflowState>,
    pub transitions: Vec<WorkflowTransition>,
    pub variables: HashMap<String, RuleValue>,
    pub created_by: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Workflow execution context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowExecutionContext {
    pub variables: HashMap<String, RuleValue>,
    pub entity_id: Option<String>,
    pub entity_type: Option<String>,
    pub executing_agent: String,
    pub permissions: Vec<String>,
    pub metadata: HashMap<String, String>,
}

/// Workflow execution status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WorkflowStatus {
    Running,
    Completed,
    Failed(String),
    Suspended(String),
    Cancelled,
}

/// Workflow execution event for audit trail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowExecutionEvent {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub event_type: WorkflowEventType,
    pub from_state: Option<String>,
    pub to_state: Option<String>,
    pub transition_id: Option<String>,
    pub agent: String,
    pub message: String,
    pub metadata: HashMap<String, String>,
}

/// Types of workflow events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkflowEventType {
    Started,
    Transitioned,
    ActionExecuted,
    ConditionEvaluated,
    Failed,
    Suspended,
    Resumed,
    Completed,
    Cancelled,
}

/// Result of workflow operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowExecutionResult {
    pub success: bool,
    pub instance_id: String,
    pub current_state: String,
    pub message: String,
    pub events: Vec<WorkflowExecutionEvent>,
    pub variables_changed: HashMap<String, RuleValue>,
}

impl fmt::Display for WorkflowStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WorkflowStatus::Running => write!(f, "running"),
            WorkflowStatus::Completed => write!(f, "completed"),
            WorkflowStatus::Failed(reason) => write!(f, "failed: {}", reason),
            WorkflowStatus::Suspended(reason) => write!(f, "suspended: {}", reason),
            WorkflowStatus::Cancelled => write!(f, "cancelled"),
        }
    }
}

/// Workflow automation engine for state machine execution
pub struct WorkflowAutomationEngine<S: Storage> {
    storage: S,
    #[allow(dead_code)]
    rule_engine: RuleExecutionEngine,
    action_executor: ActionExecutor,
    active_instances: HashMap<String, WorkflowInstance>,
    event_queue: VecDeque<WorkflowExecutionEvent>,
    #[allow(dead_code)]
    max_execution_steps: usize,
}

/// Builder for workflow automation engine
pub struct WorkflowEngineBuilder<S: Storage> {
    storage: Option<S>,
    rule_engine: Option<RuleExecutionEngine>,
    action_executor: Option<ActionExecutor>,
    max_execution_steps: usize,
}

impl<S: Storage> WorkflowEngineBuilder<S> {
    pub fn new() -> Self {
        Self {
            storage: None,
            rule_engine: None,
            action_executor: None,
            max_execution_steps: 1000, // Default limit to prevent infinite loops
        }
    }

    pub fn with_storage(mut self, storage: S) -> Self {
        self.storage = Some(storage);
        self
    }

    pub fn with_rule_engine(mut self, rule_engine: RuleExecutionEngine) -> Self {
        self.rule_engine = Some(rule_engine);
        self
    }

    pub fn with_action_executor(mut self, action_executor: ActionExecutor) -> Self {
        self.action_executor = Some(action_executor);
        self
    }

    pub fn with_max_execution_steps(mut self, max_steps: usize) -> Self {
        self.max_execution_steps = max_steps;
        self
    }

    pub fn build(self) -> Result<WorkflowAutomationEngine<S>, EngramError> {
        let storage = self
            .storage
            .ok_or_else(|| EngramError::Validation("Storage is required".to_string()))?;

        let rule_engine = self.rule_engine.unwrap_or_else(RuleExecutionEngine::new);
        let action_executor = self
            .action_executor
            .unwrap_or_else(|| ActionExecutor::new(true)); // Default: allow external commands

        Ok(WorkflowAutomationEngine {
            storage,
            rule_engine,
            action_executor,
            active_instances: HashMap::new(),
            event_queue: VecDeque::new(),
            max_execution_steps: self.max_execution_steps,
        })
    }
}

impl<S: Storage> WorkflowAutomationEngine<S> {
    /// Create a new workflow automation engine
    pub fn new(storage: S) -> Self {
        Self {
            storage,
            rule_engine: RuleExecutionEngine::new(),
            action_executor: ActionExecutor::new(true), // Default: allow external commands
            active_instances: HashMap::new(),
            event_queue: VecDeque::new(),
            max_execution_steps: 1000,
        }
    }

    /// Create a new workflow definition
    pub fn create_workflow(
        &mut self,
        name: String,
        description: Option<String>,
        creator: String,
    ) -> Result<WorkflowDefinition, EngramError> {
        let workflow = WorkflowDefinition {
            id: Uuid::new_v4().to_string(),
            name,
            version: "1.0.0".to_string(),
            description,
            states: Vec::new(),
            transitions: Vec::new(),
            variables: HashMap::new(),
            created_by: creator,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // Store workflow definition (simplified - would need proper entity storage)
        // let generic_entity = GenericEntity::from_value(serde_json::to_value(&workflow)?)?;
        // self.storage.store(&generic_entity)?;

        Ok(workflow)
    }

    /// Add a state to workflow definition
    pub fn add_state(
        &mut self,
        _workflow_id: &str,
        name: String,
        description: Option<String>,
        is_initial: bool,
        is_final: bool,
    ) -> Result<WorkflowState, EngramError> {
        let state = WorkflowState {
            id: Uuid::new_v4().to_string(),
            name,
            description,
            is_initial,
            is_final,
            metadata: HashMap::new(),
        };

        // TODO: Update workflow definition in storage
        // This would require retrieving, modifying, and storing the workflow

        Ok(state)
    }

    /// Add a transition to workflow definition
    pub fn add_transition(
        &mut self,
        _workflow_id: &str,
        name: String,
        from_state: String,
        to_state: String,
        condition: Option<String>,
        action: Option<String>,
    ) -> Result<WorkflowTransition, EngramError> {
        let transition = WorkflowTransition {
            id: Uuid::new_v4().to_string(),
            name,
            from_state,
            to_state,
            condition,
            action,
            description: None,
            required_permissions: Vec::new(),
            metadata: HashMap::new(),
        };

        // TODO: Update workflow definition in storage

        Ok(transition)
    }

    /// Start a new workflow instance
    pub fn start_workflow(
        &mut self,
        workflow_id: String,
        entity_id: Option<String>,
        entity_type: Option<String>,
        executing_agent: String,
        initial_variables: HashMap<String, RuleValue>,
    ) -> Result<WorkflowExecutionResult, EngramError> {
        let definition = self.load_workflow_definition(&workflow_id)?;

        let initial_state_name = definition
            .states
            .iter()
            .find(|s| s.id == definition.initial_state)
            .map(|s| s.name.clone())
            .unwrap_or_else(|| definition.initial_state.clone());

        let instance_id = Uuid::new_v4().to_string();
        let now = Utc::now();

        let context = WorkflowExecutionContext {
            variables: initial_variables,
            entity_id,
            entity_type,
            executing_agent: executing_agent.clone(),
            permissions: Vec::new(),
            metadata: HashMap::new(),
        };

        let start_event = WorkflowExecutionEvent {
            id: Uuid::new_v4().to_string(),
            timestamp: now,
            event_type: WorkflowEventType::Started,
            from_state: None,
            to_state: Some(initial_state_name.clone()),
            transition_id: None,
            agent: executing_agent.clone(),
            message: "Workflow started".to_string(),
            metadata: HashMap::new(),
        };

        let instance = WorkflowInstance {
            id: instance_id.clone(),
            workflow_id,
            current_state: initial_state_name.clone(),
            context,
            status: WorkflowStatus::Running,
            started_at: now,
            updated_at: now,
            completed_at: None,
            execution_history: vec![start_event.clone()],
        };

        self.active_instances
            .insert(instance_id.clone(), instance.clone());

        self.storage.store(&instance.to_generic())?;

        Ok(WorkflowExecutionResult {
            success: true,
            instance_id,
            current_state: initial_state_name,
            message: "Workflow started successfully".to_string(),
            events: vec![start_event],
            variables_changed: HashMap::new(),
        })
    }

    /// Execute a transition in a workflow instance
    pub fn execute_transition(
        &mut self,
        instance_id: &str,
        transition_name: String,
        executing_agent: String,
    ) -> Result<WorkflowExecutionResult, EngramError> {
        if !self.active_instances.contains_key(instance_id) {
            if let Some(generic) = self.storage.get(instance_id, "workflow_instance")? {
                let instance = WorkflowInstance::from_generic(generic)
                    .map_err(|e| EngramError::Validation(e.to_string()))?;
                self.active_instances
                    .insert(instance_id.to_string(), instance);
            } else {
                return Err(EngramError::NotFound(format!(
                    "Workflow instance {} not found",
                    instance_id
                )));
            }
        }

        let (current_state, workflow_id) = {
            let instance = self.active_instances.get(instance_id).unwrap();
            (instance.current_state.clone(), instance.workflow_id.clone())
        };

        let definition = self.load_workflow_definition(&workflow_id)?;

        let transition = definition
            .transitions
            .iter()
            .find(|t| {
                t.name == transition_name
                    && definition
                        .states
                        .iter()
                        .any(|s| s.id == t.from_state && s.name == current_state)
            })
            .ok_or_else(|| {
                EngramError::Validation(format!(
                    "Invalid transition '{}' from state '{}'",
                    transition_name, current_state
                ))
            })?;

        let target_state_name = definition
            .states
            .iter()
            .find(|s| s.id == transition.to_state)
            .map(|s| s.name.clone())
            .unwrap_or_else(|| transition.to_state.clone());

        let is_final = definition
            .states
            .iter()
            .any(|s| s.id == transition.to_state && s.is_final);

        let instance = self.active_instances.get_mut(instance_id).unwrap();

        let transition_event = WorkflowExecutionEvent {
            id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            event_type: WorkflowEventType::Transitioned,
            from_state: Some(current_state),
            to_state: Some(target_state_name.clone()),
            transition_id: Some(transition.id.clone()),
            agent: executing_agent,
            message: format!("Transitioned via {}", transition_name),
            metadata: HashMap::new(),
        };

        instance.current_state = target_state_name.clone();
        instance.updated_at = Utc::now();
        instance.execution_history.push(transition_event.clone());
        if is_final {
            instance.status = WorkflowStatus::Completed;
            instance.completed_at = Some(Utc::now());
        }

        self.storage.store(&instance.to_generic())?;

        Ok(WorkflowExecutionResult {
            success: true,
            instance_id: instance_id.to_string(),
            current_state: target_state_name,
            message: "Transition executed successfully".to_string(),
            events: vec![transition_event],
            variables_changed: HashMap::new(),
        })
    }

    /// Execute an action defined in a transition
    pub fn execute_transition_action(
        &self,
        action_type: &str,
        parameters: &HashMap<String, serde_json::Value>,
    ) -> Result<ActionResult, EngramError> {
        self.action_executor.execute_action(action_type, parameters)
    }

    /// Get workflow instance status
    pub fn get_instance_status(&self, instance_id: &str) -> Result<WorkflowInstance, EngramError> {
        if let Some(instance) = self.active_instances.get(instance_id) {
            return Ok(instance.clone());
        }

        if let Some(generic) = self.storage.get(instance_id, "workflow_instance")? {
            return WorkflowInstance::from_generic(generic)
                .map_err(|e| EngramError::Validation(e.to_string()));
        }

        Err(EngramError::NotFound(format!(
            "Workflow instance {} not found",
            instance_id
        )))
    }

    /// List all active workflow instances
    pub fn list_active_instances(&self) -> Vec<WorkflowInstance> {
        match self.storage.get_all("workflow_instance") {
            Ok(entities) => entities
                .into_iter()
                .filter_map(|e| WorkflowInstance::from_generic(e).ok())
                .collect(),
            Err(_) => Vec::new(),
        }
    }

    /// Cancel a workflow instance
    pub fn cancel_workflow(
        &mut self,
        instance_id: &str,
        executing_agent: String,
        reason: String,
    ) -> Result<WorkflowExecutionResult, EngramError> {
        if !self.active_instances.contains_key(instance_id) {
            if let Some(generic) = self.storage.get(instance_id, "workflow_instance")? {
                let instance = WorkflowInstance::from_generic(generic)
                    .map_err(|e| EngramError::Validation(e.to_string()))?;
                self.active_instances
                    .insert(instance_id.to_string(), instance);
            } else {
                return Err(EngramError::NotFound(format!(
                    "Workflow instance {} not found",
                    instance_id
                )));
            }
        }

        let instance = self.active_instances.get_mut(instance_id).unwrap();

        let cancel_event = WorkflowExecutionEvent {
            id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            event_type: WorkflowEventType::Cancelled,
            from_state: Some(instance.current_state.clone()),
            to_state: None,
            transition_id: None,
            agent: executing_agent,
            message: format!("Workflow cancelled: {}", reason),
            metadata: HashMap::new(),
        };

        instance.status = WorkflowStatus::Cancelled;
        instance.updated_at = Utc::now();
        instance.completed_at = Some(Utc::now());
        instance.execution_history.push(cancel_event.clone());

        self.storage.store(&instance.to_generic())?;

        Ok(WorkflowExecutionResult {
            success: true,
            instance_id: instance_id.to_string(),
            current_state: instance.current_state.clone(),
            message: "Workflow cancelled successfully".to_string(),
            events: vec![cancel_event],
            variables_changed: HashMap::new(),
        })
    }

    /// Update workflow instance variables
    pub fn update_instance_variables(
        &mut self,
        instance_id: &str,
        variables: HashMap<String, RuleValue>,
    ) -> Result<(), EngramError> {
        if !self.active_instances.contains_key(instance_id) {
            if let Some(generic) = self.storage.get(instance_id, "workflow_instance")? {
                let instance = WorkflowInstance::from_generic(generic)
                    .map_err(|e| EngramError::Validation(e.to_string()))?;
                self.active_instances
                    .insert(instance_id.to_string(), instance);
            } else {
                return Err(EngramError::NotFound(format!(
                    "Workflow instance {} not found",
                    instance_id
                )));
            }
        }

        let instance = self.active_instances.get_mut(instance_id).unwrap();

        // Merge variables
        for (key, value) in variables {
            instance.context.variables.insert(key, value);
        }

        instance.updated_at = Utc::now();
        self.storage.store(&instance.to_generic())?;

        Ok(())
    }

    /// Process pending workflow events
    pub fn process_events(&mut self) -> Result<Vec<WorkflowExecutionResult>, EngramError> {
        let results = Vec::new();

        while let Some(event) = self.event_queue.pop_front() {
            // Process event based on type
            match event.event_type {
                WorkflowEventType::Started => {
                    // Handle workflow start events
                }
                WorkflowEventType::Transitioned => {
                    // Handle transition events
                }
                _ => {
                    // Handle other event types
                }
            }
        }

        Ok(results)
    }

    /// Load a workflow definition from storage and convert to engine representation
    fn load_workflow_definition(&self, workflow_id: &str) -> Result<Workflow, EngramError> {
        let generic = self.storage.get(workflow_id, "workflow")?.ok_or_else(|| {
            EngramError::NotFound(format!("Workflow definition {} not found", workflow_id))
        })?;

        Workflow::from_generic(generic).map_err(|e| EngramError::Validation(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::MemoryStorage;

    fn create_test_engine() -> WorkflowAutomationEngine<MemoryStorage> {
        WorkflowAutomationEngine::new(MemoryStorage::new("test-agent"))
    }

    fn create_test_workflow_in_storage(
        engine: &mut WorkflowAutomationEngine<MemoryStorage>,
    ) -> String {
        let state_start = crate::entities::WorkflowState {
            id: "state-start".to_string(),
            name: "initial".to_string(),
            state_type: crate::entities::StateType::Start,
            description: "Start state".to_string(),
            is_final: false,
            prompts: None,
            guards: vec![],
            post_functions: vec![],
        };
        let state_progress = crate::entities::WorkflowState {
            id: "state-progress".to_string(),
            name: "in_progress".to_string(),
            state_type: crate::entities::StateType::InProgress,
            description: "Working".to_string(),
            is_final: false,
            prompts: None,
            guards: vec![],
            post_functions: vec![],
        };
        let state_done = crate::entities::WorkflowState {
            id: "state-done".to_string(),
            name: "completed".to_string(),
            state_type: crate::entities::StateType::Done,
            description: "Finished".to_string(),
            is_final: true,
            prompts: None,
            guards: vec![],
            post_functions: vec![],
        };

        let workflow_id = "test-workflow-def".to_string();
        let mut workflow = crate::entities::Workflow::new(
            "Test Workflow".to_string(),
            "A test workflow".to_string(),
            "test-agent".to_string(),
        );
        workflow.id = workflow_id.clone();
        workflow.states = vec![
            state_start.clone(),
            state_progress.clone(),
            state_done.clone(),
        ];
        workflow.transitions = vec![
            crate::entities::WorkflowTransition {
                id: "t-start".to_string(),
                name: "start".to_string(),
                from_state: state_start.id.clone(),
                to_state: state_progress.id.clone(),
                transition_type: crate::entities::TransitionType::Manual,
                description: "Begin work".to_string(),
                conditions: vec![],
                actions: vec![],
            },
            crate::entities::WorkflowTransition {
                id: "t-complete".to_string(),
                name: "complete".to_string(),
                from_state: state_progress.id.clone(),
                to_state: state_done.id.clone(),
                transition_type: crate::entities::TransitionType::Manual,
                description: "Finish".to_string(),
                conditions: vec![],
                actions: vec![],
            },
        ];
        workflow.initial_state = state_start.id.clone();
        workflow.final_states = vec![state_done.id.clone()];
        workflow.activate();

        engine.storage.store(&workflow.to_generic()).unwrap();
        workflow_id
    }

    #[test]
    fn test_workflow_engine_creation() {
        let engine = create_test_engine();
        assert_eq!(engine.active_instances.len(), 0);
        assert_eq!(engine.event_queue.len(), 0);
    }

    #[test]
    fn test_create_workflow() {
        let mut engine = create_test_engine();

        let workflow = engine
            .create_workflow(
                "Test Workflow".to_string(),
                Some("A test workflow".to_string()),
                "test-agent".to_string(),
            )
            .unwrap();

        assert_eq!(workflow.name, "Test Workflow");
        assert_eq!(workflow.version, "1.0.0");
        assert_eq!(workflow.created_by, "test-agent");
    }

    #[test]
    fn test_start_workflow() {
        let mut engine = create_test_engine();
        let workflow_id = create_test_workflow_in_storage(&mut engine);

        let result = engine
            .start_workflow(
                workflow_id,
                Some("entity-123".to_string()),
                Some("task".to_string()),
                "test-agent".to_string(),
                HashMap::new(),
            )
            .unwrap();

        assert!(result.success);
        assert_eq!(result.current_state, "initial");
        assert_eq!(engine.active_instances.len(), 1);
    }

    #[test]
    fn test_execute_transition() {
        let mut engine = create_test_engine();
        let workflow_id = create_test_workflow_in_storage(&mut engine);

        let start_result = engine
            .start_workflow(
                workflow_id,
                None,
                None,
                "test-agent".to_string(),
                HashMap::new(),
            )
            .unwrap();

        let transition_result = engine
            .execute_transition(
                &start_result.instance_id,
                "start".to_string(),
                "test-agent".to_string(),
            )
            .unwrap();

        assert!(transition_result.success);
        assert_eq!(transition_result.current_state, "in_progress");
    }

    #[test]
    fn test_workflow_completion() {
        let mut engine = create_test_engine();
        let workflow_id = create_test_workflow_in_storage(&mut engine);

        let start_result = engine
            .start_workflow(
                workflow_id,
                None,
                None,
                "test-agent".to_string(),
                HashMap::new(),
            )
            .unwrap();

        engine
            .execute_transition(
                &start_result.instance_id,
                "start".to_string(),
                "test-agent".to_string(),
            )
            .unwrap();

        let complete_result = engine
            .execute_transition(
                &start_result.instance_id,
                "complete".to_string(),
                "test-agent".to_string(),
            )
            .unwrap();

        assert_eq!(complete_result.current_state, "completed");

        let instance = engine
            .get_instance_status(&start_result.instance_id)
            .unwrap();
        assert_eq!(instance.status, WorkflowStatus::Completed);
    }

    #[test]
    fn test_cancel_workflow() {
        let mut engine = create_test_engine();
        let workflow_id = create_test_workflow_in_storage(&mut engine);

        let start_result = engine
            .start_workflow(
                workflow_id,
                None,
                None,
                "test-agent".to_string(),
                HashMap::new(),
            )
            .unwrap();

        let cancel_result = engine
            .cancel_workflow(
                &start_result.instance_id,
                "test-agent".to_string(),
                "Testing cancellation".to_string(),
            )
            .unwrap();

        assert!(cancel_result.success);

        let instance = engine
            .get_instance_status(&start_result.instance_id)
            .unwrap();
        assert_eq!(instance.status, WorkflowStatus::Cancelled);
    }

    #[test]
    fn test_workflow_builder() {
        let storage = MemoryStorage::new("test-agent");
        let rule_engine = RuleExecutionEngine::new();

        let engine = WorkflowEngineBuilder::new()
            .with_storage(storage)
            .with_rule_engine(rule_engine)
            .with_max_execution_steps(500)
            .build()
            .unwrap();

        assert_eq!(engine.max_execution_steps, 500);
    }

    #[test]
    fn test_invalid_transition() {
        let mut engine = create_test_engine();
        let workflow_id = create_test_workflow_in_storage(&mut engine);

        let start_result = engine
            .start_workflow(
                workflow_id,
                None,
                None,
                "test-agent".to_string(),
                HashMap::new(),
            )
            .unwrap();

        let result = engine.execute_transition(
            &start_result.instance_id,
            "invalid_transition".to_string(),
            "test-agent".to_string(),
        );

        assert!(result.is_err());
        match result {
            Err(EngramError::Validation(msg)) => {
                assert!(msg.contains("Invalid transition"));
            }
            _ => panic!("Expected ValidationError"),
        }
    }

    #[test]
    fn test_invalid_transition_from_wrong_state() {
        let mut engine = create_test_engine();
        let workflow_id = create_test_workflow_in_storage(&mut engine);

        let start_result = engine
            .start_workflow(
                workflow_id,
                None,
                None,
                "test-agent".to_string(),
                HashMap::new(),
            )
            .unwrap();

        // "complete" is only valid from in_progress, not from initial
        let result = engine.execute_transition(
            &start_result.instance_id,
            "complete".to_string(),
            "test-agent".to_string(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_list_active_instances() {
        let mut engine = create_test_engine();
        let workflow_id = create_test_workflow_in_storage(&mut engine);

        engine
            .start_workflow(
                workflow_id.clone(),
                None,
                None,
                "agent-1".to_string(),
                HashMap::new(),
            )
            .unwrap();

        engine
            .start_workflow(
                workflow_id,
                None,
                None,
                "agent-2".to_string(),
                HashMap::new(),
            )
            .unwrap();

        let instances = engine.list_active_instances();
        assert_eq!(instances.len(), 2);
    }
}

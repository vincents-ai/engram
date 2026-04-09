//! Workflow Automation Engine
//!
//! Provides state machine-based workflow automation for business processes,
//! multi-agent coordination, and automated task orchestration.

use crate::engines::action_executor::{ActionExecutor, ActionResult};
use crate::engines::rule_engine::{RuleExecutionContext, RuleExecutionEngine, RuleValue};
use crate::entities::{Entity, Task, TriggerCondition, Workflow, WorkflowInstance};
use crate::error::EngramError;
use crate::storage::{QueryFilter, Storage};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::time::Duration as StdDuration;
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
    pub condition: Option<String>,
    pub action: Option<String>,
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
    AutoTriggered,
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
    rule_engine: RuleExecutionEngine,
    action_executor: ActionExecutor,
    active_instances: HashMap<String, WorkflowInstance>,
    max_execution_steps: u64,
}

/// Builder for workflow automation engine
pub struct WorkflowEngineBuilder<S: Storage> {
    storage: Option<S>,
    rule_engine: Option<RuleExecutionEngine>,
    action_executor: Option<ActionExecutor>,
    max_execution_steps: u64,
}

impl<S: Storage> WorkflowEngineBuilder<S> {
    pub fn new() -> Self {
        Self {
            storage: None,
            rule_engine: None,
            action_executor: None,
            max_execution_steps: 1000,
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

    pub fn with_max_execution_steps(mut self, max_steps: u64) -> Self {
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
            .unwrap_or_else(|| ActionExecutor::new(true));

        Ok(WorkflowAutomationEngine {
            storage,
            rule_engine,
            action_executor,
            active_instances: HashMap::new(),
            max_execution_steps: self.max_execution_steps,
        })
    }
}

impl<S: Storage> WorkflowAutomationEngine<S> {
    pub fn new(storage: S) -> Self {
        Self {
            storage,
            rule_engine: RuleExecutionEngine::new(),
            action_executor: ActionExecutor::new(true),
            active_instances: HashMap::new(),
            max_execution_steps: 1000,
        }
    }

    pub fn create_workflow(
        &mut self,
        name: String,
        description: Option<String>,
        creator: String,
    ) -> Result<Workflow, EngramError> {
        let workflow = Workflow::new(name, description.unwrap_or_default(), creator);
        if workflow.title.is_empty() {
            return Err(EngramError::Validation(
                "Workflow title cannot be empty".to_string(),
            ));
        }
        if workflow.description.is_empty() {
            return Err(EngramError::Validation(
                "Workflow description cannot be empty".to_string(),
            ));
        }
        self.storage.store(&workflow.to_generic())?;
        Ok(workflow)
    }

    pub fn add_state(
        &mut self,
        workflow_id: &str,
        name: String,
        description: Option<String>,
        is_initial: bool,
        is_final: bool,
    ) -> Result<crate::entities::WorkflowState, EngramError> {
        let mut workflow = self.load_workflow_definition(workflow_id)?;

        let state_id = Uuid::new_v4().to_string();
        let state = crate::entities::WorkflowState {
            id: state_id.clone(),
            name: name.clone(),
            state_type: if is_initial {
                crate::entities::StateType::Start
            } else if is_final {
                crate::entities::StateType::Done
            } else {
                crate::entities::StateType::InProgress
            },
            description: description.unwrap_or_default(),
            is_final,
            prompts: None,
            guards: vec![],
            post_functions: vec![],
            commit_policy: None,
        };

        if is_initial {
            workflow.set_initial_state(state_id.clone());
        }
        if is_final {
            workflow.add_final_state(state_id);
        }
        workflow.add_state(state.clone());
        workflow.validate_entity()?;
        self.storage.store(&workflow.to_generic())?;

        Ok(state)
    }

    pub fn add_transition(
        &mut self,
        workflow_id: &str,
        name: String,
        from_state: String,
        to_state: String,
        condition: Option<String>,
        action: Option<String>,
    ) -> Result<crate::entities::WorkflowTransition, EngramError> {
        let mut workflow = self.load_workflow_definition(workflow_id)?;

        let conditions = if let Some(cond) = condition {
            vec![crate::entities::TransitionCondition {
                id: Uuid::new_v4().to_string(),
                condition_type: "field".to_string(),
                logic: serde_json::from_str(&cond)
                    .unwrap_or_else(|_| serde_json::json!({"field": cond, "equals": true})),
            }]
        } else {
            vec![]
        };

        let transition = crate::entities::WorkflowTransition {
            id: Uuid::new_v4().to_string(),
            name: name.clone(),
            from_state,
            to_state,
            transition_type: crate::entities::TransitionType::Manual,
            description: action.unwrap_or_default(),
            conditions,
            actions: vec![],
            trigger: None,
        };

        workflow.add_transition(transition.clone());
        workflow.validate_entity()?;
        self.storage.store(&workflow.to_generic())?;

        Ok(transition)
    }

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
            step_count: 0,
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

    pub fn execute_transition(
        &mut self,
        instance_id: &str,
        transition_name: String,
        executing_agent: String,
    ) -> Result<WorkflowExecutionResult, EngramError> {
        self.ensure_instance_loaded(instance_id)?;

        let (current_state, workflow_id, instance_status) = {
            let instance = self.active_instances.get(instance_id).unwrap();
            (
                instance.current_state.clone(),
                instance.workflow_id.clone(),
                instance.status.clone(),
            )
        };

        match &instance_status {
            WorkflowStatus::Completed => {
                return Err(EngramError::InvalidOperation(format!(
                    "Workflow instance {} is already completed",
                    instance_id
                )));
            }
            WorkflowStatus::Cancelled => {
                return Err(EngramError::InvalidOperation(format!(
                    "Workflow instance {} is already cancelled",
                    instance_id
                )));
            }
            WorkflowStatus::Failed(reason) => {
                return Err(EngramError::InvalidOperation(format!(
                    "Workflow instance {} is failed: {}",
                    instance_id, reason
                )));
            }
            WorkflowStatus::Suspended(reason) => {
                return Err(EngramError::InvalidOperation(format!(
                    "Workflow instance {} is suspended: {}",
                    instance_id, reason
                )));
            }
            WorkflowStatus::Running => {}
        }

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

        {
            let instance = self.active_instances.get(instance_id).unwrap();
            if instance.step_count >= self.max_execution_steps {
                return Err(EngramError::InvalidOperation(format!(
                    "Workflow instance {} exceeded max execution steps ({})",
                    instance_id, self.max_execution_steps
                )));
            }
        }

        let mut condition_events = Vec::new();
        for condition in &transition.conditions {
            let instance = self.active_instances.get(instance_id).unwrap();
            let passed = self.evaluate_transition_condition(condition, instance);
            let condition_event = WorkflowExecutionEvent {
                id: Uuid::new_v4().to_string(),
                timestamp: Utc::now(),
                event_type: WorkflowEventType::ConditionEvaluated,
                from_state: Some(current_state.clone()),
                to_state: Some(target_state_name.clone()),
                transition_id: Some(transition.id.clone()),
                agent: executing_agent.clone(),
                message: format!(
                    "Condition '{}' ({}) evaluated: {}",
                    condition.id,
                    condition.condition_type,
                    if passed { "passed" } else { "failed" }
                ),
                metadata: {
                    let mut m = HashMap::new();
                    m.insert("condition_id".to_string(), condition.id.clone());
                    m.insert("passed".to_string(), passed.to_string());
                    m
                },
            };
            {
                let instance = self.active_instances.get_mut(instance_id).unwrap();
                instance.execution_history.push(condition_event.clone());
            }
            condition_events.push(condition_event);

            if !passed {
                {
                    let instance = self.active_instances.get_mut(instance_id).unwrap();
                    instance.updated_at = Utc::now();
                    self.storage.store(&instance.to_generic())?;
                }

                return Ok(WorkflowExecutionResult {
                    success: false,
                    instance_id: instance_id.to_string(),
                    current_state: current_state.clone(),
                    message: format!(
                        "Transition '{}' blocked by condition '{}'",
                        transition_name, condition.id
                    ),
                    events: condition_events,
                    variables_changed: HashMap::new(),
                });
            }
        }

        let mut action_events = Vec::new();
        let mut action_failed = false;

        for action in &transition.actions {
            let result = self
                .action_executor
                .execute_action(&action.action_type, &action.parameters);

            let (success, message, action_metadata) = match &result {
                Ok(ar) => (ar.success, ar.message.clone(), {
                    let mut m = HashMap::new();
                    if let Some(ref output) = ar.output {
                        m.insert("output".to_string(), output.clone());
                    }
                    if let Some(ref error) = ar.error {
                        m.insert("error".to_string(), error.clone());
                    }
                    if let Some(code) = ar.exit_code {
                        m.insert("exit_code".to_string(), code.to_string());
                    }
                    m
                }),
                Err(e) => (false, e.to_string(), HashMap::new()),
            };

            let should_block =
                action.on_failure.as_ref() == Some(&crate::entities::ActionFailurePolicy::Block);

            if !success {
                tracing::warn!(
                    instance_id = instance_id,
                    transition = %transition_name,
                    action_id = %action.id,
                    action_name = %action.name,
                    "Transition action failed: {} (on_failure: {:?})",
                    message,
                    action.on_failure,
                );

                if should_block {
                    action_failed = true;
                }
            }

            let event = WorkflowExecutionEvent {
                id: Uuid::new_v4().to_string(),
                timestamp: Utc::now(),
                event_type: WorkflowEventType::ActionExecuted,
                from_state: Some(current_state.clone()),
                to_state: Some(target_state_name.clone()),
                transition_id: Some(transition.id.clone()),
                agent: executing_agent.clone(),
                message: format!(
                    "Action '{}' ({}): {}",
                    action.name,
                    action.action_type,
                    if success { "ok" } else { "failed" }
                ),
                metadata: {
                    let mut m = action_metadata;
                    m.insert("action_id".to_string(), action.id.clone());
                    m.insert("action_name".to_string(), action.name.clone());
                    m.insert("action_type".to_string(), action.action_type.clone());
                    m.insert("success".to_string(), success.to_string());
                    if !success && should_block {
                        m.insert("blocked_transition".to_string(), "true".to_string());
                    }
                    m
                },
            };
            {
                let instance = self.active_instances.get_mut(instance_id).unwrap();
                instance.execution_history.push(event.clone());
            }
            action_events.push(event);
        }

        if action_failed {
            let fail_event = WorkflowExecutionEvent {
                id: Uuid::new_v4().to_string(),
                timestamp: Utc::now(),
                event_type: WorkflowEventType::Failed,
                from_state: Some(current_state.clone()),
                to_state: None,
                transition_id: Some(transition.id.clone()),
                agent: executing_agent.clone(),
                message: "Transition blocked by failing action with on_failure=block".to_string(),
                metadata: HashMap::new(),
            };
            {
                let instance = self.active_instances.get_mut(instance_id).unwrap();
                instance.execution_history.push(fail_event.clone());
                instance.updated_at = Utc::now();
                self.storage.store(&instance.to_generic())?;
            }

            let mut all_events = action_events;
            all_events.push(fail_event);

            return Ok(WorkflowExecutionResult {
                success: false,
                instance_id: instance_id.to_string(),
                current_state: current_state.clone(),
                message: "Transition blocked by failing action".to_string(),
                events: all_events,
                variables_changed: HashMap::new(),
            });
        }

        let transition_event = WorkflowExecutionEvent {
            id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            event_type: WorkflowEventType::Transitioned,
            from_state: Some(current_state),
            to_state: Some(target_state_name.clone()),
            transition_id: Some(transition.id.clone()),
            agent: executing_agent.clone(),
            message: format!("Transitioned via {}", transition_name),
            metadata: HashMap::new(),
        };

        {
            let instance = self.active_instances.get_mut(instance_id).unwrap();
            instance.step_count += 1;
            instance.current_state = target_state_name.clone();
            instance.updated_at = Utc::now();
            instance.execution_history.push(transition_event.clone());
            if is_final {
                instance.status = WorkflowStatus::Completed;
                instance.completed_at = Some(Utc::now());
            }
        }

        let target_state = definition
            .states
            .iter()
            .find(|s| s.id == transition.to_state);
        let mut post_fn_events = Vec::new();
        if let Some(target) = target_state {
            post_fn_events =
                self.execute_state_post_functions(target, instance_id, &executing_agent);
            for ev in &post_fn_events {
                let instance = self.active_instances.get_mut(instance_id).unwrap();
                instance.execution_history.push(ev.clone());
            }
        }

        {
            let instance = self.active_instances.get_mut(instance_id).unwrap();
            self.storage.store(&instance.to_generic())?;
        }

        let mut all_events = condition_events;
        all_events.append(&mut action_events);
        all_events.push(transition_event);
        all_events.append(&mut post_fn_events);

        self.update_bound_tasks_workflow_state(instance_id, &target_state_name);

        Ok(WorkflowExecutionResult {
            success: true,
            instance_id: instance_id.to_string(),
            current_state: target_state_name,
            message: "Transition executed successfully".to_string(),
            events: all_events,
            variables_changed: HashMap::new(),
        })
    }

    pub fn execute_transition_action(
        &self,
        action_type: &str,
        parameters: &HashMap<String, serde_json::Value>,
    ) -> Result<ActionResult, EngramError> {
        self.action_executor.execute_action(action_type, parameters)
    }

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

    pub fn list_active_instances(&self) -> Vec<WorkflowInstance> {
        match self.storage.get_all("workflow_instance") {
            Ok(entities) => entities
                .into_iter()
                .filter_map(|e| WorkflowInstance::from_generic(e).ok())
                .collect(),
            Err(_) => Vec::new(),
        }
    }

    fn update_bound_tasks_workflow_state(&mut self, instance_id: &str, new_state: &str) {
        let filter = QueryFilter {
            entity_type: Some("task".to_string()),
            field_filters: {
                let mut m = std::collections::HashMap::new();
                m.insert(
                    "workflow_id".to_string(),
                    serde_json::Value::String(instance_id.to_string()),
                );
                m
            },
            limit: None,
            offset: None,
            ..Default::default()
        };

        let result = match self.storage.query(&filter) {
            Ok(r) => r,
            Err(e) => {
                tracing::warn!(
                    instance_id = instance_id,
                    error = %e,
                    "Failed to query tasks bound to workflow instance"
                );
                return;
            }
        };

        let mut updated_count = 0usize;
        for entity in result.entities {
            if let Ok(mut task) = Task::from_generic(entity) {
                if task.workflow_state.as_deref() != Some(new_state) {
                    task.update_workflow_state(new_state.to_string());
                    if self.storage.store(&task.to_generic()).is_ok() {
                        updated_count += 1;
                    }
                }
            }
        }

        if updated_count > 0 {
            tracing::info!(
                instance_id = instance_id,
                new_state = new_state,
                updated_count = updated_count,
                "Updated workflow_state on bound tasks"
            );
        }
    }

    pub fn suspend_workflow(
        &mut self,
        instance_id: &str,
        executing_agent: String,
        reason: String,
    ) -> Result<WorkflowExecutionResult, EngramError> {
        self.ensure_instance_loaded(instance_id)?;
        let instance = self.active_instances.get_mut(instance_id).unwrap();

        match &instance.status {
            WorkflowStatus::Completed => {
                return Err(EngramError::InvalidOperation(format!(
                    "Workflow instance {} is already completed",
                    instance_id
                )));
            }
            WorkflowStatus::Cancelled => {
                return Err(EngramError::InvalidOperation(format!(
                    "Workflow instance {} is already cancelled",
                    instance_id
                )));
            }
            WorkflowStatus::Suspended(_) => {
                return Err(EngramError::InvalidOperation(format!(
                    "Workflow instance {} is already suspended",
                    instance_id
                )));
            }
            WorkflowStatus::Failed(_) => {
                return Err(EngramError::InvalidOperation(format!(
                    "Workflow instance {} is failed; cannot suspend",
                    instance_id
                )));
            }
            WorkflowStatus::Running => {}
        }

        let suspend_event = WorkflowExecutionEvent {
            id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            event_type: WorkflowEventType::Suspended,
            from_state: Some(instance.current_state.clone()),
            to_state: None,
            transition_id: None,
            agent: executing_agent,
            message: format!("Workflow suspended: {}", reason),
            metadata: HashMap::new(),
        };

        instance.status = WorkflowStatus::Suspended(reason);
        instance.updated_at = Utc::now();
        instance.execution_history.push(suspend_event.clone());

        self.storage.store(&instance.to_generic())?;

        Ok(WorkflowExecutionResult {
            success: true,
            instance_id: instance_id.to_string(),
            current_state: instance.current_state.clone(),
            message: "Workflow suspended successfully".to_string(),
            events: vec![suspend_event],
            variables_changed: HashMap::new(),
        })
    }

    pub fn resume_workflow(
        &mut self,
        instance_id: &str,
        executing_agent: String,
    ) -> Result<WorkflowExecutionResult, EngramError> {
        self.ensure_instance_loaded(instance_id)?;
        let instance = self.active_instances.get_mut(instance_id).unwrap();

        match &instance.status {
            WorkflowStatus::Suspended(_) => {}
            other => {
                return Err(EngramError::InvalidOperation(format!(
                    "Workflow instance {} is not suspended (current status: {})",
                    instance_id, other
                )));
            }
        }

        let resume_event = WorkflowExecutionEvent {
            id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            event_type: WorkflowEventType::Resumed,
            from_state: Some(instance.current_state.clone()),
            to_state: None,
            transition_id: None,
            agent: executing_agent,
            message: "Workflow resumed".to_string(),
            metadata: HashMap::new(),
        };

        instance.status = WorkflowStatus::Running;
        instance.updated_at = Utc::now();
        instance.execution_history.push(resume_event.clone());

        self.storage.store(&instance.to_generic())?;

        Ok(WorkflowExecutionResult {
            success: true,
            instance_id: instance_id.to_string(),
            current_state: instance.current_state.clone(),
            message: "Workflow resumed successfully".to_string(),
            events: vec![resume_event],
            variables_changed: HashMap::new(),
        })
    }

    fn ensure_instance_loaded(&mut self, instance_id: &str) -> Result<(), EngramError> {
        if self.active_instances.contains_key(instance_id) {
            return Ok(());
        }
        if let Some(generic) = self.storage.get(instance_id, "workflow_instance")? {
            let instance = WorkflowInstance::from_generic(generic)
                .map_err(|e| EngramError::Validation(e.to_string()))?;
            self.active_instances
                .insert(instance_id.to_string(), instance);
            Ok(())
        } else {
            Err(EngramError::NotFound(format!(
                "Workflow instance {} not found",
                instance_id
            )))
        }
    }

    pub fn cancel_workflow(
        &mut self,
        instance_id: &str,
        executing_agent: String,
        reason: String,
    ) -> Result<WorkflowExecutionResult, EngramError> {
        self.ensure_instance_loaded(instance_id)?;
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

    pub fn update_instance_variables(
        &mut self,
        instance_id: &str,
        variables: HashMap<String, RuleValue>,
    ) -> Result<(), EngramError> {
        self.ensure_instance_loaded(instance_id)?;
        let instance = self.active_instances.get_mut(instance_id).unwrap();

        for (key, value) in variables {
            instance.context.variables.insert(key, value);
        }

        instance.updated_at = Utc::now();
        self.storage.store(&instance.to_generic())?;

        Ok(())
    }

    pub fn get_execution_history(
        &self,
        instance_id: &str,
    ) -> Result<Vec<WorkflowExecutionEvent>, EngramError> {
        let instance = self.get_instance_status(instance_id)?;
        Ok(instance.execution_history)
    }

    fn evaluate_transition_condition(
        &self,
        condition: &crate::entities::TransitionCondition,
        instance: &WorkflowInstance,
    ) -> bool {
        match condition.condition_type.as_str() {
            "field" => {
                if let Some(field_name) = condition.logic.get("field").and_then(|v| v.as_str()) {
                    let expected = condition.logic.get("equals");
                    let actual = instance.context.variables.get(field_name);
                    match (actual, expected) {
                        (Some(RuleValue::Boolean(b)), Some(serde_json::Value::Bool(eb))) => b == eb,
                        (Some(RuleValue::String(s)), Some(serde_json::Value::String(es))) => {
                            s == es
                        }
                        (Some(RuleValue::Number(n)), Some(serde_json::Value::Number(en))) => en
                            .as_f64()
                            .map_or(false, |en| (n - en).abs() < f64::EPSILON),
                        _ => true,
                    }
                } else {
                    true
                }
            }
            "rule" => {
                if let Some(expr) = condition.logic.get("expression").and_then(|v| v.as_str()) {
                    let rule_ctx = RuleExecutionContext {
                        variables: instance.context.variables.clone(),
                        current_entity: None,
                        executing_agent: instance.context.executing_agent.clone(),
                        execution_time: Utc::now(),
                        metadata: HashMap::new(),
                    };
                    self.rule_engine
                        .evaluate_expression(expr, &rule_ctx)
                        .unwrap_or(true)
                } else {
                    true
                }
            }
            "command_guard" => self.evaluate_command_guard(&condition.logic),
            _ => true,
        }
    }

    fn evaluate_command_guard(&self, logic: &serde_json::Value) -> bool {
        let command = match logic.get("command").and_then(|v| v.as_str()) {
            Some(cmd) => cmd,
            None => {
                tracing::warn!("command_guard: missing 'command' field, allowing transition");
                return true;
            }
        };

        let expected_exit_code = logic
            .get("expected_exit_code")
            .and_then(|v| v.as_i64())
            .unwrap_or(0) as i32;

        let timeout_secs = logic
            .get("timeout_seconds")
            .and_then(|v| v.as_u64())
            .unwrap_or(300);

        let args: Vec<String> = logic
            .get("args")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();

        let working_directory = logic
            .get("working_directory")
            .and_then(|v| v.as_str())
            .map(String::from);

        let mut environment: HashMap<String, String> = logic
            .get("environment")
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                    .collect()
            })
            .unwrap_or_default();

        if let Some(env_vars) = logic.get("inject_env_vars").and_then(|v| v.as_array()) {
            for var_name in env_vars.iter().filter_map(|v| v.as_str()) {
                if let Ok(val) = std::env::var(var_name) {
                    environment.insert(var_name.to_string(), val);
                }
            }
        }

        let mut cmd = std::process::Command::new(command);
        cmd.args(&args);
        cmd.stdout(std::process::Stdio::piped());
        cmd.stderr(std::process::Stdio::piped());

        if let Some(ref wd) = working_directory {
            cmd.current_dir(wd);
        }

        for (key, value) in &environment {
            cmd.env(key, value);
        }

        let child = match cmd.spawn() {
            Ok(c) => c,
            Err(e) => {
                tracing::warn!(
                    command = command,
                    error = %e,
                    "command_guard: failed to spawn, blocking transition"
                );
                return false;
            }
        };

        let output = match self.wait_for_guard_output(child, StdDuration::from_secs(timeout_secs)) {
            Ok(output) => output,
            Err(e) => {
                tracing::warn!(
                    command = command,
                    timeout_secs = timeout_secs,
                    error = %e,
                    "command_guard: execution error, blocking transition"
                );
                return false;
            }
        };

        let exit_code = output.status.code().unwrap_or(-1);
        let passed = exit_code == expected_exit_code;

        if !passed {
            let stderr = String::from_utf8_lossy(&output.stderr);
            tracing::info!(
                command = command,
                exit_code = exit_code,
                expected = expected_exit_code,
                stderr = %stderr,
                "command_guard: exit code mismatch, blocking transition"
            );
        }

        passed
    }

    fn wait_for_guard_output(
        &self,
        child: std::process::Child,
        timeout: StdDuration,
    ) -> Result<std::process::Output, String> {
        use std::sync::mpsc;
        use std::thread;

        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let result = child.wait_with_output();
            let _ = tx.send(result);
        });

        match rx.recv_timeout(timeout) {
            Ok(Ok(output)) => Ok(output),
            Ok(Err(e)) => Err(format!("Failed to get command output: {}", e)),
            Err(_) => Err(format!("Command timed out after {:?}", timeout)),
        }
    }

    fn execute_state_post_functions(
        &self,
        state: &crate::entities::WorkflowState,
        instance_id: &str,
        agent: &str,
    ) -> Vec<WorkflowExecutionEvent> {
        let mut events = Vec::new();
        for func in &state.post_functions {
            let result = self
                .action_executor
                .execute_action(&func.function_type, &func.parameters);

            let (success, message) = match &result {
                Ok(ar) => (ar.success, ar.message.clone()),
                Err(e) => (false, e.to_string()),
            };

            let event = WorkflowExecutionEvent {
                id: Uuid::new_v4().to_string(),
                timestamp: Utc::now(),
                event_type: WorkflowEventType::ActionExecuted,
                from_state: Some(state.name.clone()),
                to_state: Some(state.name.clone()),
                transition_id: None,
                agent: agent.to_string(),
                message: format!(
                    "Post-function '{}' ({}): {}",
                    func.name,
                    func.function_type,
                    if success { "ok" } else { "failed" }
                ),
                metadata: {
                    let mut m = HashMap::new();
                    m.insert("function_id".to_string(), func.id.clone());
                    m.insert("function_name".to_string(), func.name.clone());
                    m.insert("success".to_string(), success.to_string());
                    m
                },
            };
            events.push(event);

            if !success {
                tracing::warn!(
                    instance_id = instance_id,
                    state = %state.name,
                    function = %func.name,
                    "Post-function failed: {}",
                    message
                );
            }
        }
        events
    }

    pub fn get_workflow(&self, workflow_id: &str) -> Result<Workflow, EngramError> {
        self.load_workflow_definition(workflow_id)
    }

    pub fn list_workflows(&self) -> Result<Vec<Workflow>, EngramError> {
        let entities = self.storage.get_all("workflow")?;
        let mut workflows = Vec::new();
        for entity in entities {
            match Workflow::from_generic(entity) {
                Ok(wf) => workflows.push(wf),
                Err(e) => {
                    tracing::warn!("Skipping malformed workflow entity: {}", e);
                }
            }
        }
        Ok(workflows)
    }

    pub fn check_auto_transitions(&mut self) -> Result<Vec<WorkflowExecutionResult>, EngramError> {
        let instances = self.list_active_instances();
        let mut results = Vec::new();

        for instance in &instances {
            if instance.status != WorkflowStatus::Running {
                continue;
            }

            let definition = match self.load_workflow_definition(&instance.workflow_id) {
                Ok(d) => d,
                Err(_) => continue,
            };

            let auto_transitions: Vec<_> = definition
                .transitions
                .iter()
                .filter(|t| {
                    t.transition_type == crate::entities::TransitionType::Automatic
                        && t.trigger.is_some()
                        && definition
                            .states
                            .iter()
                            .any(|s| s.id == t.from_state && s.name == instance.current_state)
                })
                .collect();

            for transition in &auto_transitions {
                let trigger = transition.trigger.as_ref().unwrap();

                if !self.evaluate_trigger(trigger, instance)? {
                    continue;
                }

                let target_state = match definition
                    .states
                    .iter()
                    .find(|s| s.id == transition.to_state)
                {
                    Some(s) => s,
                    None => continue,
                };

                if !self.check_guards(target_state, instance)? {
                    continue;
                }

                match self.execute_transition(
                    &instance.id,
                    transition.name.clone(),
                    "auto-trigger".to_string(),
                ) {
                    Ok(result) => {
                        let trigger_event = WorkflowExecutionEvent {
                            id: Uuid::new_v4().to_string(),
                            timestamp: Utc::now(),
                            event_type: WorkflowEventType::AutoTriggered,
                            from_state: None,
                            to_state: None,
                            transition_id: Some(transition.id.clone()),
                            agent: "auto-trigger".to_string(),
                            message: format!(
                                "Auto-trigger fired: {:?} on instance {}",
                                trigger, instance.id
                            ),
                            metadata: HashMap::new(),
                        };
                        let mut all_events = result.events;
                        all_events.push(trigger_event);
                        results.push(WorkflowExecutionResult {
                            events: all_events,
                            ..result
                        });
                    }
                    Err(_) => continue,
                }
            }
        }

        Ok(results)
    }

    fn evaluate_trigger(
        &self,
        trigger: &TriggerCondition,
        instance: &WorkflowInstance,
    ) -> Result<bool, EngramError> {
        match trigger {
            TriggerCondition::AllTasksDone => {
                let entity_id = instance.context.entity_id.as_deref().unwrap_or("");
                let tasks = self.storage.get_all("task").unwrap_or_default();
                let related: Vec<_> = tasks
                    .iter()
                    .filter_map(|e| {
                        let data = &e.data;
                        let parent = data.get("parent_id")?.as_str()?;
                        if parent == entity_id {
                            let status = data.get("status")?.as_str()?;
                            Some(status.to_string())
                        } else {
                            None
                        }
                    })
                    .collect();

                Ok(!related.is_empty() && related.iter().all(|s| s == "done"))
            }
            TriggerCondition::QualityGatePassed { name } => {
                let entity_id = instance.context.entity_id.as_deref().unwrap_or("");
                let contexts = self.storage.get_all("context").unwrap_or_default();
                let gate_passed = contexts.iter().any(|e| {
                    let data = &e.data;
                    let title = data.get("title").and_then(|v| v.as_str()).unwrap_or("");
                    let content = data.get("content").and_then(|v| v.as_str()).unwrap_or("");
                    title.contains(name)
                        && content.contains("passed")
                        && data.get("entity_id").and_then(|v| v.as_str()).unwrap_or("") == entity_id
                });
                Ok(gate_passed)
            }
            TriggerCondition::Timer { duration_secs } => {
                let elapsed = Utc::now() - instance.started_at;
                Ok(elapsed >= Duration::seconds(*duration_secs as i64))
            }
            TriggerCondition::EntityCreated { entity_type } => {
                let entities = self.storage.get_all(entity_type).unwrap_or_default();
                let since_start = instance.started_at;
                let instance_entity_id = &instance.id;
                Ok(entities
                    .iter()
                    .any(|e| e.timestamp > since_start && e.id != *instance_entity_id))
            }
        }
    }

    fn check_guards(
        &self,
        state: &crate::entities::WorkflowState,
        instance: &WorkflowInstance,
    ) -> Result<bool, EngramError> {
        for guard in &state.guards {
            if guard.guard_type == "permission" {
                let required = guard
                    .condition
                    .get("permission")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                if !required.is_empty()
                    && !instance.context.permissions.iter().any(|p| p == required)
                {
                    tracing::debug!(
                        instance_id = %instance.id,
                        state = %state.name,
                        guard = %guard.id,
                        required = required,
                        "Guard blocked: permission not held"
                    );
                    return Ok(false);
                }
            }
        }
        Ok(true)
    }

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
            commit_policy: None,
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
            commit_policy: None,
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
            commit_policy: None,
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
                trigger: None,
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
                trigger: None,
            },
        ];
        workflow.initial_state = state_start.id.clone();
        workflow.final_states = vec![state_done.id.clone()];
        workflow.activate();

        engine.storage.store(&workflow.to_generic()).unwrap();
        workflow_id
    }

    #[test]
    fn test_transition_action_failure_block_policy() {
        let executor = ActionExecutor::new(false);
        let mut engine = WorkflowEngineBuilder::new()
            .with_storage(MemoryStorage::new("test-agent"))
            .with_action_executor(executor)
            .build()
            .unwrap();

        let actions = vec![crate::entities::TransitionAction {
            id: "act-block".to_string(),
            name: "blocker".to_string(),
            action_type: "external_command".to_string(),
            parameters: {
                let mut m = HashMap::new();
                m.insert("command".to_string(), serde_json::json!("echo"));
                m
            },
            on_failure: Some(crate::entities::ActionFailurePolicy::Block),
        }];
        let workflow_id = create_workflow_with_actions(&mut engine, actions);

        let start_result = engine
            .start_workflow(
                workflow_id,
                None,
                None,
                "test-agent".to_string(),
                HashMap::new(),
            )
            .unwrap();

        let result = engine
            .execute_transition(
                &start_result.instance_id,
                "go".to_string(),
                "test-agent".to_string(),
            )
            .unwrap();

        assert!(!result.success);
        assert_eq!(result.current_state, "initial");
    }

    #[test]
    fn test_create_workflow() {
        let mut engine = create_test_engine();
        let workflow = engine
            .create_workflow(
                "Test Workflow".to_string(),
                Some("A test".to_string()),
                "test-agent".to_string(),
            )
            .unwrap();
        assert_eq!(workflow.title, "Test Workflow");
    }

    #[test]
    fn test_start_workflow() {
        let mut engine = create_test_engine();
        let workflow_id = create_test_workflow_in_storage(&mut engine);
        let result = engine
            .start_workflow(
                workflow_id,
                None,
                None,
                "test-agent".to_string(),
                HashMap::new(),
            )
            .unwrap();
        assert!(result.success);
        assert_eq!(result.current_state, "initial");
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
                "Testing".to_string(),
            )
            .unwrap();
        assert!(cancel_result.success);
        assert_eq!(
            engine
                .get_instance_status(&start_result.instance_id)
                .unwrap()
                .status,
            WorkflowStatus::Cancelled
        );
    }

    #[test]
    fn test_workflow_builder() {
        let engine = WorkflowEngineBuilder::new()
            .with_storage(MemoryStorage::new("test-agent"))
            .with_rule_engine(RuleExecutionEngine::new())
            .with_max_execution_steps(500)
            .build()
            .unwrap();
        assert_eq!(engine.max_execution_steps, 500);
    }

    fn create_loop_workflow_in_storage(
        engine: &mut WorkflowAutomationEngine<MemoryStorage>,
    ) -> String {
        let state_loop = crate::entities::WorkflowState {
            id: "state-loop".to_string(),
            name: "looping".to_string(),
            state_type: crate::entities::StateType::InProgress,
            description: "Loops".to_string(),
            is_final: false,
            prompts: None,
            guards: vec![],
            post_functions: vec![],
            commit_policy: None,
        };
        let workflow_id = "loop-workflow-def".to_string();
        let mut workflow = crate::entities::Workflow::new(
            "Loop".to_string(),
            "Loop".to_string(),
            "test-agent".to_string(),
        );
        workflow.id = workflow_id.clone();
        workflow.states = vec![state_loop.clone()];
        workflow.transitions = vec![crate::entities::WorkflowTransition {
            id: "t-loop".to_string(),
            name: "loop".to_string(),
            from_state: state_loop.id.clone(),
            to_state: state_loop.id.clone(),
            transition_type: crate::entities::TransitionType::Manual,
            description: "Self-loop".to_string(),
            conditions: vec![],
            actions: vec![],
            trigger: None,
        }];
        workflow.initial_state = state_loop.id.clone();
        workflow.final_states = vec![];
        workflow.activate();
        engine.storage.store(&workflow.to_generic()).unwrap();
        workflow_id
    }

    #[test]
    fn test_max_execution_steps_guard_fires() {
        let mut engine = WorkflowEngineBuilder::new()
            .with_storage(MemoryStorage::new("test-agent"))
            .with_max_execution_steps(3)
            .build()
            .unwrap();
        let workflow_id = create_loop_workflow_in_storage(&mut engine);
        let start_result = engine
            .start_workflow(
                workflow_id,
                None,
                None,
                "test-agent".to_string(),
                HashMap::new(),
            )
            .unwrap();
        let iid = start_result.instance_id;
        for _i in 0..3 {
            engine
                .execute_transition(&iid, "loop".to_string(), "test-agent".to_string())
                .unwrap();
        }
        let result = engine.execute_transition(&iid, "loop".to_string(), "test-agent".to_string());
        assert!(result.is_err());
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
            "invalid".to_string(),
            "test-agent".to_string(),
        );
        assert!(result.is_err());
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
                "a1".to_string(),
                HashMap::new(),
            )
            .unwrap();
        engine
            .start_workflow(workflow_id, None, None, "a2".to_string(), HashMap::new())
            .unwrap();
        assert_eq!(engine.list_active_instances().len(), 2);
    }

    fn create_workflow_with_actions(
        engine: &mut WorkflowAutomationEngine<MemoryStorage>,
        actions: Vec<crate::entities::TransitionAction>,
    ) -> String {
        let state_start = crate::entities::WorkflowState {
            id: "state-start".to_string(),
            name: "initial".to_string(),
            state_type: crate::entities::StateType::Start,
            description: "Start".to_string(),
            is_final: false,
            prompts: None,
            guards: vec![],
            post_functions: vec![],
            commit_policy: None,
        };
        let state_done = crate::entities::WorkflowState {
            id: "state-done".to_string(),
            name: "completed".to_string(),
            state_type: crate::entities::StateType::Done,
            description: "Done".to_string(),
            is_final: true,
            prompts: None,
            guards: vec![],
            post_functions: vec![],
            commit_policy: None,
        };
        let workflow_id = "actions-workflow".to_string();
        let mut workflow = crate::entities::Workflow::new(
            "Actions".to_string(),
            "Wf".to_string(),
            "test-agent".to_string(),
        );
        workflow.id = workflow_id.clone();
        workflow.states = vec![state_start.clone(), state_done.clone()];
        workflow.transitions = vec![crate::entities::WorkflowTransition {
            id: "t-go".to_string(),
            name: "go".to_string(),
            from_state: state_start.id.clone(),
            to_state: state_done.id.clone(),
            transition_type: crate::entities::TransitionType::Manual,
            description: "Go".to_string(),
            conditions: vec![],
            actions,
            trigger: None,
        }];
        workflow.initial_state = state_start.id.clone();
        workflow.final_states = vec![state_done.id.clone()];
        workflow.activate();
        engine.storage.store(&workflow.to_generic()).unwrap();
        workflow_id
    }

    #[test]
    fn test_transition_executes_notification_action() {
        let mut engine = create_test_engine();
        let actions = vec![crate::entities::TransitionAction {
            id: "act-1".to_string(),
            name: "notify".to_string(),
            action_type: "notification".to_string(),
            parameters: {
                let mut m = HashMap::new();
                m.insert("message".to_string(), serde_json::json!("Hi"));
                m
            },
            on_failure: None,
        }];
        let workflow_id = create_workflow_with_actions(&mut engine, actions);
        let start_result = engine
            .start_workflow(
                workflow_id,
                None,
                None,
                "test-agent".to_string(),
                HashMap::new(),
            )
            .unwrap();
        let result = engine
            .execute_transition(
                &start_result.instance_id,
                "go".to_string(),
                "test-agent".to_string(),
            )
            .unwrap();
        assert!(result.success);
        assert_eq!(result.current_state, "completed");
    }

    #[test]
    fn test_transition_action_failure_continue_policy() {
        let mut engine = create_test_engine();
        let actions = vec![crate::entities::TransitionAction {
            id: "act-fail".to_string(),
            name: "bad-cmd".to_string(),
            action_type: "external_command".to_string(),
            parameters: {
                let mut m = HashMap::new();
                m.insert("command".to_string(), serde_json::json!("nonexistent_xyz"));
                m
            },
            on_failure: None,
        }];
        let workflow_id = create_workflow_with_actions(&mut engine, actions);
        let start_result = engine
            .start_workflow(
                workflow_id,
                None,
                None,
                "test-agent".to_string(),
                HashMap::new(),
            )
            .unwrap();
        let result = engine
            .execute_transition(
                &start_result.instance_id,
                "go".to_string(),
                "test-agent".to_string(),
            )
            .unwrap();
        assert!(result.success);
    }

    #[test]
    fn test_transition_with_multiple_actions_records_all() {
        let mut engine = create_test_engine();
        let actions = vec![
            crate::entities::TransitionAction {
                id: "a1".to_string(),
                name: "n1".to_string(),
                action_type: "notification".to_string(),
                parameters: {
                    let mut m = HashMap::new();
                    m.insert("message".to_string(), serde_json::json!("1"));
                    m
                },
                on_failure: None,
            },
            crate::entities::TransitionAction {
                id: "a2".to_string(),
                name: "n2".to_string(),
                action_type: "notification".to_string(),
                parameters: {
                    let mut m = HashMap::new();
                    m.insert("message".to_string(), serde_json::json!("2"));
                    m
                },
                on_failure: None,
            },
        ];
        let workflow_id = create_workflow_with_actions(&mut engine, actions);
        let start_result = engine
            .start_workflow(
                workflow_id,
                None,
                None,
                "test-agent".to_string(),
                HashMap::new(),
            )
            .unwrap();
        let result = engine
            .execute_transition(
                &start_result.instance_id,
                "go".to_string(),
                "test-agent".to_string(),
            )
            .unwrap();
        assert!(result.success);
        assert_eq!(
            result
                .events
                .iter()
                .filter(|e| matches!(e.event_type, WorkflowEventType::ActionExecuted))
                .count(),
            2
        );
    }

    #[test]
    fn test_transition_without_actions_still_works() {
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
        let result = engine
            .execute_transition(
                &start_result.instance_id,
                "start".to_string(),
                "test-agent".to_string(),
            )
            .unwrap();
        assert!(result.success);
        assert_eq!(
            result
                .events
                .iter()
                .filter(|e| matches!(e.event_type, WorkflowEventType::ActionExecuted))
                .count(),
            0
        );
    }

    // === Auto-transition tests ===

    fn create_auto_timer_workflow(
        engine: &mut WorkflowAutomationEngine<MemoryStorage>,
        duration_secs: u64,
    ) -> String {
        let s = crate::entities::WorkflowState {
            id: "auto-s".into(),
            name: "waiting".into(),
            state_type: crate::entities::StateType::Start,
            description: "W".into(),
            is_final: false,
            prompts: None,
            guards: vec![],
            post_functions: vec![],
            commit_policy: None,
        };
        let d = crate::entities::WorkflowState {
            id: "auto-d".into(),
            name: "timed_out".into(),
            state_type: crate::entities::StateType::Done,
            description: "D".into(),
            is_final: true,
            prompts: None,
            guards: vec![],
            post_functions: vec![],
            commit_policy: None,
        };
        let wid: String = "auto-timer-wf".into();
        let mut wf = crate::entities::Workflow::new("ATW".into(), "Auto timer".into(), "ta".into());
        wf.id = wid.clone();
        wf.states = vec![s.clone(), d.clone()];
        wf.transitions = vec![crate::entities::WorkflowTransition {
            id: "t-at".into(),
            name: "timeout".into(),
            from_state: s.id.clone(),
            to_state: d.id.clone(),
            transition_type: crate::entities::TransitionType::Automatic,
            description: "AT".into(),
            conditions: vec![],
            actions: vec![],
            trigger: Some(TriggerCondition::Timer { duration_secs }),
        }];
        wf.initial_state = s.id.clone();
        wf.final_states = vec![d.id.clone()];
        wf.activate();
        engine.storage.store(&wf.to_generic()).unwrap();
        wid
    }

    #[test]
    fn test_auto_timer_trigger_does_not_fire_early() {
        let mut engine = create_test_engine();
        let wid = create_auto_timer_workflow(&mut engine, 3600);
        let sr = engine
            .start_workflow(wid, None, None, "ta".into(), HashMap::new())
            .unwrap();
        assert_eq!(engine.check_auto_transitions().unwrap().len(), 0);
        assert_eq!(
            engine
                .get_instance_status(&sr.instance_id)
                .unwrap()
                .current_state,
            "waiting"
        );
    }

    #[test]
    fn test_auto_timer_trigger_fires_after_duration() {
        let mut engine = create_test_engine();
        let wid = create_auto_timer_workflow(&mut engine, 0);
        let sr = engine
            .start_workflow(wid, None, None, "ta".into(), HashMap::new())
            .unwrap();
        let results = engine.check_auto_transitions().unwrap();
        assert_eq!(results.len(), 1);
        assert!(results[0].success);
        assert_eq!(
            engine
                .get_instance_status(&sr.instance_id)
                .unwrap()
                .current_state,
            "timed_out"
        );
    }

    #[test]
    fn test_auto_entity_created_trigger_fires() {
        let mut engine = create_test_engine();
        let s = crate::entities::WorkflowState {
            id: "aec-s".into(),
            name: "awaiting".into(),
            state_type: crate::entities::StateType::Start,
            description: "A".into(),
            is_final: false,
            prompts: None,
            guards: vec![],
            post_functions: vec![],
            commit_policy: None,
        };
        let d = crate::entities::WorkflowState {
            id: "aec-d".into(),
            name: "received".into(),
            state_type: crate::entities::StateType::Done,
            description: "D".into(),
            is_final: true,
            prompts: None,
            guards: vec![],
            post_functions: vec![],
            commit_policy: None,
        };
        let wid: String = "auto-ec-wf".into();
        let mut wf = crate::entities::Workflow::new("AECW".into(), "Auto ec".into(), "ta".into());
        wf.id = wid.clone();
        wf.states = vec![s.clone(), d.clone()];
        wf.transitions = vec![crate::entities::WorkflowTransition {
            id: "t-ec".into(),
            name: "ec".into(),
            from_state: s.id.clone(),
            to_state: d.id.clone(),
            transition_type: crate::entities::TransitionType::Automatic,
            description: "EC".into(),
            conditions: vec![],
            actions: vec![],
            trigger: Some(TriggerCondition::EntityCreated {
                entity_type: "context".into(),
            }),
        }];
        wf.initial_state = s.id.clone();
        wf.final_states = vec![d.id.clone()];
        wf.activate();
        engine.storage.store(&wf.to_generic()).unwrap();

        let sr = engine
            .start_workflow(wid, None, None, "ta".into(), HashMap::new())
            .unwrap();
        engine
            .storage
            .store(&crate::entities::GenericEntity {
                id: "ne1".into(),
                entity_type: "context".into(),
                agent: "other".into(),
                timestamp: Utc::now(),
                data: serde_json::json!({"title": "t"}),
            })
            .unwrap();
        let results = engine.check_auto_transitions().unwrap();
        assert_eq!(results.len(), 1);
        assert!(results[0].success);
        assert_eq!(
            engine
                .get_instance_status(&sr.instance_id)
                .unwrap()
                .current_state,
            "received"
        );
    }

    #[test]
    fn test_auto_all_tasks_done_trigger_fires() {
        let mut engine = create_test_engine();
        let s = crate::entities::WorkflowState {
            id: "atd-s".into(),
            name: "in_progress".into(),
            state_type: crate::entities::StateType::InProgress,
            description: "W".into(),
            is_final: false,
            prompts: None,
            guards: vec![],
            post_functions: vec![],
            commit_policy: None,
        };
        let d = crate::entities::WorkflowState {
            id: "atd-d".into(),
            name: "all_done".into(),
            state_type: crate::entities::StateType::Done,
            description: "D".into(),
            is_final: true,
            prompts: None,
            guards: vec![],
            post_functions: vec![],
            commit_policy: None,
        };
        let wid: String = "auto-td-wf".into();
        let mut wf = crate::entities::Workflow::new("ATDW".into(), "Auto td".into(), "ta".into());
        wf.id = wid.clone();
        wf.states = vec![s.clone(), d.clone()];
        wf.transitions = vec![crate::entities::WorkflowTransition {
            id: "t-td".into(),
            name: "td".into(),
            from_state: s.id.clone(),
            to_state: d.id.clone(),
            transition_type: crate::entities::TransitionType::Automatic,
            description: "TD".into(),
            conditions: vec![],
            actions: vec![],
            trigger: Some(TriggerCondition::AllTasksDone),
        }];
        wf.initial_state = s.id.clone();
        wf.final_states = vec![d.id.clone()];
        wf.activate();
        engine.storage.store(&wf.to_generic()).unwrap();

        let eid: String = "pe-1".into();
        let sr = engine
            .start_workflow(wid, Some(eid.clone()), None, "ta".into(), HashMap::new())
            .unwrap();
        engine
            .storage
            .store(&crate::entities::GenericEntity {
                id: "t1".into(),
                entity_type: "task".into(),
                agent: "ta".into(),
                timestamp: Utc::now(),
                data: serde_json::json!({"parent_id": eid, "status": "done"}),
            })
            .unwrap();
        engine
            .storage
            .store(&crate::entities::GenericEntity {
                id: "t2".into(),
                entity_type: "task".into(),
                agent: "ta".into(),
                timestamp: Utc::now(),
                data: serde_json::json!({"parent_id": eid, "status": "done"}),
            })
            .unwrap();
        let results = engine.check_auto_transitions().unwrap();
        assert_eq!(results.len(), 1);
        assert!(results[0].success);
        assert_eq!(
            engine
                .get_instance_status(&sr.instance_id)
                .unwrap()
                .current_state,
            "all_done"
        );
    }

    #[test]
    fn test_auto_all_tasks_done_not_fired_when_incomplete() {
        let mut engine = create_test_engine();
        let s = crate::entities::WorkflowState {
            id: "ati-s".into(),
            name: "in_progress".into(),
            state_type: crate::entities::StateType::InProgress,
            description: "W".into(),
            is_final: false,
            prompts: None,
            guards: vec![],
            post_functions: vec![],
            commit_policy: None,
        };
        let d = crate::entities::WorkflowState {
            id: "ati-d".into(),
            name: "all_done".into(),
            state_type: crate::entities::StateType::Done,
            description: "D".into(),
            is_final: true,
            prompts: None,
            guards: vec![],
            post_functions: vec![],
            commit_policy: None,
        };
        let wid: String = "auto-ti-wf".into();
        let mut wf = crate::entities::Workflow::new("ATIW".into(), "Auto ti".into(), "ta".into());
        wf.id = wid.clone();
        wf.states = vec![s.clone(), d.clone()];
        wf.transitions = vec![crate::entities::WorkflowTransition {
            id: "t-ti".into(),
            name: "ti".into(),
            from_state: s.id.clone(),
            to_state: d.id.clone(),
            transition_type: crate::entities::TransitionType::Automatic,
            description: "TI".into(),
            conditions: vec![],
            actions: vec![],
            trigger: Some(TriggerCondition::AllTasksDone),
        }];
        wf.initial_state = s.id.clone();
        wf.final_states = vec![d.id.clone()];
        wf.activate();
        engine.storage.store(&wf.to_generic()).unwrap();

        let eid: String = "pe-2".into();
        engine
            .start_workflow(wid, Some(eid.clone()), None, "ta".into(), HashMap::new())
            .unwrap();
        engine
            .storage
            .store(&crate::entities::GenericEntity {
                id: "t3".into(),
                entity_type: "task".into(),
                agent: "ta".into(),
                timestamp: Utc::now(),
                data: serde_json::json!({"parent_id": eid, "status": "in_progress"}),
            })
            .unwrap();
        engine
            .storage
            .store(&crate::entities::GenericEntity {
                id: "t4".into(),
                entity_type: "task".into(),
                agent: "ta".into(),
                timestamp: Utc::now(),
                data: serde_json::json!({"parent_id": eid, "status": "done"}),
            })
            .unwrap();
        assert_eq!(engine.check_auto_transitions().unwrap().len(), 0);
    }

    #[test]
    fn test_auto_transition_blocked_by_guard() {
        let mut engine = create_test_engine();
        let s = crate::entities::WorkflowState {
            id: "gs".into(),
            name: "waiting".into(),
            state_type: crate::entities::StateType::Start,
            description: "W".into(),
            is_final: false,
            prompts: None,
            guards: vec![],
            post_functions: vec![],
            commit_policy: None,
        };
        let d = crate::entities::WorkflowState {
            id: "gd".into(),
            name: "done".into(),
            state_type: crate::entities::StateType::Done,
            description: "D".into(),
            is_final: true,
            prompts: None,
            guards: vec![crate::entities::StateGuard {
                id: "g1".into(),
                guard_type: "permission".into(),
                condition: serde_json::json!({"permission": "admin_only"}),
                error_message: "No".into(),
            }],
            post_functions: vec![],
            commit_policy: None,
        };
        let wid: String = "guard-wf".into();
        let mut wf = crate::entities::Workflow::new("GW".into(), "Guarded".into(), "ta".into());
        wf.id = wid.clone();
        wf.states = vec![s.clone(), d.clone()];
        wf.transitions = vec![crate::entities::WorkflowTransition {
            id: "tg".into(),
            name: "auto-go".into(),
            from_state: s.id.clone(),
            to_state: d.id.clone(),
            transition_type: crate::entities::TransitionType::Automatic,
            description: "AG".into(),
            conditions: vec![],
            actions: vec![],
            trigger: Some(TriggerCondition::Timer { duration_secs: 0 }),
        }];
        wf.initial_state = s.id.clone();
        wf.final_states = vec![d.id.clone()];
        wf.activate();
        engine.storage.store(&wf.to_generic()).unwrap();

        let sr = engine
            .start_workflow(wid, None, None, "ta".into(), HashMap::new())
            .unwrap();
        assert_eq!(engine.check_auto_transitions().unwrap().len(), 0);
        assert_eq!(
            engine
                .get_instance_status(&sr.instance_id)
                .unwrap()
                .current_state,
            "waiting"
        );
    }

    #[test]
    fn test_auto_transition_records_trigger_event() {
        let mut engine = create_test_engine();
        let wid = create_auto_timer_workflow(&mut engine, 0);
        let _sr = engine
            .start_workflow(wid, None, None, "ta".into(), HashMap::new())
            .unwrap();
        let results = engine.check_auto_transitions().unwrap();
        assert_eq!(results.len(), 1);
        let te: Vec<_> = results[0]
            .events
            .iter()
            .filter(|e| matches!(e.event_type, WorkflowEventType::AutoTriggered))
            .collect();
        assert_eq!(te.len(), 1);
        assert_eq!(te[0].agent, "auto-trigger");
    }

    #[test]
    fn test_auto_transition_skips_non_running_instances() {
        let mut engine = create_test_engine();
        let wid = create_auto_timer_workflow(&mut engine, 0);
        let sr = engine
            .start_workflow(wid, None, None, "ta".into(), HashMap::new())
            .unwrap();
        engine
            .cancel_workflow(&sr.instance_id, "ta".into(), "nope".into())
            .unwrap();
        assert_eq!(engine.check_auto_transitions().unwrap().len(), 0);
    }

    #[test]
    fn test_suspend_and_resume_workflow() {
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

        let suspend_result = engine
            .suspend_workflow(
                &start_result.instance_id,
                "test-agent".to_string(),
                "needs review".to_string(),
            )
            .unwrap();
        assert!(suspend_result.success);
        let instance = engine
            .get_instance_status(&start_result.instance_id)
            .unwrap();
        assert!(matches!(instance.status, WorkflowStatus::Suspended(_)));
        assert_eq!(
            instance
                .execution_history
                .iter()
                .filter(|e| matches!(e.event_type, WorkflowEventType::Suspended))
                .count(),
            1
        );

        let resume_result = engine
            .resume_workflow(&start_result.instance_id, "test-agent".to_string())
            .unwrap();
        assert!(resume_result.success);
        let instance = engine
            .get_instance_status(&start_result.instance_id)
            .unwrap();
        assert_eq!(instance.status, WorkflowStatus::Running);
        assert_eq!(
            instance
                .execution_history
                .iter()
                .filter(|e| matches!(e.event_type, WorkflowEventType::Resumed))
                .count(),
            1
        );
    }

    #[test]
    fn test_suspend_already_completed_fails() {
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
        engine
            .execute_transition(
                &start_result.instance_id,
                "complete".to_string(),
                "test-agent".to_string(),
            )
            .unwrap();
        let result = engine.suspend_workflow(
            &start_result.instance_id,
            "test-agent".to_string(),
            "no".to_string(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_resume_non_suspended_fails() {
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
        let result = engine.resume_workflow(&start_result.instance_id, "test-agent".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_get_workflow() {
        let mut engine = create_test_engine();
        let workflow_id = create_test_workflow_in_storage(&mut engine);
        let workflow = engine.get_workflow(&workflow_id).unwrap();
        assert_eq!(workflow.id, workflow_id);
        assert_eq!(workflow.states.len(), 3);
    }

    #[test]
    fn test_get_workflow_not_found() {
        let engine = create_test_engine();
        let result = engine.get_workflow("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_list_workflows() {
        let mut engine = create_test_engine();
        create_test_workflow_in_storage(&mut engine);
        let state_start = crate::entities::WorkflowState {
            id: "ls-s".to_string(),
            name: "initial".to_string(),
            state_type: crate::entities::StateType::Start,
            description: "Start state".to_string(),
            is_final: false,
            prompts: None,
            guards: vec![],
            post_functions: vec![],
            commit_policy: None,
        };
        let state_done = crate::entities::WorkflowState {
            id: "ls-d".to_string(),
            name: "completed".to_string(),
            state_type: crate::entities::StateType::Done,
            description: "Finished".to_string(),
            is_final: true,
            prompts: None,
            guards: vec![],
            post_functions: vec![],
            commit_policy: None,
        };
        let workflow_id2 = "test-workflow-def-2".to_string();
        let mut workflow2 = crate::entities::Workflow::new(
            "Test Workflow 2".to_string(),
            "A second test workflow".to_string(),
            "test-agent".to_string(),
        );
        workflow2.id = workflow_id2.clone();
        workflow2.states = vec![state_start.clone(), state_done.clone()];
        workflow2.transitions = vec![];
        workflow2.initial_state = state_start.id.clone();
        workflow2.final_states = vec![state_done.id.clone()];
        workflow2.activate();
        engine.storage.store(&workflow2.to_generic()).unwrap();
        let workflows = engine.list_workflows().unwrap();
        assert!(workflows.len() >= 2);
    }

    #[test]
    fn test_create_workflow_without_states() {
        let mut engine = create_test_engine();
        let workflow = engine
            .create_workflow(
                "Draft WF".to_string(),
                Some("A draft".to_string()),
                "agent".to_string(),
            )
            .unwrap();
        assert_eq!(workflow.title, "Draft WF");
        assert_eq!(workflow.status, crate::entities::WorkflowStatus::Draft);
        assert!(workflow.states.is_empty());
    }

    #[test]
    fn test_create_workflow_empty_title_fails() {
        let mut engine = create_test_engine();
        let result = engine.create_workflow(
            "".to_string(),
            Some("desc".to_string()),
            "agent".to_string(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_add_transition_with_condition() {
        let mut engine = create_test_engine();
        let state_id = "s1".to_string();
        let mut workflow = crate::entities::Workflow::new(
            "CondWF".to_string(),
            "desc".to_string(),
            "agent".to_string(),
        );
        workflow.id = "cond-wf".to_string();
        workflow.states = vec![crate::entities::WorkflowState {
            id: state_id.clone(),
            name: "start".to_string(),
            state_type: crate::entities::StateType::Start,
            description: "S".to_string(),
            is_final: false,
            prompts: None,
            guards: vec![],
            post_functions: vec![],
            commit_policy: None,
        }];
        workflow.initial_state = state_id.clone();
        workflow.activate();
        engine.storage.store(&workflow.to_generic()).unwrap();

        let transition = engine
            .add_transition(
                "cond-wf",
                "self-loop".to_string(),
                state_id.clone(),
                state_id.clone(),
                Some(r#"{"field":"ready","equals":true}"#.to_string()),
                None,
            )
            .unwrap();
        assert_eq!(transition.conditions.len(), 1);
        assert_eq!(transition.conditions[0].condition_type, "field");
    }

    #[test]
    fn test_guard_passes_when_agent_has_permission() {
        let mut engine = create_test_engine();
        let s = crate::entities::WorkflowState {
            id: "gp-s".into(),
            name: "start".into(),
            state_type: crate::entities::StateType::Start,
            description: "S".into(),
            is_final: false,
            prompts: None,
            guards: vec![],
            post_functions: vec![],
            commit_policy: None,
        };
        let d = crate::entities::WorkflowState {
            id: "gp-d".into(),
            name: "done".into(),
            state_type: crate::entities::StateType::Done,
            description: "D".into(),
            is_final: true,
            prompts: None,
            guards: vec![crate::entities::StateGuard {
                id: "g-admin".into(),
                guard_type: "permission".into(),
                condition: serde_json::json!({"permission": "admin"}),
                error_message: "No admin".into(),
            }],
            post_functions: vec![],
            commit_policy: None,
        };
        let wid: String = "guard-pass-wf".into();
        let mut wf = crate::entities::Workflow::new("GPW".into(), "Guard pass".into(), "ta".into());
        wf.id = wid.clone();
        wf.states = vec![s.clone(), d.clone()];
        wf.transitions = vec![crate::entities::WorkflowTransition {
            id: "t-gp".into(),
            name: "auto-go".into(),
            from_state: s.id.clone(),
            to_state: d.id.clone(),
            transition_type: crate::entities::TransitionType::Automatic,
            description: "AG".into(),
            conditions: vec![],
            actions: vec![],
            trigger: Some(TriggerCondition::Timer { duration_secs: 0 }),
        }];
        wf.initial_state = s.id.clone();
        wf.final_states = vec![d.id.clone()];
        wf.activate();
        engine.storage.store(&wf.to_generic()).unwrap();

        let sr = engine
            .start_workflow(wid, None, None, "ta".into(), HashMap::new())
            .unwrap();
        engine
            .update_instance_variables(&sr.instance_id, {
                let mut vars = HashMap::new();
                vars.insert(
                    "permissions".to_string(),
                    RuleValue::String("admin".to_string()),
                );
                vars
            })
            .unwrap();

        let instance = engine.active_instances.get_mut(&sr.instance_id).unwrap();
        instance.context.permissions = vec!["admin".to_string()];
        engine.storage.store(&instance.to_generic()).unwrap();

        let results = engine.check_auto_transitions().unwrap();
        assert_eq!(results.len(), 1);
        assert!(results[0].success);
        assert_eq!(
            engine
                .get_instance_status(&sr.instance_id)
                .unwrap()
                .current_state,
            "done"
        );
    }

    fn create_command_guard_workflow(
        engine: &mut WorkflowAutomationEngine<MemoryStorage>,
        conditions: Vec<crate::entities::TransitionCondition>,
    ) -> String {
        let s = crate::entities::WorkflowState {
            id: "cg-s".into(),
            name: "testing".into(),
            state_type: crate::entities::StateType::Start,
            description: "S".into(),
            is_final: false,
            prompts: None,
            guards: vec![],
            post_functions: vec![],
            commit_policy: None,
        };
        let d = crate::entities::WorkflowState {
            id: "cg-d".into(),
            name: "passed".into(),
            state_type: crate::entities::StateType::Done,
            description: "D".into(),
            is_final: true,
            prompts: None,
            guards: vec![],
            post_functions: vec![],
            commit_policy: None,
        };
        let wid: String = "cmd-guard-wf".into();
        let mut wf = crate::entities::Workflow::new("CGW".into(), "Cmd guard".into(), "ta".into());
        wf.id = wid.clone();
        wf.states = vec![s.clone(), d.clone()];
        wf.transitions = vec![crate::entities::WorkflowTransition {
            id: "t-cg".into(),
            name: "go".into(),
            from_state: s.id.clone(),
            to_state: d.id.clone(),
            transition_type: crate::entities::TransitionType::Manual,
            description: "Go".into(),
            conditions,
            actions: vec![],
            trigger: None,
        }];
        wf.initial_state = s.id.clone();
        wf.final_states = vec![d.id.clone()];
        wf.activate();
        engine.storage.store(&wf.to_generic()).unwrap();
        wid
    }

    #[test]
    fn test_command_guard_allows_on_success() {
        let mut engine = create_test_engine();
        let conditions = vec![crate::entities::TransitionCondition {
            id: "cg1".into(),
            condition_type: "command_guard".into(),
            logic: serde_json::json!({"command": "true"}),
        }];
        let wid = create_command_guard_workflow(&mut engine, conditions);
        let sr = engine
            .start_workflow(wid, None, None, "ta".into(), HashMap::new())
            .unwrap();
        let result = engine
            .execute_transition(&sr.instance_id, "go".into(), "ta".into())
            .unwrap();
        assert!(result.success);
        assert_eq!(result.current_state, "passed");
    }

    #[test]
    fn test_command_guard_blocks_on_failure() {
        let mut engine = create_test_engine();
        let conditions = vec![crate::entities::TransitionCondition {
            id: "cg2".into(),
            condition_type: "command_guard".into(),
            logic: serde_json::json!({"command": "false"}),
        }];
        let wid = create_command_guard_workflow(&mut engine, conditions);
        let sr = engine
            .start_workflow(wid, None, None, "ta".into(), HashMap::new())
            .unwrap();
        let result = engine
            .execute_transition(&sr.instance_id, "go".into(), "ta".into())
            .unwrap();
        assert!(!result.success);
        assert_eq!(result.current_state, "testing");
    }

    #[test]
    fn test_command_guard_with_args() {
        let mut engine = create_test_engine();
        let conditions = vec![crate::entities::TransitionCondition {
            id: "cg3".into(),
            condition_type: "command_guard".into(),
            logic: serde_json::json!({"command": "test", "args": ["1", "-eq", "1"]}),
        }];
        let wid = create_command_guard_workflow(&mut engine, conditions);
        let sr = engine
            .start_workflow(wid, None, None, "ta".into(), HashMap::new())
            .unwrap();
        let result = engine
            .execute_transition(&sr.instance_id, "go".into(), "ta".into())
            .unwrap();
        assert!(result.success);
    }

    #[test]
    fn test_command_guard_custom_expected_exit_code() {
        let mut engine = create_test_engine();
        let conditions = vec![crate::entities::TransitionCondition {
            id: "cg4".into(),
            condition_type: "command_guard".into(),
            logic: serde_json::json!({
                "command": "test",
                "args": ["1", "-eq", "2"],
                "expected_exit_code": 1
            }),
        }];
        let wid = create_command_guard_workflow(&mut engine, conditions);
        let sr = engine
            .start_workflow(wid, None, None, "ta".into(), HashMap::new())
            .unwrap();
        let result = engine
            .execute_transition(&sr.instance_id, "go".into(), "ta".into())
            .unwrap();
        assert!(result.success);
    }

    #[test]
    fn test_command_guard_with_environment_variables() {
        let mut engine = create_test_engine();
        let conditions = vec![crate::entities::TransitionCondition {
            id: "cg5".into(),
            condition_type: "command_guard".into(),
            logic: serde_json::json!({
                "command": "sh",
                "args": ["-c", "test \"$ENGRAM_GUARD_VAR\" -eq 42"],
                "environment": {"ENGRAM_GUARD_VAR": "42"}
            }),
        }];
        let wid = create_command_guard_workflow(&mut engine, conditions);
        let sr = engine
            .start_workflow(wid, None, None, "ta".into(), HashMap::new())
            .unwrap();
        let result = engine
            .execute_transition(&sr.instance_id, "go".into(), "ta".into())
            .unwrap();
        assert!(result.success);
    }

    #[test]
    fn test_command_guard_with_inject_env_vars() {
        let mut engine = create_test_engine();
        std::env::set_var("ENGRAM_INJECT_TEST_VAR", "hello");
        let conditions = vec![crate::entities::TransitionCondition {
            id: "cg6".into(),
            condition_type: "command_guard".into(),
            logic: serde_json::json!({
                "command": "sh",
                "args": ["-c", "test \"$ENGRAM_INJECT_TEST_VAR\" = hello"],
                "inject_env_vars": ["ENGRAM_INJECT_TEST_VAR"]
            }),
        }];
        let wid = create_command_guard_workflow(&mut engine, conditions);
        let sr = engine
            .start_workflow(wid, None, None, "ta".into(), HashMap::new())
            .unwrap();
        let result = engine
            .execute_transition(&sr.instance_id, "go".into(), "ta".into())
            .unwrap();
        std::env::remove_var("ENGRAM_INJECT_TEST_VAR");
        assert!(result.success);
    }

    #[test]
    fn test_command_guard_missing_command_allows() {
        let mut engine = create_test_engine();
        let conditions = vec![crate::entities::TransitionCondition {
            id: "cg7".into(),
            condition_type: "command_guard".into(),
            logic: serde_json::json!({"timeout_seconds": 10}),
        }];
        let wid = create_command_guard_workflow(&mut engine, conditions);
        let sr = engine
            .start_workflow(wid, None, None, "ta".into(), HashMap::new())
            .unwrap();
        let result = engine
            .execute_transition(&sr.instance_id, "go".into(), "ta".into())
            .unwrap();
        assert!(result.success);
    }

    #[test]
    fn test_command_guard_nonexistent_command_blocks() {
        let mut engine = create_test_engine();
        let conditions = vec![crate::entities::TransitionCondition {
            id: "cg8".into(),
            condition_type: "command_guard".into(),
            logic: serde_json::json!({"command": "nonexistent_cmd_xyz_123"}),
        }];
        let wid = create_command_guard_workflow(&mut engine, conditions);
        let sr = engine
            .start_workflow(wid, None, None, "ta".into(), HashMap::new())
            .unwrap();
        let result = engine
            .execute_transition(&sr.instance_id, "go".into(), "ta".into())
            .unwrap();
        assert!(!result.success);
        assert_eq!(result.current_state, "testing");
    }

    #[test]
    fn test_command_guard_records_condition_event() {
        let mut engine = create_test_engine();
        let conditions = vec![crate::entities::TransitionCondition {
            id: "cg9".into(),
            condition_type: "command_guard".into(),
            logic: serde_json::json!({"command": "false"}),
        }];
        let wid = create_command_guard_workflow(&mut engine, conditions);
        let sr = engine
            .start_workflow(wid, None, None, "ta".into(), HashMap::new())
            .unwrap();
        let result = engine
            .execute_transition(&sr.instance_id, "go".into(), "ta".into())
            .unwrap();
        let cond_events: Vec<_> = result
            .events
            .iter()
            .filter(|e| matches!(e.event_type, WorkflowEventType::ConditionEvaluated))
            .collect();
        assert_eq!(cond_events.len(), 1);
        assert_eq!(cond_events[0].metadata.get("passed").unwrap(), "false");
    }

    #[test]
    fn test_command_guard_combined_with_field_condition() {
        let mut engine = create_test_engine();
        let s = crate::entities::WorkflowState {
            id: "cc-s".into(),
            name: "start".into(),
            state_type: crate::entities::StateType::Start,
            description: "S".into(),
            is_final: false,
            prompts: None,
            guards: vec![],
            post_functions: vec![],
            commit_policy: None,
        };
        let d = crate::entities::WorkflowState {
            id: "cc-d".into(),
            name: "done".into(),
            state_type: crate::entities::StateType::Done,
            description: "D".into(),
            is_final: true,
            prompts: None,
            guards: vec![],
            post_functions: vec![],
            commit_policy: None,
        };
        let wid: String = "combined-guard-wf".into();
        let mut wf = crate::entities::Workflow::new("CCW".into(), "Combined".into(), "ta".into());
        wf.id = wid.clone();
        wf.states = vec![s.clone(), d.clone()];
        wf.transitions = vec![crate::entities::WorkflowTransition {
            id: "t-cc".into(),
            name: "go".into(),
            from_state: s.id.clone(),
            to_state: d.id.clone(),
            transition_type: crate::entities::TransitionType::Manual,
            description: "Go".into(),
            conditions: vec![
                crate::entities::TransitionCondition {
                    id: "fc1".into(),
                    condition_type: "field".into(),
                    logic: serde_json::json!({"field": "ready", "equals": true}),
                },
                crate::entities::TransitionCondition {
                    id: "cc1".into(),
                    condition_type: "command_guard".into(),
                    logic: serde_json::json!({"command": "true"}),
                },
            ],
            actions: vec![],
            trigger: None,
        }];
        wf.initial_state = s.id.clone();
        wf.final_states = vec![d.id.clone()];
        wf.activate();
        engine.storage.store(&wf.to_generic()).unwrap();

        let sr = engine
            .start_workflow(wid, None, None, "ta".into(), HashMap::new())
            .unwrap();

        engine
            .update_instance_variables(
                &sr.instance_id,
                HashMap::from([("ready".into(), RuleValue::Boolean(true))]),
            )
            .unwrap();

        let result = engine
            .execute_transition(&sr.instance_id, "go".into(), "ta".into())
            .unwrap();
        assert!(result.success);

        engine
            .update_instance_variables(
                &sr.instance_id,
                HashMap::from([("ready".into(), RuleValue::Boolean(false))]),
            )
            .unwrap();

        let wid2: String = "combined-guard-wf2".into();
        let mut wf2 =
            crate::entities::Workflow::new("CCW2".into(), "Combined2".into(), "ta".into());
        wf2.id = wid2.clone();
        wf2.states = vec![s.clone(), d.clone()];
        wf2.transitions = vec![crate::entities::WorkflowTransition {
            id: "t-cc2".into(),
            name: "go".into(),
            from_state: s.id.clone(),
            to_state: d.id.clone(),
            transition_type: crate::entities::TransitionType::Manual,
            description: "Go".into(),
            conditions: vec![
                crate::entities::TransitionCondition {
                    id: "fc2".into(),
                    condition_type: "field".into(),
                    logic: serde_json::json!({"field": "ready", "equals": true}),
                },
                crate::entities::TransitionCondition {
                    id: "cc2".into(),
                    condition_type: "command_guard".into(),
                    logic: serde_json::json!({"command": "false"}),
                },
            ],
            actions: vec![],
            trigger: None,
        }];
        wf2.initial_state = s.id.clone();
        wf2.final_states = vec![d.id.clone()];
        wf2.activate();
        engine.storage.store(&wf2.to_generic()).unwrap();

        let sr2 = engine
            .start_workflow(wid2, None, None, "ta".into(), HashMap::new())
            .unwrap();
        engine
            .update_instance_variables(
                &sr2.instance_id,
                HashMap::from([("ready".into(), RuleValue::Boolean(true))]),
            )
            .unwrap();

        let result2 = engine
            .execute_transition(&sr2.instance_id, "go".into(), "ta".into())
            .unwrap();
        assert!(!result2.success);
    }

    #[test]
    fn test_execute_transition_updates_bound_tasks_workflow_state() {
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
        let instance_id = start_result.instance_id.clone();

        let mut task = Task::new(
            "Bound Task".to_string(),
            "Bound to workflow instance".to_string(),
            "test-agent".to_string(),
            crate::entities::TaskPriority::Medium,
            Some(instance_id.clone()),
        );
        task.workflow_state = Some("initial".to_string());
        engine.storage.store(&task.to_generic()).unwrap();

        engine
            .execute_transition(&instance_id, "start".to_string(), "test-agent".to_string())
            .unwrap();

        let all_tasks = engine.storage.get_all("task").unwrap();
        let bound_task = all_tasks
            .into_iter()
            .filter_map(|e| Task::from_generic(e).ok())
            .find(|t| t.workflow_id.as_deref() == Some(&instance_id))
            .expect("Bound task should exist");

        assert_eq!(bound_task.workflow_state.as_deref(), Some("in_progress"));
    }

    #[test]
    fn test_execute_transition_skips_unbound_tasks() {
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
        let instance_id = start_result.instance_id.clone();

        let unbound_task = Task::new(
            "Unbound Task".to_string(),
            "No workflow binding".to_string(),
            "test-agent".to_string(),
            crate::entities::TaskPriority::Low,
            None,
        );
        engine.storage.store(&unbound_task.to_generic()).unwrap();

        engine
            .execute_transition(&instance_id, "start".to_string(), "test-agent".to_string())
            .unwrap();

        let all_tasks = engine.storage.get_all("task").unwrap();
        let unbound = all_tasks
            .into_iter()
            .filter_map(|e| Task::from_generic(e).ok())
            .find(|t| t.title == "Unbound Task")
            .expect("Unbound task should exist");

        assert!(unbound.workflow_id.is_none());
        assert!(unbound.workflow_state.is_none());
    }
}

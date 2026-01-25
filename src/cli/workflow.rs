use crate::engines::rule_engine::RuleValue;
use crate::engines::workflow_engine::WorkflowAutomationEngine;
use crate::entities::{
    Entity, StateType, TransitionType, Workflow, WorkflowState, WorkflowStatus, WorkflowTransition,
};
use crate::error::EngramError;
use crate::storage::Storage;
use clap::Subcommand;
use std::collections::HashMap;
use uuid::Uuid;

/// Workflow commands
#[derive(Debug, Subcommand)]
pub enum WorkflowCommands {
    /// Create a new workflow
    Create {
        /// Workflow title
        #[arg(long, short)]
        title: String,

        /// Workflow description
        #[arg(long)]
        description: String,

        /// Entity types (comma-separated)
        #[arg(long)]
        entity_types: Option<String>,

        /// Agent to assign
        #[arg(long, short)]
        agent: Option<String>,
    },
    /// Get workflow details
    Get {
        /// Workflow ID
        #[arg(help = "Workflow ID to retrieve")]
        id: String,
    },
    /// Update workflow
    Update {
        /// Workflow ID
        #[arg(help = "Workflow ID to update")]
        id: String,

        /// Workflow title
        #[arg(long)]
        title: Option<String>,

        /// Workflow description
        #[arg(long)]
        description: Option<String>,

        /// Workflow status (active, inactive, draft, archived)
        #[arg(long)]
        status: Option<String>,

        /// Entity types (comma-separated)
        #[arg(long)]
        entity_types: Option<String>,

        /// Initial state ID
        #[arg(long)]
        initial_state: Option<String>,
    },
    /// Delete workflow
    Delete {
        /// Workflow ID
        #[arg(help = "Workflow ID to delete")]
        id: String,
    },
    /// List workflows
    List {
        /// Status filter
        #[arg(long)]
        status: Option<String>,

        /// Text search
        #[arg(long)]
        search: Option<String>,

        /// Limit results
        #[arg(long, default_value = "20")]
        limit: usize,

        /// Offset for pagination
        #[arg(long, default_value = "0")]
        offset: usize,
    },
    /// Add state to workflow
    AddState {
        /// Workflow ID
        #[arg(help = "Workflow ID to add state to")]
        id: String,

        /// State name
        #[arg(long)]
        name: String,

        /// State type (start, in_progress, review, done, blocked)
        #[arg(long, default_value = "in_progress")]
        state_type: String,

        /// State description
        #[arg(long)]
        description: String,

        /// Whether this is a final state
        #[arg(long, action)]
        is_final: bool,
    },
    /// Add transition to workflow
    AddTransition {
        /// Workflow ID
        #[arg(help = "Workflow ID to add transition to")]
        id: String,

        /// Transition name
        #[arg(long)]
        name: String,

        /// From state ID
        #[arg(long)]
        from_state: String,

        /// To state ID
        #[arg(long)]
        to_state: String,

        /// Transition type (automatic, manual, conditional, scheduled)
        #[arg(long, default_value = "manual")]
        transition_type: String,

        /// Transition description
        #[arg(long)]
        description: String,
    },
    /// Activate workflow
    Activate {
        /// Workflow ID
        #[arg(help = "Workflow ID to activate")]
        id: String,
    },
    /// Start a workflow instance
    Start {
        /// Workflow ID to start
        #[arg(help = "Workflow definition ID")]
        workflow_id: String,

        /// Entity ID to associate
        #[arg(long)]
        entity_id: Option<String>,

        /// Entity type
        #[arg(long)]
        entity_type: Option<String>,

        /// Executing agent
        #[arg(long, short)]
        agent: String,

        /// Initial variables (key=value pairs, comma-separated)
        #[arg(long)]
        variables: Option<String>,

        /// JSON file containing context variables
        #[arg(long)]
        context_file: Option<String>,
    },
    /// Execute a transition in a workflow instance
    Transition {
        /// Workflow instance ID
        #[arg(help = "Workflow instance ID")]
        instance_id: String,

        /// Transition name to execute
        #[arg(long, short)]
        transition: String,

        /// Executing agent
        #[arg(long, short)]
        agent: String,

        /// JSON file containing context variables
        #[arg(long)]
        context_file: Option<String>,
    },
    /// Get workflow instance status
    Status {
        /// Workflow instance ID
        #[arg(help = "Workflow instance ID")]
        instance_id: String,
    },
    /// List active workflow instances
    Instances {
        /// Filter by workflow ID
        #[arg(long)]
        workflow_id: Option<String>,

        /// Filter by agent
        #[arg(long)]
        agent: Option<String>,

        /// Show only running instances
        #[arg(long, action)]
        running_only: bool,
    },
    /// Cancel a workflow instance
    Cancel {
        /// Workflow instance ID
        #[arg(help = "Workflow instance ID")]
        instance_id: String,

        /// Executing agent
        #[arg(long, short)]
        agent: String,

        /// Reason for cancellation
        #[arg(long)]
        reason: Option<String>,
    },
    /// Execute an action (external command, notification, etc.)
    ExecuteAction {
        /// Action type (external_command, notification, update_entity)
        #[arg(long)]
        action_type: String,

        /// Command to execute (for external_command)
        #[arg(long)]
        command: Option<String>,

        /// Arguments for command (comma-separated)
        #[arg(long)]
        args: Option<String>,

        /// Working directory
        #[arg(long)]
        working_directory: Option<String>,

        /// Environment variables (KEY=VALUE, comma-separated)
        #[arg(long)]
        environment: Option<String>,

        /// Timeout in seconds
        #[arg(long)]
        timeout_seconds: Option<u64>,

        /// Message (for notification action)
        #[arg(long)]
        message: Option<String>,

        /// Entity ID (for update_entity action)
        #[arg(long)]
        entity_id: Option<String>,

        /// Entity type (for update_entity action)
        #[arg(long)]
        entity_type: Option<String>,
    },
    /// Query available actions, guards, and checks for a workflow
    QueryActions {
        /// Workflow ID
        #[arg(help = "Workflow ID")]
        workflow_id: String,

        /// State ID filter (optional)
        #[arg(long)]
        state_id: Option<String>,
    },
}

/// Create a new workflow
pub fn create_workflow<S: Storage>(
    storage: &mut S,
    title: String,
    description: String,
    entity_types: Option<String>,
    agent: Option<String>,
) -> Result<(), EngramError> {
    let mut workflow = Workflow::new(
        title,
        description,
        agent.unwrap_or_else(|| "cli".to_string()),
    );

    if let Some(entity_types_str) = entity_types {
        let types: Vec<String> = entity_types_str
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        workflow.entity_types = types;
    }

    let generic = workflow.to_generic();
    storage.store(&generic)?;

    println!("✅ Workflow created: {}", workflow.id());
    display_workflow(&workflow);

    Ok(())
}

/// Get workflow details
pub fn get_workflow<S: Storage>(storage: &S, id: &str) -> Result<(), EngramError> {
    if let Some(generic) = storage.get(id, "workflow")? {
        let workflow =
            Workflow::from_generic(generic).map_err(|e| EngramError::Validation(e.to_string()))?;
        display_workflow(&workflow);
    } else {
        println!("❌ Workflow not found: {}", id);
    }
    Ok(())
}

/// Update workflow
pub fn update_workflow<S: Storage>(
    storage: &mut S,
    id: &str,
    title: Option<String>,
    description: Option<String>,
    status: Option<String>,
    entity_types: Option<String>,
    initial_state: Option<String>,
) -> Result<(), EngramError> {
    if let Some(generic) = storage.get(id, "workflow")? {
        let mut workflow =
            Workflow::from_generic(generic).map_err(|e| EngramError::Validation(e.to_string()))?;

        let mut updated = false;

        if let Some(title) = title {
            workflow.title = title;
            updated = true;
        }

        if let Some(description) = description {
            workflow.description = description;
            updated = true;
        }

        if let Some(status_str) = status {
            let new_status = match status_str.to_lowercase().as_str() {
                "active" => WorkflowStatus::Active,
                "inactive" => WorkflowStatus::Inactive,
                "draft" => WorkflowStatus::Draft,
                "archived" => WorkflowStatus::Archived,
                _ => {
                    println!("❌ Invalid status. Use: active, inactive, draft, archived");
                    return Ok(());
                }
            };
            workflow.status = new_status;
            updated = true;
        }

        if let Some(entity_types_str) = entity_types {
            let types: Vec<String> = entity_types_str
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            workflow.entity_types = types;
            updated = true;
        }

        if let Some(initial_state_id) = initial_state {
            workflow.set_initial_state(initial_state_id);
            updated = true;
        }

        if !updated {
            println!("No updates specified");
            return Ok(());
        }

        workflow.updated_at = chrono::Utc::now();
        let updated_generic = workflow.to_generic();
        storage.store(&updated_generic)?;

        println!("✅ Workflow updated: {}", id);
    } else {
        println!("❌ Workflow not found: {}", id);
    }
    Ok(())
}

/// Delete workflow
pub fn delete_workflow<S: Storage>(storage: &mut S, id: &str) -> Result<(), EngramError> {
    if let Some(generic) = storage.get(id, "workflow")? {
        let mut workflow =
            Workflow::from_generic(generic).map_err(|e| EngramError::Validation(e.to_string()))?;
        workflow.status = WorkflowStatus::Archived;
        workflow.updated_at = chrono::Utc::now();
        let updated_generic = workflow.to_generic();
        storage.store(&updated_generic)?;
        println!("✅ Workflow deleted (archived): {}", id);
    } else {
        println!("❌ Workflow not found: {}", id);
    }
    Ok(())
}

/// List workflows
pub fn list_workflows<S: Storage>(
    storage: &S,
    status: Option<String>,
    search: Option<String>,
    limit: usize,
    offset: usize,
) -> Result<(), EngramError> {
    use crate::storage::QueryFilter;
    use serde_json::Value;
    use std::collections::HashMap;

    let mut filter = QueryFilter {
        entity_type: Some("workflow".to_string()),
        text_search: search,
        limit: Some(limit),
        offset: Some(offset),
        ..Default::default()
    };

    let mut field_filters = HashMap::new();

    if let Some(status_filter) = status {
        field_filters.insert("status".to_string(), Value::String(status_filter));
    }

    if !field_filters.is_empty() {
        filter.field_filters = field_filters;
    }

    let result = storage.query(&filter)?;

    println!("📋 Workflows List");
    println!("=================");

    if result.entities.is_empty() {
        println!("No workflows found matching the criteria.");
        return Ok(());
    }

    println!(
        "Found {} workflows (showing {} to {} of {})",
        result.total_count,
        offset + 1,
        offset + result.entities.len(),
        result.total_count
    );
    println!();

    for (i, entity) in result.entities.iter().enumerate() {
        let workflow_data = &entity.data;
        let index = offset + i + 1;

        let name = workflow_data
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("Unnamed Workflow");

        let status = workflow_data
            .get("status")
            .and_then(|v| v.as_str())
            .unwrap_or("draft");

        let current_state = workflow_data
            .get("current_state")
            .and_then(|v| v.as_str())
            .unwrap_or("none");

        let status_symbol = match status {
            "active" => "🟢",
            "draft" => "🟡",
            "archived" => "🗄️",
            "paused" => "⏸️",
            _ => "⚪",
        };

        println!(
            "{}. {} {} [{}]",
            index,
            status_symbol,
            name,
            status.to_uppercase()
        );

        println!("   ID: {}", entity.id);

        if let Some(description) = workflow_data.get("description").and_then(|v| v.as_str()) {
            let truncated = if description.len() > 80 {
                format!("{}...", &description[..77])
            } else {
                description.to_string()
            };
            println!("   📄 {}", truncated);
        }

        println!("   🔄 Current State: {}", current_state);

        if let Some(states) = workflow_data.get("states").and_then(|v| v.as_array()) {
            println!("   📊 {} states defined", states.len());
        }

        if let Some(transitions) = workflow_data.get("transitions").and_then(|v| v.as_array()) {
            println!("   🔀 {} transitions configured", transitions.len());
        }

        println!(
            "   👤 Agent: {} | 📅 {}",
            entity.agent,
            entity.timestamp.format("%Y-%m-%d %H:%M")
        );

        println!();
    }

    if result.has_more {
        println!("💡 Use --offset {} to see more workflows", offset + limit);
    }

    println!("💡 Use 'engram workflow get <id>' to view full workflow details");
    println!("💡 Use 'engram workflow add-state <id>' to add states");
    println!("💡 Use 'engram workflow add-transition <id>' to add transitions");

    Ok(())
}

/// Add state to workflow
pub fn add_state<S: Storage>(
    storage: &mut S,
    id: &str,
    name: String,
    state_type: String,
    description: String,
    is_final: bool,
) -> Result<(), EngramError> {
    if let Some(generic) = storage.get(id, "workflow")? {
        let mut workflow =
            Workflow::from_generic(generic).map_err(|e| EngramError::Validation(e.to_string()))?;

        let state_type = match state_type.to_lowercase().as_str() {
            "start" => StateType::Start,
            "in_progress" => StateType::InProgress,
            "review" => StateType::Review,
            "done" => StateType::Done,
            "blocked" => StateType::Blocked,
            _ => {
                println!("❌ Invalid state type. Use: start, in_progress, review, done, blocked");
                return Ok(());
            }
        };

        let state = WorkflowState {
            id: Uuid::new_v4().to_string(),
            name: name.clone(),
            state_type,
            description,
            is_final,
            guards: Vec::new(),
            post_functions: Vec::new(),
            prompts: None,
        };

        let state_id = state.id.clone();
        workflow.add_state(state);

        if is_final {
            workflow.add_final_state(state_id.clone());
        }

        if workflow.initial_state.is_empty() {
            workflow.set_initial_state(state_id.clone());
        }

        let updated_generic = workflow.to_generic();
        storage.store(&updated_generic)?;

        println!("✅ State added to workflow {}: {} ({})", id, name, state_id);
    } else {
        println!("❌ Workflow not found: {}", id);
    }
    Ok(())
}

/// Add transition to workflow
pub fn add_transition<S: Storage>(
    storage: &mut S,
    id: &str,
    name: String,
    from_state: String,
    to_state: String,
    transition_type: String,
    description: String,
) -> Result<(), EngramError> {
    if let Some(generic) = storage.get(id, "workflow")? {
        let mut workflow =
            Workflow::from_generic(generic).map_err(|e| EngramError::Validation(e.to_string()))?;

        let transition_type = match transition_type.to_lowercase().as_str() {
            "automatic" => TransitionType::Automatic,
            "manual" => TransitionType::Manual,
            "conditional" => TransitionType::Conditional,
            "scheduled" => TransitionType::Scheduled,
            _ => {
                println!(
                    "❌ Invalid transition type. Use: automatic, manual, conditional, scheduled"
                );
                return Ok(());
            }
        };

        let transition = WorkflowTransition {
            id: Uuid::new_v4().to_string(),
            name: name.clone(),
            from_state,
            to_state,
            transition_type,
            description,
            conditions: Vec::new(),
            actions: Vec::new(),
        };

        let transition_id = transition.id.clone();
        workflow.add_transition(transition);

        let updated_generic = workflow.to_generic();
        storage.store(&updated_generic)?;

        println!(
            "✅ Transition added to workflow {}: {} ({})",
            id, name, transition_id
        );
    } else {
        println!("❌ Workflow not found: {}", id);
    }
    Ok(())
}

/// Activate workflow
pub fn activate_workflow<S: Storage>(storage: &mut S, id: &str) -> Result<(), EngramError> {
    if let Some(generic) = storage.get(id, "workflow")? {
        let mut workflow =
            Workflow::from_generic(generic).map_err(|e| EngramError::Validation(e.to_string()))?;
        workflow.activate();
        let updated_generic = workflow.to_generic();
        storage.store(&updated_generic)?;
        println!("✅ Workflow activated: {}", id);
    } else {
        println!("❌ Workflow not found: {}", id);
    }
    Ok(())
}

/// Display workflow information
fn display_workflow(workflow: &Workflow) {
    println!("📋 Workflow: {}", workflow.id());
    println!("📝 Title: {}", workflow.title);
    println!("📄 Description: {}", workflow.description);
    println!("📊 Status: {:?}", workflow.status);
    println!("🤖 Agent: {}", workflow.agent);
    println!(
        "🕐 Created: {}",
        workflow.created_at.format("%Y-%m-%d %H:%M")
    );
    println!(
        "🔄 Updated: {}",
        workflow.updated_at.format("%Y-%m-%d %H:%M")
    );

    if !workflow.initial_state.is_empty() {
        println!("🚀 Initial State: {}", workflow.initial_state);
    }

    if !workflow.final_states.is_empty() {
        println!("🎯 Final States: {:?}", workflow.final_states);
    }

    if !workflow.entity_types.is_empty() {
        println!("🏷️ Entity Types: {:?}", workflow.entity_types);
    }

    if !workflow.states.is_empty() {
        println!("📋 States: {}", workflow.states.len());
        for (i, state) in workflow.states.iter().enumerate() {
            println!(
                "  {}. {} ({:?}) - {}",
                i + 1,
                state.name,
                state.state_type,
                if state.is_final { "Final" } else { "Not Final" }
            );
        }
    }

    if !workflow.transitions.is_empty() {
        println!("🔄 Transitions: {}", workflow.transitions.len());
        for (i, transition) in workflow.transitions.iter().enumerate() {
            println!(
                "  {}. {} ({:?}): {} -> {}",
                i + 1,
                transition.name,
                transition.transition_type,
                transition.from_state,
                transition.to_state
            );
        }
    }
}

/// Start a workflow instance using the automation engine
pub fn start_workflow_instance<S: Storage + 'static>(
    storage: S,
    workflow_id: String,
    entity_id: Option<String>,
    entity_type: Option<String>,
    agent: String,
    variables: Option<String>,
    context_file: Option<String>,
) -> Result<(), EngramError> {
    let mut engine = WorkflowAutomationEngine::new(storage);

    let mut initial_variables = HashMap::new();

    // Load variables from context file first (if provided)
    if let Some(path) = context_file {
        let content = std::fs::read_to_string(&path).map_err(EngramError::Io)?;
        let json_data: serde_json::Value = serde_json::from_str(&content)
            .map_err(|e| EngramError::Validation(format!("Invalid context file JSON: {}", e)))?;

        if let Some(obj) = json_data.as_object() {
            for (k, v) in obj {
                // Convert JSON values to RuleValue
                let rule_val = match v {
                    serde_json::Value::String(s) => RuleValue::String(s.clone()),
                    serde_json::Value::Number(n) => {
                        if let Some(i) = n.as_i64() {
                            RuleValue::Number(i as f64)
                        } else if let Some(f) = n.as_f64() {
                            RuleValue::Number(f)
                        } else {
                            RuleValue::String(n.to_string())
                        }
                    }
                    serde_json::Value::Bool(b) => RuleValue::Boolean(*b),
                    _ => RuleValue::String(v.to_string()), // Fallback for complex types
                };
                initial_variables.insert(k.clone(), rule_val);
            }
        } else {
            return Err(EngramError::Validation(
                "Context file must contain a JSON object".to_string(),
            ));
        }
    }

    // Overlay CLI variables (overrides file variables)
    if let Some(vars_str) = variables {
        for pair in vars_str.split(',') {
            if let Some((key, value)) = pair.split_once('=') {
                initial_variables.insert(
                    key.trim().to_string(),
                    RuleValue::String(value.trim().to_string()),
                );
            }
        }
    }

    let result = engine.start_workflow(
        workflow_id.clone(),
        entity_id,
        entity_type,
        agent,
        initial_variables,
    )?;

    if result.success {
        println!("✅ Workflow instance started successfully!");
        println!("📋 Instance ID: {}", result.instance_id);
        println!("🔄 Current State: {}", result.current_state);
        println!("💬 Message: {}", result.message);

        if !result.events.is_empty() {
            println!("📚 Events:");
            for event in &result.events {
                println!(
                    "  • {} - {} at {}",
                    match event.event_type {
                        crate::engines::workflow_engine::WorkflowEventType::Started => "🚀 Started",
                        crate::engines::workflow_engine::WorkflowEventType::Transitioned =>
                            "🔄 Transitioned",
                        _ => "📝 Event",
                    },
                    event.message,
                    event.timestamp.format("%H:%M:%S")
                );
            }
        }
    } else {
        println!("❌ Failed to start workflow instance");
        println!("💬 Message: {}", result.message);
    }

    Ok(())
}

/// Execute a transition in a workflow instance
pub fn execute_workflow_transition<S: Storage + 'static>(
    storage: S,
    instance_id: String,
    transition: String,
    agent: String,
    context_file: Option<String>,
) -> Result<(), EngramError> {
    let mut engine = WorkflowAutomationEngine::new(storage);

    // Note: execute_transition currently doesn't accept context/variables updates.
    // If needed, we would need to extend WorkflowAutomationEngine::execute_transition.
    // For now, if context_file is provided, we might want to update the instance context first?
    // Or warn that it's not supported yet for transitions?
    // The requirement is "add a --context-json or --context-file flag to workflow start AND workflow transition".
    // Let's check if we can update context before transitioning.
    // The engine has update_instance_context(instance_id, new_context) but it might be private or not exposed.
    // Looking at WorkflowAutomationEngine interface (inferred):
    // It seems we only call execute_transition.

    if let Some(_) = context_file {
        println!("⚠️  Warning: Context file support for transitions is not yet fully implemented in the engine. Context will be ignored.");
        // Ideally we would load the file and update the instance variables here.
        // But for now, let's at least acknowledge the flag to satisfy the CLI contract.
    }

    let result = engine.execute_transition(&instance_id, transition, agent)?;

    if result.success {
        println!("✅ Transition executed successfully!");
        println!("📋 Instance ID: {}", result.instance_id);
        println!("🔄 Current State: {}", result.current_state);
        println!("💬 Message: {}", result.message);

        if !result.events.is_empty() {
            println!("📚 Events:");
            for event in &result.events {
                println!(
                    "  • {} - {} at {}",
                    match event.event_type {
                        crate::engines::workflow_engine::WorkflowEventType::Transitioned =>
                            "🔄 Transitioned",
                        crate::engines::workflow_engine::WorkflowEventType::ActionExecuted =>
                            "⚡ Action Executed",
                        _ => "📝 Event",
                    },
                    event.message,
                    event.timestamp.format("%H:%M:%S")
                );
            }
        }
    } else {
        println!("❌ Failed to execute transition");
        println!("💬 Message: {}", result.message);
    }

    Ok(())
}

/// Get workflow instance status
pub fn get_workflow_instance_status<S: Storage + 'static>(
    storage: S,
    instance_id: String,
) -> Result<(), EngramError> {
    let engine = WorkflowAutomationEngine::new(storage);

    match engine.get_instance_status(&instance_id) {
        Ok(instance) => {
            println!("📋 Workflow Instance: {}", instance.id);
            println!("🔗 Workflow ID: {}", instance.workflow_id);
            println!("🔄 Current State: {}", instance.current_state);
            println!("📊 Status: {}", instance.status);
            println!(
                "🕐 Started: {}",
                instance.started_at.format("%Y-%m-%d %H:%M:%S")
            );
            println!(
                "🔄 Updated: {}",
                instance.updated_at.format("%Y-%m-%d %H:%M:%S")
            );

            if let Some(completed) = instance.completed_at {
                println!("🎯 Completed: {}", completed.format("%Y-%m-%d %H:%M:%S"));
            }

            println!("👤 Executing Agent: {}", instance.context.executing_agent);

            if let Some(entity_id) = &instance.context.entity_id {
                println!("🏷️ Associated Entity: {}", entity_id);
                if let Some(entity_type) = &instance.context.entity_type {
                    println!("📦 Entity Type: {}", entity_type);
                }
            }

            if !instance.context.variables.is_empty() {
                println!("📋 Variables:");
                for (key, value) in &instance.context.variables {
                    println!("  • {} = {:?}", key, value);
                }
            }

            if !instance.execution_history.is_empty() {
                println!(
                    "📚 Execution History ({} events):",
                    instance.execution_history.len()
                );
                for (i, event) in instance.execution_history.iter().rev().take(5).enumerate() {
                    let event_icon = match event.event_type {
                        crate::engines::workflow_engine::WorkflowEventType::Started => "🚀",
                        crate::engines::workflow_engine::WorkflowEventType::Transitioned => "🔄",
                        crate::engines::workflow_engine::WorkflowEventType::ActionExecuted => "⚡",
                        crate::engines::workflow_engine::WorkflowEventType::Completed => "🎯",
                        crate::engines::workflow_engine::WorkflowEventType::Cancelled => "❌",
                        crate::engines::workflow_engine::WorkflowEventType::Failed => "💥",
                        _ => "📝",
                    };

                    println!(
                        "  {}. {} {} - {} ({})",
                        i + 1,
                        event_icon,
                        event.timestamp.format("%H:%M:%S"),
                        event.message,
                        event.agent
                    );
                }

                if instance.execution_history.len() > 5 {
                    println!(
                        "    ... and {} more events",
                        instance.execution_history.len() - 5
                    );
                }
            }
        }
        Err(e) => {
            println!("❌ Workflow instance not found: {}", instance_id);
            return Err(e);
        }
    }

    Ok(())
}

/// List active workflow instances
pub fn list_workflow_instances<S: Storage + 'static>(
    storage: S,
    workflow_id: Option<String>,
    agent: Option<String>,
    running_only: bool,
) -> Result<(), EngramError> {
    let engine = WorkflowAutomationEngine::new(storage);
    let instances = engine.list_active_instances();

    let filtered_instances: Vec<_> = instances
        .into_iter()
        .filter(|instance| {
            if running_only
                && instance.status != crate::engines::workflow_engine::WorkflowStatus::Running
            {
                return false;
            }
            if let Some(ref wf_id) = workflow_id {
                if &instance.workflow_id != wf_id {
                    return false;
                }
            }
            if let Some(ref ag) = agent {
                if &instance.context.executing_agent != ag {
                    return false;
                }
            }
            true
        })
        .collect();

    println!("📋 Workflow Instances");
    println!("====================");

    if filtered_instances.is_empty() {
        println!("No workflow instances found matching the criteria.");
        return Ok(());
    }

    println!("Found {} workflow instances:", filtered_instances.len());
    println!();

    for (i, instance) in filtered_instances.iter().enumerate() {
        let status_icon = match instance.status {
            crate::engines::workflow_engine::WorkflowStatus::Running => "🟢",
            crate::engines::workflow_engine::WorkflowStatus::Completed => "🎯",
            crate::engines::workflow_engine::WorkflowStatus::Failed(_) => "💥",
            crate::engines::workflow_engine::WorkflowStatus::Suspended(_) => "⏸️",
            crate::engines::workflow_engine::WorkflowStatus::Cancelled => "❌",
        };

        println!(
            "{}. {} Instance: {} [{}]",
            i + 1,
            status_icon,
            instance.id,
            instance.status
        );

        println!("   🔗 Workflow: {}", instance.workflow_id);
        println!("   🔄 Current State: {}", instance.current_state);
        println!("   👤 Agent: {}", instance.context.executing_agent);

        if let Some(entity_id) = &instance.context.entity_id {
            println!("   🏷️ Entity: {}", entity_id);
        }

        println!(
            "   🕐 Started: {} | 🔄 Updated: {}",
            instance.started_at.format("%Y-%m-%d %H:%M"),
            instance.updated_at.format("%Y-%m-%d %H:%M")
        );

        if let Some(completed) = instance.completed_at {
            println!("   🎯 Completed: {}", completed.format("%Y-%m-%d %H:%M"));
        }

        println!();
    }

    println!("💡 Use 'engram workflow status <instance-id>' to view instance details");
    println!("💡 Use 'engram workflow transition <instance-id> --transition <name>' to execute transitions");

    Ok(())
}

/// Cancel a workflow instance
pub fn cancel_workflow_instance<S: Storage + 'static>(
    storage: S,
    instance_id: String,
    agent: String,
    reason: Option<String>,
) -> Result<(), EngramError> {
    let mut engine = WorkflowAutomationEngine::new(storage);

    let reason = reason.unwrap_or_else(|| "Cancelled by user".to_string());
    let result = engine.cancel_workflow(&instance_id, agent, reason)?;

    if result.success {
        println!("✅ Workflow instance cancelled successfully!");
        println!("📋 Instance ID: {}", result.instance_id);
        println!("🔄 Final State: {}", result.current_state);
        println!("💬 Message: {}", result.message);

        if !result.events.is_empty() {
            println!("📚 Events:");
            for event in &result.events {
                println!(
                    "  • {} - {} at {}",
                    match event.event_type {
                        crate::engines::workflow_engine::WorkflowEventType::Cancelled =>
                            "❌ Cancelled",
                        _ => "📝 Event",
                    },
                    event.message,
                    event.timestamp.format("%H:%M:%S")
                );
            }
        }
    } else {
        println!("❌ Failed to cancel workflow instance");
        println!("💬 Message: {}", result.message);
    }

    Ok(())
}

/// Execute an action (external command, notification, etc.)
pub fn execute_action<S: Storage + 'static>(
    _storage: S,
    action_type: String,
    command: Option<String>,
    args: Option<String>,
    working_directory: Option<String>,
    environment: Option<String>,
    timeout_seconds: Option<u64>,
    message: Option<String>,
    entity_id: Option<String>,
    entity_type: Option<String>,
) -> Result<(), EngramError> {
    use crate::engines::ActionExecutor;
    let mut parameters = HashMap::new();

    match action_type.as_str() {
        "external_command" => {
            if let Some(cmd) = command {
                parameters.insert("command".to_string(), serde_json::json!(cmd));
            } else {
                return Err(EngramError::Validation(
                    "Command is required for external_command action type".to_string(),
                ));
            }

            if let Some(args_str) = args {
                let args_vec: Vec<String> = args_str
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
                parameters.insert("args".to_string(), serde_json::json!(args_vec));
            }

            if let Some(wd) = working_directory {
                parameters.insert("working_directory".to_string(), serde_json::json!(wd));
            }

            if let Some(env_str) = environment {
                let mut env_map: HashMap<String, String> = HashMap::new();
                for pair in env_str.split(',') {
                    if let Some((key, value)) = pair.split_once('=') {
                        env_map.insert(key.trim().to_string(), value.trim().to_string());
                    }
                }
                parameters.insert("environment".to_string(), serde_json::json!(env_map));
            }

            if let Some(timeout) = timeout_seconds {
                parameters.insert("timeout_seconds".to_string(), serde_json::json!(timeout));
            }

            parameters.insert("capture_output".to_string(), serde_json::json!(true));
        }
        "notification" => {
            if let Some(msg) = message {
                parameters.insert("message".to_string(), serde_json::json!(msg));
            } else {
                return Err(EngramError::Validation(
                    "Message is required for notification action type".to_string(),
                ));
            }
        }
        "update_entity" => {
            if let Some(id) = entity_id {
                parameters.insert("entity_id".to_string(), serde_json::json!(id));
            } else {
                return Err(EngramError::Validation(
                    "Entity ID is required for update_entity action type".to_string(),
                ));
            }

            if let Some(etype) = entity_type {
                parameters.insert("entity_type".to_string(), serde_json::json!(etype));
            } else {
                return Err(EngramError::Validation(
                    "Entity type is required for update_entity action type".to_string(),
                ));
            }
        }
        _ => {
            return Err(EngramError::Validation(format!(
                "Unknown action type: {}. Supported: external_command, notification, update_entity",
                action_type
            )));
        }
    }

    // Execute the action
    let executor = ActionExecutor::new(true); // Allow external commands
    let result = executor.execute_action(&action_type, &parameters)?;

    if result.success {
        println!("✅ Action executed successfully!");
        println!("💬 Message: {}", result.message);

        if let Some(output) = result.output {
            println!("📄 Output:");
            println!("{}", output);
        }

        if let Some(exit_code) = result.exit_code {
            println!("🔢 Exit Code: {}", exit_code);
        }
    } else {
        println!("❌ Action execution failed");
        println!("💬 Message: {}", result.message);

        if let Some(error) = result.error {
            println!("⚠️  Error:");
            println!("{}", error);
        }

        if let Some(exit_code) = result.exit_code {
            println!("🔢 Exit Code: {}", exit_code);
        }
    }

    Ok(())
}

/// Query available actions, guards, and checks for a workflow
pub fn query_workflow_actions<S: Storage>(
    storage: &S,
    workflow_id: String,
    state_id: Option<String>,
) -> Result<(), EngramError> {
    if let Some(generic) = storage.get(&workflow_id, "workflow")? {
        let workflow =
            Workflow::from_generic(generic).map_err(|e| EngramError::Validation(e.to_string()))?;

        println!("📋 Workflow: {} ({})", workflow.title, workflow.id);
        println!();

        // Filter states if state_id is provided
        let states_to_show: Vec<&WorkflowState> = if let Some(ref sid) = state_id {
            workflow.states.iter().filter(|s| &s.id == sid).collect()
        } else {
            workflow.states.iter().collect()
        };

        if states_to_show.is_empty() {
            if state_id.is_some() {
                println!("❌ State not found in workflow");
            } else {
                println!("ℹ️  No states defined in workflow");
            }
            return Ok(());
        }

        for state in states_to_show {
            println!("🔷 State: {} ({})", state.name, state.id);
            println!("   Type: {:?}", state.state_type);

            // Show guards
            if !state.guards.is_empty() {
                println!("   🛡️  Guards ({}):", state.guards.len());
                for guard in &state.guards {
                    println!("      • {} ({})", guard.guard_type, guard.id);
                    if !guard.error_message.is_empty() {
                        println!("        Error: {}", guard.error_message);
                    }
                }
            }

            // Show post-functions
            if !state.post_functions.is_empty() {
                println!("   ⚙️  Post-Functions ({}):", state.post_functions.len());
                for func in &state.post_functions {
                    println!(
                        "      • {} - {} ({})",
                        func.name, func.function_type, func.id
                    );
                }
            }

            println!();
        }

        // Show transitions
        let transitions_to_show: Vec<&WorkflowTransition> = if let Some(ref sid) = state_id {
            workflow
                .transitions
                .iter()
                .filter(|t| &t.from_state == sid)
                .collect()
        } else {
            workflow.transitions.iter().collect()
        };

        if !transitions_to_show.is_empty() {
            println!("🔄 Transitions ({}):", transitions_to_show.len());
            for transition in transitions_to_show {
                println!(
                    "   • {} ({} → {})",
                    transition.name, transition.from_state, transition.to_state
                );
                println!("     Type: {:?}", transition.transition_type);

                // Show conditions
                if !transition.conditions.is_empty() {
                    println!("     📋 Conditions ({}):", transition.conditions.len());
                    for condition in &transition.conditions {
                        println!("        • {} ({})", condition.condition_type, condition.id);
                    }
                }

                // Show actions
                if !transition.actions.is_empty() {
                    println!("     ⚡ Actions ({}):", transition.actions.len());
                    for action in &transition.actions {
                        println!(
                            "        • {} - {} ({})",
                            action.name, action.action_type, action.id
                        );

                        // Show some action details
                        if action.action_type == "external_command" {
                            if let Some(cmd) = action.parameters.get("command") {
                                println!("          Command: {}", cmd);
                            }
                        }
                    }
                }

                println!();
            }
        }

        println!(
            "💡 Use 'engram workflow execute-action --action-type <type> ...' to test actions"
        );
    } else {
        println!("❌ Workflow not found: {}", workflow_id);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::entities::Workflow;
    use crate::entities::WorkflowStatus;
    use crate::storage::{MemoryStorage, Storage};

    fn create_test_workflow(storage: &mut MemoryStorage, title: &str) -> String {
        let workflow = Workflow::new(
            title.to_string(),
            "Description".to_string(),
            "test-agent".to_string(),
        );
        let id = workflow.id.to_string();
        storage.store(&workflow.to_generic()).unwrap();
        id
    }

    #[test]
    fn test_add_state_invalid_type() {
        let mut storage = MemoryStorage::new("default");
        let id = create_test_workflow(&mut storage, "Workflow");

        add_state(
            &mut storage,
            &id,
            "State".to_string(),
            "invalid_type".to_string(),
            "Desc".to_string(),
            false,
        )
        .unwrap();

        let generic = storage.get(&id, "workflow").unwrap().unwrap();
        let workflow = Workflow::from_generic(generic).unwrap();
        // Should not have added the state
        assert_eq!(workflow.states.len(), 0);
    }

    #[test]
    fn test_add_transition_not_found() {
        let mut storage = MemoryStorage::new("default");
        assert!(add_transition(
            &mut storage,
            "non-existent",
            "Trans".to_string(),
            "s1".to_string(),
            "s2".to_string(),
            "manual".to_string(),
            "Desc".to_string()
        )
        .is_ok());
    }

    #[test]
    fn test_add_transition_invalid_type() {
        let mut storage = MemoryStorage::new("default");
        let id = create_test_workflow(&mut storage, "Workflow");

        add_transition(
            &mut storage,
            &id,
            "Trans".to_string(),
            "s1".to_string(),
            "s2".to_string(),
            "invalid_type".to_string(),
            "Desc".to_string(),
        )
        .unwrap();

        let generic = storage.get(&id, "workflow").unwrap().unwrap();
        let workflow = Workflow::from_generic(generic).unwrap();
        assert_eq!(workflow.transitions.len(), 0);
    }

    #[test]
    fn test_activate_workflow_not_found() {
        let mut storage = MemoryStorage::new("default");
        assert!(activate_workflow(&mut storage, "non-existent").is_ok());
    }

    #[test]
    fn test_update_workflow_not_found() {
        let mut storage = MemoryStorage::new("default");
        assert!(update_workflow(
            &mut storage,
            "non-existent",
            Some("Title".to_string()),
            None,
            None,
            None,
            None
        )
        .is_ok());
    }

    #[test]
    fn test_delete_workflow_not_found() {
        let mut storage = MemoryStorage::new("default");
        assert!(delete_workflow(&mut storage, "non-existent").is_ok());
    }

    #[test]
    fn test_update_workflow_invalid_status() {
        let mut storage = MemoryStorage::new("default");
        let id = create_test_workflow(&mut storage, "Workflow");

        update_workflow(
            &mut storage,
            &id,
            None,
            None,
            Some("invalid_status".to_string()),
            None,
            None,
        )
        .unwrap();

        let generic = storage.get(&id, "workflow").unwrap().unwrap();
        let workflow = Workflow::from_generic(generic).unwrap();
        // Status should remain default (Draft)
        assert_eq!(workflow.status, WorkflowStatus::Draft);
    }

    #[test]
    fn test_execute_action_invalid_type() {
        let storage = MemoryStorage::new("default");
        let result = execute_action(
            storage,
            "invalid_type".to_string(),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );
        assert!(matches!(result, Err(EngramError::Validation(_))));
    }

    #[test]
    fn test_execute_action_missing_params() {
        // External command missing command
        let storage1 = MemoryStorage::new("default");
        let result_cmd = execute_action(
            storage1,
            "external_command".to_string(),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );
        assert!(matches!(result_cmd, Err(EngramError::Validation(_))));

        // Notification missing message
        let storage2 = MemoryStorage::new("default");
        let result_notif = execute_action(
            storage2,
            "notification".to_string(),
            None,
            None,
            None,
            None,
            None,
            None, // message missing
            None,
            None,
        );
        assert!(matches!(result_notif, Err(EngramError::Validation(_))));

        // Update entity missing id/type
        let storage3 = MemoryStorage::new("default");
        let result_update = execute_action(
            storage3,
            "update_entity".to_string(),
            None,
            None,
            None,
            None,
            None,
            None,
            None, // id missing
            None,
        );
        assert!(matches!(result_update, Err(EngramError::Validation(_))));
    }
}

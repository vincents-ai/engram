use crate::engines::rule_engine::{RuleExecutionEngine, RuleValue};
use crate::engines::workflow_engine::WorkflowAutomationEngine;
use crate::entities::{
    Entity, StateType, TransitionType, Workflow, WorkflowState, WorkflowStatus, WorkflowTransition,
};
use crate::error::EngramError;
use crate::storage::Storage;
use clap::{Parser, Subcommand};
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
        let workflow = Workflow::from_generic(generic).map_err(|e| EngramError::Validation(e))?;
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
            Workflow::from_generic(generic).map_err(|e| EngramError::Validation(e))?;

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
            Workflow::from_generic(generic).map_err(|e| EngramError::Validation(e))?;
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
            Workflow::from_generic(generic).map_err(|e| EngramError::Validation(e))?;

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
            Workflow::from_generic(generic).map_err(|e| EngramError::Validation(e))?;

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
            Workflow::from_generic(generic).map_err(|e| EngramError::Validation(e))?;
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
) -> Result<(), EngramError> {
    let mut engine = WorkflowAutomationEngine::new(storage);

    let mut initial_variables = HashMap::new();
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
) -> Result<(), EngramError> {
    let mut engine = WorkflowAutomationEngine::new(storage);

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

/// Parse workflow variables from string format (key=value,key2=value2)
fn parse_workflow_variables(variables_str: &str) -> HashMap<String, RuleValue> {
    let mut variables = HashMap::new();

    for pair in variables_str.split(',') {
        if let Some((key, value)) = pair.split_once('=') {
            let key = key.trim().to_string();
            let value_str = value.trim();

            // Try to parse as different types
            let rule_value = if value_str == "true" || value_str == "false" {
                RuleValue::Boolean(value_str.parse().unwrap())
            } else if let Ok(num) = value_str.parse::<f64>() {
                RuleValue::Number(num)
            } else {
                RuleValue::String(value_str.to_string())
            };

            variables.insert(key, rule_value);
        }
    }

    variables
}

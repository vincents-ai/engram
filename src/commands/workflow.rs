//! Workflow command handler

use engram::cli;
use engram::error::EngramError;
use engram::storage::GitRefsStorage;

/// Handle workflow commands
pub fn handle_workflow_command<S: engram::storage::Storage>(
    command: engram::cli::WorkflowCommands,
    storage: &mut S,
) -> Result<(), EngramError> {
    match command {
        cli::WorkflowCommands::Create {
            title,
            description,
            entity_types,
            agent,
        } => {
            cli::create_workflow(storage, title, description, entity_types, agent)?;
        }
        cli::WorkflowCommands::Get { id } => {
            cli::get_workflow(storage, &id)?;
        }
        cli::WorkflowCommands::Update {
            id,
            title,
            description,
            status,
            entity_types,
            initial_state,
        } => {
            cli::update_workflow(
                storage,
                &id,
                title,
                description,
                status,
                entity_types,
                initial_state,
            )?;
        }
        cli::WorkflowCommands::Delete { id } => {
            cli::delete_workflow(storage, &id)?;
        }
        cli::WorkflowCommands::List {
            status,
            search,
            limit,
            offset,
            all,
            output,
        } => {
            cli::list_workflows(
                &mut std::io::stdout(),
                storage,
                status,
                search,
                limit,
                offset,
                all,
                &output,
            )?;
        }
        cli::WorkflowCommands::AddState {
            id,
            name,
            state_type,
            description,
            is_final,
        } => {
            cli::add_state(storage, &id, name, state_type, description, is_final)?;
        }
        cli::WorkflowCommands::AddTransition {
            id,
            name,
            from_state,
            to_state,
            transition_type,
            description,
        } => {
            cli::add_transition(
                storage,
                &id,
                name,
                from_state,
                to_state,
                transition_type,
                description,
            )?;
        }
        cli::WorkflowCommands::Activate { id } => {
            cli::activate_workflow(storage, &id)?;
        }
        cli::WorkflowCommands::Start {
            workflow_id,
            entity_id,
            entity_type,
            agent,
            variables,
            context_file,
        } => {
            let storage_for_workflow = GitRefsStorage::new(".", "default")?;
            cli::start_workflow_instance(
                storage_for_workflow,
                workflow_id,
                entity_id,
                entity_type,
                agent,
                variables,
                context_file,
            )?;
        }
        cli::WorkflowCommands::Transition {
            instance_id,
            transition,
            agent,
            context_file,
        } => {
            let storage_for_workflow = GitRefsStorage::new(".", "default")?;
            cli::execute_workflow_transition(
                storage_for_workflow,
                instance_id,
                transition,
                agent,
                context_file,
            )?;
        }
        cli::WorkflowCommands::Status { instance_id } => {
            let storage_for_workflow = GitRefsStorage::new(".", "default")?;
            cli::get_workflow_instance_status(storage_for_workflow, instance_id)?;
        }
        cli::WorkflowCommands::Instances {
            workflow_id,
            agent,
            running_only,
        } => {
            let storage_for_workflow = GitRefsStorage::new(".", "default")?;
            cli::list_workflow_instances(storage_for_workflow, workflow_id, agent, running_only)?;
        }
        cli::WorkflowCommands::Cancel {
            instance_id,
            agent,
            reason,
        } => {
            let storage_for_workflow = GitRefsStorage::new(".", "default")?;
            cli::cancel_workflow_instance(storage_for_workflow, instance_id, agent, reason)?;
        }
        cli::WorkflowCommands::ExecuteAction {
            action_type,
            command,
            args,
            working_directory,
            environment,
            timeout_seconds,
            message,
            entity_id,
            entity_type,
        } => {
            let storage_for_workflow = GitRefsStorage::new(".", "default")?;
            cli::execute_action(
                storage_for_workflow,
                action_type,
                command,
                args,
                working_directory,
                environment,
                timeout_seconds,
                message,
                entity_id,
                entity_type,
            )?;
        }
        cli::WorkflowCommands::QueryActions {
            workflow_id,
            state_id,
        } => {
            cli::query_workflow_actions(storage, workflow_id, state_id)?;
        }
    }
    Ok(())
}

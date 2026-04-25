//! Task command handler

use engram::cli;
use engram::error::EngramError;

pub fn handle_task_command<
    S: engram::storage::Storage + engram::storage::RelationshipStorage + 'static,
>(
    command: cli::TaskCommands,
    storage: &mut S,
) -> Result<(), EngramError> {
    match command {
        cli::TaskCommands::Create {
            title,
            description,
            priority,
            agent,
            parent,
            tags,
            output,
            title_stdin,
            title_file,
            description_stdin,
            description_file,
            json,
            json_file,
        } => {
            cli::create_task(
                storage,
                title,
                description,
                &priority,
                agent,
                parent,
                tags,
                title_stdin,
                title_file,
                description_stdin,
                description_file,
                json,
                json_file,
                output,
            )?;
        }
        cli::TaskCommands::List {
            agent,
            status,
            workflow_instance_id,
            workflow_state,
            limit,
            all,
            offset,
            stale,
            stale_threshold,
            output,
        } => {
            cli::list_tasks(
                storage,
                agent.as_deref(),
                status.as_deref(),
                workflow_instance_id.as_deref(),
                workflow_state.as_deref(),
                limit,
                all,
                offset,
                stale,
                stale_threshold,
                &output,
            )?;
        }
        cli::TaskCommands::Show { id } => {
            cli::show_task(storage, &id)?;
        }
        cli::TaskCommands::Update {
            id,
            status,
            outcome,
            reason,
        } => {
            cli::update_task(storage, &id, &status, outcome.as_deref(), reason.as_deref())?;
        }
        cli::TaskCommands::Archive { id, reason } => {
            cli::archive_task(storage, &id, reason.as_deref())?;
        }
        cli::TaskCommands::ArchiveBulk {
            older_than,
            status,
            dry_run,
            output,
        } => {
            cli::archive_tasks_bulk(storage, older_than, status.as_deref(), dry_run, &output)?;
        }
        cli::TaskCommands::Resolve { id, message } => {
            cli::resolve_task(storage, &id, message.as_deref())?;
        }
        cli::TaskCommands::CreateBatch {
            file,
            json,
            titles_file,
            parent,
            priority,
            agent,
            output,
            no_fail_fast,
        } => {
            cli::create_task_batch(
                storage,
                file,
                json,
                titles_file,
                parent,
                &priority,
                agent,
                &output,
                no_fail_fast,
            )?;
        }
    }
    Ok(())
}

//! Reasoning command handler

use engram::cli;
use engram::error::EngramError;

/// Handle reasoning commands
pub fn handle_reasoning_command<
    S: engram::storage::Storage + engram::storage::RelationshipStorage,
>(
    command: engram::cli::ReasoningCommands,
    storage: &mut S,
) -> Result<(), EngramError> {
    match command {
        cli::ReasoningCommands::Create {
            title,
            task_id,
            agent,
            confidence,
            content,
            tags,
            title_stdin,
            title_file,
            content_stdin,
            content_file,
            json,
            json_file,
            supersedes,
        } => {
            cli::create_reasoning(
                storage,
                title,
                task_id,
                agent,
                confidence,
                content,
                tags,
                title_stdin,
                title_file,
                content_stdin,
                content_file,
                json,
                json_file,
                supersedes,
            )?;
        }
        cli::ReasoningCommands::AddStep {
            id,
            description,
            conclusion,
            confidence,
            description_stdin,
            description_file,
            conclusion_stdin,
            conclusion_file,
            ibis_type,
            ibis_polarity,
            parent_step,
        } => {
            cli::add_reasoning_step(
                storage,
                &id,
                description,
                conclusion,
                confidence,
                description_stdin,
                description_file,
                conclusion_stdin,
                conclusion_file,
                ibis_type,
                ibis_polarity,
                parent_step,
            )?;
        }
        cli::ReasoningCommands::Conclude {
            id,
            conclusion,
            confidence,
            conclusion_stdin,
            conclusion_file,
        } => {
            cli::conclude_reasoning(
                storage,
                &id,
                conclusion,
                confidence,
                conclusion_stdin,
                conclusion_file,
            )?;
        }
        cli::ReasoningCommands::List {
            agent,
            task_id,
            tags,
            limit,
            all,
            offset,
        } => {
            cli::list_reasoning(
                storage,
                agent.as_deref(),
                task_id.as_deref(),
                tags,
                limit,
                all,
                offset,
            )?;
        }
        cli::ReasoningCommands::Show { id } => {
            cli::show_reasoning(storage, &id)?;
        }
        cli::ReasoningCommands::Delete { id } => {
            cli::delete_reasoning(storage, &id)?;
        }
        cli::ReasoningCommands::History { id } => {
            cli::show_reasoning_history(storage, &id)?;
        }
        cli::ReasoningCommands::Export { id, format } => {
            cli::export_reasoning(storage, &id, &format)?;
        }
        cli::ReasoningCommands::Log {
            reasoning_id,
            event_type,
            content,
        } => {
            cli::log_reasoning_event(storage, &reasoning_id, event_type, content)?;
        }
        cli::ReasoningCommands::Search {
            ibis_type,
            polarity,
            keyword,
            agent,
            task_id,
            limit,
            all,
        } => {
            cli::search_reasoning(
                storage,
                ibis_type,
                polarity,
                keyword,
                agent.as_deref(),
                task_id.as_deref(),
                limit,
                all,
            )?;
        }
    }
    Ok(())
}

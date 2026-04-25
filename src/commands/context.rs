//! Context command handler

use engram::cli;
use engram::error::EngramError;

/// Handle context commands
pub fn handle_context_command<S: engram::storage::Storage>(
    command: engram::cli::ContextCommands,
    storage: &mut S,
) -> Result<(), EngramError> {
    match command {
        cli::ContextCommands::Create {
            title,
            content,
            source,
            relevance,
            source_id,
            agent,
            tags,
            title_stdin,
            title_file,
            content_stdin,
            content_file,
            json,
            json_file,
        } => {
            cli::create_context(
                storage,
                title,
                content,
                source,
                &relevance,
                source_id,
                agent,
                tags,
                title_stdin,
                title_file,
                content_stdin,
                content_file,
                json,
                json_file,
            )?;
        }
        cli::ContextCommands::List {
            agent,
            relevance,
            tags,
            limit,
            all,
            offset,
            output,
        } => {
            cli::list_contexts(
                storage,
                agent.as_deref(),
                relevance.as_deref(),
                tags,
                limit,
                all,
                offset,
                output,
            )?;
        }
        cli::ContextCommands::Show { id } => {
            cli::show_context(storage, &id)?;
        }
        cli::ContextCommands::Update { id, content } => {
            cli::update_context(storage, &id, &content)?;
        }
        cli::ContextCommands::Delete { id } => {
            cli::delete_context(storage, &id)?;
        }
    }
    Ok(())
}

//! Knowledge and lesson command handlers

use engram::cli;
use engram::error::EngramError;

/// Handle knowledge commands
pub fn handle_knowledge_command<S: engram::storage::Storage>(
    command: engram::cli::KnowledgeCommands,
    storage: &mut S,
) -> Result<(), EngramError> {
    match command {
        cli::KnowledgeCommands::Create {
            title,
            content,
            knowledge_type,
            confidence,
            source,
            agent,
            tags,
            title_stdin,
            title_file,
            content_stdin,
            content_file,
            json,
            json_file,
        } => {
            cli::create_knowledge(
                storage,
                title,
                content,
                knowledge_type,
                confidence,
                source,
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
        cli::KnowledgeCommands::List {
            agent,
            kind,
            tags,
            limit,
            all,
            offset,
            output,
        } => {
            cli::list_knowledge(storage, agent, kind, tags, limit, all, offset, output)?;
        }
        cli::KnowledgeCommands::Show { id } => {
            cli::show_knowledge(storage, &id)?;
        }
        cli::KnowledgeCommands::Update { id, field, value } => {
            cli::update_knowledge(storage, &id, &field, &value)?;
        }
        cli::KnowledgeCommands::Delete { id } => {
            cli::delete_knowledge(storage, &id)?;
        }
    }
    Ok(())
}

/// Handle lesson commands
pub fn handle_lesson_command<S: engram::storage::Storage>(
    command: engram::cli::LessonCommands,
    storage: &mut S,
) -> Result<(), EngramError> {
    match command {
        cli::LessonCommands::Create {
            title,
            mistake,
            correction,
            prevention_rule,
            domain,
            category,
            severity,
            agent,
            tags,
        } => {
            cli::create_lesson(
                storage,
                title,
                mistake,
                correction,
                prevention_rule,
                domain,
                category,
                severity,
                agent,
                tags,
            )?;
        }
        cli::LessonCommands::List {
            agent,
            category,
            domain,
            severity,
            tags,
            limit,
            all,
            offset,
        } => {
            cli::list_lessons(
                storage, agent, category, domain, severity, tags, limit, all, offset,
            )?;
        }
        cli::LessonCommands::Show { id } => {
            cli::show_lesson(storage, &id)?;
        }
        cli::LessonCommands::Update {
            id,
            mistake,
            correction,
            prevention_rule,
            add_tag,
        } => {
            cli::update_lesson(storage, &id, mistake, correction, prevention_rule, add_tag)?;
        }
        cli::LessonCommands::Delete { id } => {
            cli::delete_lesson(storage, &id)?;
        }
    }
    Ok(())
}

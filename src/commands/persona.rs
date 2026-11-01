//! Persona command handler

use engram::cli;
use engram::error::EngramError;

/// Handle persona commands
pub fn handle_persona_command<S: engram::storage::Storage>(
    command: engram::cli::PersonaCommands,
    storage: &mut S,
) -> Result<(), EngramError> {
    match command {
        cli::PersonaCommands::Create {
            slug,
            title,
            description,
            instructions,
            domain,
            base_persona,
            agent,
            tags,
            cov_questions,
            fap_entries,
            ov_requirements,
        } => {
            cli::create_persona(
                storage,
                slug,
                title,
                description,
                instructions,
                domain,
                base_persona,
                agent,
                tags,
                cov_questions,
                fap_entries,
                ov_requirements,
            )?;
        }
        cli::PersonaCommands::List {
            agent,
            domain,
            tag,
            limit,
            all,
            offset,
            output,
        } => {
            cli::list_personas(storage, agent, domain, tag, limit, all, offset, &output)?;
        }
        cli::PersonaCommands::Show { id } => {
            cli::show_persona(storage, &id)?;
        }
        cli::PersonaCommands::Update {
            id,
            title,
            description,
            instructions,
            domain,
            add_tag,
            add_cov,
            add_ov,
            add_fap,
        } => {
            cli::update_persona(
                storage,
                &id,
                title,
                description,
                instructions,
                domain,
                add_tag,
                add_cov,
                add_ov,
                add_fap,
            )?;
        }
        cli::PersonaCommands::Delete { id } => {
            cli::delete_persona(storage, &id)?;
        }
        cli::PersonaCommands::Submit {
            id,
            submit_type,
            repo,
            message,
        } => {
            cli::submit_persona(storage, &id, submit_type, repo, message)?;
        }
    }
    Ok(())
}

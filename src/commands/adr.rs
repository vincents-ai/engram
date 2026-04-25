//! ADR (Architectural Decision Record) command handler

use engram::cli;
use engram::error::EngramError;

/// Handle ADR commands
pub fn handle_adr_command<S: engram::storage::Storage>(
    command: engram::cli::AdrCommands,
    storage: &mut S,
) -> Result<(), EngramError> {
    match command {
        cli::AdrCommands::Create {
            title,
            number,
            context,
            agent,
        } => {
            cli::create_adr(storage, title, number, context, agent)?;
        }
        cli::AdrCommands::Get { id } => {
            cli::get_adr(storage, &id)?;
        }
        cli::AdrCommands::Update {
            id,
            title,
            status,
            context,
            decision,
            consequences,
            implementation,
            superseded_by,
        } => {
            cli::update_adr(
                storage,
                &id,
                title,
                status,
                context,
                decision,
                consequences,
                implementation,
                superseded_by,
            )?;
        }
        cli::AdrCommands::Delete { id } => {
            cli::delete_adr(storage, &id)?;
        }
        cli::AdrCommands::List {
            status,
            search,
            limit,
            offset,
            all,
        } => {
            cli::list_adrs(storage, status, search, limit, offset, all)?;
        }
        cli::AdrCommands::Accept {
            id,
            decision,
            consequences,
        } => {
            cli::accept_adr(storage, &id, decision, consequences)?;
        }
        cli::AdrCommands::AddAlternative { id, description } => {
            cli::add_alternative(storage, &id, description)?;
        }
        cli::AdrCommands::AddStakeholder { id, stakeholder } => {
            cli::add_stakeholder(storage, &id, stakeholder)?;
        }
    }
    Ok(())
}

//! Sandbox command handler

use engram::cli::sandbox::*;
use engram::error::EngramError;

/// Handle sandbox commands
pub fn handle_sandbox_command<S: engram::storage::Storage>(
    command: engram::cli::SandboxCommands,
    storage: &mut S,
) -> Result<(), EngramError> {
    match command {
        engram::cli::SandboxCommands::Create {
            agent,
            level,
            created_by,
            stdin,
            file,
            json,
        } => {
            create_sandbox(storage, agent, level, created_by, stdin, file, json)?;
        }
        engram::cli::SandboxCommands::List {
            agent_id,
            level,
            agent,
            json,
        } => {
            list_sandboxes(storage, agent_id, level, agent, json)?;
        }
        engram::cli::SandboxCommands::Get { id, json } => {
            get_sandbox(storage, id, json)?;
        }
        engram::cli::SandboxCommands::Update {
            id,
            level,
            stdin,
            file,
            json,
        } => {
            update_sandbox(storage, id, level, stdin, file, json)?;
        }
        engram::cli::SandboxCommands::Delete { id, force } => {
            delete_sandbox(storage, id, force)?;
        }
        engram::cli::SandboxCommands::Validate {
            agent_id,
            operation,
            resource_type,
            stdin,
            file,
            json,
        } => {
            validate_operation(
                storage,
                agent_id,
                operation,
                resource_type,
                stdin,
                file,
                json,
            )?;
        }
        engram::cli::SandboxCommands::Stats { agent_id, json } => {
            show_stats(storage, agent_id, json)?;
        }
        engram::cli::SandboxCommands::Check { json } => {
            check_preflight(json)?;
        }
        engram::cli::SandboxCommands::Reset {
            agent_id,
            force,
            json,
        } => {
            reset_sandbox(storage, agent_id, force, json)?;
        }
    }

    Ok(())
}

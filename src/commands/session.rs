//! Session command handler

use engram::cli::session::*;
use engram::error::EngramError;

/// Handle session commands
pub fn handle_session_command<S: engram::storage::Storage>(
    command: engram::cli::SessionCommands,
    storage: &mut S,
) -> Result<(), EngramError> {
    match command {
        engram::cli::SessionCommands::Start { name, auto_detect } => {
            start_session(storage, name, auto_detect)?;
        }
        engram::cli::SessionCommands::Status { id, metrics } => {
            show_session_status(storage, id, metrics)?;
        }
        engram::cli::SessionCommands::End {
            id,
            generate_summary,
        } => {
            end_session(storage, id, generate_summary)?;
        }
        engram::cli::SessionCommands::List {
            agent,
            since,
            limit,
            all,
            offset,
        } => {
            list_sessions(
                &mut std::io::stdout(),
                storage,
                agent,
                since,
                limit,
                all,
                offset,
            )?;
        }
        engram::cli::SessionCommands::Zombies {
            max_age_hours,
            check_git,
        } => {
            detect_zombie_sessions(&mut std::io::stdout(), storage, max_age_hours, check_git)?;
        }
        engram::cli::SessionCommands::Summaries {
            agent,
            since,
            limit,
            all,
        } => {
            summarize_sessions(&mut std::io::stdout(), storage, agent, since, limit, all)?;
        }
    }

    Ok(())
}

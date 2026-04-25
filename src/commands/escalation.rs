//! Escalation command handler

use engram::cli::escalation::*;
use engram::error::EngramError;

/// Handle escalation commands
pub fn handle_escalation_command<S: engram::storage::Storage>(
    command: engram::cli::EscalationCommands,
    storage: &mut S,
) -> Result<(), EngramError> {
    match command {
        engram::cli::EscalationCommands::Create {
            agent,
            operation_type,
            operation,
            block_reason,
            justification,
            priority,
            impact,
            reviewer,
            stdin,
            file,
            json,
        } => {
            create_escalation(
                storage,
                agent,
                operation_type,
                operation,
                block_reason,
                justification,
                priority,
                impact,
                reviewer,
                stdin,
                file,
                json,
            )?;
        }
        engram::cli::EscalationCommands::List {
            agent_id,
            status,
            priority,
            operation_type,
            expired_only,
            actionable_only,
            agent,
            json,
        } => {
            list_escalations(
                storage,
                agent_id,
                status,
                priority,
                operation_type,
                expired_only,
                actionable_only,
                agent,
                json,
            )?;
        }
        engram::cli::EscalationCommands::Get { id, json } => {
            get_escalation(storage, id, json)?;
        }
        engram::cli::EscalationCommands::Review {
            id,
            status,
            reason,
            reviewer_id,
            reviewer_name,
            duration,
            create_policy,
            notes,
            stdin,
            file,
            json,
        } => {
            review_escalation(
                storage,
                id,
                status,
                reason,
                reviewer_id,
                reviewer_name,
                duration,
                create_policy,
                notes,
                stdin,
                file,
                json,
            )?;
        }
        engram::cli::EscalationCommands::Cancel {
            id,
            reason,
            force,
            json,
        } => {
            cancel_escalation(storage, id, reason, force, json)?;
        }
        engram::cli::EscalationCommands::Cleanup { apply, json } => {
            cleanup_escalations(storage, apply, json)?;
        }
        engram::cli::EscalationCommands::Stats {
            agent_id,
            days,
            json,
        } => {
            show_escalation_stats(storage, agent_id, days, json)?;
        }
        engram::cli::EscalationCommands::Approve {
            id,
            reason,
            reviewer_id,
            reviewer_name,
            duration,
            json,
        } => {
            approve_escalation(
                storage,
                id,
                reason,
                reviewer_id,
                reviewer_name,
                duration,
                json,
            )?;
        }
        engram::cli::EscalationCommands::Deny {
            id,
            reason,
            reviewer_id,
            reviewer_name,
            json,
        } => {
            deny_escalation(storage, id, reason, reviewer_id, reviewer_name, json)?;
        }
    }

    Ok(())
}

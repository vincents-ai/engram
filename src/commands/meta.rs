//! Meta-cognitive command handlers: theory and reflection

use engram::cli;
use engram::cli::state_reflection::*;
use engram::cli::theory::*;
use engram::error::EngramError;

/// Handle theory commands (Naur, 1985 - Programming as Theory Building)
pub fn handle_theory_command<S: engram::storage::Storage + engram::storage::RelationshipStorage>(
    command: cli::TheoryCommands,
    storage: &mut S,
) -> Result<(), EngramError> {
    match command {
        cli::TheoryCommands::Create {
            domain,
            agent,
            task,
            json,
            json_file,
            supersedes,
        } => {
            create_theory(storage, domain, agent, task, json, json_file, supersedes)?;
        }
        cli::TheoryCommands::List {
            agent,
            domain,
            limit,
            all,
            offset,
        } => {
            list_theories(storage, agent, domain, limit, all, offset)?;
        }
        cli::TheoryCommands::Show { id, show_metrics } => {
            show_theory(storage, &id, show_metrics)?;
        }
        cli::TheoryCommands::Update {
            id,
            concept,
            mapping,
            rationale,
            invariant,
        } => {
            update_theory(storage, &id, concept, mapping, rationale, invariant)?;
        }
        cli::TheoryCommands::Delete { id } => {
            delete_theory(storage, &id)?;
        }
        cli::TheoryCommands::History { id } => {
            cli::show_theory_history(storage, &id)?;
        }
        cli::TheoryCommands::Decay {
            threshold,
            max_weight,
        } => {
            cli::list_stale_theories(storage, threshold, max_weight)?;
        }
        cli::TheoryCommands::ApplyReflection {
            theory_id,
            reflection_id,
            updates_file,
        } => {
            apply_reflection(storage, &theory_id, &reflection_id, &updates_file)?;
        }
    }

    Ok(())
}

/// Handle state reflection commands (Cognitive Dissonance Detection)
pub fn handle_reflection_command<S: engram::storage::Storage>(
    command: cli::StateReflectionCommands,
    storage: &mut S,
) -> Result<(), EngramError> {
    match command {
        cli::StateReflectionCommands::Create {
            theory,
            context,
            observed,
            trigger_type,
            loop_type,
            agent,
            json,
            json_file,
        } => {
            create_reflection(
                storage,
                Some(theory),
                Some(context),
                Some(observed),
                trigger_type,
                loop_type,
                agent,
                json,
                json_file,
            )?;
        }
        cli::StateReflectionCommands::List {
            theory,
            trigger_type,
            unresolved,
            limit,
            all,
            offset,
        } => {
            list_reflections(
                storage,
                theory,
                trigger_type,
                unresolved,
                limit,
                all,
                offset,
            )?;
        }
        cli::StateReflectionCommands::Show { id } => {
            show_reflection(storage, &id)?;
        }
        cli::StateReflectionCommands::RecordDissonance {
            id,
            description,
            score,
        } => {
            record_dissonance(storage, &id, &description, score)?;
        }
        cli::StateReflectionCommands::ProposeUpdate { id, update } => {
            propose_update(storage, &id, &update)?;
        }
        cli::StateReflectionCommands::Resolve { id, new_theory } => {
            resolve_reflection(storage, &id, &new_theory)?;
        }
        cli::StateReflectionCommands::Delete { id } => {
            delete_reflection(storage, &id)?;
        }
        cli::StateReflectionCommands::RequiresMutation { id, threshold } => {
            requires_mutation(storage, &id, threshold)?;
        }
    }

    Ok(())
}

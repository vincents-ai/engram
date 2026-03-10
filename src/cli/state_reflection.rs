//! State Reflection command implementations (Naur, 1985 - Cognitive Dissonance Detection)

use crate::entities::{Entity, StateReflection, TriggerType};
use crate::error::EngramError;
use crate::storage::Storage;
use clap::Subcommand;
use serde::Deserialize;
use std::fs;
use std::io::{self, Read};

/// State reflection input structure for JSON
#[derive(Debug, Deserialize)]
pub struct StateReflectionInput {
    pub theory_id: String,
    pub trigger_context_id: String,
    pub observed_state: String,
    #[serde(default)]
    pub trigger_type: Option<String>,
    #[serde(default)]
    pub cognitive_dissonance: Vec<String>,
    #[serde(default)]
    pub proposed_theory_updates: Vec<String>,
    #[serde(default)]
    pub dissonance_score: Option<f64>,
    #[serde(default)]
    pub agent: Option<String>,
}

/// State reflection commands
#[derive(Debug, Subcommand)]
pub enum StateReflectionCommands {
    /// Create a new state reflection
    Create {
        /// Theory ID being reflected upon
        #[arg(long, short)]
        theory: String,

        /// Trigger context ID (task or reasoning chain)
        #[arg(long)]
        context: String,

        /// Observed state (error, test output, etc.)
        #[arg(long, short)]
        observed: String,

        /// Trigger type (test_failure, runtime_error, unexpected_output, etc.)
        #[arg(long)]
        trigger_type: Option<String>,

        /// Agent name
        #[arg(long, short)]
        agent: Option<String>,

        /// Create from JSON input (stdin or file)
        #[arg(long, conflicts_with_all = ["theory", "context", "observed"])]
        json: bool,

        /// JSON file path (requires --json)
        #[arg(long, requires = "json")]
        json_file: Option<String>,
    },
    /// List state reflections
    List {
        /// Theory ID filter
        #[arg(long, short)]
        theory: Option<String>,

        /// Trigger type filter
        #[arg(long)]
        trigger_type: Option<String>,

        /// Show only unresolved
        #[arg(long)]
        unresolved: bool,

        /// Limit results
        #[arg(long, short)]
        limit: Option<usize>,
    },
    /// Show state reflection details
    Show {
        /// Reflection ID
        #[arg(long, short)]
        id: String,
    },
    /// Record cognitive dissonance in a reflection
    RecordDissonance {
        /// Reflection ID
        #[arg(long, short)]
        id: String,

        /// Dissonance description
        #[arg(long)]
        description: String,

        /// Impact score (0.0-1.0)
        #[arg(long)]
        score: f64,
    },
    /// Propose a theory update
    ProposeUpdate {
        /// Reflection ID
        #[arg(long, short)]
        id: String,

        /// Proposed update
        #[arg(long)]
        update: String,
    },
    /// Resolve a reflection (mark as resolved with new theory ID)
    Resolve {
        /// Reflection ID
        #[arg(long, short)]
        id: String,

        /// New theory ID (after applying updates)
        #[arg(long)]
        new_theory: String,
    },
    /// Delete state reflection
    Delete {
        /// Reflection ID
        #[arg(long, short)]
        id: String,
    },
    /// Check if theory mutation is required
    RequiresMutation {
        /// Reflection ID
        #[arg(long, short)]
        id: String,

        /// Threshold (default: 0.7)
        #[arg(long, default_value = "0.7")]
        threshold: f64,
    },
}

fn read_stdin() -> Result<String, EngramError> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer.trim().to_string())
}

fn read_file(path: &str) -> Result<String, EngramError> {
    fs::read_to_string(path)
        .map(|s| s.trim().to_string())
        .map_err(EngramError::Io)
}

fn parse_trigger_type(type_str: &str) -> Result<TriggerType, EngramError> {
    match type_str.to_lowercase().as_str() {
        "test_failure" | "testfailure" => Ok(TriggerType::TestFailure),
        "runtime_error" | "runtimeerror" => Ok(TriggerType::RuntimeError),
        "unexpected_output" | "unexpectedoutput" => Ok(TriggerType::UnexpectedOutput),
        "type_mismatch" | "typemismatch" => Ok(TriggerType::TypeMismatch),
        "behavioral_deviation" | "behavioraldeviation" => Ok(TriggerType::BehavioralDeviation),
        "manual_observation" | "manualobservation" => Ok(TriggerType::ManualObservation),
        "performance_anomaly" | "performanceanomaly" => Ok(TriggerType::PerformanceAnomaly),
        "security_concern" | "securityconcern" => Ok(TriggerType::SecurityConcern),
        _ => Err(EngramError::Validation(format!(
            "Invalid trigger type '{}'. Must be one of: test_failure, runtime_error, unexpected_output, type_mismatch, behavioral_deviation, manual_observation, performance_anomaly, security_concern",
            type_str
        ))),
    }
}

/// Create a new state reflection
#[allow(clippy::too_many_arguments)]
pub fn create_reflection<S: Storage>(
    storage: &mut S,
    theory_id: Option<String>,
    trigger_context_id: Option<String>,
    observed_state: Option<String>,
    trigger_type: Option<String>,
    agent: Option<String>,
    json: bool,
    json_file: Option<String>,
) -> Result<(), EngramError> {
    let reflection = if json {
        let json_str = if let Some(file) = json_file {
            read_file(&file)?
        } else {
            read_stdin()?
        };

        let input: StateReflectionInput = serde_json::from_str(&json_str)
            .map_err(|e| EngramError::Validation(format!("Invalid JSON: {}", e)))?;

        let agent_name = input.agent.unwrap_or_else(|| "default".to_string());
        let mut reflection = StateReflection::new(
            input.theory_id,
            input.trigger_context_id,
            input.observed_state,
            agent_name,
        );

        if let Some(tt) = input.trigger_type {
            reflection.trigger_type = Some(parse_trigger_type(&tt)?);
        }

        for dissonance in input.cognitive_dissonance {
            reflection.cognitive_dissonance.push(dissonance);
        }

        for update in input.proposed_theory_updates {
            reflection.proposed_theory_updates.push(update);
        }

        if let Some(score) = input.dissonance_score {
            reflection.dissonance_score = score;
        }

        reflection
    } else {
        let theory = theory_id
            .ok_or_else(|| EngramError::Validation("Theory ID is required".to_string()))?;
        let context = trigger_context_id
            .ok_or_else(|| EngramError::Validation("Context ID is required".to_string()))?;
        let observed = observed_state
            .ok_or_else(|| EngramError::Validation("Observed state is required".to_string()))?;
        let agent_name = agent.unwrap_or_else(|| "default".to_string());

        let mut reflection = StateReflection::new(theory, context, observed, agent_name);

        if let Some(tt) = trigger_type {
            reflection.trigger_type = Some(parse_trigger_type(&tt)?);
        }

        reflection
    };

    reflection.validate_entity()?;
    let generic = reflection.to_generic();
    storage.store(&generic)?;

    println!(
        "State reflection created successfully with ID: {} (severity: {})",
        reflection.id,
        reflection.severity()
    );
    Ok(())
}

use crate::cli::utils::{create_table, truncate};
use prettytable::row;

/// List state reflections
pub fn list_reflections<S: Storage>(
    storage: &S,
    theory_id: Option<String>,
    trigger_type: Option<String>,
    unresolved_only: bool,
    limit: Option<usize>,
) -> Result<(), EngramError> {
    let ids = storage.list_ids(StateReflection::entity_type())?;

    let mut items: Vec<StateReflection> = Vec::new();

    for id in ids {
        if let Some(entity) = storage.get(&id, StateReflection::entity_type())? {
            if let Ok(reflection) = StateReflection::from_generic(entity) {
                if let Some(ref theory_filter) = theory_id {
                    if reflection.theory_id != *theory_filter {
                        continue;
                    }
                }

                if let Some(ref tt_filter) = trigger_type {
                    if let Some(ref tt) = reflection.trigger_type {
                        let tt_str = format!("{:?}", tt).to_lowercase();
                        if tt_str != tt_filter.to_lowercase() {
                            continue;
                        }
                    } else {
                        continue;
                    }
                }

                if unresolved_only && reflection.resolved == Some(true) {
                    continue;
                }

                items.push(reflection);
            }
        }
    }

    if let Some(limit_val) = limit {
        items.truncate(limit_val);
    }

    if items.is_empty() {
        println!("No state reflections found matching the criteria.");
        return Ok(());
    }

    let mut table = create_table();
    table.set_titles(row![
        "ID",
        "Theory",
        "Severity",
        "Dissonance",
        "Resolved",
        "Trigger",
        "Agent"
    ]);

    for reflection in items {
        let resolved_str = match reflection.resolved {
            Some(true) => "Yes",
            Some(false) => "No",
            None => "-",
        };
        let trigger_str = reflection
            .trigger_type
            .as_ref()
            .map(|t| format!("{:?}", t).to_lowercase())
            .unwrap_or_else(|| "-".to_string());

        table.add_row(row![
            &reflection.id[..8],
            &reflection.theory_id[..8],
            format!("{}", reflection.severity()),
            format!("{:.2}", reflection.dissonance_score),
            resolved_str,
            truncate(&trigger_str, 15),
            truncate(&reflection.agent, 12)
        ]);
    }

    table.printstd();
    Ok(())
}

/// Show state reflection details
pub fn show_reflection<S: Storage>(storage: &S, id: &str) -> Result<(), EngramError> {
    let entity = storage
        .get(id, StateReflection::entity_type())?
        .ok_or_else(|| EngramError::NotFound(format!("State reflection not found: {}", id)))?;

    let reflection = StateReflection::from_generic(entity)
        .map_err(|e: EngramError| EngramError::Validation(e.to_string()))?;

    println!("State Reflection Details:");
    println!("=========================");
    println!("ID: {}", reflection.id);
    println!("Theory ID: {}", reflection.theory_id);
    println!("Trigger Context ID: {}", reflection.trigger_context_id);
    println!("Agent: {}", reflection.agent);
    println!("Timestamp: {}", reflection.timestamp);
    println!("Severity: {}", reflection.severity());
    println!("Dissonance Score: {:.2}", reflection.dissonance_score);

    if let Some(tt) = &reflection.trigger_type {
        println!("Trigger Type: {:?}", tt);
    }

    println!("\nObserved State:");
    println!("  {}", reflection.observed_state);

    if !reflection.cognitive_dissonance.is_empty() {
        println!("\nCognitive Dissonance:");
        for dissonance in &reflection.cognitive_dissonance {
            println!("  - {}", dissonance);
        }
    }

    if !reflection.proposed_theory_updates.is_empty() {
        println!("\nProposed Theory Updates:");
        for update in &reflection.proposed_theory_updates {
            println!("  - {}", update);
        }
    }

    if let Some(resolved) = reflection.resolved {
        println!("\nResolved: {}", resolved);
        if let Some(new_theory_id) = reflection.resolved_theory_id {
            println!("Resolved Theory ID: {}", new_theory_id);
        }
    }

    Ok(())
}

/// Record cognitive dissonance
pub fn record_dissonance<S: Storage>(
    storage: &mut S,
    id: &str,
    description: &str,
    score: f64,
) -> Result<(), EngramError> {
    let entity = storage
        .get(id, StateReflection::entity_type())?
        .ok_or_else(|| EngramError::NotFound(format!("State reflection not found: {}", id)))?;

    let mut reflection = StateReflection::from_generic(entity)
        .map_err(|e: EngramError| EngramError::Validation(e.to_string()))?;

    reflection.record_dissonance(description.to_string(), score);

    reflection.validate_entity()?;
    let generic = reflection.to_generic();
    storage.store(&generic)?;

    println!(
        "Dissonance recorded: {} (new severity: {})",
        description,
        reflection.severity()
    );
    Ok(())
}

/// Propose a theory update
pub fn propose_update<S: Storage>(
    storage: &mut S,
    id: &str,
    update: &str,
) -> Result<(), EngramError> {
    let entity = storage
        .get(id, StateReflection::entity_type())?
        .ok_or_else(|| EngramError::NotFound(format!("State reflection not found: {}", id)))?;

    let mut reflection = StateReflection::from_generic(entity)
        .map_err(|e: EngramError| EngramError::Validation(e.to_string()))?;

    reflection.propose_update(update.to_string());

    let generic = reflection.to_generic();
    storage.store(&generic)?;

    println!("Proposed update recorded");
    Ok(())
}

/// Resolve a reflection
pub fn resolve_reflection<S: Storage>(
    storage: &mut S,
    id: &str,
    new_theory_id: &str,
) -> Result<(), EngramError> {
    let entity = storage
        .get(id, StateReflection::entity_type())?
        .ok_or_else(|| EngramError::NotFound(format!("State reflection not found: {}", id)))?;

    let mut reflection = StateReflection::from_generic(entity)
        .map_err(|e: EngramError| EngramError::Validation(e.to_string()))?;

    reflection.resolve(new_theory_id.to_string());

    let generic = reflection.to_generic();
    storage.store(&generic)?;

    println!("Reflection resolved with new theory: {}", new_theory_id);
    Ok(())
}

/// Delete state reflection
pub fn delete_reflection<S: Storage>(storage: &mut S, id: &str) -> Result<(), EngramError> {
    storage.delete(id, StateReflection::entity_type())?;
    println!("State reflection deleted successfully: {}", id);
    Ok(())
}

/// Check if theory mutation is required
pub fn requires_mutation<S: Storage>(
    storage: &S,
    id: &str,
    threshold: f64,
) -> Result<(), EngramError> {
    let entity = storage
        .get(id, StateReflection::entity_type())?
        .ok_or_else(|| EngramError::NotFound(format!("State reflection not found: {}", id)))?;

    let reflection = StateReflection::from_generic(entity)
        .map_err(|e: EngramError| EngramError::Validation(e.to_string()))?;

    if reflection.requires_theory_mutation(threshold) {
        println!(
            "YES - Theory mutation required (score: {:.2} >= threshold: {:.2})",
            reflection.dissonance_score, threshold
        );
        println!("\nProposed updates:");
        for update in &reflection.proposed_theory_updates {
            println!("  - {}", update);
        }
    } else {
        println!(
            "NO - Theory mutation not required (score: {:.2} < threshold: {:.2})",
            reflection.dissonance_score, threshold
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::MemoryStorage;

    fn create_test_storage() -> MemoryStorage {
        MemoryStorage::new("default")
    }

    #[test]
    fn test_create_reflection_basic() {
        let mut storage = create_test_storage();
        let result = create_reflection(
            &mut storage,
            Some("theory-1".to_string()),
            Some("task-1".to_string()),
            Some("Test failed".to_string()),
            Some("test_failure".to_string()),
            Some("the-forensic".to_string()),
            false,
            None,
        );
        assert!(result.is_ok());

        let ids = storage.list_ids("state_reflection").unwrap();
        assert_eq!(ids.len(), 1);
    }

    #[test]
    fn test_record_dissonance() {
        let mut storage = create_test_storage();
        create_reflection(
            &mut storage,
            Some("theory-1".to_string()),
            Some("task-1".to_string()),
            Some("Error".to_string()),
            None,
            None,
            false,
            None,
        )
        .unwrap();

        let ids = storage.list_ids("state_reflection").unwrap();
        let id = &ids[0];

        record_dissonance(&mut storage, id, "Concept mismatch", 0.8).unwrap();

        let entity = storage.get(id, "state_reflection").unwrap().unwrap();
        let reflection = StateReflection::from_generic(entity).unwrap();
        assert_eq!(reflection.cognitive_dissonance.len(), 1);
        assert_eq!(reflection.dissonance_score, 0.8);
    }

    #[test]
    fn test_propose_update() {
        let mut storage = create_test_storage();
        create_reflection(
            &mut storage,
            Some("theory-1".to_string()),
            Some("task-1".to_string()),
            Some("Error".to_string()),
            None,
            None,
            false,
            None,
        )
        .unwrap();

        let ids = storage.list_ids("state_reflection").unwrap();
        let id = &ids[0];

        propose_update(&mut storage, id, "Update concept A definition").unwrap();

        let entity = storage.get(id, "state_reflection").unwrap().unwrap();
        let reflection = StateReflection::from_generic(entity).unwrap();
        assert!(reflection
            .proposed_theory_updates
            .contains(&"Update concept A definition".to_string()));
    }

    #[test]
    fn test_resolve_reflection() {
        let mut storage = create_test_storage();
        create_reflection(
            &mut storage,
            Some("theory-1".to_string()),
            Some("task-1".to_string()),
            Some("Error".to_string()),
            None,
            None,
            false,
            None,
        )
        .unwrap();

        let ids = storage.list_ids("state_reflection").unwrap();
        let id = &ids[0];

        resolve_reflection(&mut storage, id, "theory-2").unwrap();

        let entity = storage.get(id, "state_reflection").unwrap().unwrap();
        let reflection = StateReflection::from_generic(entity).unwrap();
        assert_eq!(reflection.resolved, Some(true));
        assert_eq!(reflection.resolved_theory_id, Some("theory-2".to_string()));
    }

    #[test]
    fn test_requires_mutation() {
        let mut storage = create_test_storage();
        create_reflection(
            &mut storage,
            Some("theory-1".to_string()),
            Some("task-1".to_string()),
            Some("Error".to_string()),
            None,
            None,
            false,
            None,
        )
        .unwrap();

        let ids = storage.list_ids("state_reflection").unwrap();
        let id = &ids[0];

        // Add dissonance and proposed update
        record_dissonance(&mut storage, &ids[0], "Issue", 0.8).unwrap();
        propose_update(&mut storage, &ids[0], "Fix needed").unwrap();

        assert!(requires_mutation(&storage, id, 0.7).is_ok());
    }

    #[test]
    fn test_parse_trigger_type() {
        assert!(matches!(
            parse_trigger_type("test_failure"),
            Ok(TriggerType::TestFailure)
        ));
        assert!(matches!(
            parse_trigger_type("runtime_error"),
            Ok(TriggerType::RuntimeError)
        ));
        assert!(parse_trigger_type("invalid").is_err());
    }
}

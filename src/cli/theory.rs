//! Theory command implementations (Naur, 1985 - Programming as Theory Building)

use crate::cli::utils::{create_table, truncate};
use crate::entities::{Entity, Theory};
use crate::error::EngramError;
use crate::storage::Storage;
use clap::Subcommand;
use prettytable::row;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Read};

#[derive(Debug, Deserialize)]
pub struct TheoryInput {
    pub domain_name: String,
    #[serde(default)]
    pub conceptual_model: HashMap<String, String>,
    #[serde(default)]
    pub system_mapping: HashMap<String, String>,
    #[serde(default)]
    pub design_rationale: HashMap<String, String>,
    #[serde(default)]
    pub invariants: Vec<String>,
    #[serde(default)]
    pub agent: Option<String>,
    #[serde(default)]
    pub task_id: Option<String>,
}

/// Theory commands
#[derive(Debug, Subcommand)]
pub enum TheoryCommands {
    Create {
        domain: Option<String>,
        #[arg(long, short)]
        agent: Option<String>,
        #[arg(long, short)]
        task: Option<String>,
        #[arg(long, short)]
        json: bool,
        #[arg(long)]
        json_file: Option<String>,
    },
    List {
        #[arg(long, short)]
        agent: Option<String>,
        #[arg(long, short)]
        domain: Option<String>,
        #[arg(long, short)]
        limit: Option<usize>,
    },
    Show {
        #[arg(long, short)]
        id: String,
        #[arg(long)]
        show_metrics: bool,
    },
    Update {
        #[arg(long, short)]
        id: String,
        #[arg(long)]
        concept: Option<String>,
        #[arg(long)]
        mapping: Option<String>,
        #[arg(long)]
        rationale: Option<String>,
        #[arg(long)]
        invariant: Option<String>,
    },
    Delete {
        #[arg(long, short)]
        id: String,
    },
    ApplyReflection {
        #[arg(long, short)]
        theory_id: String,
        #[arg(long, short)]
        reflection_id: String,
        #[arg(long)]
        updates_file: String,
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

fn create_theory_from_input<S: Storage>(
    storage: &mut S,
    input: TheoryInput,
) -> Result<(), EngramError> {
    let agent = input.agent.unwrap_or_else(|| "default".to_string());
    let mut theory = if let Some(task_id) = input.task_id {
        Theory::for_task(input.domain_name, agent, task_id)
    } else {
        Theory::new(input.domain_name, agent)
    };

    for (concept, definition) in input.conceptual_model {
        theory.add_concept(concept, definition);
    }

    for (concept, implementation) in input.system_mapping {
        theory.add_mapping(concept, implementation);
    }

    for (decision, reason) in input.design_rationale {
        theory.add_rationale(decision, reason);
    }

    for invariant in input.invariants {
        theory.add_invariant(invariant);
    }

    let generic = theory.to_generic();
    storage.store(&generic)?;

    println!("Theory created successfully with ID: {}", theory.id);
    Ok(())
}

pub fn create_theory<S: Storage>(
    storage: &mut S,
    domain: Option<String>,
    agent: Option<String>,
    task: Option<String>,
    json: bool,
    json_file: Option<String>,
) -> Result<(), EngramError> {
    if json {
        let json_str = if let Some(file) = json_file {
            read_file(&file)?
        } else {
            read_stdin()?
        };

        let input: TheoryInput = serde_json::from_str(&json_str).map_err(|e| {
            let line = e.line();
            let col = e.column();
            let lines: Vec<&str> = json_str.lines().collect();
            let snippet = if line > 0 && line <= lines.len() {
                let context_line = lines[line - 1];
                format!("\n\nContext (Line {}):\n> {}", line, context_line)
            } else {
                String::new()
            };

            EngramError::Validation(format!(
                "Invalid JSON format\n\nError: {}\nLocation: Line {}, Column {}{}\n\nTip: Ensure your JSON has valid structure and quotes around strings.",
                e, line, col, snippet
            ))
        })?;

        return create_theory_from_input(storage, input);
    }

    let domain_name = domain.ok_or_else(|| {
        EngramError::Validation(
            "Domain name is required (use positional argument or --json)".to_string(),
        )
    })?;

    let agent_name = agent.unwrap_or_else(|| "default".to_string());
    let theory = if let Some(task_id) = task {
        Theory::for_task(domain_name, agent_name, task_id)
    } else {
        Theory::new(domain_name, agent_name)
    };

    let generic = theory.to_generic();
    storage.store(&generic)?;

    println!("Theory created successfully with ID: {}", theory.id);
    Ok(())
}

pub fn list_theories<S: Storage>(
    storage: &S,
    agent: Option<String>,
    domain: Option<String>,
    limit: Option<usize>,
) -> Result<(), EngramError> {
    let ids = storage.list_ids(Theory::entity_type())?;

    let mut items: Vec<Theory> = Vec::new();

    for id in ids {
        if let Some(entity) = storage.get(&id, Theory::entity_type())? {
            if let Ok(theory) = Theory::from_generic(entity) {
                if let Some(ref agent_filter) = agent {
                    if theory.agent != *agent_filter {
                        continue;
                    }
                }

                if let Some(ref domain_filter) = domain {
                    if !theory
                        .domain_name
                        .to_lowercase()
                        .contains(&domain_filter.to_lowercase())
                    {
                        continue;
                    }
                }

                items.push(theory);
            }
        }
    }

    if let Some(limit_val) = limit {
        items.truncate(limit_val);
    }

    if items.is_empty() {
        println!("No theories found matching the criteria.");
        return Ok(());
    }

    let mut table = create_table();
    table.set_titles(row![
        "ID",
        "Domain",
        "Agent",
        "Concepts",
        "Invariants",
        "Iterations",
        "Updated"
    ]);

    for theory in items {
        table.add_row(row![
            &theory.id[..8],
            truncate(&theory.domain_name, 30),
            truncate(&theory.agent, 15),
            theory.conceptual_model.len().to_string(),
            theory.invariants.len().to_string(),
            theory.iteration_count.to_string(),
            theory.last_updated.format("%Y-%m-%d")
        ]);
    }

    table.printstd();
    Ok(())
}

pub fn show_theory<S: Storage>(
    storage: &S,
    id: &str,
    show_metrics: bool,
) -> Result<(), EngramError> {
    let entity = storage
        .get(id, Theory::entity_type())?
        .ok_or_else(|| EngramError::NotFound(format!("Theory not found: {}", id)))?;

    let theory =
        Theory::from_generic(entity).map_err(|e| EngramError::Validation(e.to_string()))?;

    println!("Theory Details:");
    println!("===============");
    println!("ID: {}", theory.id);
    println!("Domain: {}", theory.domain_name);
    println!("Agent: {}", theory.agent);
    println!("Created: {}", theory.created_at);
    println!("Last Updated: {}", theory.last_updated);
    println!("Iteration Count: {}", theory.iteration_count);

    if let Some(task_id) = &theory.task_id {
        println!("Task ID: {}", task_id);
    }

    if !theory.conceptual_model.is_empty() {
        println!("\nConceptual Model:");
        for (concept, definition) in &theory.conceptual_model {
            println!("  {}: {}", concept, definition);
        }
    }

    if !theory.system_mapping.is_empty() {
        println!("\nSystem Mapping:");
        for (concept, implementation) in &theory.system_mapping {
            println!("  {} -> {}", concept, implementation);
        }
    }

    if !theory.design_rationale.is_empty() {
        println!("\nDesign Rationale:");
        for (decision, reason) in &theory.design_rationale {
            println!("  {}: {}", decision, reason);
        }
    }

    if !theory.invariants.is_empty() {
        println!("\nInvariants:");
        for invariant in &theory.invariants {
            println!("  - {}", invariant);
        }
    }

    if !theory.reflection_ids.is_empty() {
        println!("\nReflection IDs: {}", theory.reflection_ids.join(", "));
    }

    if show_metrics {
        println!("\nMetrics:");
        println!("  Concepts: {}", theory.conceptual_model.len());
        println!("  Mappings: {}", theory.system_mapping.len());
        println!("  Rationales: {}", theory.design_rationale.len());
        println!("  Invariants: {}", theory.invariants.len());
        println!("  Reflections Applied: {}", theory.reflection_ids.len());
    }

    Ok(())
}

pub fn update_theory<S: Storage>(
    storage: &mut S,
    id: &str,
    concept: Option<String>,
    mapping: Option<String>,
    rationale: Option<String>,
    invariant: Option<String>,
) -> Result<(), EngramError> {
    let entity = storage
        .get(id, Theory::entity_type())?
        .ok_or_else(|| EngramError::NotFound(format!("Theory not found: {}", id)))?;

    let mut theory =
        Theory::from_generic(entity).map_err(|e| EngramError::Validation(e.to_string()))?;

    if let Some(concept_str) = concept {
        let parts: Vec<&str> = concept_str.splitn(2, ':').collect();
        if parts.len() != 2 {
            return Err(EngramError::Validation(
                "Concept must be in format 'name:definition'".to_string(),
            ));
        }
        theory.add_concept(parts[0].trim().to_string(), parts[1].trim().to_string());
    }

    if let Some(mapping_str) = mapping {
        let parts: Vec<&str> = mapping_str.splitn(2, ':').collect();
        if parts.len() != 2 {
            return Err(EngramError::Validation(
                "Mapping must be in format 'concept:implementation'".to_string(),
            ));
        }
        theory.add_mapping(parts[0].trim().to_string(), parts[1].trim().to_string());
    }

    if let Some(rationale_str) = rationale {
        let parts: Vec<&str> = rationale_str.splitn(2, ':').collect();
        if parts.len() != 2 {
            return Err(EngramError::Validation(
                "Rationale must be in format 'decision:reason'".to_string(),
            ));
        }
        theory.add_rationale(parts[0].trim().to_string(), parts[1].trim().to_string());
    }

    if let Some(invariant_str) = invariant {
        theory.add_invariant(invariant_str);
    }

    let generic = theory.to_generic();
    storage.store(&generic)?;

    println!("Theory updated successfully: {}", id);
    Ok(())
}

pub fn delete_theory<S: Storage>(storage: &mut S, id: &str) -> Result<(), EngramError> {
    storage.delete(id, Theory::entity_type())?;
    println!("Theory deleted successfully: {}", id);
    Ok(())
}

pub fn apply_reflection<S: Storage>(
    storage: &mut S,
    theory_id: &str,
    reflection_id: &str,
    updates_file: &str,
) -> Result<(), EngramError> {
    let entity = storage
        .get(theory_id, Theory::entity_type())?
        .ok_or_else(|| EngramError::NotFound(format!("Theory not found: {}", theory_id)))?;

    let mut theory =
        Theory::from_generic(entity).map_err(|e| EngramError::Validation(e.to_string()))?;

    let json_str = read_file(updates_file)?;
    let updates: HashMap<String, String> = serde_json::from_str(&json_str)
        .map_err(|e| EngramError::Validation(format!("Invalid JSON in updates file: {}", e)))?;

    theory.apply_reflection_updates(updates, reflection_id.to_string());

    let generic = theory.to_generic();
    storage.store(&generic)?;

    println!(
        "Reflection {} applied to theory {}",
        reflection_id, theory_id
    );
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
    fn test_create_theory_basic() {
        let mut storage = create_test_storage();
        let result = create_theory(
            &mut storage,
            Some("Test Domain".to_string()),
            Some("test-agent".to_string()),
            None,
            false,
            None,
        );
        assert!(result.is_ok());

        let ids = storage.list_ids("theory").unwrap();
        assert_eq!(ids.len(), 1);

        let entity = storage.get(&ids[0], "theory").unwrap().unwrap();
        let theory = Theory::from_generic(entity).unwrap();
        assert_eq!(theory.domain_name, "Test Domain");
        assert_eq!(theory.agent, "test-agent");
    }

    #[test]
    fn test_create_theory_missing_domain() {
        let mut storage = create_test_storage();
        let result = create_theory(&mut storage, None, None, None, false, None);
        assert!(matches!(result, Err(EngramError::Validation(_))));
    }

    #[test]
    fn test_list_theories() {
        let mut storage = create_test_storage();
        create_theory(
            &mut storage,
            Some("Domain A".to_string()),
            Some("agent1".to_string()),
            None,
            false,
            None,
        )
        .unwrap();
        create_theory(
            &mut storage,
            Some("Domain B".to_string()),
            Some("agent2".to_string()),
            None,
            false,
            None,
        )
        .unwrap();

        assert!(list_theories(&storage, None, None, None).is_ok());
        assert!(list_theories(&storage, Some("agent1".to_string()), None, None).is_ok());
    }

    #[test]
    fn test_show_theory() {
        let mut storage = create_test_storage();
        create_theory(
            &mut storage,
            Some("Show Domain".to_string()),
            None,
            None,
            false,
            None,
        )
        .unwrap();

        let ids = storage.list_ids("theory").unwrap();
        let id = &ids[0];

        assert!(show_theory(&storage, id, false).is_ok());
        assert!(show_theory(&storage, id, true).is_ok());
    }

    #[test]
    fn test_show_theory_not_found() {
        let storage = create_test_storage();
        let result = show_theory(&storage, "missing-id", false);
        assert!(matches!(result, Err(EngramError::NotFound(_))));
    }

    #[test]
    fn test_update_theory() {
        let mut storage = create_test_storage();
        create_theory(
            &mut storage,
            Some("Update Domain".to_string()),
            None,
            None,
            false,
            None,
        )
        .unwrap();

        let ids = storage.list_ids("theory").unwrap();
        let id = &ids[0];

        update_theory(
            &mut storage,
            id,
            Some("User:A person who uses the system".to_string()),
            None,
            None,
            None,
        )
        .unwrap();

        let entity = storage.get(id, "theory").unwrap().unwrap();
        let theory = Theory::from_generic(entity).unwrap();
        assert_eq!(
            theory.conceptual_model.get("User"),
            Some(&"A person who uses the system".to_string())
        );
    }

    #[test]
    fn test_delete_theory() {
        let mut storage = create_test_storage();
        create_theory(
            &mut storage,
            Some("Delete Domain".to_string()),
            None,
            None,
            false,
            None,
        )
        .unwrap();

        let ids = storage.list_ids("theory").unwrap();
        let id = &ids[0];

        delete_theory(&mut storage, id).unwrap();
        let result = storage.get(id, "theory").unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_update_theory_invalid_concept_format() {
        let mut storage = create_test_storage();
        create_theory(
            &mut storage,
            Some("Domain".to_string()),
            None,
            None,
            false,
            None,
        )
        .unwrap();

        let ids = storage.list_ids("theory").unwrap();
        let id = &ids[0];

        let result = update_theory(
            &mut storage,
            id,
            Some("invalid_no_colon".to_string()),
            None,
            None,
            None,
        );
        assert!(matches!(result, Err(EngramError::Validation(_))));
    }
}

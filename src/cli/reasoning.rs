//! Reasoning command implementations

use crate::entities::{Entity, IbisPosition, IbisPositionType, Reasoning};
use crate::error::EngramError;
use crate::storage::relationship_storage::RelationshipStorage;
use crate::storage::Storage;
use clap::Subcommand;
use serde::Deserialize;
use std::fs;
use std::io::{self, Read};

#[derive(Debug, Deserialize)]
pub struct ReasoningInput {
    pub title: String,
    pub task_id: String,
    pub agent: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct IbisFileInput {
    pub positions: Vec<IbisFilePosition>,
}

#[derive(Debug, Deserialize)]
pub struct IbisFilePosition {
    pub position_type: String,
    pub content: String,
    pub responds_to: Option<String>,
}

/// Reasoning commands
#[derive(Debug, Subcommand)]
pub enum ReasoningCommands {
    /// Create a new reasoning chain
    Create {
        #[arg(long, short, conflicts_with_all = ["title_stdin", "title_file"])]
        title: Option<String>,

        #[arg(long, required_unless_present = "json")]
        task_id: Option<String>,

        #[arg(long, short)]
        agent: Option<String>,

        #[arg(long, short)]
        confidence: Option<f64>,

        #[arg(long, conflicts_with_all = ["content_stdin", "content_file"])]
        content: Option<String>,

        #[arg(long)]
        tags: Option<String>,

        #[arg(long, conflicts_with_all = ["title", "title_stdin", "title_file"])]
        title_stdin: bool,

        #[arg(long, conflicts_with_all = ["title", "title_stdin"])]
        title_file: Option<String>,

        #[arg(long, conflicts_with_all = ["content", "content_file"])]
        content_stdin: bool,

        #[arg(long, conflicts_with_all = ["content", "content_stdin"])]
        content_file: Option<String>,

        #[arg(long, conflicts_with_all = ["title", "title_stdin", "title_file"])]
        json: bool,

        #[arg(long, requires = "json")]
        json_file: Option<String>,

        #[arg(long)]
        ibis: bool,

        #[arg(long, requires = "ibis")]
        ibis_file: Option<String>,

        #[arg(long, requires = "ibis")]
        issue: Option<String>,

        #[arg(long, requires = "ibis", num_args = 1..)]
        position: Vec<String>,

        #[arg(long, requires = "ibis", num_args = 1..)]
        argument: Vec<String>,

        #[arg(long, num_args = 1..)]
        prov_used: Vec<String>,

        #[arg(long, num_args = 1..)]
        prov_generated: Vec<String>,

        #[arg(long)]
        prov_attributed_to: Option<String>,
    },
    /// Add a reasoning step
    AddStep {
        /// Reasoning ID
        #[arg(help = "Reasoning ID to add step to")]
        id: String,

        /// Step description
        #[arg(long, short, conflicts_with_all = ["description_stdin", "description_file"])]
        description: Option<String>,

        /// Step conclusion
        #[arg(long, short, conflicts_with_all = ["conclusion_stdin", "conclusion_file"])]
        conclusion: Option<String>,

        /// Confidence level (0.0 to 1.0)
        #[arg(long, short = 'f')]
        confidence: f64,

        /// Read description from stdin
        #[arg(long, conflicts_with_all = ["description", "description_file"])]
        description_stdin: bool,

        /// Read description from file
        #[arg(long, conflicts_with_all = ["description", "description_stdin"])]
        description_file: Option<String>,

        /// Read conclusion from stdin
        #[arg(long, conflicts_with_all = ["conclusion", "conclusion_file"])]
        conclusion_stdin: bool,

        /// Read conclusion from file
        #[arg(long, conflicts_with_all = ["conclusion", "conclusion_stdin"])]
        conclusion_file: Option<String>,
    },
    /// Set final conclusion
    Conclude {
        /// Reasoning ID
        #[arg(help = "Reasoning ID to conclude")]
        id: String,

        /// Final conclusion
        #[arg(long, short, conflicts_with_all = ["conclusion_stdin", "conclusion_file"])]
        conclusion: Option<String>,

        /// Overall confidence
        #[arg(long, short = 'f')]
        confidence: f64,

        /// Read conclusion from stdin
        #[arg(long, conflicts_with_all = ["conclusion", "conclusion_file"])]
        conclusion_stdin: bool,

        /// Read conclusion from file
        #[arg(long, conflicts_with_all = ["conclusion", "conclusion_stdin"])]
        conclusion_file: Option<String>,
    },
    /// List reasoning chains
    List {
        /// Filter by agent
        #[arg(long, short)]
        agent: Option<String>,

        /// Filter by task ID
        #[arg(long, short)]
        task_id: Option<String>,

        /// Limit number of results
        #[arg(long, short)]
        limit: Option<usize>,

        /// Show all results (no limit)
        #[arg(long, conflicts_with = "limit")]
        all: bool,

        /// Offset for pagination
        #[arg(long, short)]
        offset: Option<usize>,
    },
    /// Show reasoning details
    Show {
        /// Reasoning ID
        #[arg(help = "Reasoning ID to show")]
        id: String,
    },
    /// Delete reasoning
    Delete {
        /// Reasoning ID
        #[arg(help = "Reasoning ID to delete")]
        id: String,
    },
}

fn read_stdin() -> Result<String, EngramError> {
    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .map_err(|e| EngramError::Io(e))?;
    Ok(buffer.trim().to_string())
}

fn read_file(path: &str) -> Result<String, EngramError> {
    fs::read_to_string(path).map_err(EngramError::Io)
}

fn create_reasoning_from_input<S: Storage>(
    storage: &mut S,
    input: ReasoningInput,
) -> Result<(), EngramError> {
    let agent = input.agent.unwrap_or_else(|| "default".to_string());

    let reasoning = Reasoning::new(input.title, input.task_id, agent.clone());

    let generic_entity = reasoning.to_generic();
    storage.store(&generic_entity)?;

    println!("Reasoning '{}' created successfully", reasoning.id);
    println!("ID: {}", reasoning.id);
    println!("Agent: {}", agent);

    Ok(())
}

pub fn create_reasoning<S: Storage>(
    storage: &mut S,
    title: Option<String>,
    task_id: Option<String>,
    agent: Option<String>,
    confidence: Option<f64>,
    content: Option<String>,
    _tags: Option<String>,
    title_stdin: bool,
    title_file: Option<String>,
    content_stdin: bool,
    content_file: Option<String>,
    json: bool,
    json_file: Option<String>,
    ibis: bool,
    ibis_file: Option<String>,
    issue: Option<String>,
    positions: Vec<String>,
    arguments: Vec<String>,
    prov_used: Vec<String>,
    prov_generated: Vec<String>,
    prov_attributed_to: Option<String>,
) -> Result<(), EngramError> {
    if json {
        let json_content = if let Some(ref file_path) = json_file {
            read_file(file_path)?
        } else {
            read_stdin()?
        };

        let reasoning_input: ReasoningInput = serde_json::from_str(&json_content).map_err(|e| {
            let line = e.line();
            let col = e.column();

            let lines: Vec<&str> = json_content.lines().collect();
            let snippet = if line > 0 && line <= lines.len() {
                let context_line = lines[line - 1];
                format!("\n\nContext (Line {}):\n> {}", line, context_line)
            } else {
                String::new()
            };

            EngramError::Validation(format!(
                "Invalid JSON format\n\nError: {}\nLocation: Line {}, Column {}{}\n\nTip: Ensure your JSON has valid structure and quotes around strings.",
                e,
                line,
                col,
                snippet
            ))
        })?;

        return create_reasoning_from_input(storage, reasoning_input);
    }

    let final_title = if title_stdin {
        read_stdin()?
    } else if let Some(ref file_path) = title_file {
        read_file(file_path)?
    } else if let Some(ref t) = title {
        t.clone()
    } else {
        return Err(EngramError::Validation(
            "Title required: use --title, --title-stdin, or --title-file".to_string(),
        ));
    };

    let final_task_id = task_id
        .ok_or_else(|| EngramError::Validation("Task ID required: use --task-id".to_string()))?;

    let final_agent = agent.unwrap_or_else(|| "default".to_string());

    let mut reasoning = Reasoning::new(final_title, final_task_id, final_agent.clone());

    if let Some(conf) = confidence {
        if conf < 0.0 || conf > 1.0 {
            return Err(EngramError::Validation(
                "Confidence must be between 0.0 and 1.0".to_string(),
            ));
        }
        reasoning.confidence = conf;
    }

    if content_stdin {
        reasoning.conclusion = read_stdin()?;
    } else if let Some(ref file_path) = content_file {
        reasoning.conclusion = read_file(file_path)?;
    } else if let Some(ref c) = content {
        reasoning.conclusion = c.clone();
    }

    if ibis {
        reasoning.ibis_mode = Some(true);

        if let Some(ref path) = ibis_file {
            let file_content = read_file(path)?;
            let ibis_input: IbisFileInput = serde_json::from_str(&file_content)
                .map_err(|e| EngramError::Validation(format!("Invalid IBIS JSON file: {}", e)))?;
            for pos in ibis_input.positions {
                let position_type = match pos.position_type.to_lowercase().as_str() {
                    "issue" => IbisPositionType::Issue,
                    "position" => IbisPositionType::Position,
                    "argument" => IbisPositionType::Argument,
                    other => {
                        return Err(EngramError::Validation(format!(
                            "Unknown IBIS position type: {}",
                            other
                        )));
                    }
                };
                reasoning.positions.push(IbisPosition {
                    position_type,
                    content: pos.content,
                    responds_to: pos.responds_to,
                });
            }
        } else {
            if let Some(iss) = issue {
                reasoning.positions.push(IbisPosition {
                    position_type: IbisPositionType::Issue,
                    content: iss,
                    responds_to: None,
                });
            }
            for p in &positions {
                reasoning.positions.push(IbisPosition {
                    position_type: IbisPositionType::Position,
                    content: p.clone(),
                    responds_to: None,
                });
            }
            for a in &arguments {
                reasoning.positions.push(IbisPosition {
                    position_type: IbisPositionType::Argument,
                    content: a.clone(),
                    responds_to: None,
                });
            }

            if reasoning.positions.is_empty() {
                return Err(EngramError::Validation(
                    "IBIS mode requires at least one position: use --issue, --position, or --argument"
                        .to_string(),
                ));
            }
        }

        reasoning.flatten_positions_to_steps();
    }

    if !prov_used.is_empty() {
        reasoning.prov_used = prov_used;
    }
    if !prov_generated.is_empty() {
        reasoning.prov_generated = prov_generated;
    }
    if let Some(ref attr) = prov_attributed_to {
        reasoning.prov_attributed_to = Some(attr.clone());
    }

    let generic_entity = reasoning.to_generic();
    storage.store(&generic_entity)?;

    println!("Reasoning '{}' created successfully", reasoning.id);
    println!("ID: {}", reasoning.id);
    println!("Title: {}", reasoning.title);
    println!("Task ID: {}", reasoning.task_id);
    println!("Agent: {}", final_agent);

    Ok(())
}

pub fn add_reasoning_step<S: Storage>(
    storage: &mut S,
    id: &str,
    description: Option<String>,
    conclusion: Option<String>,
    confidence: f64,
    description_stdin: bool,
    description_file: Option<String>,
    conclusion_stdin: bool,
    conclusion_file: Option<String>,
) -> Result<(), EngramError> {
    let final_description = if description_stdin {
        read_stdin()?
    } else if let Some(ref file_path) = description_file {
        read_file(file_path)?
    } else if let Some(ref d) = description {
        d.clone()
    } else {
        return Err(EngramError::Validation(
            "Description required: use --description, --description-stdin, or --description-file"
                .to_string(),
        ));
    };

    let final_conclusion = if conclusion_stdin {
        read_stdin()?
    } else if let Some(ref file_path) = conclusion_file {
        read_file(file_path)?
    } else if let Some(ref c) = conclusion {
        c.clone()
    } else {
        return Err(EngramError::Validation(
            "Conclusion required: use --conclusion, --conclusion-stdin, or --conclusion-file"
                .to_string(),
        ));
    };

    if confidence < 0.0 || confidence > 1.0 {
        return Err(EngramError::Validation(
            "Confidence must be between 0.0 and 1.0".to_string(),
        ));
    }

    let entity = storage.get(id, "reasoning")?;
    match entity {
        Some(generic_entity) => {
            let mut reasoning = Reasoning::from_generic(generic_entity)
                .map_err(|e| EngramError::Validation(e.to_string()))?;

            reasoning.add_step(final_description, final_conclusion, confidence);

            let updated_entity = reasoning.to_generic();
            storage.store(&updated_entity)?;

            println!("Added step to reasoning '{}' successfully", reasoning.title);
            println!("Step count: {}", reasoning.steps.len());
        }
        None => {
            return Err(EngramError::NotFound(format!(
                "Reasoning with ID '{}' not found",
                id
            )));
        }
    }

    Ok(())
}

pub fn conclude_reasoning<S: Storage>(
    storage: &mut S,
    id: &str,
    conclusion: Option<String>,
    confidence: f64,
    conclusion_stdin: bool,
    conclusion_file: Option<String>,
) -> Result<(), EngramError> {
    let final_conclusion = if conclusion_stdin {
        read_stdin()?
    } else if let Some(ref file_path) = conclusion_file {
        read_file(file_path)?
    } else if let Some(ref c) = conclusion {
        c.clone()
    } else {
        return Err(EngramError::Validation(
            "Conclusion required: use --conclusion, --conclusion-stdin, or --conclusion-file"
                .to_string(),
        ));
    };

    if confidence < 0.0 || confidence > 1.0 {
        return Err(EngramError::Validation(
            "Confidence must be between 0.0 and 1.0".to_string(),
        ));
    }

    let entity = storage.get(id, "reasoning")?;
    match entity {
        Some(generic_entity) => {
            let mut reasoning = Reasoning::from_generic(generic_entity)
                .map_err(|e| EngramError::Validation(e.to_string()))?;

            reasoning.set_conclusion(final_conclusion, confidence);

            let updated_entity = reasoning.to_generic();
            storage.store(&updated_entity)?;

            println!("Reasoning '{}' concluded successfully", reasoning.title);
            println!("Final confidence: {}", reasoning.confidence);
        }
        None => {
            return Err(EngramError::NotFound(format!(
                "Reasoning with ID '{}' not found",
                id
            )));
        }
    }

    Ok(())
}

use crate::cli::utils::{create_table, truncate};
use prettytable::row;

pub fn list_reasoning<S: Storage>(
    storage: &S,
    agent: Option<&str>,
    task_id: Option<&str>,
    limit: Option<usize>,
    all: bool,
    offset: Option<usize>,
) -> Result<(), EngramError> {
    let mut filter = crate::storage::QueryFilter {
        entity_type: Some("reasoning".to_string()),
        agent: agent.map(|s| s.to_string()),
        limit: if all { None } else { limit },
        offset,
        ..Default::default()
    };

    if let Some(tid) = task_id {
        filter.field_filters.insert(
            "task_id".to_string(),
            serde_json::Value::String(tid.to_string()),
        );
    }

    let result = storage.query(&filter)?;

    if result.entities.is_empty() {
        println!("No reasoning chains found");
        return Ok(());
    }

    println!(
        "Found {} reasoning chain(s) (showing {} of {})",
        result.total_count,
        result.entities.len(),
        result.total_count
    );

    let mut table = create_table();
    table.set_titles(row!["ID", "Status", "Title", "Task ID", "Agent"]);

    for entity in result.entities {
        if let Ok(reasoning) = Reasoning::from_generic(entity) {
            let status = if !reasoning.conclusion.is_empty() {
                "✅ Concluded"
            } else {
                "🚧 In Progress"
            };

            table.add_row(row![
                &reasoning.id[..8],
                status,
                truncate(&reasoning.title, 40),
                truncate(&reasoning.task_id, 15),
                truncate(&reasoning.agent, 10)
            ]);
        }
    }

    table.printstd();

    if result.has_more {
        println!("(More results available — use --all, --offset N, or --limit N)");
    }

    Ok(())
}

pub fn show_reasoning<S: Storage + RelationshipStorage>(
    storage: &S,
    id: &str,
) -> Result<(), EngramError> {
    let entity = storage.get(id, "reasoning")?;

    match entity {
        Some(generic_entity) => {
            let reasoning = Reasoning::from_generic(generic_entity)
                .map_err(|e| EngramError::Validation(e.to_string()))?;

            println!("Reasoning Details:");
            println!("==================");
            println!("ID: {}", reasoning.id);
            println!("Title: {}", reasoning.title);
            println!("Task ID: {}", reasoning.task_id);
            println!("Agent: {}", reasoning.agent);
            println!(
                "Created: {}",
                reasoning.created_at.format("%Y-%m-%d %H:%M:%S UTC")
            );

            if reasoning.ibis_mode == Some(true) {
                println!();
                println!("IBIS Mode: enabled");
                println!("Positions: {}", reasoning.positions.len());
                for (i, pos) in reasoning.positions.iter().enumerate() {
                    println!("  {}. [{}] {}", i + 1, pos.position_type, pos.content);
                    if let Some(ref responds) = pos.responds_to {
                        println!("     responds to: {}", responds);
                    }
                }
            }

            if reasoning.steps.is_empty() {
                println!("Steps: None");
            } else {
                println!("Steps: {}", reasoning.steps.len());
                println!();
                for (i, step) in reasoning.steps.iter().enumerate() {
                    println!("Step {} (Confidence: {:.2}):", i + 1, step.confidence);
                    println!("  Description: {}", step.description);
                    println!("  Conclusion: {}", step.conclusion);
                    println!(
                        "  Created: {}",
                        step.timestamp.format("%Y-%m-%d %H:%M:%S UTC")
                    );
                    if !step.evidence.is_empty() {
                        println!("  Evidence: {}", step.evidence.join(", "));
                    }
                    println!();
                }
            }

            if reasoning.conclusion.is_empty() {
                println!("Final Conclusion: Not yet concluded");
            } else {
                println!("Final Conclusion:");
                println!("  {}", reasoning.conclusion);
                println!("  Overall Confidence: {:.2}", reasoning.confidence);
            }

            if !reasoning.tags.is_empty() {
                println!("Tags: {}", reasoning.tags.join(", "));
            }

            let prov_used = if reasoning.prov_used.is_empty() {
                storage
                    .get_inbound_relationships(id)
                    .ok()
                    .map(|rels| rels.iter().map(|r| r.source_id.clone()).collect::<Vec<_>>())
                    .unwrap_or_default()
            } else {
                reasoning.prov_used.clone()
            };

            let prov_generated = if reasoning.prov_generated.is_empty() {
                storage
                    .get_outbound_relationships(id)
                    .ok()
                    .map(|rels| rels.iter().map(|r| r.target_id.clone()).collect::<Vec<_>>())
                    .unwrap_or_default()
            } else {
                reasoning.prov_generated.clone()
            };

            if !prov_used.is_empty()
                || !prov_generated.is_empty()
                || reasoning.prov_attributed_to.is_some()
            {
                println!();
                println!("PROV-O Provenance:");
                if !prov_used.is_empty() {
                    println!("  Used: {}", prov_used.join(", "));
                }
                if !prov_generated.is_empty() {
                    println!("  Generated: {}", prov_generated.join(", "));
                }
                if let Some(ref attr) = reasoning.prov_attributed_to {
                    println!("  Attributed to: {}", attr);
                }
            }
        }
        None => {
            return Err(EngramError::NotFound(format!(
                "Reasoning with ID '{}' not found",
                id
            )));
        }
    }

    Ok(())
}

pub fn delete_reasoning<S: Storage>(storage: &mut S, id: &str) -> Result<(), EngramError> {
    let entity = storage.get(id, "reasoning")?;

    match entity {
        Some(generic_entity) => {
            let reasoning = Reasoning::from_generic(generic_entity)
                .map_err(|e| EngramError::Validation(e.to_string()))?;

            storage.delete(id, "reasoning")?;

            println!("Reasoning '{}' deleted successfully", reasoning.title);
            println!("ID: {}", reasoning.id);
            println!("Task ID: {}", reasoning.task_id);
        }
        None => {
            return Err(EngramError::NotFound(format!(
                "Reasoning with ID '{}' not found",
                id
            )));
        }
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

    fn create_reasoning_minimal(
        storage: &mut MemoryStorage,
        title: &str,
        task_id: &str,
        agent: Option<&str>,
    ) -> String {
        create_reasoning(
            storage,
            Some(title.to_string()),
            Some(task_id.to_string()),
            agent.map(|s| s.to_string()),
            None,
            None,
            None,
            false,
            None,
            false,
            None,
            false,
            None,
            false,
            None,
            None,
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            None,
        )
        .unwrap();
        let chains = storage
            .query_by_agent(agent.unwrap_or("default"), Some("reasoning"))
            .unwrap();
        chains[0].id.clone()
    }

    #[test]
    fn test_create_reasoning_basic() {
        let mut storage = create_test_storage();
        create_reasoning(
            &mut storage,
            Some("Test Reasoning".to_string()),
            Some("task-123".to_string()),
            Some("agent1".to_string()),
            Some(0.5),
            Some("Initial thought".to_string()),
            None,
            false,
            None,
            false,
            None,
            false,
            None,
            false,
            None,
            None,
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            None,
        )
        .unwrap();
        let chains = storage.query_by_agent("agent1", Some("reasoning")).unwrap();
        assert_eq!(chains.len(), 1);
        let reasoning = Reasoning::from_generic(chains[0].clone()).unwrap();
        assert_eq!(reasoning.title, "Test Reasoning");
        assert_eq!(reasoning.task_id, "task-123");
        assert_eq!(reasoning.conclusion, "Initial thought");
    }

    #[test]
    fn test_create_reasoning_validation() {
        let mut storage = create_test_storage();

        let result = create_reasoning(
            &mut storage,
            None,
            Some("task-123".to_string()),
            None,
            None,
            None,
            None,
            false,
            None,
            false,
            None,
            false,
            None,
            false,
            None,
            None,
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            None,
        );
        assert!(matches!(result, Err(EngramError::Validation(_))));

        let result = create_reasoning(
            &mut storage,
            Some("Title".to_string()),
            None,
            None,
            None,
            None,
            None,
            false,
            None,
            false,
            None,
            false,
            None,
            false,
            None,
            None,
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            None,
        );
        assert!(matches!(result, Err(EngramError::Validation(_))));
    }

    #[test]
    fn test_add_reasoning_step() {
        let mut storage = create_test_storage();
        let id = create_reasoning_minimal(&mut storage, "Test Reasoning", "task-123", None);

        let result = add_reasoning_step(
            &mut storage,
            &id,
            Some("Step 1".to_string()),
            Some("Conclusion 1".to_string()),
            0.8,
            false,
            None,
            false,
            None,
        );
        assert!(result.is_ok());

        let entity = storage.get(&id, "reasoning").unwrap().unwrap();
        let reasoning = Reasoning::from_generic(entity).unwrap();
        assert_eq!(reasoning.steps.len(), 1);
        assert_eq!(reasoning.steps[0].description, "Step 1");
        assert_eq!(reasoning.steps[0].confidence, 0.8);
    }

    #[test]
    fn test_conclude_reasoning() {
        let mut storage = create_test_storage();
        let id = create_reasoning_minimal(&mut storage, "Test Reasoning", "task-123", None);

        let result = conclude_reasoning(
            &mut storage,
            &id,
            Some("Final conclusion".to_string()),
            0.95,
            false,
            None,
        );
        assert!(result.is_ok());

        let entity = storage.get(&id, "reasoning").unwrap().unwrap();
        let reasoning = Reasoning::from_generic(entity).unwrap();
        assert_eq!(reasoning.conclusion, "Final conclusion");
        assert_eq!(reasoning.confidence, 0.95);
    }

    #[test]
    fn test_delete_reasoning() {
        let mut storage = create_test_storage();
        let id = create_reasoning_minimal(&mut storage, "Delete Me", "task-123", None);

        delete_reasoning(&mut storage, &id).unwrap();

        let result = storage.get(&id, "reasoning").unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_add_reasoning_step_not_found() {
        let mut storage = create_test_storage();
        let result = add_reasoning_step(
            &mut storage,
            "non-existent-id",
            Some("Step 1".to_string()),
            Some("Conclusion".to_string()),
            0.5,
            false,
            None,
            false,
            None,
        );
        assert!(matches!(result, Err(EngramError::NotFound(_))));
    }

    #[test]
    fn test_add_reasoning_step_invalid_confidence() {
        let mut storage = create_test_storage();
        let id = create_reasoning_minimal(&mut storage, "Test Reasoning", "task-123", None);

        let result = add_reasoning_step(
            &mut storage,
            &id,
            Some("Step 1".to_string()),
            Some("Conclusion".to_string()),
            1.5,
            false,
            None,
            false,
            None,
        );
        assert!(matches!(result, Err(EngramError::Validation(_))));
    }

    #[test]
    fn test_add_reasoning_step_missing_description() {
        let mut storage = create_test_storage();
        let id = create_reasoning_minimal(&mut storage, "Test Reasoning", "task-123", None);

        let result = add_reasoning_step(
            &mut storage,
            &id,
            None,
            Some("Conclusion".to_string()),
            0.5,
            false,
            None,
            false,
            None,
        );
        assert!(matches!(result, Err(EngramError::Validation(_))));
    }

    #[test]
    fn test_conclude_reasoning_not_found() {
        let mut storage = create_test_storage();
        let result = conclude_reasoning(
            &mut storage,
            "non-existent-id",
            Some("Final conclusion".to_string()),
            0.9,
            false,
            None,
        );
        assert!(matches!(result, Err(EngramError::NotFound(_))));
    }

    #[test]
    fn test_conclude_reasoning_invalid_confidence() {
        let mut storage = create_test_storage();
        let id = create_reasoning_minimal(&mut storage, "Test Reasoning", "task-123", None);

        let result = conclude_reasoning(
            &mut storage,
            &id,
            Some("Final conclusion".to_string()),
            -0.1,
            false,
            None,
        );
        assert!(matches!(result, Err(EngramError::Validation(_))));
    }

    #[test]
    fn test_delete_reasoning_not_found() {
        let mut storage = create_test_storage();
        let result = delete_reasoning(&mut storage, "non-existent-id");
        assert!(matches!(result, Err(EngramError::NotFound(_))));
    }

    #[test]
    fn test_show_reasoning_not_found() {
        let storage = create_test_storage();
        let result = show_reasoning(&storage, "non-existent-id");
        assert!(matches!(result, Err(EngramError::NotFound(_))));
    }

    #[test]
    fn test_list_reasoning() {
        let mut storage = create_test_storage();
        create_reasoning_minimal(&mut storage, "R1", "task-1", Some("agent1"));
        create_reasoning_minimal(&mut storage, "R2", "task-2", Some("agent2"));

        assert!(list_reasoning(&storage, None, None, None, false, None).is_ok());
        assert!(list_reasoning(&storage, Some("agent1"), None, None, false, None).is_ok());
        assert!(list_reasoning(&storage, None, Some("task-2"), None, false, None).is_ok());
    }

    #[test]
    fn test_show_reasoning() {
        let mut storage = create_test_storage();
        let id = create_reasoning_minimal(&mut storage, "Show Me", "task-1", None);
        assert!(show_reasoning(&storage, &id).is_ok());
    }

    #[test]
    fn test_create_reasoning_invalid_confidence() {
        let mut storage = create_test_storage();
        let result = create_reasoning(
            &mut storage,
            Some("Bad Confidence".to_string()),
            Some("task-1".to_string()),
            None,
            Some(1.5),
            None,
            None,
            false,
            None,
            false,
            None,
            false,
            None,
            false,
            None,
            None,
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            None,
        );
        assert!(matches!(result, Err(EngramError::Validation(_))));
    }

    #[test]
    fn test_create_reasoning_ibis_inline() {
        let mut storage = create_test_storage();
        create_reasoning(
            &mut storage,
            Some("IBIS Decision".to_string()),
            Some("task-1".to_string()),
            Some("agent".to_string()),
            None,
            None,
            None,
            false,
            None,
            false,
            None,
            false,
            None,
            true,
            None,
            Some("Which library?".to_string()),
            vec!["Use gix".to_string()],
            vec!["Pure Rust".to_string()],
            Vec::new(),
            Vec::new(),
            None,
        )
        .unwrap();

        let chains = storage.query_by_agent("agent", Some("reasoning")).unwrap();
        let reasoning = Reasoning::from_generic(chains[0].clone()).unwrap();
        assert_eq!(reasoning.ibis_mode, Some(true));
        assert_eq!(reasoning.positions.len(), 3);
        assert_eq!(
            reasoning.positions[0].position_type,
            IbisPositionType::Issue
        );
        assert_eq!(
            reasoning.positions[1].position_type,
            IbisPositionType::Position
        );
        assert_eq!(
            reasoning.positions[2].position_type,
            IbisPositionType::Argument
        );
        assert_eq!(reasoning.steps.len(), 3);
        assert!(reasoning.steps[0].description.contains("[Issue]"));
    }

    #[test]
    fn test_create_reasoning_ibis_no_positions_errors() {
        let mut storage = create_test_storage();
        let result = create_reasoning(
            &mut storage,
            Some("Bad IBIS".to_string()),
            Some("task-1".to_string()),
            None,
            None,
            None,
            None,
            false,
            None,
            false,
            None,
            false,
            None,
            true,
            None,
            None,
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            None,
        );
        assert!(matches!(result, Err(EngramError::Validation(_))));
    }

    #[test]
    fn test_create_reasoning_prov_o() {
        let mut storage = create_test_storage();
        create_reasoning(
            &mut storage,
            Some("PROV test".to_string()),
            Some("task-1".to_string()),
            Some("agent".to_string()),
            None,
            None,
            None,
            false,
            None,
            false,
            None,
            false,
            None,
            false,
            None,
            None,
            Vec::new(),
            Vec::new(),
            vec!["entity-a".to_string()],
            vec!["entity-c".to_string()],
            Some("orchestrator".to_string()),
        )
        .unwrap();

        let chains = storage.query_by_agent("agent", Some("reasoning")).unwrap();
        let reasoning = Reasoning::from_generic(chains[0].clone()).unwrap();
        assert_eq!(reasoning.prov_used, vec!["entity-a"]);
        assert_eq!(reasoning.prov_generated, vec!["entity-c"]);
        assert_eq!(
            reasoning.prov_attributed_to.as_deref(),
            Some("orchestrator")
        );
    }

    #[test]
    fn test_show_reasoning_with_ibis() {
        let mut storage = create_test_storage();
        create_reasoning(
            &mut storage,
            Some("IBIS Show".to_string()),
            Some("task-1".to_string()),
            None,
            None,
            None,
            None,
            false,
            None,
            false,
            None,
            false,
            None,
            true,
            None,
            Some("Issue?".to_string()),
            vec!["Pos A".to_string()],
            vec!["Arg B".to_string()],
            Vec::new(),
            Vec::new(),
            None,
        )
        .unwrap();

        let chains = storage
            .query_by_agent("default", Some("reasoning"))
            .unwrap();
        let id = &chains[0].id;
        assert!(show_reasoning(&storage, id).is_ok());
    }

    #[test]
    fn test_show_reasoning_with_prov_o() {
        let mut storage = create_test_storage();
        create_reasoning(
            &mut storage,
            Some("PROV Show".to_string()),
            Some("task-1".to_string()),
            None,
            None,
            None,
            None,
            false,
            None,
            false,
            None,
            false,
            None,
            false,
            None,
            None,
            Vec::new(),
            Vec::new(),
            vec!["used-1".to_string()],
            vec!["gen-1".to_string()],
            Some("agent-x".to_string()),
        )
        .unwrap();

        let chains = storage
            .query_by_agent("default", Some("reasoning"))
            .unwrap();
        let id = &chains[0].id;
        assert!(show_reasoning(&storage, id).is_ok());
    }
}

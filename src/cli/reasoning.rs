//! Reasoning command implementations

use crate::entities::{Entity, Reasoning};
use crate::error::EngramError;
use crate::storage::Storage;
use clap::Subcommand;
use serde::Deserialize;
use std::fs;
use std::io::{self, Read};

/// Reasoning input structure for JSON
#[derive(Debug, Deserialize)]
pub struct ReasoningInput {
    pub title: String,
    pub task_id: String,
    pub agent: Option<String>,
    pub tags: Option<Vec<String>>,
}

/// Reasoning commands
#[derive(Debug, Subcommand)]
pub enum ReasoningCommands {
    /// Create a new reasoning chain
    Create {
        /// Reasoning title
        #[arg(long, short, conflicts_with_all = ["title_stdin", "title_file"])]
        title: Option<String>,

        /// Task ID this reasoning belongs to
        #[arg(long)]
        task_id: Option<String>,

        /// Assigned agent
        #[arg(long, short)]
        agent: Option<String>,

        /// Initial confidence level (0.0 to 1.0)
        #[arg(long, short)]
        confidence: Option<f64>,

        /// Initial content/conclusion
        #[arg(long, conflicts_with_all = ["content_stdin", "content_file"])]
        content: Option<String>,

        /// Tags (comma-separated)
        #[arg(long)]
        tags: Option<String>,

        /// Read title from stdin
        #[arg(long, conflicts_with_all = ["title", "title_file"])]
        title_stdin: bool,

        /// Read title from file
        #[arg(long, conflicts_with_all = ["title", "title_stdin"])]
        title_file: Option<String>,

        /// Read content from stdin
        #[arg(long, conflicts_with_all = ["content", "content_file"])]
        content_stdin: bool,

        /// Read content from file
        #[arg(long, conflicts_with_all = ["content", "content_stdin"])]
        content_file: Option<String>,

        /// Create reasoning from JSON input (stdin or file)
        #[arg(long, conflicts_with_all = ["title", "title_stdin", "title_file"])]
        json: bool,

        /// JSON file path (requires --json)
        #[arg(long, requires = "json")]
        json_file: Option<String>,
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
) -> Result<(), EngramError> {
    if json {
        let json_content = if let Some(ref file_path) = json_file {
            read_file(file_path)?
        } else {
            read_stdin()?
        };

        let reasoning_input: ReasoningInput = serde_json::from_str(&json_content)
            .map_err(|e| EngramError::Validation(format!("Invalid JSON: {}", e)))?;

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

    // Set initial confidence if provided
    if let Some(conf) = confidence {
        reasoning.confidence = conf;
    }

    // Set initial content/conclusion if provided
    if content_stdin {
        reasoning.conclusion = read_stdin()?;
    } else if let Some(ref file_path) = content_file {
        reasoning.conclusion = read_file(file_path)?;
    } else if let Some(ref c) = content {
        reasoning.conclusion = c.clone();
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

pub fn list_reasoning<S: Storage>(
    storage: &S,
    agent: Option<&str>,
    task_id: Option<&str>,
    limit: Option<usize>,
) -> Result<(), EngramError> {
    let mut filter = crate::storage::QueryFilter {
        entity_type: Some("reasoning".to_string()),
        agent: agent.map(|s| s.to_string()),
        limit,
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

    println!("Found {} reasoning chain(s)", result.entities.len());
    println!();

    for entity in result.entities {
        if let Ok(reasoning) = Reasoning::from_generic(entity) {
            println!("ID: {}", reasoning.id);
            println!("Title: {}", reasoning.title);
            println!("Task ID: {}", reasoning.task_id);
            println!("Agent: {}", reasoning.agent);
            println!("Steps: {}", reasoning.steps.len());
            println!("Confidence: {:.2}", reasoning.confidence);
            if !reasoning.conclusion.is_empty() {
                println!("Status: Concluded");
            } else {
                println!("Status: In Progress");
            }
            println!(
                "Created: {}",
                reasoning.created_at.format("%Y-%m-%d %H:%M:%S")
            );
            println!("---");
        }
    }

    if result.has_more {
        println!("(More results available - use --limit to see more)");
    }

    Ok(())
}

pub fn show_reasoning<S: Storage>(storage: &S, id: &str) -> Result<(), EngramError> {
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

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

        /// Tags (comma-separated)
        #[arg(long)]
        tags: Option<String>,

        /// Read title from stdin
        #[arg(long, conflicts_with_all = ["title", "title_file"])]
        title_stdin: bool,

        /// Read title from file
        #[arg(long, conflicts_with_all = ["title", "title_stdin"])]
        title_file: Option<String>,

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
        #[arg(long, short)]
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
        #[arg(long, short)]
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
    tags: Option<String>,
    title_stdin: bool,
    title_file: Option<String>,
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

    let reasoning = Reasoning::new(final_title, final_task_id, final_agent.clone());

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

    println!("Add reasoning step - to be implemented");
    println!("ID: {}", id);
    println!("Description: {}", final_description);
    println!("Conclusion: {}", final_conclusion);
    println!("Confidence: {}", confidence);
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

    println!("Conclude reasoning - to be implemented");
    println!("ID: {}", id);
    println!("Conclusion: {}", final_conclusion);
    println!("Confidence: {}", confidence);
    Ok(())
}

pub fn list_reasoning<S: Storage>(
    _storage: &S,
    agent: Option<&str>,
    task_id: Option<&str>,
    limit: Option<usize>,
) -> Result<(), EngramError> {
    println!("List reasoning command - to be implemented");
    println!("Agent filter: {:?}", agent);
    println!("Task ID filter: {:?}", task_id);
    println!("Limit: {:?}", limit);
    Ok(())
}

pub fn show_reasoning<S: Storage>(_storage: &S, id: &str) -> Result<(), EngramError> {
    println!("Show reasoning command - to be implemented");
    println!("ID: {}", id);
    Ok(())
}

pub fn delete_reasoning<S: Storage>(_storage: &mut S, id: &str) -> Result<(), EngramError> {
    println!("Delete reasoning command - to be implemented");
    println!("ID: {}", id);
    Ok(())
}

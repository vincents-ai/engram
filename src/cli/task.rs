//! Task command implementations

use crate::entities::{Entity, Task, TaskPriority};
use crate::error::EngramError;
use crate::storage::Storage;
use clap::Subcommand;
use serde::Deserialize;
use std::fs;
use std::io::{self, Read};

/// Task input structure for JSON
#[derive(Debug, Deserialize)]
pub struct TaskInput {
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<String>,
    pub agent: Option<String>,
    pub parent: Option<String>,
    pub tags: Option<Vec<String>>,
}

/// Task commands
#[derive(Subcommand)]
pub enum TaskCommands {
    /// Create a new task
    Create {
        /// Task title
        #[arg(long, short, conflicts_with_all = ["title_stdin", "title_file"])]
        title: Option<String>,

        /// Task description
        #[arg(long, short, conflicts_with_all = ["description_stdin", "description_file"])]
        description: Option<String>,

        /// Priority (low, medium, high, critical)
        #[arg(long, short, default_value = "medium")]
        priority: String,

        /// Assigned agent
        #[arg(long, short)]
        agent: Option<String>,

        /// Parent task ID
        #[arg(long)]
        parent: Option<String>,

        /// Tags (comma-separated)
        #[arg(long)]
        tags: Option<String>,

        /// Read title from stdin
        #[arg(long, conflicts_with_all = ["title", "title_file"])]
        title_stdin: bool,

        /// Read title from file
        #[arg(long, conflicts_with_all = ["title", "title_stdin"])]
        title_file: Option<String>,

        /// Read description from stdin
        #[arg(long, conflicts_with_all = ["description", "description_file"])]
        description_stdin: bool,

        /// Read description from file
        #[arg(long, conflicts_with_all = ["description", "description_stdin"])]
        description_file: Option<String>,

        /// Create task from JSON input (stdin or file)
        #[arg(long, conflicts_with_all = ["title", "description", "title_stdin", "title_file", "description_stdin", "description_file"])]
        json: bool,

        /// JSON file path
        #[arg(long, requires = "json")]
        json_file: Option<String>,
    },
    /// List tasks
    List {
        /// Filter by agent
        #[arg(long, short)]
        agent: Option<String>,

        /// Filter by status
        #[arg(long, short)]
        status: Option<String>,

        /// Limit number of results
        #[arg(long, short)]
        limit: Option<usize>,
    },
    /// Show task details
    Show {
        /// Task ID
        #[arg(help = "Task ID to show")]
        id: String,
    },
    /// Update task status
    Update {
        /// Task ID
        #[arg(help = "Task ID to update")]
        id: String,

        /// New status
        #[arg(long, short)]
        status: String,

        /// Outcome (when completing task)
        #[arg(long)]
        outcome: Option<String>,
    },
    /// Delete a task
    Delete {
        /// Task ID
        #[arg(help = "Task ID to delete")]
        id: String,
    },
}

/// Read content from stdin
fn read_stdin() -> Result<String, EngramError> {
    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .map_err(|e| EngramError::Io(e))?;
    Ok(buffer.trim().to_string())
}

/// Read content from file
fn read_file(path: &str) -> Result<String, EngramError> {
    fs::read_to_string(path).map_err(EngramError::Io)
}

/// Create task from JSON input
fn create_task_from_input<S: Storage>(
    storage: &mut S,
    input: TaskInput,
) -> Result<(), EngramError> {
    let priority_enum = match input.priority.as_deref().unwrap_or("medium") {
        "low" => TaskPriority::Low,
        "medium" => TaskPriority::Medium,
        "high" => TaskPriority::High,
        "critical" => TaskPriority::Critical,
        _ => TaskPriority::Medium,
    };

    let mut task = Task::new(
        input.title,
        input.description.unwrap_or_default(),
        input.agent.unwrap_or_else(|| "default".to_string()),
        priority_enum,
    );

    if let Some(parent_id) = input.parent {
        task.parent = Some(parent_id);
    }

    if let Some(tags_vec) = input.tags {
        task.tags = tags_vec;
    }

    let generic = task.to_generic();
    storage.store(&generic)?;

    println!("✅ Task created:");
    display_task(&task);

    Ok(())
}

/// Create task command
pub fn create_task<S: Storage>(
    storage: &mut S,
    title: Option<String>,
    description: Option<String>,
    priority: &str,
    agent: Option<String>,
    parent: Option<String>,
    tags: Option<String>,
    // New parameters for flexible input
    title_stdin: bool,
    title_file: Option<String>,
    description_stdin: bool,
    description_file: Option<String>,
    json: bool,
    json_file: Option<String>,
) -> Result<(), EngramError> {
    // Handle JSON input first (overrides all other inputs)
    if json {
        let json_content = if let Some(ref file_path) = json_file {
            read_file(file_path)?
        } else {
            read_stdin()?
        };

        let task_input: TaskInput = serde_json::from_str(&json_content)
            .map_err(|e| EngramError::Validation(format!("Invalid JSON: {}", e)))?;

        return create_task_from_input(storage, task_input);
    }

    // Resolve title from various sources
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

    // Resolve description from various sources
    let _final_description = if description_stdin {
        Some(read_stdin()?)
    } else if let Some(ref file_path) = description_file {
        Some(read_file(file_path)?)
    } else {
        description.as_ref().map(|s| s.clone())
    };

    // Resolve description from various sources
    let final_description = if description_stdin {
        Some(read_stdin()?)
    } else if let Some(file_path) = description_file {
        Some(read_file(&file_path)?)
    } else {
        description.map(|s| s.to_string())
    };

    let priority_enum = match priority {
        "low" => TaskPriority::Low,
        "medium" => TaskPriority::Medium,
        "high" => TaskPriority::High,
        "critical" => TaskPriority::Critical,
        _ => TaskPriority::Medium,
    };

    let mut task = Task::new(
        final_title,
        final_description.unwrap_or_default(),
        agent.unwrap_or_else(|| "default".to_string()),
        priority_enum,
    );

    if let Some(parent_id) = parent {
        task.parent = Some(parent_id.to_string());
    }

    if let Some(tags_str) = tags {
        task.tags = tags_str.split(',').map(|s| s.trim().to_string()).collect();
    }

    let generic = task.to_generic();
    storage.store(&generic)?;

    println!("✅ Task created:");
    display_task(&task);

    Ok(())
}

/// List tasks command
pub fn list_tasks<S: Storage>(
    storage: &S,
    agent: Option<&str>,
    status: Option<&str>,
    limit: Option<usize>,
) -> Result<(), EngramError> {
    let mut tasks = storage.query_by_agent(agent.unwrap_or("default"), Some("task"))?;

    // Filter by status if specified
    if let Some(status_filter) = status {
        tasks.retain(|generic_task| {
            if let Ok(task_obj) = Task::from_generic(generic_task.clone()) {
                format!("{:?}", task_obj.status).to_lowercase() == status_filter.to_lowercase()
            } else {
                false
            }
        });
    }

    // Apply limit
    if let Some(limit_val) = limit {
        tasks.truncate(limit_val);
    }

    if tasks.is_empty() {
        println!("No tasks found");
        return Ok(());
    }

    println!("📋 Tasks ({} found):", tasks.len());
    for generic_task in &tasks {
        if let Ok(task_obj) = Task::from_generic(generic_task.clone()) {
            display_task_summary(&task_obj);
        }
    }

    Ok(())
}

pub fn show_task<S: Storage>(storage: &S, id: &str) -> Result<(), EngramError> {
    if let Some(generic_task) = storage.get(id, "task")? {
        if let Ok(task_obj) = Task::from_generic(generic_task) {
            println!("📋 Task Details:");
            display_task(&task_obj);
        } else {
            return Err(EngramError::Validation("Invalid task type".to_string()));
        }
    } else {
        return Err(EngramError::NotFound(format!("Task '{}' not found", id)));
    }

    Ok(())
}

/// Update task command
pub fn update_task<S: Storage>(
    storage: &mut S,
    id: &str,
    status: &str,
    outcome: Option<&str>,
) -> Result<(), EngramError> {
    let existing_generic = storage
        .get(id, "task")?
        .ok_or_else(|| EngramError::NotFound(format!("Task '{}' not found", id)))?;

    if let Ok(task) = Task::from_generic(existing_generic) {
        let mut updated_task = task;

        match status.to_lowercase().as_str() {
            "in_progress" | "in-progress" => updated_task.start(),
            "done" | "completed" => {
                if let Some(outcome_text) = outcome {
                    updated_task.complete(outcome_text.to_string());
                } else {
                    updated_task.complete("Task completed".to_string());
                }
            }
            "blocked" => updated_task.status = crate::entities::TaskStatus::Blocked,
            "cancelled" => updated_task.status = crate::entities::TaskStatus::Cancelled,
            _ => {
                return Err(EngramError::Validation(format!(
                    "Invalid status: {}",
                    status
                )))
            }
        }

        let updated_generic = updated_task.to_generic();
        storage.store(&updated_generic)?;

        println!("✅ Task updated:");
        display_task(&updated_task);

        Ok(())
    } else {
        Err(EngramError::Validation("Invalid task type".to_string()))
    }
}

/// Delete task command
pub fn delete_task<S: Storage>(storage: &mut S, id: &str) -> Result<(), EngramError> {
    storage.delete(id, "task")?;
    println!("✅ Task '{}' deleted", id);
    Ok(())
}

/// Display task information
fn display_task(task: &Task) {
    println!("  ID: {}", task.id);
    println!("  Title: {}", task.title);
    println!("  Description: {}", task.description);
    println!("  Status: {:?}", task.status);
    println!("  Priority: {:?}", task.priority);
    println!("  Agent: {}", task.agent);
    println!(
        "  Created: {}",
        task.start_time.format("%Y-%m-%d %H:%M:%S UTC")
    );
    if let Some(end_time) = task.end_time {
        println!("  Completed: {}", end_time.format("%Y-%m-%d %H:%M:%S UTC"));
    }
    if let Some(outcome) = &task.outcome {
        println!("  Outcome: {}", outcome);
    }
    if !task.tags.is_empty() {
        println!("  Tags: {}", task.tags.join(", "));
    }
    if !task.context_ids.is_empty() {
        println!("  Contexts: {}", task.context_ids.join(", "));
    }
    println!();
}

/// Display task summary for lists
fn display_task_summary(task: &Task) {
    println!(
        "  • {} [{}] - {} ({})",
        task.id,
        format!("{:?}", task.status).to_lowercase(),
        task.title,
        task.agent
    );
    if let Some(outcome) = &task.outcome {
        println!("    Outcome: {}", outcome);
    }
}

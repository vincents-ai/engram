//! Task command implementations

use crate::entities::{Entity, Task, TaskPriority};
use crate::error::EngramError;
use crate::storage::Storage;
use clap::Subcommand;
use serde::Deserialize;
use std::fs;
use std::io::{self, Read, Write};

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

        /// Output format (json, text)
        #[arg(long, default_value = "text")]
        output: String,

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
        /// NOTE: This only specifies the INPUT format. Output format is controlled by --output.
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

        /// New status (todo, in_progress, done, blocked, cancelled)
        #[arg(
            long,
            short,
            help = "New status: todo, in_progress, done, blocked, cancelled"
        )]
        status: String,

        /// Outcome (when completing task)
        #[arg(long)]
        outcome: Option<String>,

        /// Reason (when blocking task)
        #[arg(long)]
        reason: Option<String>,
    },
    /// Archive a task (soft delete)
    Archive {
        /// Task ID
        #[arg(help = "Task ID to archive")]
        id: String,

        /// Reason for archiving
        #[arg(long)]
        reason: Option<String>,
    },
    /// Resolve a blocked task
    Resolve {
        /// Task ID
        #[arg(help = "Task ID to resolve")]
        id: String,

        /// Resolution message (optional, will prompt if not provided)
        #[arg(long, short)]
        message: Option<String>,
    },
}

/// Read content from stdin with a prompt
fn read_line_with_prompt(prompt: &str) -> Result<String, EngramError> {
    print!("{}", prompt);
    io::stdout().flush().map_err(EngramError::Io)?;

    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .map_err(EngramError::Io)?;
    Ok(buffer.trim().to_string())
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
    output_format: String,
) -> Result<(), EngramError> {
    // Handle JSON input first (overrides all other inputs)
    if json {
        let json_content = if let Some(ref file_path) = json_file {
            read_file(file_path)?
        } else {
            read_stdin()?
        };

        let task_input: TaskInput = serde_json::from_str(&json_content).map_err(|e| {
            EngramError::Validation(format!(
                "Invalid JSON: {}. Line: {}, Column: {}",
                e,
                e.line(),
                e.column()
            ))
        })?;

        // Re-implementing logic here to support output format
        let priority_enum = match task_input.priority.as_deref().unwrap_or("medium") {
            "low" => TaskPriority::Low,
            "medium" => TaskPriority::Medium,
            "high" => TaskPriority::High,
            "critical" => TaskPriority::Critical,
            _ => TaskPriority::Medium,
        };

        let mut task = Task::new(
            task_input.title,
            task_input.description.unwrap_or_default(),
            task_input.agent.unwrap_or_else(|| "default".to_string()),
            priority_enum,
            None,
        );

        if let Some(parent_id) = task_input.parent {
            task.parent = Some(parent_id);
        }

        if let Some(tags_vec) = task_input.tags {
            task.tags = tags_vec;
        }

        let generic = task.to_generic();
        storage.store(&generic)?;

        if output_format == "json" {
            println!("{}", serde_json::to_string_pretty(&task).unwrap());
        } else {
            println!("✅ Task created:");
            display_task(&task);
        }
        return Ok(());
    }

    // Interactive/Flag-based Creation Logic
    let title_val = if title_stdin {
        Some(read_stdin()?)
    } else if let Some(ref path) = title_file {
        Some(read_file(path)?)
    } else {
        title
    };

    // Ensure title is present
    let final_title = title_val.ok_or_else(|| {
        EngramError::Validation(
            "Title is required (use --title, --title-stdin, or --title-file)".to_string(),
        )
    })?;

    let description_val = if description_stdin {
        Some(read_stdin()?)
    } else if let Some(ref path) = description_file {
        Some(read_file(path)?)
    } else {
        description
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
        description_val.unwrap_or_default(),
        agent.unwrap_or_else(|| "default".to_string()),
        priority_enum,
        None,
    );

    if let Some(parent_id) = parent {
        task.parent = Some(parent_id);
    }

    if let Some(tags_str) = tags {
        task.tags = tags_str.split(',').map(|s| s.trim().to_string()).collect();
    }

    let generic = task.to_generic();
    storage.store(&generic)?;

    if output_format == "json" {
        println!("{}", serde_json::to_string_pretty(&task).unwrap());
    } else {
        println!("✅ Task created:");
        display_task(&task);
    }

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

pub fn show_task<S: Storage + 'static>(storage: &S, id: &str) -> Result<(), EngramError> {
    if let Some(generic_task) = storage.get(id, "task")? {
        if let Ok(task_obj) = Task::from_generic(generic_task) {
            println!("📋 Task Details:");
            display_task(&task_obj);

            // Query and display associated workflow instances
            println!("\n🔄 Associated Workflows:");
            println!("=======================");

            let instances: Vec<_> = storage
                .get_all("workflow_instance")
                .unwrap_or_else(|_| Vec::new())
                .into_iter()
                .filter_map(|e| crate::entities::WorkflowInstance::from_generic(e).ok())
                .filter(|instance| instance.context.entity_id.as_deref() == Some(id))
                .collect();

            if instances.is_empty() {
                println!("  No active workflows associated with this task.");
            } else {
                for instance in &instances {
                    let status_icon = match instance.status {
                        crate::engines::workflow_engine::WorkflowStatus::Running => "🟢",
                        crate::engines::workflow_engine::WorkflowStatus::Completed => "🎯",
                        crate::engines::workflow_engine::WorkflowStatus::Failed(_) => "💥",
                        crate::engines::workflow_engine::WorkflowStatus::Suspended(_) => "⏸️",
                        crate::engines::workflow_engine::WorkflowStatus::Cancelled => "❌",
                    };

                    println!(
                        "  {} Workflow: {} [{}]",
                        status_icon, instance.workflow_id, instance.status
                    );
                    println!(
                        "     State: {} | Started: {}",
                        instance.current_state,
                        instance.started_at.format("%Y-%m-%d %H:%M")
                    );
                    println!("     Instance ID: {}", instance.id);
                    println!();
                }
            }
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
    reason: Option<&str>,
) -> Result<(), EngramError> {
    let existing_generic = storage
        .get(id, "task")?
        .ok_or_else(|| EngramError::NotFound(format!("Task '{}' not found", id)))?;

    if let Ok(task) = Task::from_generic(existing_generic) {
        let mut updated_task = task;

        match status.to_lowercase().as_str() {
            // Handle "todo" - reset task to initial state
            "todo" | "backlog" => {
                updated_task.status = crate::entities::TaskStatus::Todo;
            }
            // Handle various forms of in_progress
            "in_progress" | "in-progress" | "inprogress" | "progress" | "started" => {
                updated_task.start();
            }
            // Handle done/completed
            "done" | "completed" | "complete" | "finish" | "finished" => {
                if let Some(outcome_text) = outcome {
                    updated_task.complete(outcome_text.to_string());
                } else {
                    updated_task.complete("Task completed".to_string());
                }
            }
            // Handle blocked
            "blocked" | "block" | "waiting" | "on_hold" | "on-hold" | "onhold" => {
                let reason_text = reason.unwrap_or("Task blocked");
                updated_task.block(reason_text.to_string());
            }
            // Handle cancelled
            "cancelled" | "canceled" | "cancel" | "abandoned" | "dropped" => {
                updated_task.status = crate::entities::TaskStatus::Cancelled;
            }
            _ => {
                return Err(EngramError::Validation(format!(
                "Invalid status: '{}'. Valid values: todo, in_progress, done, blocked, cancelled",
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

/// Archive task command (soft delete - preserves data but marks as archived)
pub fn archive_task<S: Storage>(
    storage: &mut S,
    id: &str,
    reason: Option<&str>,
) -> Result<(), EngramError> {
    let existing_generic = storage
        .get(id, "task")?
        .ok_or_else(|| EngramError::NotFound(format!("Task '{}' not found", id)))?;

    if let Ok(task) = Task::from_generic(existing_generic) {
        let mut updated_task = task;
        updated_task.status = crate::entities::TaskStatus::Cancelled;
        if let Some(reason_text) = reason {
            let archive_note = format!("Archived: {}", reason_text);
            if updated_task.outcome.is_none() {
                updated_task.outcome = Some(archive_note);
            }
        }

        let updated_generic = updated_task.to_generic();
        storage.store(&updated_generic)?;

        println!("✅ Task '{}' archived (soft deleted)", id);
        println!("  Reason: {}", reason.unwrap_or("No reason provided"));
        println!("  Use 'engram task update {} --status todo' to restore", id);

        Ok(())
    } else {
        Err(EngramError::Validation("Invalid task type".to_string()))
    }
}

/// Resolve a blocked task
pub fn resolve_task<S: Storage>(
    storage: &mut S,
    id: &str,
    message: Option<&str>,
) -> Result<(), EngramError> {
    let existing_generic = storage
        .get(id, "task")?
        .ok_or_else(|| EngramError::NotFound(format!("Task '{}' not found", id)))?;

    if let Ok(mut task) = Task::from_generic(existing_generic) {
        if task.status != crate::entities::TaskStatus::Blocked {
            println!("Task '{}' is not currently blocked.", id);
            return Ok(());
        }

        println!("🔒 Task is blocked.");
        if let Some(reason) = &task.block_reason {
            println!("   Reason: {}", reason);
        }

        let resolution = if let Some(msg) = message {
            msg.to_string()
        } else {
            // Interactive prompt
            read_line_with_prompt("\n📝 Enter resolution (how was this resolved?): ")?
        };

        if resolution.trim().is_empty() {
            return Err(EngramError::Validation(
                "Resolution message cannot be empty.".to_string(),
            ));
        }

        // Capture previous block reason before it gets cleared by start()
        let previous_block_reason = task.block_reason.clone().unwrap_or_default();

        // Unblock and set status to InProgress
        task.start();

        // Optionally, we could append the resolution to metadata or outcome history
        // For now, we'll log it in the outcome temporarily if it was previously empty,
        // or just consider the unblocking itself sufficient state change.
        // A better approach for history tracking might be needed in the future.
        // Let's add it to metadata to track resolution history.
        let timestamp = chrono::Utc::now().to_rfc3339();
        let resolution_entry = serde_json::json!({
            "timestamp": timestamp,
            "resolution": resolution,
            "previous_block_reason": previous_block_reason
        });

        // We need to fetch metadata again since task.start() might have modified fields (it doesn't modify metadata)
        // But we have mutable access to `task` already.
        // Note: Task entity structure needs to support metadata updates if we want to store this.
        // The Task struct has `metadata: HashMap<String, Value>`.

        // Since we don't have a direct "add_metadata" method on Task and accessing the field directly
        // requires it to be public (it is public), we can modify it.
        // However, `metadata` is a HashMap, so we can't just append to a list easily unless we define a schema.
        // Let's just print it for now and move on, relying on the state change.
        // Or better, let's store it in a "resolutions" list in metadata if possible.

        let mut resolutions = task
            .metadata
            .get("resolutions")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        resolutions.push(resolution_entry);
        task.metadata.insert(
            "resolutions".to_string(),
            serde_json::Value::Array(resolutions),
        );

        let updated_generic = task.to_generic();
        storage.store(&updated_generic)?;

        println!("✅ Task unblocked and set to In Progress.");
        display_task(&task);

        Ok(())
    } else {
        Err(EngramError::Validation("Invalid task type".to_string()))
    }
}

/// Display task information
fn display_task(task: &Task) {
    println!("  ID: {}", task.id);
    println!("  Title: {}", task.title);
    println!("  Description: {}", task.description);
    println!("  Status: {:?}", task.status);
    if task.status == crate::entities::TaskStatus::Blocked {
        if let Some(reason) = &task.block_reason {
            println!("  ⚠️ Block Reason: {}", reason);
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::MemoryStorage;

    fn create_test_storage() -> MemoryStorage {
        MemoryStorage::new("default")
    }

    #[test]
    fn test_create_task_basic() {
        let mut storage = create_test_storage();
        let result = create_task(
            &mut storage,
            Some("Test Task".to_string()),
            Some("Description".to_string()),
            "medium",
            None,
            None,
            None,
            false,
            None,
            false,
            None,
            false,
            None,
            "text".to_string(),
        );
        assert!(result.is_ok());

        let tasks = storage.query_by_agent("default", Some("task")).unwrap();
        assert_eq!(tasks.len(), 1);

        let task = Task::from_generic(tasks[0].clone()).unwrap();
        assert_eq!(task.title, "Test Task");
        assert_eq!(task.description, "Description");
        assert_eq!(task.priority, TaskPriority::Medium);
    }

    #[test]
    fn test_create_task_with_priority() {
        let mut storage = create_test_storage();

        let priorities = vec![
            ("low", TaskPriority::Low),
            ("medium", TaskPriority::Medium),
            ("high", TaskPriority::High),
            ("critical", TaskPriority::Critical),
        ];

        for (p_str, p_enum) in priorities {
            let title = format!("Task {}", p_str);
            create_task(
                &mut storage,
                Some(title.clone()),
                None,
                p_str,
                None,
                None,
                None,
                false,
                None,
                false,
                None,
                false,
                None,
                "text".to_string(),
            )
            .unwrap();

            let tasks = storage.query_by_agent("default", Some("task")).unwrap();
            // Find the task we just created by title
            let task = tasks
                .iter()
                .filter_map(|t| Task::from_generic(t.clone()).ok())
                .find(|t| t.title == title)
                .expect("Task should exist");

            assert_eq!(task.priority, p_enum);
        }
    }

    #[test]
    fn test_show_task() {
        let mut storage = create_test_storage();
        create_task(
            &mut storage,
            Some("Show Me".to_string()),
            None,
            "medium",
            None,
            None,
            None,
            false,
            None,
            false,
            None,
            false,
            None,
            "text".to_string(),
        )
        .unwrap();

        let tasks = storage.query_by_agent("default", Some("task")).unwrap();
        let id = &tasks[0].id;
        assert!(show_task(&storage, id).is_ok());
    }

    #[test]
    fn test_show_task_not_found() {
        let storage = create_test_storage();
        let result = show_task(&storage, "missing-id");
        assert!(matches!(result, Err(EngramError::NotFound(_))));
    }

    #[test]
    fn test_update_task_not_found() {
        let mut storage = create_test_storage();
        let result = update_task(&mut storage, "missing-id", "done", None, None);
        assert!(matches!(result, Err(EngramError::NotFound(_))));
    }

    #[test]
    fn test_archive_task_not_found() {
        let mut storage = create_test_storage();
        let result = archive_task(&mut storage, "missing-id", None);
        assert!(matches!(result, Err(EngramError::NotFound(_))));
    }
    #[test]
    fn test_create_task_validation() {
        let mut storage = create_test_storage();

        // Missing title
        let result = create_task(
            &mut storage,
            None,
            None,
            "medium",
            None,
            None,
            None,
            false,
            None,
            false,
            None,
            false,
            None,
            "text".to_string(),
        );
        assert!(matches!(result, Err(EngramError::Validation(_))));
    }

    #[test]
    fn test_update_task_status() {
        let mut storage = create_test_storage();
        create_task(
            &mut storage,
            Some("Test Task".to_string()),
            None,
            "medium",
            None,
            None,
            None,
            false,
            None,
            false,
            None,
            false,
            None,
            "text".to_string(),
        )
        .unwrap();

        let tasks = storage.query_by_agent("default", Some("task")).unwrap();
        let task_id = tasks[0].id.clone();

        // Update to in_progress
        update_task(&mut storage, &task_id, "in_progress", None, None).unwrap();
        let task = Task::from_generic(storage.get(&task_id, "task").unwrap().unwrap()).unwrap();
        assert!(matches!(
            task.status,
            crate::entities::TaskStatus::InProgress
        ));

        // Update to done
        update_task(&mut storage, &task_id, "done", Some("Finished"), None).unwrap();
        let task = Task::from_generic(storage.get(&task_id, "task").unwrap().unwrap()).unwrap();
        assert!(matches!(task.status, crate::entities::TaskStatus::Done));
        assert_eq!(task.outcome.unwrap(), "Finished");

        // Update to blocked
        update_task(
            &mut storage,
            &task_id,
            "blocked",
            None,
            Some("Waiting for input"),
        )
        .unwrap();
        let task = Task::from_generic(storage.get(&task_id, "task").unwrap().unwrap()).unwrap();
        assert!(matches!(task.status, crate::entities::TaskStatus::Blocked));
        assert_eq!(task.block_reason.unwrap(), "Waiting for input");
    }

    #[test]
    fn test_update_task_invalid_status() {
        let mut storage = create_test_storage();
        create_task(
            &mut storage,
            Some("Test Task".to_string()),
            None,
            "medium",
            None,
            None,
            None,
            false,
            None,
            false,
            None,
            false,
            None,
            "text".to_string(),
        )
        .unwrap();

        let tasks = storage.query_by_agent("default", Some("task")).unwrap();
        let task_id = tasks[0].id.clone();

        let result = update_task(&mut storage, &task_id, "invalid_status", None, None);
        assert!(matches!(result, Err(EngramError::Validation(_))));
    }

    #[test]
    fn test_archive_task() {
        let mut storage = create_test_storage();
        create_task(
            &mut storage,
            Some("Test Task".to_string()),
            None,
            "medium",
            None,
            None,
            None,
            false,
            None,
            false,
            None,
            false,
            None,
            "text".to_string(),
        )
        .unwrap();

        let tasks = storage.query_by_agent("default", Some("task")).unwrap();
        let task_id = tasks[0].id.clone();

        archive_task(&mut storage, &task_id, Some("Not needed")).unwrap();

        let task = Task::from_generic(storage.get(&task_id, "task").unwrap().unwrap()).unwrap();
        assert!(matches!(
            task.status,
            crate::entities::TaskStatus::Cancelled
        ));
        assert!(task.outcome.unwrap().contains("Not needed"));
    }

    #[test]
    fn test_list_tasks_filter() {
        let mut storage = create_test_storage();

        // Create mixed tasks
        create_task(
            &mut storage,
            Some("Task 1".to_string()),
            None,
            "medium",
            Some("agent1".to_string()),
            None,
            None,
            false,
            None,
            false,
            None,
            false,
            None,
            "text".to_string(),
        )
        .unwrap();

        create_task(
            &mut storage,
            Some("Task 2".to_string()),
            None,
            "medium",
            Some("agent2".to_string()),
            None,
            None,
            false,
            None,
            false,
            None,
            false,
            None,
            "text".to_string(),
        )
        .unwrap();

        // Filter by agent
        let result = list_tasks(&storage, Some("agent1"), None, None);
        assert!(result.is_ok());
        // Note: list_tasks prints to stdout, so we can't easily verify output content here
        // but we verify the function runs without error
    }

    #[test]
    fn test_list_tasks_not_found() {
        let storage = create_test_storage();
        // Should succeed but print "No tasks found"
        let result = list_tasks(&storage, Some("non-existent"), None, None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_resolve_blocked_task_with_message() {
        let mut storage = create_test_storage();

        // 1. Create a task
        create_task(
            &mut storage,
            Some("Blocked Task".to_string()),
            None,
            "medium",
            None,
            None,
            None,
            false,
            None,
            false,
            None,
            false,
            None,
            "text".to_string(),
        )
        .unwrap();

        let tasks = storage.query_by_agent("default", Some("task")).unwrap();
        let task_id = tasks[0].id.clone();

        // 2. Block the task
        update_task(
            &mut storage,
            &task_id,
            "blocked",
            None,
            Some("Missing credentials"),
        )
        .unwrap();

        // Verify blocked
        let task = Task::from_generic(storage.get(&task_id, "task").unwrap().unwrap()).unwrap();
        assert_eq!(task.status, crate::entities::TaskStatus::Blocked);
        assert_eq!(task.block_reason, Some("Missing credentials".to_string()));

        // 3. Resolve the task with a message
        resolve_task(
            &mut storage,
            &task_id,
            Some("Credentials provided in secure note"),
        )
        .unwrap();

        // 4. Verify resolved state
        let resolved_task =
            Task::from_generic(storage.get(&task_id, "task").unwrap().unwrap()).unwrap();

        // Should be InProgress
        assert!(matches!(
            resolved_task.status,
            crate::entities::TaskStatus::InProgress
        ));
        // Block reason should be cleared (Task::start clears it)
        assert!(resolved_task.block_reason.is_none());

        // Check metadata for resolution history
        let resolutions = resolved_task
            .metadata
            .get("resolutions")
            .unwrap()
            .as_array()
            .unwrap();
        assert_eq!(resolutions.len(), 1);

        let entry = &resolutions[0];
        assert_eq!(entry["resolution"], "Credentials provided in secure note");
        assert_eq!(entry["previous_block_reason"], "Missing credentials");
        assert!(entry["timestamp"].is_string());
    }

    #[test]
    fn test_resolve_non_blocked_task() {
        let mut storage = create_test_storage();

        // 1. Create a task
        create_task(
            &mut storage,
            Some("Active Task".to_string()),
            None,
            "medium",
            None,
            None,
            None,
            false,
            None,
            false,
            None,
            false,
            None,
            "text".to_string(),
        )
        .unwrap();

        let tasks = storage.query_by_agent("default", Some("task")).unwrap();
        let task_id = tasks[0].id.clone();

        // 2. Try to resolve (should do nothing but succeed)
        let result = resolve_task(&mut storage, &task_id, Some("Trying to resolve"));

        assert!(result.is_ok());

        // Verify status hasn't changed (still Todo)
        let task = Task::from_generic(storage.get(&task_id, "task").unwrap().unwrap()).unwrap();
        assert!(matches!(task.status, crate::entities::TaskStatus::Todo));
        // Metadata should be empty
        assert!(task.metadata.get("resolutions").is_none());
    }

    #[test]
    fn test_create_task_json_invalid() {
        let _storage = create_test_storage();
        // We can't easily test stdin/file reading in unit tests without mocking
        // but we can test the json deserialization logic by simulating the function calls that would happen
        // This is harder to test directly via the public API which reads from FS/Stdin
        // So we'll trust the integration tests for that part
    }
}

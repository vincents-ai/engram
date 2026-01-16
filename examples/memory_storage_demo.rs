//! Demonstration using memory storage (no Git dependency)

use engram::entities::Task;
use engram::storage::{memory_only_storage::MemoryStorage, Storage};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Engram Rust Implementation - Memory Storage Demo");
    println!("=============================================");

    // Initialize memory storage
    let mut storage = MemoryStorage::new("demo-agent");
    println!("✅ Memory storage initialized");

    // Create some sample tasks
    let tasks = vec![
        Task::new(
            "Setup project structure".to_string(),
            "Create directories and initial files".to_string(),
            "demo-agent".to_string(),
            engram::entities::TaskPriority::High,
        ),
        Task::new(
            "Implement core entities".to_string(),
            "Define Task, Context, Reasoning types".to_string(),
            "demo-agent".to_string(),
            engram::entities::TaskPriority::Medium,
        ),
        Task::new(
            "Add CLI interface".to_string(),
            "Create modular command structure".to_string(),
            "demo-agent".to_string(),
            engram::entities::TaskPriority::Medium,
        ),
    ];

    // Store tasks
    for task in &tasks {
        let generic_task = task.to_generic();
        storage.store(&generic_task)?;
        println!("✅ Task created: {}", task.id);
    }

    // Query tasks by agent
    let retrieved_tasks = storage.query_by_agent("demo-agent", Some("task"))?;
    println!("✅ Retrieved {} tasks", retrieved_tasks.len());

    // Display task details
    for task in &retrieved_tasks {
        if let Ok(task_obj) = task.as_any().downcast_ref::<Task>() {
            println!(
                "📋 Task: {} [{}] - {}",
                task_obj.id,
                format!("{:?}", task_obj.status).to_lowercase(),
                task_obj.title
            );
            println!("   Description: {}", task_obj.description);
            println!("   Priority: {:?}", task_obj.priority);
            println!(
                "   Created: {}",
                task_obj.start_time.format("%Y-%m-%d %H:%M")
            );
        }
    }

    // Show commit history
    let history = storage.history(Some(5))?;
    println!("✅ Recent operations:");
    for commit in &history {
        println!(
            "  • {} - {} ({})",
            commit.timestamp.format("%H:%M:%S"),
            commit.message,
            commit.author
        );
    }

    println!();
    println!("This demonstrates:");
    println!("1. ✅ In-memory storage without external dependencies");
    println!("2. ✅ Entity creation and validation");
    println!("3. ✅ Query operations with filtering");
    println!("4. ✅ Extensible entity system");
    println!("5. ✅ Operation history tracking");

    Ok(())
}

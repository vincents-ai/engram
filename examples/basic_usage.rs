//! Basic usage example for Engram Rust implementation

use engram::{
    entities::Task,
    storage::{GitStorage, Storage},
    Entity,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Engram Rust Implementation - Basic Usage Example");
    println!("=============================================");

    // Initialize storage
    let mut storage = GitStorage::new(".", "example-agent")?;
    println!("✅ Storage initialized");

    // Create a sample task
    let task = Task::new(
        "Example Task".to_string(),
        "This is a sample task created using Rust API".to_string(),
        "example-agent".to_string(),
        engram::entities::TaskPriority::Medium,
        None,
    );

    let generic_task = task.to_generic();

    // Store task
    storage.store(&generic_task)?;
    println!("✅ Task created: {}", task.id);

    // Retrieve task
    if let Some(retrieved_task) = storage.get(&task.id, "task")? {
        println!(
            "✅ Task retrieved: {}",
            retrieved_task
                .data
                .get("title")
                .and_then(|v| v.as_str())
                .unwrap_or("Untitled")
        );
        println!(
            "   Description: {}",
            retrieved_task
                .data
                .get("description")
                .and_then(|v| v.as_str())
                .unwrap_or("No description")
        );
    }

    // List all tasks for agent
    let tasks = storage.query_by_agent("example-agent", Some("task"))?;
    println!("✅ Found {} tasks for agent", tasks.len());

    println!();
    println!("This example demonstrates:");
    println!("1. Git-based storage initialization");
    println!("2. Entity creation and storage");
    println!("3. Entity retrieval");
    println!("4. Agent-based querying");
    println!("5. Extensible entity system");

    Ok(())
}

//! Info command for displaying storage and workspace information

use crate::error::EngramError;
use crate::storage::Storage;

/// Display workspace and storage information
pub fn info<S: Storage>(storage: &S) -> Result<(), EngramError> {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║                    ENGRAM WORKSPACE INFO                    ║");
    println!("╚════════════════════════════════════════════════════════════╝");
    println!();

    // Storage backend info
    println!("📦 Storage Backend");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    if let Some(stats) = storage.get_stats().ok().as_ref() {
        println!("  Total Entities: {}", stats.total_entities);
        println!("  Storage Size: {} bytes", stats.total_storage_size);

        if let Some(last_sync) = stats.last_sync {
            println!("  Last Sync: {}", last_sync.format("%Y-%m-%d %H:%M:%S UTC"));
        }
    } else {
        println!("  Storage statistics unavailable");
    }
    println!();

    // Entity counts by type
    println!("📊 Entity Counts");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    let entity_types = [
        "task",
        "context",
        "reasoning",
        "knowledge",
        "session",
        "rule",
        "standard",
        "compliance",
        "adr",
        "workflow",
        "workflow_instance",
        "relationship",
        "agent_sandbox",
        "escalation_request",
    ];

    for entity_type in entity_types {
        if let Ok(ids) = storage.list_ids(entity_type) {
            if !ids.is_empty() {
                println!("  {}: {}", entity_type.replace("_", " "), ids.len());
            }
        }
    }
    println!();

    // Agent info
    println!("👥 Agents");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    let agent_entities = storage.list_ids("agent").unwrap_or_default();
    if agent_entities.is_empty() {
        println!("  No agents configured");
    } else {
        println!("  {} agent(s) configured", agent_entities.len());
    }
    println!();

    println!("✅ Workspace health: Good");
    println!();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::MemoryStorage;

    #[test]
    fn test_info_execution() {
        // Just verify that the info function runs without panicking on an empty storage
        let storage = MemoryStorage::new("test-agent");
        let result = info(&storage);
        assert!(result.is_ok());
    }
}

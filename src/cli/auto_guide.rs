//! Auto-Guide logic for providing context-aware suggestions
//!
//! This module replicates the 'Auto-Guide' functionality from the TypeScript plugin,
//! analyzing the current session state and recently modified tasks to suggest
//! logical next steps to the user.

use crate::error::EngramError;
use crate::storage::Storage;

/// Configuration for Auto-Guide
#[derive(Debug, Clone)]
pub struct AutoGuideConfig {
    pub enabled: bool,
}

impl Default for AutoGuideConfig {
    fn default() -> Self {
        Self { enabled: true }
    }
}

/// Generate a suggestion based on current context
pub fn get_auto_guide_suggestion<S: Storage>(
    storage: &S,
    config: &AutoGuideConfig,
    context: Option<&str>,
) -> Result<Option<String>, EngramError> {
    if !config.enabled {
        return Ok(None);
    }

    // 1. Try to find active session
    // In a real implementation, we would track the current session ID in a local file
    // For now, let's query the most recently modified session
    let filter = crate::storage::QueryFilter {
        entity_type: Some("session".to_string()),
        limit: Some(1),
        sort_by: Some("timestamp".to_string()),
        sort_order: crate::storage::SortOrder::Desc,
        ..Default::default()
    };

    let _sessions = storage.query(&filter)?;

    // Heuristic 1: If we just committed to a task, suggest adding reasoning
    if let Some(ctx) = context {
        if ctx.contains("commit") {
            return Ok(Some("Consider adding a reasoning node to document why you made these changes: `engram reasoning create --task-id <ID>`".to_string()));
        }
    }

    // Heuristic 2: Check for open high-priority tasks
    let task_filter = crate::storage::QueryFilter {
        entity_type: Some("task".to_string()),
        field_filters: {
            let mut map = std::collections::HashMap::new();
            map.insert("status".to_string(), serde_json::json!("todo"));
            map.insert("priority".to_string(), serde_json::json!("high"));
            map
        },
        limit: Some(1),
        ..Default::default()
    };

    let tasks = storage.query(&task_filter)?;
    if let Some(task) = tasks.entities.first() {
        let title = task
            .data
            .get("title")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown Task");
        return Ok(Some(format!("You have a high-priority task pending: '{}'. Use `engram next` to start working on it.", title)));
    }

    // Heuristic 3: General nudge
    Ok(Some(
        "Tip: Keep your task graph connected by using `engram relationship create`.".to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::GenericEntity;
    use crate::storage::MemoryStorage;
    use crate::storage::Storage;
    use chrono::Utc;
    use serde_json::json;

    #[test]
    fn test_auto_guide_disabled() {
        let storage = MemoryStorage::new("test");
        let config = AutoGuideConfig { enabled: false };
        let result = get_auto_guide_suggestion(&storage, &config, None).unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn test_auto_guide_commit_context() {
        let storage = MemoryStorage::new("test");
        let config = AutoGuideConfig::default();
        let result =
            get_auto_guide_suggestion(&storage, &config, Some("git commit -m 'fix'")).unwrap();

        assert!(result.is_some());
        assert!(result.unwrap().contains("Consider adding a reasoning node"));
    }

    #[test]
    fn test_auto_guide_high_priority_task() {
        let mut storage = MemoryStorage::new("test");
        let task = GenericEntity {
            id: "task-1".to_string(),
            entity_type: "task".to_string(),
            agent: "test".to_string(),
            timestamp: Utc::now(),
            data: json!({
                "title": "Critical Bug",
                "status": "todo",
                "priority": "high"
            }),
        };
        storage.store(&task).unwrap();

        let config = AutoGuideConfig::default();
        let result = get_auto_guide_suggestion(&storage, &config, None).unwrap();

        assert!(result.is_some());
        let msg = result.unwrap();
        assert!(msg.contains("high-priority task pending"));
        assert!(msg.contains("Critical Bug"));
    }

    #[test]
    fn test_auto_guide_default_nudge() {
        let storage = MemoryStorage::new("test");
        let config = AutoGuideConfig::default();
        // No high priority tasks, no commit context
        let result = get_auto_guide_suggestion(&storage, &config, None).unwrap();

        assert!(result.is_some());
        assert!(result.unwrap().contains("Keep your task graph connected"));
    }
}

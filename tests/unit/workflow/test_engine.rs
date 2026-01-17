#[cfg(test)]
mod tests {
    use crate::storage::MemoryStorage;
    use crate::workflow::engine::WorkflowEngine;
    use std::sync::Arc;

    #[test]
    fn test_workflow_engine_creation() {
        let storage = Arc::new(MemoryStorage::new());
        let engine = WorkflowEngine::new(storage.clone());

        assert!(engine.is_ok());
    }

    #[test]
    fn test_can_advance_with_manual_trigger() {
        let storage = Arc::new(MemoryStorage::new());
        let engine = WorkflowEngine::new(storage.clone()).unwrap();

        // Create test task and workflow
        let task_id = "test-task-id".to_string();

        // This should return false initially (no workflow assigned)
        let can_advance = engine.can_advance(&task_id, "development").unwrap();
        assert!(!can_advance);
    }
}

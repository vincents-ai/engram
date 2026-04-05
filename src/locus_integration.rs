//! Integration layer between Locus and Engram
//!
//! Provides Locus with access to Engram's storage, entities,
//! and functionality through a clean API boundary.

use crate::entities::*;
use crate::error::EngramError;
use crate::storage::{RelationshipStorage, Storage};

/// Integration interface for Locus to access Engram functionality
pub struct LocusIntegration<S: Storage + RelationshipStorage> {
    storage: S,
}

impl<S: Storage + RelationshipStorage> LocusIntegration<S> {
    /// Create new Locus integration instance
    pub fn new(storage: S) -> Self {
        Self { storage }
    }

    /// Get all workflows from Engram storage
    pub fn get_workflows(&self) -> Result<Vec<Workflow>, EngramError> {
        let mut workflows = Vec::new();

        // Query Workflow entities from storage
        let entities = self.storage.get_all("workflow")?;

        for entity in entities {
            if let Ok(workflow) = serde_json::from_value::<Workflow>(entity.data) {
                workflows.push(workflow);
            }
        }

        Ok(workflows)
    }

    /// Get workflow by name
    pub fn get_workflow(&self, name: &str) -> Result<Option<Workflow>, EngramError> {
        let workflow_id = format!("workflow-{}", name);

        if let Some(entity) = self.storage.get(&workflow_id, "workflow")? {
            let workflow = serde_json::from_value::<Workflow>(entity.data)?;
            Ok(Some(workflow))
        } else {
            Ok(None)
        }
    }

    /// Save workflow to Engram storage
    pub fn save_workflow(&mut self, workflow: &Workflow) -> Result<String, EngramError> {
        let workflow_id = workflow.id.clone();

        // Convert to generic entity
        let entity = GenericEntity {
            id: workflow_id.clone(),
            entity_type: "workflow".to_string(),
            agent: workflow.agent.clone(),
            timestamp: workflow.created_at,
            data: serde_json::to_value(workflow)?,
        };

        self.storage.store(&entity)?;
        Ok(workflow_id)
    }

    /// Get all tasks for visualization
    pub fn get_tasks(&self, agent_filter: Option<&str>) -> Result<Vec<Task>, EngramError> {
        let mut tasks = Vec::new();

        let entities = self.storage.get_all("task")?;

        for entity in entities {
            if let Ok(task) = serde_json::from_value::<Task>(entity.data) {
                if let Some(agent) = agent_filter {
                    if task.agent.as_str() == agent {
                        tasks.push(task);
                    }
                } else {
                    tasks.push(task);
                }
            }
        }

        Ok(tasks)
    }

    /// Get execution results for dashboard
    pub fn get_execution_results(
        &self,
        task_id: Option<&str>,
        time_range: Option<(chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>)>,
    ) -> Result<Vec<ExecutionResult>, EngramError> {
        let mut results = Vec::new();

        let entities = self.storage.get_all("execution_result")?;

        for entity in entities {
            if let Ok(result) = serde_json::from_value::<ExecutionResult>(entity.data) {
                // Apply filters
                if let Some(filter_task_id) = task_id {
                    if result.task_id.to_string() != filter_task_id {
                        continue;
                    }
                }

                if let Some((start, end)) = time_range {
                    if result.timestamp < start || result.timestamp > end {
                        continue;
                    }
                }

                results.push(result);
            }
        }

        Ok(results)
    }

    /// Get system health metrics
    pub fn get_system_health(&self) -> Result<SystemHealth, EngramError> {
        let task_count = self.storage.get_all("task")?.len();
        let workflow_count = self.storage.get_all("workflow")?.len();
        let error_count = self
            .storage
            .get_all("execution_result")?
            .iter()
            .filter_map(|entity| {
                serde_json::from_value::<ExecutionResult>(entity.data.clone()).ok()
            })
            .filter(|result| {
                matches!(
                    result.validation_status,
                    crate::entities::ValidationStatus::Failed { .. }
                )
            })
            .count();

        Ok(SystemHealth {
            total_tasks: task_count,
            total_workflows: workflow_count,
            error_count,
            last_updated: chrono::Utc::now(),
        })
    }

    /// Create emergency override
    pub fn create_emergency_override(
        &mut self,
        task_id: &str,
        reason: &str,
        override_type: &str,
    ) -> Result<String, EngramError> {
        let override_id = uuid::Uuid::new_v4().to_string();

        // Create override entity
        let override_entity = GenericEntity {
            id: override_id.clone(),
            entity_type: "emergency_override".to_string(),
            agent: "locus".to_string(),
            timestamp: chrono::Utc::now(),
            data: serde_json::json!({
                "task_id": task_id,
                "reason": reason,
                "override_type": override_type,
                "status": "active",
                "created_by": "locus"
            }),
        };

        self.storage.store(&override_entity)?;

        // Create relationship to task
        let relationship = EntityRelationship::new(
            format!("rel-{}", uuid::Uuid::new_v4().to_string().split_at(8).0),
            "locus".to_string(),
            override_id.clone(),
            "emergency_override".to_string(),
            task_id.to_string(),
            "task".to_string(),
            EntityRelationType::Custom("overrides".to_string()),
        );
        self.storage.store_relationship(&relationship)?;

        Ok(override_id)
    }
}

/// System health metrics for dashboard
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SystemHealth {
    pub total_tasks: usize,
    pub total_workflows: usize,
    pub error_count: usize,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::TaskPriority;
    use crate::storage::memory_only_storage::MemoryStorage;

    fn make_integration() -> LocusIntegration<MemoryStorage> {
        LocusIntegration::new(MemoryStorage::new("test-agent"))
    }

    fn make_integration_mut() -> LocusIntegration<MemoryStorage> {
        LocusIntegration::new(MemoryStorage::new("test-agent"))
    }

    fn make_workflow_entity(id: &str, agent: &str) -> GenericEntity {
        GenericEntity {
            id: id.to_string(),
            entity_type: "workflow".to_string(),
            agent: agent.to_string(),
            timestamp: chrono::Utc::now(),
            data: serde_json::to_value(Workflow::new(
                format!("Workflow {}", id),
                "desc".to_string(),
                agent.to_string(),
            ))
            .unwrap(),
        }
    }

    fn make_task_entity(id: &str, agent: &str) -> GenericEntity {
        GenericEntity {
            id: id.to_string(),
            entity_type: "task".to_string(),
            agent: agent.to_string(),
            timestamp: chrono::Utc::now(),
            data: serde_json::to_value(Task::new(
                format!("Task {}", id),
                "desc".to_string(),
                agent.to_string(),
                TaskPriority::Medium,
                None,
            ))
            .unwrap(),
        }
    }

    fn make_execution_result_entity(
        id: &str,
        task_id: &str,
        agent: &str,
        status: ValidationStatus,
    ) -> GenericEntity {
        let mut result = ExecutionResult::new(
            task_id.to_string(),
            "dev".to_string(),
            "gate-1".to_string(),
            "cmd".to_string(),
            agent.to_string(),
        );
        result.id = id.to_string();
        result.validation_status = status;
        GenericEntity {
            id: id.to_string(),
            entity_type: "execution_result".to_string(),
            agent: agent.to_string(),
            timestamp: chrono::Utc::now(),
            data: serde_json::to_value(&result).unwrap(),
        }
    }

    #[test]
    fn test_new() {
        let _integration = make_integration();
    }

    #[test]
    fn test_get_workflows_empty() {
        let integration = make_integration();
        let workflows = integration.get_workflows().unwrap();
        assert!(workflows.is_empty());
    }

    #[test]
    fn test_get_workflows_success() {
        let mut storage = MemoryStorage::new("test-agent");
        storage
            .store(&make_workflow_entity("wf-1", "agent-a"))
            .unwrap();
        storage
            .store(&make_workflow_entity("wf-2", "agent-b"))
            .unwrap();
        let integration = LocusIntegration::new(storage);

        let workflows = integration.get_workflows().unwrap();
        assert_eq!(workflows.len(), 2);
    }

    #[test]
    fn test_get_workflows_skips_invalid_data() {
        let mut storage = MemoryStorage::new("test-agent");
        let bad_entity = GenericEntity {
            id: "bad".to_string(),
            entity_type: "workflow".to_string(),
            agent: "agent".to_string(),
            timestamp: chrono::Utc::now(),
            data: serde_json::json!({"not": "a workflow"}),
        };
        storage.store(&bad_entity).unwrap();
        let wf = Workflow::new(
            "Good Workflow".to_string(),
            "desc".to_string(),
            "agent".to_string(),
        );
        let entity = GenericEntity {
            id: wf.id.clone(),
            entity_type: "workflow".to_string(),
            agent: "agent".to_string(),
            timestamp: chrono::Utc::now(),
            data: serde_json::to_value(&wf).unwrap(),
        };
        storage.store(&entity).unwrap();
        let integration = LocusIntegration::new(storage);

        let workflows = integration.get_workflows().unwrap();
        assert_eq!(workflows.len(), 1);
        assert_eq!(workflows[0].title, "Good Workflow");
    }

    #[test]
    fn test_get_workflow_found() {
        let mut storage = MemoryStorage::new("test-agent");
        let wf = Workflow::new(
            "My Workflow".to_string(),
            "desc".to_string(),
            "agent".to_string(),
        );
        let entity = GenericEntity {
            id: format!("workflow-{}", wf.id),
            entity_type: "workflow".to_string(),
            agent: "agent".to_string(),
            timestamp: chrono::Utc::now(),
            data: serde_json::to_value(&wf).unwrap(),
        };
        storage.store(&entity).unwrap();
        let integration = LocusIntegration::new(storage);

        let result = integration.get_workflow(&wf.id).unwrap();
        assert!(result.is_some());
    }

    #[test]
    fn test_get_workflow_not_found() {
        let integration = make_integration();
        let result = integration.get_workflow("nonexistent").unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_save_workflow() {
        let mut integration = make_integration_mut();
        let workflow = Workflow::new("Test".to_string(), "desc".to_string(), "agent".to_string());
        let workflow_id = workflow.id.clone();

        let saved_id = integration.save_workflow(&workflow).unwrap();
        assert_eq!(saved_id, workflow_id);
    }

    #[test]
    fn test_get_tasks_empty() {
        let integration = make_integration();
        let tasks = integration.get_tasks(None).unwrap();
        assert!(tasks.is_empty());
    }

    #[test]
    fn test_get_tasks_all() {
        let mut storage = MemoryStorage::new("test-agent");
        storage.store(&make_task_entity("t-1", "agent-a")).unwrap();
        storage.store(&make_task_entity("t-2", "agent-b")).unwrap();
        let integration = LocusIntegration::new(storage);

        let tasks = integration.get_tasks(None).unwrap();
        assert_eq!(tasks.len(), 2);
    }

    #[test]
    fn test_get_tasks_with_agent_filter() {
        let mut storage = MemoryStorage::new("test-agent");
        storage.store(&make_task_entity("t-1", "agent-a")).unwrap();
        storage.store(&make_task_entity("t-2", "agent-b")).unwrap();
        let integration = LocusIntegration::new(storage);

        let tasks = integration.get_tasks(Some("agent-a")).unwrap();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].agent, "agent-a");
    }

    #[test]
    fn test_get_tasks_filter_no_match() {
        let mut storage = MemoryStorage::new("test-agent");
        storage.store(&make_task_entity("t-1", "agent-a")).unwrap();
        let integration = LocusIntegration::new(storage);

        let tasks = integration.get_tasks(Some("nonexistent-agent")).unwrap();
        assert!(tasks.is_empty());
    }

    #[test]
    fn test_get_tasks_skips_invalid() {
        let mut storage = MemoryStorage::new("test-agent");
        let bad = GenericEntity {
            id: "bad".to_string(),
            entity_type: "task".to_string(),
            agent: "agent".to_string(),
            timestamp: chrono::Utc::now(),
            data: serde_json::json!({"garbage": true}),
        };
        storage.store(&bad).unwrap();
        storage.store(&make_task_entity("t-1", "agent")).unwrap();
        let integration = LocusIntegration::new(storage);

        let tasks = integration.get_tasks(None).unwrap();
        assert_eq!(tasks.len(), 1);
    }

    #[test]
    fn test_get_execution_results_empty() {
        let integration = make_integration();
        let results = integration.get_execution_results(None, None).unwrap();
        assert!(results.is_empty());
    }

    #[test]
    fn test_get_execution_results_all() {
        let mut storage = MemoryStorage::new("test-agent");
        storage
            .store(&make_execution_result_entity(
                "er-1",
                "task-1",
                "agent",
                ValidationStatus::Passed,
            ))
            .unwrap();
        storage
            .store(&make_execution_result_entity(
                "er-2",
                "task-2",
                "agent",
                ValidationStatus::Failed {
                    reason: "oops".to_string(),
                },
            ))
            .unwrap();
        let integration = LocusIntegration::new(storage);

        let results = integration.get_execution_results(None, None).unwrap();
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_get_execution_results_filter_by_task_id() {
        let mut storage = MemoryStorage::new("test-agent");
        storage
            .store(&make_execution_result_entity(
                "er-1",
                "task-1",
                "agent",
                ValidationStatus::Passed,
            ))
            .unwrap();
        storage
            .store(&make_execution_result_entity(
                "er-2",
                "task-2",
                "agent",
                ValidationStatus::Passed,
            ))
            .unwrap();
        let integration = LocusIntegration::new(storage);

        let results = integration
            .get_execution_results(Some("task-1"), None)
            .unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].task_id, "task-1");
    }

    #[test]
    fn test_get_execution_results_filter_by_time_range() {
        let mut storage = MemoryStorage::new("test-agent");
        let mut result = ExecutionResult::new(
            "task-1".to_string(),
            "dev".to_string(),
            "gate".to_string(),
            "cmd".to_string(),
            "agent".to_string(),
        );
        result.id = "er-1".to_string();
        result.timestamp = chrono::Utc::now() - chrono::Duration::days(10);
        let entity = GenericEntity {
            id: "er-1".to_string(),
            entity_type: "execution_result".to_string(),
            agent: "agent".to_string(),
            timestamp: result.timestamp,
            data: serde_json::to_value(&result).unwrap(),
        };
        storage.store(&entity).unwrap();

        let integration = LocusIntegration::new(storage);

        let now = chrono::Utc::now();
        let results = integration
            .get_execution_results(None, Some((now - chrono::Duration::days(1), now)))
            .unwrap();
        assert!(results.is_empty());
    }

    #[test]
    fn test_get_execution_results_combined_filters() {
        let mut storage = MemoryStorage::new("test-agent");
        storage
            .store(&make_execution_result_entity(
                "er-1",
                "task-1",
                "agent",
                ValidationStatus::Passed,
            ))
            .unwrap();
        storage
            .store(&make_execution_result_entity(
                "er-2",
                "task-2",
                "agent",
                ValidationStatus::Passed,
            ))
            .unwrap();
        let integration = LocusIntegration::new(storage);

        let now = chrono::Utc::now();
        let results = integration
            .get_execution_results(
                Some("task-1"),
                Some((
                    now - chrono::Duration::days(1),
                    now + chrono::Duration::days(1),
                )),
            )
            .unwrap();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_get_execution_results_skips_invalid() {
        let mut storage = MemoryStorage::new("test-agent");
        let bad = GenericEntity {
            id: "bad".to_string(),
            entity_type: "execution_result".to_string(),
            agent: "agent".to_string(),
            timestamp: chrono::Utc::now(),
            data: serde_json::json!({"x": 1}),
        };
        storage.store(&bad).unwrap();
        storage
            .store(&make_execution_result_entity(
                "er-1",
                "task-1",
                "agent",
                ValidationStatus::Passed,
            ))
            .unwrap();
        let integration = LocusIntegration::new(storage);

        let results = integration.get_execution_results(None, None).unwrap();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_get_system_health_empty() {
        let integration = make_integration();
        let health = integration.get_system_health().unwrap();
        assert_eq!(health.total_tasks, 0);
        assert_eq!(health.total_workflows, 0);
        assert_eq!(health.error_count, 0);
    }

    #[test]
    fn test_get_system_health_with_data() {
        let mut storage = MemoryStorage::new("test-agent");
        storage.store(&make_task_entity("t-1", "agent")).unwrap();
        storage
            .store(&make_workflow_entity("wf-1", "agent"))
            .unwrap();
        storage
            .store(&make_execution_result_entity(
                "er-1",
                "task-1",
                "agent",
                ValidationStatus::Failed {
                    reason: "err".to_string(),
                },
            ))
            .unwrap();
        storage
            .store(&make_execution_result_entity(
                "er-2",
                "task-1",
                "agent",
                ValidationStatus::Passed,
            ))
            .unwrap();
        let integration = LocusIntegration::new(storage);

        let health = integration.get_system_health().unwrap();
        assert_eq!(health.total_tasks, 1);
        assert_eq!(health.total_workflows, 1);
        assert_eq!(health.error_count, 1);
    }

    #[test]
    fn test_create_emergency_override() {
        let mut integration = make_integration_mut();

        let override_id = integration
            .create_emergency_override("task-123", "urgent fix", "manual")
            .unwrap();

        assert!(!override_id.is_empty());

        let stored = integration
            .storage
            .get(&override_id, "emergency_override")
            .unwrap();
        assert!(stored.is_some());
        let data = stored.unwrap().data;
        assert_eq!(data["task_id"], "task-123");
        assert_eq!(data["reason"], "urgent fix");
        assert_eq!(data["override_type"], "manual");
        assert_eq!(data["status"], "active");
    }
}

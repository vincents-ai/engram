//! Integration layer between Locus and Engram
//!
//! Provides Locus with access to Engram's storage, entities,
//! and functionality through a clean API boundary.

use crate::config::Config;
use crate::entities::*;
use crate::error::EngramError;
use crate::storage::{RelationshipStorage, Storage};
use std::collections::HashMap;

/// Integration interface for Locus to access Engram functionality
pub struct LocusIntegration<S: Storage + RelationshipStorage> {
    storage: S,
    config: Config,
}

impl<S: Storage + RelationshipStorage> LocusIntegration<S> {
    /// Create new Locus integration instance
    pub fn new(storage: S, config: Config) -> Self {
        Self { storage, config }
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
        let entity = crate::entities::Entity {
            id: workflow_id.clone(),
            entity_type: "workflow".to_string(),
            created_at: workflow.created_at.unwrap_or_else(|| chrono::Utc::now()),
            updated_at: chrono::Utc::now(),
            data: serde_json::to_value(workflow)?,
            metadata: HashMap::new(),
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
                    if task.agent.as_ref().map_or(false, |a| a == agent) {
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
            .filter_map(|entity| serde_json::from_value::<ExecutionResult>(entity.data).ok())
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
        let override_entity = crate::entities::Entity {
            id: override_id.clone(),
            entity_type: "emergency_override".to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            data: serde_json::json!({
                "task_id": task_id,
                "reason": reason,
                "override_type": override_type,
                "status": "active",
                "created_by": "locus"
            }),
            metadata: HashMap::new(),
        };

        self.storage.store(Box::new(override_entity))?;

        // Create relationship to task
        crate::cli::relationship::create_relationship(
            &mut self.storage,
            &override_id,
            "emergency_override",
            task_id,
            "task",
            "overrides",
            "locus",
        )?;

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

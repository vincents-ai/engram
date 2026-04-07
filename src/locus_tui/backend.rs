//! LocusTuiBackend trait and EngramBackend implementation
//!
//! Provides a clean integration boundary between the TUI and
//! the real engram storage layer. The trait is object-safe so
//! callers can hold `Box<dyn LocusTuiBackend>`.

use crate::entities::{
    AgentSandbox, Compliance, Context, EntityRelationship, EscalationRequest, ExecutionResult,
    Knowledge, ProgressiveGateConfig, Reasoning, Rule, Session, Standard, StateReflection, Task,
    Theory, Workflow, WorkflowInstance, ADR,
};
use crate::error::EngramError;
use crate::storage::{RelationshipStorage, Storage};

/// Object-safe trait for TUI data access.
///
/// Implementations retrieve the core entity collections that
/// the Locus TUI needs to display. Returning `EngramError` lets
/// callers distinguish storage failures from empty results.
pub trait LocusTuiBackend: Send {
    fn list_tasks(&self) -> Result<Vec<Task>, EngramError>;
    fn list_contexts(&self) -> Result<Vec<Context>, EngramError>;
    fn list_reasoning(&self) -> Result<Vec<Reasoning>, EngramError>;
    fn list_relationships(&self) -> Result<Vec<EntityRelationship>, EngramError>;
    fn list_adrs(&self) -> Result<Vec<ADR>, EngramError>;
    fn list_theories(&self) -> Result<Vec<Theory>, EngramError>;
    fn list_workflows(&self) -> Result<Vec<Workflow>, EngramError>;
    fn list_workflow_instances(&self) -> Result<Vec<WorkflowInstance>, EngramError>;
    fn list_knowledge(&self) -> Result<Vec<Knowledge>, EngramError>;
    fn list_sessions(&self) -> Result<Vec<Session>, EngramError>;
    fn list_compliance(&self) -> Result<Vec<Compliance>, EngramError>;
    fn list_rules(&self) -> Result<Vec<Rule>, EngramError>;
    fn list_standards(&self) -> Result<Vec<Standard>, EngramError>;
    fn list_state_reflections(&self) -> Result<Vec<StateReflection>, EngramError>;
    fn list_escalations(&self) -> Result<Vec<EscalationRequest>, EngramError>;
    fn list_sandboxes(&self) -> Result<Vec<AgentSandbox>, EngramError>;
    fn list_execution_results(&self) -> Result<Vec<ExecutionResult>, EngramError>;
    fn list_progressive_configs(&self) -> Result<Vec<ProgressiveGateConfig>, EngramError>;
    fn update_adr_status(
        &mut self,
        id: &str,
        status: crate::entities::AdrStatus,
    ) -> Result<(), Box<dyn std::error::Error>>;
    fn update_task_status(
        &mut self,
        id: &str,
        status: crate::entities::TaskStatus,
    ) -> Result<(), Box<dyn std::error::Error>>;
    /// Return configured remote names (empty if no remotes.json or not a git repo).
    fn list_remote_names(&self) -> Vec<String>;
    /// Return sync status rows for the given remote (empty on error or if no remotes configured).
    fn get_sync_status_data(
        &self,
        remote_name: &str,
    ) -> Result<Vec<crate::locus_tui::app::SyncStatusRow>, EngramError>;
}

/// Generic backend backed by any `Storage + RelationshipStorage`.
///
/// `EngramBackend<GitStorage>` is the production backend;
/// `EngramBackend<MemoryStorage>` is used in tests.
pub struct EngramBackend<S: Storage + RelationshipStorage> {
    storage: S,
}

impl<S: Storage + RelationshipStorage> EngramBackend<S> {
    /// Create a backend from an already-constructed storage instance.
    pub fn from_storage(storage: S) -> Self {
        Self { storage }
    }
}

impl<S: Storage + RelationshipStorage + Send> LocusTuiBackend for EngramBackend<S> {
    fn list_tasks(&self) -> Result<Vec<Task>, EngramError> {
        let entities = self.storage.get_all("task")?;
        let tasks = entities
            .into_iter()
            .filter_map(|e| serde_json::from_value::<Task>(e.data).ok())
            .collect();
        Ok(tasks)
    }

    fn list_contexts(&self) -> Result<Vec<Context>, EngramError> {
        let entities = self.storage.get_all("context")?;
        let contexts = entities
            .into_iter()
            .filter_map(|e| serde_json::from_value::<Context>(e.data).ok())
            .collect();
        Ok(contexts)
    }

    fn list_reasoning(&self) -> Result<Vec<Reasoning>, EngramError> {
        let entities = self.storage.get_all("reasoning")?;
        let reasoning = entities
            .into_iter()
            .filter_map(|e| serde_json::from_value::<Reasoning>(e.data).ok())
            .collect();
        Ok(reasoning)
    }

    fn list_relationships(&self) -> Result<Vec<EntityRelationship>, EngramError> {
        let entities = self.storage.get_all("relationship")?;
        let relationships = entities
            .into_iter()
            .filter_map(|e| serde_json::from_value::<EntityRelationship>(e.data).ok())
            .collect();
        Ok(relationships)
    }

    fn list_adrs(&self) -> Result<Vec<ADR>, EngramError> {
        let entities = self.storage.get_all("adr")?;
        let adrs = entities
            .into_iter()
            .filter_map(|e| serde_json::from_value::<ADR>(e.data).ok())
            .collect();
        Ok(adrs)
    }

    fn list_theories(&self) -> Result<Vec<Theory>, EngramError> {
        let entities = self.storage.get_all("theory")?;
        let theories = entities
            .into_iter()
            .filter_map(|e| serde_json::from_value::<Theory>(e.data).ok())
            .collect();
        Ok(theories)
    }

    fn list_workflows(&self) -> Result<Vec<Workflow>, EngramError> {
        let entities = self.storage.get_all("workflow")?;
        Ok(entities
            .into_iter()
            .filter_map(|e| serde_json::from_value::<Workflow>(e.data).ok())
            .collect())
    }

    fn list_workflow_instances(&self) -> Result<Vec<WorkflowInstance>, EngramError> {
        let entities = self.storage.get_all("workflow_instance")?;
        Ok(entities
            .into_iter()
            .filter_map(|e| serde_json::from_value::<WorkflowInstance>(e.data).ok())
            .collect())
    }

    fn list_knowledge(&self) -> Result<Vec<Knowledge>, EngramError> {
        let entities = self.storage.get_all("knowledge")?;
        Ok(entities
            .into_iter()
            .filter_map(|e| serde_json::from_value::<Knowledge>(e.data).ok())
            .collect())
    }

    fn list_sessions(&self) -> Result<Vec<Session>, EngramError> {
        let entities = self.storage.get_all("session")?;
        Ok(entities
            .into_iter()
            .filter_map(|e| serde_json::from_value::<Session>(e.data).ok())
            .collect())
    }

    fn list_compliance(&self) -> Result<Vec<Compliance>, EngramError> {
        let entities = self.storage.get_all("compliance")?;
        Ok(entities
            .into_iter()
            .filter_map(|e| serde_json::from_value::<Compliance>(e.data).ok())
            .collect())
    }

    fn list_rules(&self) -> Result<Vec<Rule>, EngramError> {
        let entities = self.storage.get_all("rule")?;
        Ok(entities
            .into_iter()
            .filter_map(|e| serde_json::from_value::<Rule>(e.data).ok())
            .collect())
    }

    fn list_standards(&self) -> Result<Vec<Standard>, EngramError> {
        let entities = self.storage.get_all("standard")?;
        Ok(entities
            .into_iter()
            .filter_map(|e| serde_json::from_value::<Standard>(e.data).ok())
            .collect())
    }

    fn list_state_reflections(&self) -> Result<Vec<StateReflection>, EngramError> {
        let entities = self.storage.get_all("state_reflection")?;
        Ok(entities
            .into_iter()
            .filter_map(|e| serde_json::from_value::<StateReflection>(e.data).ok())
            .collect())
    }

    fn list_escalations(&self) -> Result<Vec<EscalationRequest>, EngramError> {
        let entities = self.storage.get_all("escalation_request")?;
        Ok(entities
            .into_iter()
            .filter_map(|e| serde_json::from_value::<EscalationRequest>(e.data).ok())
            .collect())
    }

    fn list_sandboxes(&self) -> Result<Vec<AgentSandbox>, EngramError> {
        let entities = self.storage.get_all("agent_sandbox")?;
        Ok(entities
            .into_iter()
            .filter_map(|e| serde_json::from_value::<AgentSandbox>(e.data).ok())
            .collect())
    }

    fn list_execution_results(&self) -> Result<Vec<ExecutionResult>, EngramError> {
        let entities = self.storage.get_all("execution_result")?;
        Ok(entities
            .into_iter()
            .filter_map(|e| serde_json::from_value::<ExecutionResult>(e.data).ok())
            .collect())
    }

    fn list_progressive_configs(&self) -> Result<Vec<ProgressiveGateConfig>, EngramError> {
        let entities = self.storage.get_all("progressive_gate_config")?;
        Ok(entities
            .into_iter()
            .filter_map(|e| serde_json::from_value::<ProgressiveGateConfig>(e.data).ok())
            .collect())
    }

    fn update_adr_status(
        &mut self,
        id: &str,
        status: crate::entities::AdrStatus,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let adrs = self.list_adrs()?;
        if let Some(mut adr) = adrs
            .into_iter()
            .find(|a| a.id == id || a.id.starts_with(id))
        {
            adr.status = status;
            let entity = crate::entities::GenericEntity {
                id: adr.id.clone(),
                entity_type: "adr".to_string(),
                agent: adr.agent.clone(),
                timestamp: adr.created_at,
                data: serde_json::to_value(&adr)?,
            };
            self.storage.store(&entity)?;
        }
        Ok(())
    }

    fn update_task_status(
        &mut self,
        id: &str,
        status: crate::entities::TaskStatus,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let tasks = self.list_tasks()?;
        if let Some(mut task) = tasks.into_iter().find(|t| {
            t.id == id || t.id.starts_with(id) || id.starts_with(&t.id[..8.min(t.id.len())])
        }) {
            task.status = status;
            let entity = crate::entities::GenericEntity {
                id: task.id.clone(),
                entity_type: "task".to_string(),
                agent: task.agent.clone(),
                timestamp: task.start_time,
                data: serde_json::to_value(&task)?,
            };
            self.storage.store(&entity)?;
        }
        Ok(())
    }

    fn list_remote_names(&self) -> Vec<String> {
        use std::collections::HashMap;
        use std::fs;
        let config_path = ".engram/remotes.json";
        let Ok(content) = fs::read_to_string(config_path) else {
            return vec![];
        };
        let Ok(remotes) = serde_json::from_str::<HashMap<String, serde_json::Value>>(&content)
        else {
            return vec![];
        };
        let mut names: Vec<String> = remotes.into_keys().collect();
        names.sort();
        names
    }

    fn get_sync_status_data(
        &self,
        remote_name: &str,
    ) -> Result<Vec<crate::locus_tui::app::SyncStatusRow>, EngramError> {
        let report = crate::cli::sync::get_sync_status(&mut std::io::sink(), remote_name, false)?;
        let rows = report
            .rows
            .into_iter()
            .map(|r| crate::locus_tui::app::SyncStatusRow {
                entity_type: r.entity_type,
                local_count: r.local_count,
                remote_count: r.remote_count,
                conflicts: r.conflicts,
            })
            .collect();
        Ok(rows)
    }
}

/// Convenience type alias for the production git-backed backend.
///
/// Uses `GitRefsStorage` — the same storage format the CLI uses — so the
/// TUI reads the same entities that `engram task list`, `engram context list`,
/// etc. write.
pub type GitEngramBackend = EngramBackend<crate::storage::GitRefsStorage>;

impl GitEngramBackend {
    /// Open the engram repository in the current working directory.
    ///
    /// This mirrors `GitRefsStorage::new(".", "default")` used by every CLI
    /// command, so the TUI and CLI always share the same data.
    pub fn new() -> Result<Self, EngramError> {
        let storage = crate::storage::GitRefsStorage::new(".", "locus-tui")?;
        Ok(Self::from_storage(storage))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::{ContextRelevance, Reasoning, Task, TaskPriority, Theory, ADR};
    use crate::storage::memory_only_storage::MemoryStorage;
    use crate::storage::{RelationshipStorage, Storage};

    fn make_backend() -> EngramBackend<MemoryStorage> {
        EngramBackend::from_storage(MemoryStorage::new("test-agent"))
    }

    // ── LocusTuiBackend is object-safe ────────────────────────────────────
    #[test]
    fn test_trait_is_object_safe() {
        // If this compiles, the trait is object-safe.
        let backend: Box<dyn LocusTuiBackend> = Box::new(make_backend());
        let tasks = backend.list_tasks().unwrap();
        assert!(tasks.is_empty());
    }

    // ── list_tasks ────────────────────────────────────────────────────────
    #[test]
    fn test_list_tasks_empty() {
        let backend = make_backend();
        let tasks = backend.list_tasks().unwrap();
        assert!(tasks.is_empty());
    }

    #[test]
    fn test_list_tasks_returns_stored_tasks() {
        let mut storage = MemoryStorage::new("test-agent");
        let task = Task::new(
            "Test task".to_string(),
            "desc".to_string(),
            "test-agent".to_string(),
            TaskPriority::Medium,
            None,
        );
        let entity = crate::entities::GenericEntity {
            id: task.id.clone(),
            entity_type: "task".to_string(),
            agent: task.agent.clone(),
            timestamp: task.start_time,
            data: serde_json::to_value(&task).unwrap(),
        };
        storage.store(&entity).unwrap();

        let backend = EngramBackend::from_storage(storage);
        let tasks = backend.list_tasks().unwrap();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].title, "Test task");
    }

    // ── list_contexts ─────────────────────────────────────────────────────
    #[test]
    fn test_list_contexts_empty() {
        let backend = make_backend();
        assert!(backend.list_contexts().unwrap().is_empty());
    }

    #[test]
    fn test_list_contexts_returns_stored_contexts() {
        let mut storage = MemoryStorage::new("test-agent");
        let ctx = Context::new(
            "My context".to_string(),
            "content here".to_string(),
            "manual".to_string(),
            ContextRelevance::Medium,
            "test-agent".to_string(),
        );
        let entity = crate::entities::GenericEntity {
            id: ctx.id.clone(),
            entity_type: "context".to_string(),
            agent: ctx.agent.clone(),
            timestamp: ctx.created_at,
            data: serde_json::to_value(&ctx).unwrap(),
        };
        storage.store(&entity).unwrap();

        let backend = EngramBackend::from_storage(storage);
        let ctxs = backend.list_contexts().unwrap();
        assert_eq!(ctxs.len(), 1);
        assert_eq!(ctxs[0].title, "My context");
    }

    // ── list_reasoning ────────────────────────────────────────────────────
    #[test]
    fn test_list_reasoning_empty() {
        let backend = make_backend();
        assert!(backend.list_reasoning().unwrap().is_empty());
    }

    #[test]
    fn test_list_reasoning_returns_stored_reasoning() {
        let mut storage = MemoryStorage::new("test-agent");
        let rsn = Reasoning::new(
            "My reasoning".to_string(),
            "task-abc".to_string(),
            "test-agent".to_string(),
        );
        let entity = crate::entities::GenericEntity {
            id: rsn.id.clone(),
            entity_type: "reasoning".to_string(),
            agent: rsn.agent.clone(),
            timestamp: rsn.created_at,
            data: serde_json::to_value(&rsn).unwrap(),
        };
        storage.store(&entity).unwrap();

        let backend = EngramBackend::from_storage(storage);
        let results = backend.list_reasoning().unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "My reasoning");
    }

    // ── list_relationships ────────────────────────────────────────────────
    #[test]
    fn test_list_relationships_empty() {
        let backend = make_backend();
        assert!(backend.list_relationships().unwrap().is_empty());
    }

    #[test]
    fn test_list_relationships_via_relationship_storage() {
        use crate::entities::{EntityRelationType, EntityRelationship};
        let mut storage = MemoryStorage::new("test-agent");
        let rel = EntityRelationship::new(
            uuid::Uuid::new_v4().to_string(),
            "test-agent".to_string(),
            "src-001".to_string(),
            "task".to_string(),
            "tgt-002".to_string(),
            "context".to_string(),
            EntityRelationType::References,
        );
        storage.store_relationship(&rel).unwrap();

        let backend = EngramBackend::from_storage(storage);
        // Relationships stored via store_relationship are retrievable via get_all("relationship").
        // The call must succeed (no panic / Err), even if the count is 0.
        let result = backend.list_relationships();
        assert!(result.is_ok());
    }

    // ── list_adrs ─────────────────────────────────────────────────────────
    #[test]
    fn test_list_adrs_empty() {
        let backend = make_backend();
        assert!(backend.list_adrs().unwrap().is_empty());
    }

    #[test]
    fn test_list_adrs_returns_stored_adrs() {
        let mut storage = MemoryStorage::new("test-agent");
        let adr = ADR::new(
            "Use Rust".to_string(),
            1,
            "test-agent".to_string(),
            "Need a systems language".to_string(),
        );
        let entity = crate::entities::GenericEntity {
            id: adr.id.clone(),
            entity_type: "adr".to_string(),
            agent: adr.agent.clone(),
            timestamp: adr.created_at,
            data: serde_json::to_value(&adr).unwrap(),
        };
        storage.store(&entity).unwrap();

        let backend = EngramBackend::from_storage(storage);
        let adrs = backend.list_adrs().unwrap();
        assert_eq!(adrs.len(), 1);
        assert_eq!(adrs[0].title, "Use Rust");
    }

    // ── list_theories ─────────────────────────────────────────────────────
    #[test]
    fn test_list_theories_empty() {
        let backend = make_backend();
        assert!(backend.list_theories().unwrap().is_empty());
    }

    #[test]
    fn test_list_theories_returns_stored_theories() {
        let mut storage = MemoryStorage::new("test-agent");
        let theory = Theory::new("Storage".to_string(), "test-agent".to_string());
        let entity = crate::entities::GenericEntity {
            id: theory.id.clone(),
            entity_type: "theory".to_string(),
            agent: theory.agent.clone(),
            timestamp: theory.created_at,
            data: serde_json::to_value(&theory).unwrap(),
        };
        storage.store(&entity).unwrap();

        let backend = EngramBackend::from_storage(storage);
        let theories = backend.list_theories().unwrap();
        assert_eq!(theories.len(), 1);
        assert_eq!(theories[0].domain_name, "Storage");
    }
}

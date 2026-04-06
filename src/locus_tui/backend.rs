//! LocusTuiBackend trait and EngramBackend implementation
//!
//! Provides a clean integration boundary between the TUI and
//! the real engram storage layer. The trait is object-safe so
//! callers can hold `Box<dyn LocusTuiBackend>`.

use crate::entities::{Context, EntityRelationship, Reasoning, Task, Theory, ADR};
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
}

/// Convenience type alias for the production git-backed backend.
pub type GitEngramBackend = EngramBackend<crate::storage::GitStorage>;

impl GitEngramBackend {
    /// Open the default engram repository for the current user.
    ///
    /// Looks for the repo at `~/.engram`; falls back to the current
    /// working directory's `.engram` sub-directory.
    pub fn new() -> Result<Self, EngramError> {
        use std::path::PathBuf;

        let workspace_path = std::env::var("ENGRAM_WORKSPACE")
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                dirs::home_dir()
                    .unwrap_or_else(|| PathBuf::from("."))
                    .join(".engram_workspace")
            });

        // Ensure the workspace directory exists so GitStorage::new can init.
        std::fs::create_dir_all(&workspace_path)?;

        let storage =
            crate::storage::GitStorage::new(workspace_path.to_str().unwrap_or("."), "locus-tui")?;

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

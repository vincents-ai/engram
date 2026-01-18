//! Git refs-based storage implementation
//!
//! This implementation stores entities as Git objects referenced by Git refs,
//! eliminating the need for a separate .engram directory structure.
//! Entities are stored as Git blobs and referenced by refs in the format:
//! refs/engram/{entity_type}/{entity_id}

use super::{
    GitCommit, MemoryEntity, QueryFilter, QueryResult, RelationshipIndex, RelationshipStats,
    RelationshipStorage, SortOrder, Storage, StorageStats, TraversalAlgorithm,
};
use crate::entities::{
    Entity, EntityRegistry, EntityRelationship, GenericEntity, RelationshipDirection,
    RelationshipFilter,
};
use crate::error::EngramError;
use chrono::{DateTime, Utc};
use git2::{Oid, Repository, Signature};
use serde_json::Value;
use std::collections::{HashMap, HashSet, VecDeque};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

/// Git refs-based storage for entities
///
/// Stores entities as Git blobs with refs pointing to them in the format:
/// refs/engram/{entity_type}/{entity_id}
///
/// This eliminates the need for .engram directory structure and provides
/// better integration with Git tooling and distributed workflows.
pub struct GitRefsStorage {
    repository: Arc<Mutex<Repository>>,
    workspace_path: PathBuf,
    entity_registry: EntityRegistry,
    current_agent: String,
    relationship_index: Arc<Mutex<RelationshipIndex>>,
}

impl std::fmt::Debug for GitRefsStorage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GitRefsStorage")
            .field("workspace_path", &self.workspace_path)
            .field("current_agent", &self.current_agent)
            .finish()
    }
}

impl GitRefsStorage {
    /// Create new Git refs storage instance
    pub fn new(workspace_path: &str, agent: &str) -> Result<Self, EngramError> {
        let workspace_path = PathBuf::from(workspace_path);

        let repository = if !workspace_path.join(".git").exists() {
            Repository::init(&workspace_path).map_err(|e| EngramError::Git(e.to_string()))?
        } else {
            Repository::open(&workspace_path).map_err(|e| EngramError::Git(e.to_string()))?
        };

        let mut registry = EntityRegistry::new();
        let mut registry = EntityRegistry::new();
        registry.register::<crate::entities::Task>();
        registry.register::<crate::entities::Context>();
        registry.register::<crate::entities::Reasoning>();
        registry.register::<crate::entities::Knowledge>();
        registry.register::<crate::entities::Session>();
        registry.register::<crate::entities::Compliance>();
        registry.register::<crate::entities::EntityRelationship>();

        let mut storage = GitRefsStorage {
            repository: Arc::new(Mutex::new(repository)),
            workspace_path,
            entity_registry: registry,
            current_agent: agent.to_string(),
            relationship_index: Arc::new(Mutex::new(RelationshipIndex::new())),
        };

        let _ = storage.rebuild_relationship_index();

        Ok(storage)
    }

    /// Get ref name for an entity
    fn get_entity_ref(&self, entity_type: &str, entity_id: &str) -> String {
        format!("refs/engram/{}/{}", entity_type, entity_id)
    }

    /// Store entity as Git blob and create ref
    fn store_entity_as_ref(&self, entity: &GenericEntity) -> Result<(), EngramError> {
        let repo = self
            .repository
            .lock()
            .map_err(|_| EngramError::Storage("Repository lock failed".to_string()))?;

        let memory_entity = MemoryEntity::new(
            entity.id.clone(),
            entity.entity_type.clone(),
            entity.agent.clone(),
            entity.created.unwrap_or_else(Utc::now),
            entity.updated.unwrap_or_else(Utc::now),
            entity.data.clone(),
        );

        let json_content = serde_json::to_string_pretty(&memory_entity)
            .map_err(|e| EngramError::Serialization(e.to_string()))?;

        let blob_oid = repo
            .blob(json_content.as_bytes())
            .map_err(|e| EngramError::Git(format!("Failed to create blob: {}", e)))?;

        let ref_name = self.get_entity_ref(&entity.entity_type, &entity.id);
        repo.reference(
            &ref_name,
            blob_oid,
            true,
            &format!("Update {} {}", entity.entity_type, entity.id),
        )
        .map_err(|e| EngramError::Git(format!("Failed to create ref: {}", e)))?;

        Ok(())
    }

    /// Load entity from Git ref
    fn load_entity_from_ref(
        &self,
        entity_type: &str,
        entity_id: &str,
    ) -> Result<Option<GenericEntity>, EngramError> {
        let repo = self
            .repository
            .lock()
            .map_err(|_| EngramError::Storage("Repository lock failed".to_string()))?;

        let ref_name = self.get_entity_ref(entity_type, entity_id);

        // Try to find the reference
        match repo.find_reference(&ref_name) {
            Ok(reference) => {
                // Get the OID that the ref points to
                let oid = reference.target().ok_or_else(|| {
                    EngramError::Storage(format!("Ref {} has no target", ref_name))
                })?;

                // Get the blob object
                let blob = repo
                    .find_blob(oid)
                    .map_err(|e| EngramError::Git(format!("Failed to find blob {}: {}", oid, e)))?;

                // Deserialize JSON content
                let json_content = std::str::from_utf8(blob.content())
                    .map_err(|e| EngramError::Storage(format!("Invalid UTF-8 in blob: {}", e)))?;

                let memory_entity: MemoryEntity = serde_json::from_str(json_content)
                    .map_err(|e| EngramError::Deserialization(e.to_string()))?;

                // Convert to GenericEntity
                let generic_entity = GenericEntity {
                    id: memory_entity.id,
                    entity_type: memory_entity.entity_type,
                    agent: memory_entity.agent,
                    created: Some(memory_entity.created),
                    updated: Some(memory_entity.updated),
                    data: memory_entity.data,
                };

                Ok(Some(generic_entity))
            }
            Err(_) => Ok(None), // Reference doesn't exist
        }
    }

    /// Delete entity ref
    fn delete_entity_ref(&self, entity_type: &str, entity_id: &str) -> Result<bool, EngramError> {
        let mut repo = self
            .repository
            .lock()
            .map_err(|_| EngramError::Storage("Repository lock failed".to_string()))?;

        let ref_name = self.get_entity_ref(entity_type, entity_id);

        match repo.find_reference(&ref_name) {
            Ok(mut reference) => {
                reference
                    .delete()
                    .map_err(|e| EngramError::Git(format!("Failed to delete ref: {}", e)))?;
                Ok(true)
            }
            Err(_) => Ok(false), // Reference doesn't exist
        }
    }

    /// List all entity refs of a given type
    fn list_entity_refs(&self, entity_type: &str) -> Result<Vec<String>, EngramError> {
        let repo = self
            .repository
            .lock()
            .map_err(|_| EngramError::Storage("Repository lock failed".to_string()))?;

        let ref_prefix = format!("refs/engram/{}/", entity_type);
        let mut entity_ids = Vec::new();

        // Iterate through all references
        let refs = repo
            .references()
            .map_err(|e| EngramError::Git(format!("Failed to list references: {}", e)))?;

        for reference in refs {
            let reference = reference
                .map_err(|e| EngramError::Git(format!("Failed to read reference: {}", e)))?;

            if let Some(name) = reference.name() {
                if name.starts_with(&ref_prefix) {
                    // Extract entity ID from ref name
                    let entity_id = name.strip_prefix(&ref_prefix).unwrap();
                    entity_ids.push(entity_id.to_string());
                }
            }
        }

        Ok(entity_ids)
    }

    /// Rebuild relationship index from all stored entities
    fn rebuild_relationship_index(&mut self) -> Result<(), EngramError> {
        let mut index = self
            .relationship_index
            .lock()
            .map_err(|_| EngramError::Storage("Index lock failed".to_string()))?;
        index.clear();

        // Get all entity types and rebuild index
        let entity_types = [
            "task",
            "context",
            "reasoning",
            "knowledge",
            "session",
            "compliance",
            "relationship",
        ];

        for entity_type in &entity_types {
            let entity_ids = self.list_entity_refs(entity_type)?;

            for entity_id in entity_ids {
                if let Some(entity) = self.load_entity_from_ref(entity_type, &entity_id)? {
                    if entity_type == "relationship" {
                        // Special handling for relationship entities
                        if let Ok(relationship) =
                            serde_json::from_value::<EntityRelationship>(entity.data)
                        {
                            index.add_relationship(&relationship);
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

// Storage trait implementation will be added next
impl Storage for GitRefsStorage {
    fn store(&mut self, entity: &GenericEntity) -> Result<(), EngramError> {
        self.store_entity_as_ref(entity)?;

        // Update relationship index if this is a relationship entity
        if entity.entity_type == "relationship" {
            if let Ok(relationship) =
                serde_json::from_value::<EntityRelationship>(entity.data.clone())
            {
                let mut index = self
                    .relationship_index
                    .lock()
                    .map_err(|_| EngramError::Storage("Index lock failed".to_string()))?;
                index.add_relationship(&relationship);
            }
        }

        Ok(())
    }

    fn get(&self, id: &str, entity_type: &str) -> Result<Option<GenericEntity>, EngramError> {
        self.load_entity_from_ref(entity_type, id)
    }

    fn delete(&mut self, id: &str, entity_type: &str) -> Result<bool, EngramError> {
        // Remove from relationship index if it's a relationship
        if entity_type == "relationship" {
            if let Some(entity) = self.load_entity_from_ref(entity_type, id)? {
                if let Ok(relationship) = serde_json::from_value::<EntityRelationship>(entity.data)
                {
                    let mut index = self
                        .relationship_index
                        .lock()
                        .map_err(|_| EngramError::Storage("Index lock failed".to_string()))?;
                    index.remove_relationship(&relationship.id);
                }
            }
        }

        self.delete_entity_ref(entity_type, id)
    }

    fn query(&self, filter: &QueryFilter) -> Result<QueryResult, EngramError> {
        let mut results = Vec::new();

        // Determine which entity types to search
        let entity_types = if let Some(entity_type) = &filter.entity_type {
            vec![entity_type.clone()]
        } else {
            vec![
                "task".to_string(),
                "context".to_string(),
                "reasoning".to_string(),
                "knowledge".to_string(),
                "session".to_string(),
                "compliance".to_string(),
            ]
        };

        for entity_type in entity_types {
            let entity_ids = self.list_entity_refs(&entity_type)?;

            for entity_id in entity_ids {
                if let Some(entity) = self.load_entity_from_ref(&entity_type, &entity_id)? {
                    // Apply filters
                    if let Some(agent_filter) = &filter.agent {
                        if entity.agent != *agent_filter {
                            continue;
                        }
                    }

                    // Apply field filters
                    let mut matches = true;
                    for (field, value) in &filter.field_filters {
                        if let Some(entity_value) = entity.data.get(field) {
                            if entity_value != value {
                                matches = false;
                                break;
                            }
                        } else {
                            matches = false;
                            break;
                        }
                    }

                    if matches {
                        results.push(entity);
                    }
                }
            }
        }

        // Apply sorting
        results.sort_by(|a, b| {
            if let Some(sort_field) = &filter.sort_by {
                let a_val = a.data.get(sort_field);
                let b_val = b.data.get(sort_field);
                match filter.sort_order {
                    SortOrder::Asc => a_val.cmp(&b_val),
                    SortOrder::Desc => b_val.cmp(&a_val),
                }
            } else {
                // Default sort by updated timestamp
                match filter.sort_order {
                    SortOrder::Asc => a.updated.cmp(&b.updated),
                    SortOrder::Desc => b.updated.cmp(&a.updated),
                }
            }
        });

        // Apply pagination
        let offset = filter.offset.unwrap_or(0);
        let limit = filter.limit.unwrap_or(50);
        let total = results.len();

        let paginated_results = results.into_iter().skip(offset).take(limit).collect();

        Ok(QueryResult {
            entities: paginated_results,
            total_count: total,
        })
    }

    fn get_stats(&self) -> Result<StorageStats, EngramError> {
        let mut stats = StorageStats::default();

        let entity_types = [
            "task",
            "context",
            "reasoning",
            "knowledge",
            "session",
            "compliance",
            "relationship",
        ];

        for entity_type in &entity_types {
            let entity_ids = self.list_entity_refs(entity_type)?;
            match *entity_type {
                "task" => stats.task_count = entity_ids.len() as u32,
                "context" => stats.context_count = entity_ids.len() as u32,
                "reasoning" => stats.reasoning_count = entity_ids.len() as u32,
                "knowledge" => stats.knowledge_count = entity_ids.len() as u32,
                "session" => stats.session_count = entity_ids.len() as u32,
                "compliance" => stats.compliance_count = entity_ids.len() as u32,
                "relationship" => stats.relationship_count = entity_ids.len() as u32,
                _ => {}
            }
        }

        Ok(stats)
    }

    fn get_all(&self, entity_type: &str) -> Result<Vec<GenericEntity>, EngramError> {
        let entity_ids = self.list_entity_refs(entity_type)?;
        let mut entities = Vec::new();

        for entity_id in entity_ids {
            if let Some(entity) = self.load_entity_from_ref(entity_type, &entity_id)? {
                entities.push(entity);
            }
        }

        Ok(entities)
    }

    fn update(&mut self, entity: &GenericEntity) -> Result<(), EngramError> {
        // For refs storage, update is the same as store
        self.store(entity)
    }

    fn set_agent(&mut self, agent: &str) {
        self.current_agent = agent.to_string();
    }

    fn get_agent(&self) -> &str {
        &self.current_agent
    }

    fn get_git_commits(
        &self,
        entity_type: &str,
        limit: Option<usize>,
    ) -> Result<Vec<GitCommit>, EngramError> {
        let repo = self
            .repository
            .lock()
            .map_err(|_| EngramError::Storage("Repository lock failed".to_string()))?;
        let mut commits = Vec::new();

        // For Git refs storage, we need to track refs changes in Git history
        // This is a simplified implementation - in practice we'd need to traverse
        // Git history looking for ref updates

        // For now, return empty list as this feature needs more sophisticated implementation
        Ok(commits)
    }

    fn commit_changes(&self, message: &str) -> Result<(), EngramError> {
        let repo = self
            .repository
            .lock()
            .map_err(|_| EngramError::Storage("Repository lock failed".to_string()))?;

        // For Git refs storage, changes are automatically committed when refs are updated
        // This method could be used to create summary commits or update working directory

        // For now, this is a no-op as refs are updated atomically
        Ok(())
    }
}

// RelationshipStorage trait implementation
impl RelationshipStorage for GitRefsStorage {
    fn store_relationship(&mut self, relationship: &EntityRelationship) -> Result<(), EngramError> {
        let generic_entity = GenericEntity {
            id: relationship.id.clone(),
            entity_type: "relationship".to_string(),
            agent: relationship.agent.clone(),
            created: Some(relationship.created),
            updated: Some(relationship.updated),
            data: serde_json::to_value(relationship)
                .map_err(|e| EngramError::Serialization(e.to_string()))?,
        };

        self.store(&generic_entity)
    }

    fn get_relationship(&self, id: &str) -> Result<Option<EntityRelationship>, EngramError> {
        if let Some(entity) = self.get(id, "relationship")? {
            let relationship = serde_json::from_value(entity.data)
                .map_err(|e| EngramError::Deserialization(e.to_string()))?;
            Ok(Some(relationship))
        } else {
            Ok(None)
        }
    }

    fn delete_relationship(&mut self, id: &str) -> Result<bool, EngramError> {
        self.delete(id, "relationship")
    }

    fn find_relationships(
        &self,
        filter: &RelationshipFilter,
    ) -> Result<Vec<EntityRelationship>, EngramError> {
        let index = self
            .relationship_index
            .lock()
            .map_err(|_| EngramError::Storage("Index lock failed".to_string()))?;
        Ok(index.find_relationships(filter))
    }

    fn find_connected_entities(
        &self,
        entity_id: &str,
        direction: RelationshipDirection,
        relationship_type: Option<&str>,
    ) -> Result<Vec<EntityRelationship>, EngramError> {
        let index = self
            .relationship_index
            .lock()
            .map_err(|_| EngramError::Storage("Index lock failed".to_string()))?;
        Ok(index.find_connected_entities(entity_id, direction, relationship_type))
    }

    fn get_relationship_stats(&self) -> Result<RelationshipStats, EngramError> {
        let index = self
            .relationship_index
            .lock()
            .map_err(|_| EngramError::Storage("Index lock failed".to_string()))?;
        Ok(index.get_stats())
    }

    fn find_path(
        &self,
        source_id: &str,
        target_id: &str,
        max_depth: usize,
        algorithm: TraversalAlgorithm,
    ) -> Result<Option<Vec<EntityRelationship>>, EngramError> {
        let index = self
            .relationship_index
            .lock()
            .map_err(|_| EngramError::Storage("Index lock failed".to_string()))?;
        Ok(index.find_path(source_id, target_id, max_depth, algorithm))
    }

    fn get_connected_component(
        &self,
        entity_id: &str,
    ) -> Result<Vec<EntityRelationship>, EngramError> {
        let index = self
            .relationship_index
            .lock()
            .map_err(|_| EngramError::Storage("Index lock failed".to_string()))?;
        Ok(index.get_connected_component(entity_id))
    }
}

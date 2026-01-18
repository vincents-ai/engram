//! Git-based storage implementation

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
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

pub struct GitStorage {
    repository: Arc<Mutex<Repository>>,
    workspace_path: PathBuf,
    engram_dir: PathBuf,
    entity_registry: EntityRegistry,
    current_agent: String,
    relationship_index: Arc<Mutex<RelationshipIndex>>,
}

impl std::fmt::Debug for GitStorage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GitStorage")
            .field("workspace_path", &self.workspace_path)
            .field("engram_dir", &self.engram_dir)
            .field("current_agent", &self.current_agent)
            .finish()
    }
}

impl GitStorage {
    /// Create new Git storage instance
    pub fn new(workspace_path: &str, agent: &str) -> Result<Self, EngramError> {
        let workspace_path = PathBuf::from(workspace_path);
        let engram_dir = workspace_path.join(".engram");

        // Initialize repository if it doesn't exist
        let repository = if !workspace_path.join(".git").exists() {
            Repository::init(&workspace_path).map_err(|e| EngramError::Git(e.to_string()))?
        } else {
            Repository::open(&workspace_path).map_err(|e| EngramError::Git(e.to_string()))?
        };

        // Initialize engram directory
        fs::create_dir_all(&engram_dir).map_err(EngramError::Io)?;

        // Initialize entity registry with all entity types
        let mut registry = EntityRegistry::new();
        registry.register::<crate::entities::Task>();
        registry.register::<crate::entities::Context>();
        registry.register::<crate::entities::Reasoning>();
        registry.register::<crate::entities::Knowledge>();
        registry.register::<crate::entities::Session>();
        registry.register::<crate::entities::Compliance>();
        registry.register::<crate::entities::EntityRelationship>();

        let mut storage = GitStorage {
            repository: Arc::new(Mutex::new(repository)),
            workspace_path,
            engram_dir,
            entity_registry: registry,
            current_agent: agent.to_string(),
            relationship_index: Arc::new(Mutex::new(RelationshipIndex::new())),
        };

        let _ = storage.rebuild_relationship_index();

        Ok(storage)
    }

    /// Get file path for an entity
    fn get_entity_path(&self, entity_type: &str, entity_id: &str) -> PathBuf {
        let mut path = self.engram_dir.join(entity_type);
        path.push(format!("{}.json", entity_id));
        path
    }

    /// Ensure type directory exists
    fn ensure_type_directory(&self, entity_type: &str) -> Result<(), EngramError> {
        let type_dir = self.engram_dir.join(entity_type);
        fs::create_dir_all(&type_dir).map_err(EngramError::Io)?;
        Ok(())
    }

    /// Serialize entity to file
    fn serialize_entity_to_file(
        &self,
        entity: &GenericEntity,
        path: &Path,
    ) -> Result<(), EngramError> {
        let memory_entity = MemoryEntity::new(
            entity.id.clone(),
            entity.entity_type.clone(),
            entity.agent.clone(),
            entity.timestamp,
            HashMap::from([("entity".to_string(), entity.data.clone())]),
        );

        let json_content = memory_entity
            .to_json()
            .map_err(EngramError::Serialization)?;

        fs::write(path, json_content).map_err(EngramError::Io)?;
        Ok(())
    }

    /// Deserialize entity from file
    fn deserialize_entity_from_file(
        &self,
        path: &Path,
    ) -> Result<Option<GenericEntity>, EngramError> {
        let content = fs::read_to_string(path).map_err(EngramError::Io)?;
        let memory_entity: MemoryEntity =
            MemoryEntity::from_json(&content).map_err(EngramError::Serialization)?;

        if let Some(entity_data) = memory_entity.get_field("entity") {
            let generic = GenericEntity {
                id: memory_entity.id.clone(),
                entity_type: memory_entity.entity_type.clone(),
                agent: memory_entity.agent.clone(),
                timestamp: memory_entity.timestamp,
                data: entity_data.clone(),
            };

            Ok(Some(generic))
        } else {
            Ok(None)
        }
    }

    /// Add files to git index
    fn add_to_index(&self, paths: &[&Path]) -> Result<(), EngramError> {
        let repo = self
            .repository
            .lock()
            .map_err(|e| EngramError::Git(format!("Failed to lock repository: {}", e)))?;
        let mut index = repo.index().map_err(|e| EngramError::Git(e.to_string()))?;

        for path in paths {
            let relative_path = path
                .strip_prefix(&self.workspace_path)
                .map_err(|e| EngramError::Git(format!("Failed to make path relative: {}", e)))?;

            index
                .add_path(relative_path)
                .map_err(|e| EngramError::Git(e.to_string()))?;
        }

        index.write().map_err(|e| EngramError::Git(e.to_string()))?;

        Ok(())
    }

    /// Create a commit
    fn create_commit(&self, message: &str) -> Result<Oid, EngramError> {
        let repo = self
            .repository
            .lock()
            .map_err(|e| EngramError::Git(format!("Failed to lock repository: {}", e)))?;
        let mut index = repo.index().map_err(|e| EngramError::Git(e.to_string()))?;

        let tree_id = index
            .write_tree()
            .map_err(|e| EngramError::Git(e.to_string()))?;
        let tree = repo
            .find_tree(tree_id)
            .map_err(|e| EngramError::Git(e.to_string()))?;

        // Get signature
        let signature = self.signature()?;

        // Get parent commit
        let parent_commit = repo.head().and_then(|head| head.peel_to_commit()).ok();

        let parents = if let Some(ref parent) = parent_commit {
            vec![parent]
        } else {
            vec![]
        };

        repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            message,
            &tree,
            &parents,
        )
        .map_err(|e| EngramError::Git(e.to_string()))
    }

    /// Get git signature for current agent
    fn signature(&self) -> Result<Signature<'_>, EngramError> {
        Signature::now(
            &self.current_agent,
            &format!("{}@engram.local", self.current_agent),
        )
        .map_err(|e| EngramError::Git(e.to_string()))
    }
}

impl Storage for GitStorage {
    fn store(&mut self, entity: &GenericEntity) -> Result<(), EngramError> {
        // Ensure type directory exists
        self.ensure_type_directory(&entity.entity_type)?;

        // Serialize to file
        let file_path = self.get_entity_path(&entity.entity_type, &entity.id);
        self.serialize_entity_to_file(entity, &file_path)?;

        // Add to git and commit
        self.add_to_index(&[&file_path])?;

        let commit_message = format!(
            "Engram: Update {} {} by agent {}",
            entity.entity_type, entity.id, entity.agent
        );

        self.create_commit(&commit_message)?;

        Ok(())
    }

    fn get(&self, id: &str, entity_type: &str) -> Result<Option<GenericEntity>, EngramError> {
        let file_path = self.get_entity_path(entity_type, id);

        if !file_path.exists() {
            return Ok(None);
        }

        self.deserialize_entity_from_file(&file_path)
    }

    fn query_by_agent(
        &self,
        agent: &str,
        entity_type: Option<&str>,
    ) -> Result<Vec<GenericEntity>, EngramError> {
        let mut results = Vec::new();
        let types_to_query = entity_type.map(|t| vec![t.to_string()]).unwrap_or_else(|| {
            self.entity_registry
                .list_types()
                .into_iter()
                .map(String::from)
                .collect()
        });

        for entity_type in types_to_query {
            let type_dir = self.engram_dir.join(&entity_type);
            if !type_dir.exists() {
                continue;
            }

            let entries = fs::read_dir(type_dir).map_err(EngramError::Io)?;
            for entry in entries {
                let entry = entry.map_err(EngramError::Io)?;
                let path = entry.path();

                if let Some(entity) = self.deserialize_entity_from_file(&path)? {
                    if entity.agent == agent {
                        results.push(entity);
                    }
                }
            }
        }

        Ok(results)
    }

    fn query_by_time_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<GenericEntity>, EngramError> {
        let mut results = Vec::new();

        for entity_type in self.entity_registry.list_types() {
            let type_dir = self.engram_dir.join(entity_type);
            if !type_dir.exists() {
                continue;
            }

            let entries = fs::read_dir(type_dir).map_err(EngramError::Io)?;
            for entry in entries {
                let entry = entry.map_err(EngramError::Io)?;
                let path = entry.path();

                if let Some(entity) = self.deserialize_entity_from_file(&path)? {
                    if entity.timestamp >= start && entity.timestamp <= end {
                        results.push(entity);
                    }
                }
            }
        }

        Ok(results)
    }

    fn delete(&mut self, id: &str, entity_type: &str) -> Result<(), EngramError> {
        let file_path = self.get_entity_path(entity_type, id);

        if !file_path.exists() {
            return Err(EngramError::NotFound(format!(
                "Entity {} of type {} not found",
                id, entity_type
            )));
        }

        // Remove file
        fs::remove_file(&file_path).map_err(EngramError::Io)?;

        // Remove from git and commit
        let repo = self
            .repository
            .lock()
            .map_err(|e| EngramError::Git(format!("Failed to lock repository: {}", e)))?;
        let mut index = repo.index().map_err(|e| EngramError::Git(e.to_string()))?;

        let relative_path = file_path
            .strip_prefix(&self.workspace_path)
            .map_err(|e| EngramError::Git(format!("Failed to make path relative: {}", e)))?;

        index
            .remove_path(relative_path)
            .map_err(|e| EngramError::Git(e.to_string()))?;

        index.write().map_err(|e| EngramError::Git(e.to_string()))?;

        let commit_message = format!(
            "Engram: Delete {} {} by agent {}",
            entity_type, id, self.current_agent
        );

        self.create_commit(&commit_message)?;

        Ok(())
    }

    fn list_ids(&self, entity_type: &str) -> Result<Vec<String>, EngramError> {
        let type_dir = self.engram_dir.join(entity_type);

        if !type_dir.exists() {
            return Ok(Vec::new());
        }

        let mut ids = Vec::new();
        let entries = fs::read_dir(type_dir).map_err(EngramError::Io)?;

        for entry in entries {
            let entry = entry.map_err(EngramError::Io)?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    ids.push(stem.to_string());
                }
            }
        }

        Ok(ids)
    }

    fn sync(&mut self) -> Result<(), EngramError> {
        Ok(())
    }

    fn current_branch(&self) -> Result<String, EngramError> {
        Ok("main".to_string())
    }

    fn create_branch(&mut self, _branch_name: &str) -> Result<(), EngramError> {
        Ok(())
    }

    fn switch_branch(&mut self, _branch_name: &str) -> Result<(), EngramError> {
        Ok(())
    }

    fn merge_branches(&mut self, _source: &str, _target: &str) -> Result<(), EngramError> {
        Ok(())
    }

    fn history(&self, _limit: Option<usize>) -> Result<Vec<GitCommit>, EngramError> {
        Ok(Vec::new())
    }

    fn query(&self, filter: &QueryFilter) -> Result<QueryResult, EngramError> {
        let mut all_entities = Vec::new();

        let types_to_query: Vec<String> = if let Some(entity_type) = &filter.entity_type {
            vec![entity_type.clone()]
        } else {
            self.entity_registry
                .list_types()
                .into_iter()
                .map(String::from)
                .collect()
        };

        for entity_type in types_to_query {
            let type_dir = self.engram_dir.join(&entity_type);
            if !type_dir.exists() {
                continue;
            }

            let entries = fs::read_dir(type_dir).map_err(EngramError::Io)?;
            for entry in entries {
                let entry = entry.map_err(EngramError::Io)?;
                let path = entry.path();

                if let Some(entity) = self.deserialize_entity_from_file(&path)? {
                    if let Some(agent_filter) = &filter.agent {
                        if entity.agent != *agent_filter {
                            continue;
                        }
                    }

                    if let Some(time_range) = &filter.time_range {
                        if entity.timestamp < time_range.start || entity.timestamp > time_range.end
                        {
                            continue;
                        }
                    }

                    if let Some(search_query) = &filter.text_search {
                        let search_lower: String = search_query.to_lowercase();
                        let entity_json = serde_json::to_string(&entity.data).unwrap_or_default();
                        if !entity_json.to_lowercase().contains(&search_lower) {
                            continue;
                        }
                    }

                    let mut matches_field_filters = true;
                    for (field, expected_value) in &filter.field_filters {
                        if let Some(actual_value) = entity.data.get(field) {
                            if actual_value != expected_value {
                                matches_field_filters = false;
                                break;
                            }
                        } else {
                            matches_field_filters = false;
                            break;
                        }
                    }

                    if !matches_field_filters {
                        continue;
                    }

                    all_entities.push(entity);
                }
            }
        }

        if let Some(sort_field) = &filter.sort_by {
            all_entities.sort_by(|a, b| {
                let a_value = a.data.get(sort_field);
                let b_value = b.data.get(sort_field);

                let cmp = match (a_value, b_value) {
                    (Some(a_val), Some(b_val)) => {
                        let a_str = a_val.as_str().unwrap_or("");
                        let b_str = b_val.as_str().unwrap_or("");
                        a_str.cmp(b_str)
                    }
                    (Some(_), None) => std::cmp::Ordering::Greater,
                    (None, Some(_)) => std::cmp::Ordering::Less,
                    (None, None) => std::cmp::Ordering::Equal,
                };

                match filter.sort_order {
                    SortOrder::Asc => cmp,
                    SortOrder::Desc => cmp.reverse(),
                }
            });
        } else {
            all_entities.sort_by(|a, b| match filter.sort_order {
                SortOrder::Asc => a.timestamp.cmp(&b.timestamp),
                SortOrder::Desc => b.timestamp.cmp(&a.timestamp),
            });
        }

        let total_count = all_entities.len();
        let offset = filter.offset.unwrap_or(0);
        let limit = filter.limit.unwrap_or(50);

        let entities = if offset < total_count {
            let end_idx = std::cmp::min(offset + limit, total_count);
            all_entities[offset..end_idx].to_vec()
        } else {
            Vec::new()
        };

        let has_more = offset + entities.len() < total_count;

        Ok(QueryResult {
            entities,
            total_count,
            has_more,
        })
    }

    fn query_by_type(
        &self,
        entity_type: &str,
        filters: Option<&HashMap<String, Value>>,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> Result<QueryResult, EngramError> {
        let mut filter = QueryFilter {
            entity_type: Some(entity_type.to_string()),
            limit,
            offset,
            ..Default::default()
        };

        if let Some(field_filters) = filters {
            filter.field_filters = field_filters.clone();
        }

        self.query(&filter)
    }

    fn text_search(
        &self,
        query: &str,
        entity_types: Option<&[String]>,
        limit: Option<usize>,
    ) -> Result<Vec<GenericEntity>, EngramError> {
        let mut filter = QueryFilter {
            text_search: Some(query.to_string()),
            limit,
            ..Default::default()
        };

        if let Some(types) = entity_types {
            if types.len() == 1 {
                filter.entity_type = Some(types[0].clone());
            }
        }

        let result = self.query(&filter)?;
        Ok(result.entities)
    }

    fn count(&self, filter: &QueryFilter) -> Result<usize, EngramError> {
        let count_filter = QueryFilter {
            limit: None,
            offset: None,
            ..filter.clone()
        };

        let result = self.query(&count_filter)?;
        Ok(result.total_count)
    }

    fn get_all(&self, entity_type: &str) -> Result<Vec<GenericEntity>, EngramError> {
        let filter = QueryFilter {
            entity_type: Some(entity_type.to_string()),
            limit: None,
            offset: None,
            ..Default::default()
        };

        let result = self.query(&filter)?;
        Ok(result.entities)
    }

    fn bulk_store(&mut self, entities: &[GenericEntity]) -> Result<(), EngramError> {
        let mut all_paths = Vec::new();

        for entity in entities {
            self.ensure_type_directory(&entity.entity_type)?;
            let file_path = self.get_entity_path(&entity.entity_type, &entity.id);
            self.serialize_entity_to_file(entity, &file_path)?;
            all_paths.push(file_path);
        }

        let path_refs: Vec<&Path> = all_paths.iter().map(|p| p.as_path()).collect();
        self.add_to_index(&path_refs)?;

        let commit_message = format!(
            "Engram: Bulk store {} entities by agent {}",
            entities.len(),
            self.current_agent
        );

        self.create_commit(&commit_message)?;

        Ok(())
    }

    fn get_stats(&self) -> Result<StorageStats, EngramError> {
        let mut total_entities = 0;
        let mut entities_by_type = HashMap::new();
        let mut entities_by_agent = HashMap::new();
        let mut total_storage_size = 0u64;

        for entity_type in self.entity_registry.list_types() {
            let type_dir = self.engram_dir.join(entity_type);
            if !type_dir.exists() {
                entities_by_type.insert(entity_type.to_string(), 0);
                continue;
            }

            let mut type_count = 0;
            let entries = fs::read_dir(&type_dir).map_err(EngramError::Io)?;

            for entry in entries {
                let entry = entry.map_err(EngramError::Io)?;
                let path = entry.path();

                if let Ok(metadata) = entry.metadata() {
                    total_storage_size += metadata.len();
                }

                if let Some(entity) = self.deserialize_entity_from_file(&path)? {
                    total_entities += 1;
                    type_count += 1;

                    *entities_by_agent.entry(entity.agent.clone()).or_insert(0) += 1;
                }
            }

            entities_by_type.insert(entity_type.to_string(), type_count);
        }

        Ok(StorageStats {
            total_entities,
            entities_by_type,
            entities_by_agent,
            total_storage_size,
            last_sync: None,
        })
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl RelationshipStorage for GitStorage {
    fn store_relationship(&mut self, relationship: &EntityRelationship) -> Result<(), EngramError> {
        let generic = GenericEntity {
            id: relationship.id.clone(),
            entity_type: EntityRelationship::entity_type().to_string(),
            agent: relationship.agent.clone(),
            timestamp: relationship.timestamp,
            data: serde_json::to_value(relationship)?,
        };

        self.store(&generic)?;

        let mut index = self.relationship_index.lock().unwrap();
        index.add_relationship(relationship);

        Ok(())
    }

    fn get_relationship(&self, id: &str) -> Result<Option<EntityRelationship>, EngramError> {
        if let Some(generic) = self.get(id, EntityRelationship::entity_type())? {
            if let Ok(relationship) = serde_json::from_value::<EntityRelationship>(generic.data) {
                return Ok(Some(relationship));
            }
        }
        Ok(None)
    }

    fn query_relationships(
        &self,
        filter: &RelationshipFilter,
    ) -> Result<Vec<EntityRelationship>, EngramError> {
        let all_rels = self.get_all(EntityRelationship::entity_type())?;
        let mut relationships = Vec::new();

        for generic in all_rels {
            if let Ok(relationship) = serde_json::from_value::<EntityRelationship>(generic.data) {
                if filter.matches(&relationship) {
                    relationships.push(relationship);
                }
            }
        }

        Ok(relationships)
    }

    fn get_entity_relationships(
        &self,
        entity_id: &str,
    ) -> Result<Vec<EntityRelationship>, EngramError> {
        let index = self.relationship_index.lock().unwrap();
        let rel_ids = index.get_all_relationships(entity_id);

        let mut relationships = Vec::new();
        for rel_id in rel_ids {
            if let Some(relationship) = self.get_relationship(&rel_id)? {
                relationships.push(relationship);
            }
        }

        Ok(relationships)
    }

    fn get_outbound_relationships(
        &self,
        entity_id: &str,
    ) -> Result<Vec<EntityRelationship>, EngramError> {
        let index = self.relationship_index.lock().unwrap();
        let rel_ids = index.get_outbound(entity_id);

        let mut relationships = Vec::new();
        for rel_id in rel_ids {
            if let Some(relationship) = self.get_relationship(&rel_id)? {
                relationships.push(relationship);
            }
        }

        Ok(relationships)
    }

    fn get_inbound_relationships(
        &self,
        entity_id: &str,
    ) -> Result<Vec<EntityRelationship>, EngramError> {
        let index = self.relationship_index.lock().unwrap();
        let rel_ids = index.get_inbound(entity_id);

        let mut relationships = Vec::new();
        for rel_id in rel_ids {
            if let Some(relationship) = self.get_relationship(&rel_id)? {
                relationships.push(relationship);
            }
        }

        Ok(relationships)
    }

    fn find_paths(
        &self,
        source_id: &str,
        target_id: &str,
        algorithm: TraversalAlgorithm,
        max_depth: Option<usize>,
    ) -> Result<Vec<super::EntityPath>, EngramError> {
        match algorithm {
            TraversalAlgorithm::BreadthFirst => {
                self.bfs_path_search(source_id, target_id, max_depth)
            }
            TraversalAlgorithm::DepthFirst => self.dfs_path_search(source_id, target_id, max_depth),
            TraversalAlgorithm::Dijkstra => {
                self.dijkstra_path_search(source_id, target_id, max_depth)
            }
        }
    }

    fn get_connected_entities(
        &self,
        entity_id: &str,
        algorithm: TraversalAlgorithm,
        max_depth: Option<usize>,
    ) -> Result<Vec<String>, EngramError> {
        match algorithm {
            TraversalAlgorithm::BreadthFirst => self.bfs_connected_search(entity_id, max_depth),
            TraversalAlgorithm::DepthFirst => self.dfs_connected_search(entity_id, max_depth),
            TraversalAlgorithm::Dijkstra => self.dijkstra_connected_search(entity_id, max_depth),
        }
    }

    fn delete_relationship(&mut self, id: &str) -> Result<(), EngramError> {
        if let Some(relationship) = self.get_relationship(id)? {
            let mut index = self.relationship_index.lock().unwrap();
            index.remove_relationship(&relationship);
        }

        self.delete(id, EntityRelationship::entity_type())
    }

    fn get_relationship_index(&self) -> Result<&RelationshipIndex, EngramError> {
        Err(EngramError::Validation(
            "Direct index access not supported for GitStorage. Use query methods instead."
                .to_string(),
        ))
    }

    fn rebuild_relationship_index(&mut self) -> Result<(), EngramError> {
        let all_rels = self.get_all(EntityRelationship::entity_type())?;
        let mut index = self.relationship_index.lock().unwrap();

        *index = RelationshipIndex::new();

        for generic in all_rels {
            if let Ok(relationship) = serde_json::from_value::<EntityRelationship>(generic.data) {
                index.add_relationship(&relationship);
            }
        }

        Ok(())
    }

    fn get_relationship_stats(&self) -> Result<RelationshipStats, EngramError> {
        use crate::entities::EntityRelationType;
        use crate::storage::relationship_storage::RelationshipStats;
        use std::collections::HashMap;

        let all_rels = self.get_all(EntityRelationship::entity_type())?;
        let mut total_relationships = 0;
        let mut relationships_by_type: HashMap<EntityRelationType, usize> = HashMap::new();
        let mut bidirectional_count = 0;
        let mut entity_connections: HashMap<String, usize> = HashMap::new();

        for generic in all_rels {
            if let Ok(relationship) = serde_json::from_value::<EntityRelationship>(generic.data) {
                total_relationships += 1;

                *relationships_by_type
                    .entry(relationship.relationship_type.clone())
                    .or_insert(0) += 1;

                if relationship.direction == RelationshipDirection::Bidirectional {
                    bidirectional_count += 1;
                }

                *entity_connections
                    .entry(relationship.source_id.clone())
                    .or_insert(0) += 1;
                *entity_connections
                    .entry(relationship.target_id.clone())
                    .or_insert(0) += 1;
            }
        }

        let entity_count = entity_connections.len();
        let average_connections_per_entity = if entity_count > 0 {
            entity_connections.values().sum::<usize>() as f64 / entity_count as f64
        } else {
            0.0
        };

        let most_connected_entity = entity_connections
            .into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(entity, count)| (entity, count));

        let max_possible_edges = if entity_count > 1 {
            entity_count * (entity_count - 1)
        } else {
            1
        };
        let relationship_density = total_relationships as f64 / max_possible_edges as f64;

        Ok(RelationshipStats {
            total_relationships,
            relationships_by_type,
            bidirectional_count,
            average_connections_per_entity,
            most_connected_entity,
            relationship_density,
        })
    }
}

impl GitStorage {
    fn bfs_path_search(
        &self,
        source_id: &str,
        target_id: &str,
        max_depth: Option<usize>,
    ) -> Result<Vec<super::EntityPath>, EngramError> {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let max_depth = max_depth.unwrap_or(10);

        queue.push_back((source_id.to_string(), Vec::new(), Vec::new(), 0));
        visited.insert(source_id.to_string());

        let mut paths = Vec::new();

        while let Some((current_id, mut path_entities, path_relationships, depth)) =
            queue.pop_front()
        {
            if depth >= max_depth {
                continue;
            }

            if current_id == target_id && !path_entities.is_empty() {
                path_entities.push(current_id.clone());
                paths.push(super::EntityPath {
                    entities: path_entities.clone(),
                    relationships: path_relationships.clone(),
                    total_weight: path_relationships.len() as f64,
                    path_type: super::PathType::Shortest,
                });
                continue;
            }

            let outbound_rels = self.get_outbound_relationships(&current_id)?;
            for relationship in outbound_rels {
                let next_id = &relationship.target_id;
                if !visited.contains(next_id) {
                    visited.insert(next_id.clone());
                    let mut new_path_entities = path_entities.clone();
                    new_path_entities.push(current_id.clone());
                    let mut new_path_relationships = path_relationships.clone();
                    new_path_relationships.push(relationship.id().to_string());
                    queue.push_back((
                        next_id.clone(),
                        new_path_entities,
                        new_path_relationships,
                        depth + 1,
                    ));
                }
            }
        }

        Ok(paths)
    }

    fn dfs_path_search(
        &self,
        source_id: &str,
        target_id: &str,
        max_depth: Option<usize>,
    ) -> Result<Vec<super::EntityPath>, EngramError> {
        let mut stack = Vec::new();
        let mut visited = HashSet::new();
        let max_depth = max_depth.unwrap_or(10);

        stack.push((source_id.to_string(), Vec::new(), Vec::new(), 0));

        let mut paths = Vec::new();

        while let Some((current_id, mut path_entities, path_relationships, depth)) = stack.pop() {
            if depth >= max_depth {
                continue;
            }

            if visited.contains(&current_id) {
                continue;
            }
            visited.insert(current_id.clone());

            if current_id == target_id && !path_entities.is_empty() {
                path_entities.push(current_id.clone());
                paths.push(super::EntityPath {
                    entities: path_entities.clone(),
                    relationships: path_relationships.clone(),
                    total_weight: path_relationships.len() as f64,
                    path_type: super::PathType::AllPaths,
                });
                continue;
            }

            let outbound_rels = self.get_outbound_relationships(&current_id)?;
            for relationship in outbound_rels {
                let next_id = &relationship.target_id;
                if !visited.contains(next_id) {
                    let mut new_path_entities = path_entities.clone();
                    new_path_entities.push(current_id.clone());
                    let mut new_path_relationships = path_relationships.clone();
                    new_path_relationships.push(relationship.id().to_string());
                    stack.push((
                        next_id.clone(),
                        new_path_entities,
                        new_path_relationships,
                        depth + 1,
                    ));
                }
            }
        }

        Ok(paths)
    }

    fn dijkstra_path_search(
        &self,
        source_id: &str,
        target_id: &str,
        max_depth: Option<usize>,
    ) -> Result<Vec<super::EntityPath>, EngramError> {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        #[derive(Debug, Clone, PartialEq)]
        struct DijkstraState {
            cost: i64,
            entity_id: String,
            path_entities: Vec<String>,
            path_relationships: Vec<String>,
        }

        impl Eq for DijkstraState {}

        impl Ord for DijkstraState {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.cost.cmp(&other.cost)
            }
        }

        impl PartialOrd for DijkstraState {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        let mut heap = BinaryHeap::new();
        let mut distances: HashMap<String, i64> = HashMap::new();
        let max_depth = max_depth.unwrap_or(10);

        heap.push(Reverse(DijkstraState {
            cost: 0,
            entity_id: source_id.to_string(),
            path_entities: Vec::new(),
            path_relationships: Vec::new(),
        }));

        distances.insert(source_id.to_string(), 0);

        let mut paths = Vec::new();

        while let Some(Reverse(state)) = heap.pop() {
            if state.path_entities.len() >= max_depth {
                continue;
            }

            if state.entity_id == target_id && !state.path_entities.is_empty() {
                let mut final_entities = state.path_entities.clone();
                final_entities.push(state.entity_id.clone());
                paths.push(super::EntityPath {
                    entities: final_entities,
                    relationships: state.path_relationships.clone(),
                    total_weight: state.cost as f64 / 1000.0,
                    path_type: super::PathType::Shortest,
                });
                continue;
            }

            if let Some(&best_distance) = distances.get(&state.entity_id) {
                if state.cost > best_distance {
                    continue;
                }
            }

            let outbound_rels = self.get_outbound_relationships(&state.entity_id)?;
            for relationship in outbound_rels {
                let next_id = &relationship.target_id;
                let edge_weight = ((1.0 - relationship.strength.weight()) * 1000.0) as i64;
                let next_cost = state.cost + edge_weight;

                if let Some(&current_best) = distances.get(next_id) {
                    if next_cost >= current_best {
                        continue;
                    }
                }

                distances.insert(next_id.clone(), next_cost);

                let mut new_path_entities = state.path_entities.clone();
                new_path_entities.push(state.entity_id.clone());
                let mut new_path_relationships = state.path_relationships.clone();
                new_path_relationships.push(relationship.id.clone());

                heap.push(Reverse(DijkstraState {
                    cost: next_cost,
                    entity_id: next_id.clone(),
                    path_entities: new_path_entities,
                    path_relationships: new_path_relationships,
                }));
            }
        }

        Ok(paths)
    }

    fn bfs_connected_search(
        &self,
        entity_id: &str,
        max_depth: Option<usize>,
    ) -> Result<Vec<String>, EngramError> {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut connected = Vec::new();
        let max_depth = max_depth.unwrap_or(10);

        queue.push_back((entity_id.to_string(), 0));
        visited.insert(entity_id.to_string());

        while let Some((current_id, depth)) = queue.pop_front() {
            if depth >= max_depth {
                continue;
            }

            if depth > 0 {
                connected.push(current_id.clone());
            }

            let entity_rels = self.get_entity_relationships(&current_id)?;
            for relationship in entity_rels {
                let next_id = if relationship.source_id == current_id {
                    &relationship.target_id
                } else {
                    &relationship.source_id
                };

                if !visited.contains(next_id) {
                    visited.insert(next_id.clone());
                    queue.push_back((next_id.clone(), depth + 1));
                }
            }
        }

        Ok(connected)
    }

    fn dfs_connected_search(
        &self,
        entity_id: &str,
        max_depth: Option<usize>,
    ) -> Result<Vec<String>, EngramError> {
        let mut stack = Vec::new();
        let mut visited = HashSet::new();
        let mut connected = Vec::new();
        let max_depth = max_depth.unwrap_or(10);

        stack.push((entity_id.to_string(), 0));

        while let Some((current_id, depth)) = stack.pop() {
            if depth >= max_depth {
                continue;
            }

            if visited.contains(&current_id) {
                continue;
            }
            visited.insert(current_id.clone());

            if depth > 0 {
                connected.push(current_id.clone());
            }

            let entity_rels = self.get_entity_relationships(&current_id)?;
            for relationship in entity_rels {
                let next_id = if relationship.source_id == current_id {
                    &relationship.target_id
                } else {
                    &relationship.source_id
                };

                if !visited.contains(next_id) {
                    stack.push((next_id.clone(), depth + 1));
                }
            }
        }

        Ok(connected)
    }

    fn dijkstra_connected_search(
        &self,
        entity_id: &str,
        max_depth: Option<usize>,
    ) -> Result<Vec<String>, EngramError> {
        self.bfs_connected_search(entity_id, max_depth)
    }
}

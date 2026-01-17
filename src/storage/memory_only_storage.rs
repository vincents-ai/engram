//! In-memory storage implementation for testing and development

use super::{
    GitCommit, MemoryEntity, QueryFilter, QueryResult, RelationshipIndex, RelationshipStats,
    RelationshipStorage, SortOrder, Storage, StorageStats, TraversalAlgorithm,
};
use crate::entities::{
    Entity, EntityRegistry, EntityRelationType, EntityRelationship, GenericEntity,
    RelationshipDirection, RelationshipFilter,
};
use crate::error::EngramError;
use chrono::{DateTime, Utc};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// In-memory storage backend
pub struct MemoryStorage {
    entities: Arc<Mutex<HashMap<String, MemoryEntity>>>,
    #[allow(dead_code)]
    entity_registry: EntityRegistry,
    current_agent: String,
    commits: Vec<GitCommit>,
    relationship_index: Arc<Mutex<RelationshipIndex>>,
}

impl MemoryStorage {
    /// Create new memory storage instance
    pub fn new(agent: &str) -> Self {
        let mut registry = EntityRegistry::new();
        registry.register::<crate::entities::Task>();
        registry.register::<crate::entities::Context>();
        registry.register::<crate::entities::Reasoning>();
        registry.register::<crate::entities::Knowledge>();
        registry.register::<crate::entities::Session>();
        registry.register::<crate::entities::Compliance>();
        registry.register::<crate::entities::Rule>();
        registry.register::<crate::entities::Standard>();
        registry.register::<crate::entities::ADR>();
        registry.register::<crate::entities::Workflow>();

        Self {
            entities: Arc::new(Mutex::new(HashMap::new())),
            entity_registry: registry,
            current_agent: agent.to_string(),
            commits: Vec::new(),
            relationship_index: Arc::new(Mutex::new(RelationshipIndex::new())),
        }
    }
}

impl Storage for MemoryStorage {
    fn store(&mut self, entity: &GenericEntity) -> Result<(), EngramError> {
        let memory_entity = MemoryEntity::new(
            entity.id.clone(),
            entity.entity_type.clone(),
            entity.agent.clone(),
            entity.timestamp,
            HashMap::from([("entity".to_string(), entity.data.clone())]),
        );

        let mut entities = self.entities.lock().unwrap();
        entities.insert(memory_entity.id.clone(), memory_entity);

        // Create a commit record
        let commit = GitCommit {
            id: format!("commit-{}", uuid::Uuid::new_v4()),
            author: self.current_agent.clone(),
            message: format!("Store {} {}", entity.entity_type, entity.id),
            timestamp: Utc::now(),
            parents: Vec::new(),
        };
        self.commits.push(commit);

        Ok(())
    }

    fn get(&self, id: &str, entity_type: &str) -> Result<Option<GenericEntity>, EngramError> {
        let entities = self.entities.lock().unwrap();
        if let Some(memory_entity) = entities.get(id) {
            if memory_entity.entity_type != entity_type {
                return Ok(None);
            }

            // Extract entity data
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
        } else {
            Ok(None)
        }
    }

    fn query_by_agent(
        &self,
        agent: &str,
        entity_type: Option<&str>,
    ) -> Result<Vec<GenericEntity>, EngramError> {
        let entities = self.entities.lock().unwrap();
        let mut results = Vec::new();

        for memory_entity in entities.values() {
            if memory_entity.agent != agent {
                continue;
            }

            if let Some(filter_type) = entity_type {
                if memory_entity.entity_type != filter_type {
                    continue;
                }
            }

            if let Some(entity_data) = memory_entity.get_field("entity") {
                let generic = GenericEntity {
                    id: memory_entity.id.clone(),
                    entity_type: memory_entity.entity_type.clone(),
                    agent: memory_entity.agent.clone(),
                    timestamp: memory_entity.timestamp,
                    data: entity_data.clone(),
                };

                results.push(generic);
            }
        }

        Ok(results)
    }

    fn query_by_time_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<GenericEntity>, EngramError> {
        let entities = self.entities.lock().unwrap();
        let mut results = Vec::new();

        for memory_entity in entities.values() {
            if memory_entity.timestamp < start || memory_entity.timestamp > end {
                continue;
            }

            if let Some(entity_data) = memory_entity.get_field("entity") {
                let generic = GenericEntity {
                    id: memory_entity.id.clone(),
                    entity_type: memory_entity.entity_type.clone(),
                    agent: memory_entity.agent.clone(),
                    timestamp: memory_entity.timestamp,
                    data: entity_data.clone(),
                };

                results.push(generic);
            }
        }

        Ok(results)
    }

    fn delete(&mut self, id: &str, entity_type: &str) -> Result<(), EngramError> {
        let mut entities = self.entities.lock().unwrap();
        if let Some(memory_entity) = entities.remove(id) {
            if memory_entity.entity_type != entity_type {
                return Err(EngramError::NotFound(format!(
                    "Entity {} of type {} not found",
                    id, entity_type
                )));
            }

            // Create a commit record
            let commit = GitCommit {
                id: format!("commit-{}", uuid::Uuid::new_v4()),
                author: self.current_agent.clone(),
                message: format!("Delete {} {}", entity_type, id),
                timestamp: Utc::now(),
                parents: self
                    .commits
                    .iter()
                    .rev()
                    .take(1)
                    .map(|c| c.id.clone())
                    .collect(),
            };
            self.commits.push(commit);

            Ok(())
        } else {
            Err(EngramError::NotFound(format!("Entity {} not found", id)))
        }
    }

    fn list_ids(&self, entity_type: &str) -> Result<Vec<String>, EngramError> {
        let entities = self.entities.lock().unwrap();
        let mut ids = Vec::new();

        for (id, memory_entity) in entities.iter() {
            if memory_entity.entity_type == entity_type {
                ids.push(id.clone());
            }
        }

        Ok(ids)
    }

    fn sync(&mut self) -> Result<(), EngramError> {
        // Memory storage doesn't need syncing
        Ok(())
    }

    fn current_branch(&self) -> Result<String, EngramError> {
        Ok("main".to_string())
    }

    fn create_branch(&mut self, _branch_name: &str) -> Result<(), EngramError> {
        // Memory storage doesn't support branches
        Ok(())
    }

    fn switch_branch(&mut self, _branch_name: &str) -> Result<(), EngramError> {
        // Memory storage doesn't support branches
        Ok(())
    }

    fn merge_branches(&mut self, _source: &str, _target: &str) -> Result<(), EngramError> {
        // Memory storage doesn't support branches
        Ok(())
    }

    fn history(&self, limit: Option<usize>) -> Result<Vec<GitCommit>, EngramError> {
        let limit = limit.unwrap_or(usize::MAX);
        Ok(self.commits.iter().rev().take(limit).cloned().collect())
    }

    fn query(&self, filter: &QueryFilter) -> Result<QueryResult, EngramError> {
        let entities = self.entities.lock().unwrap();
        let mut all_entities = Vec::new();

        for memory_entity in entities.values() {
            if let Some(entity_type_filter) = &filter.entity_type {
                if memory_entity.entity_type != *entity_type_filter {
                    continue;
                }
            }

            if let Some(agent_filter) = &filter.agent {
                if memory_entity.agent != *agent_filter {
                    continue;
                }
            }

            if let Some(time_range) = &filter.time_range {
                if memory_entity.timestamp < time_range.start
                    || memory_entity.timestamp > time_range.end
                {
                    continue;
                }
            }

            if let Some(entity_data) = memory_entity.get_field("entity") {
                if let Some(search_query) = &filter.text_search {
                    let search_lower: String = search_query.to_lowercase();
                    let entity_json = serde_json::to_string(&entity_data).unwrap_or_default();
                    if !entity_json.to_lowercase().contains(&search_lower) {
                        continue;
                    }
                }

                let mut matches_field_filters = true;
                for (field, expected_value) in &filter.field_filters {
                    if let Some(actual_value) = entity_data.get(field) {
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

                let generic = GenericEntity {
                    id: memory_entity.id.clone(),
                    entity_type: memory_entity.entity_type.clone(),
                    agent: memory_entity.agent.clone(),
                    timestamp: memory_entity.timestamp,
                    data: entity_data.clone(),
                };

                all_entities.push(generic);
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
        for entity in entities {
            self.store(entity)?;
        }
        Ok(())
    }

    fn get_stats(&self) -> Result<StorageStats, EngramError> {
        let entities = self.entities.lock().unwrap();
        let mut total_entities = 0;
        let mut entities_by_type = HashMap::new();
        let mut entities_by_agent = HashMap::new();
        let mut total_storage_size = 0u64;

        for memory_entity in entities.values() {
            total_entities += 1;

            *entities_by_type
                .entry(memory_entity.entity_type.clone())
                .or_insert(0) += 1;
            *entities_by_agent
                .entry(memory_entity.agent.clone())
                .or_insert(0) += 1;

            let entity_json = memory_entity.to_json().unwrap_or_default();
            total_storage_size += entity_json.len() as u64;
        }

        Ok(StorageStats {
            total_entities,
            entities_by_type,
            entities_by_agent,
            total_storage_size,
            last_sync: None,
        })
    }
}

impl RelationshipStorage for MemoryStorage {
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
        for rel_id in &rel_ids {
            if let Some(rel) = self.get_relationship(&rel_id)? {
                relationships.push(rel);
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
        for rel_id in &rel_ids {
            if let Some(rel) = self.get_relationship(&rel_id)? {
                relationships.push(rel);
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
        for rel_id in &rel_ids {
            if let Some(rel) = self.get_relationship(&rel_id)? {
                relationships.push(rel);
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
                let path = super::GraphAnalyzer::bfs(self, source_id, Some(target_id), max_depth)?;
                if path.len() > 1 && path.last() == Some(&target_id.to_string()) {
                    Ok(vec![super::EntityPath {
                        entities: path,
                        relationships: Vec::new(),
                        total_weight: 0.0,
                        path_type: super::PathType::Shortest,
                    }])
                } else {
                    Ok(vec![])
                }
            }
            TraversalAlgorithm::DepthFirst => {
                let path = super::GraphAnalyzer::dfs(self, source_id, Some(target_id), max_depth)?;
                if path.len() > 1 && path.last() == Some(&target_id.to_string()) {
                    Ok(vec![super::EntityPath {
                        entities: path,
                        relationships: Vec::new(),
                        total_weight: 0.0,
                        path_type: super::PathType::Shortest,
                    }])
                } else {
                    Ok(vec![])
                }
            }
            TraversalAlgorithm::Dijkstra => {
                if let Some(path) = super::GraphAnalyzer::dijkstra(self, source_id, target_id)? {
                    Ok(vec![path])
                } else {
                    Ok(vec![])
                }
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
            TraversalAlgorithm::BreadthFirst => {
                super::GraphAnalyzer::bfs(self, entity_id, None, max_depth)
            }
            TraversalAlgorithm::DepthFirst => {
                super::GraphAnalyzer::dfs(self, entity_id, None, max_depth)
            }
            TraversalAlgorithm::Dijkstra => {
                super::GraphAnalyzer::bfs(self, entity_id, None, max_depth)
            }
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
            "Direct index access not supported for MemoryStorage. Use query methods instead."
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

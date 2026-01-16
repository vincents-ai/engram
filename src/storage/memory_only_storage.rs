//! In-memory storage implementation for testing and development

use super::{GitCommit, MemoryEntity, QueryFilter, QueryResult, SortOrder, Storage, StorageStats};
use crate::entities::{EntityRegistry, GenericEntity};
use crate::error::EngramError;
use chrono::{DateTime, Utc};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// In-memory storage backend
pub struct MemoryStorage {
    entities: Arc<Mutex<HashMap<String, MemoryEntity>>>,
    entity_registry: EntityRegistry,
    current_agent: String,
    commits: Vec<GitCommit>,
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

//! Relationship-specific storage operations and graph indexing

use std::cmp::{Ordering, Reverse};
use std::collections::{HashMap, HashSet, VecDeque};

use super::Storage;
use crate::entities::{
    Entity, EntityRelationType, EntityRelationship, RelationshipDirection, RelationshipFilter,
};
use crate::error::EngramError;

/// Path in the entity graph
#[derive(Debug, Clone)]
pub struct EntityPath {
    pub entities: Vec<String>,
    pub relationships: Vec<String>,
    pub total_weight: f64,
    pub path_type: PathType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PathType {
    Shortest,
    Strongest,
    AllPaths,
}

/// Graph traversal algorithm type
#[derive(Debug, Clone)]
pub enum TraversalAlgorithm {
    BreadthFirst,
    DepthFirst,
    Dijkstra,
}

/// Relationship index for efficient graph operations
#[derive(Debug, Clone, Default)]
pub struct RelationshipIndex {
    /// Outbound relationships from each entity
    pub outbound: HashMap<String, Vec<String>>,
    /// Inbound relationships to each entity  
    pub inbound: HashMap<String, Vec<String>>,
    /// Relationships by type
    pub by_type: HashMap<EntityRelationType, Vec<String>>,
    /// Bidirectional relationships
    pub bidirectional: HashSet<String>,
}

impl RelationshipIndex {
    /// Create new empty index
    pub fn new() -> Self {
        Self::default()
    }

    /// Add relationship to index
    pub fn add_relationship(&mut self, relationship: &EntityRelationship) {
        let rel_id = relationship.id.clone();

        self.outbound
            .entry(relationship.source_id.clone())
            .or_insert_with(Vec::new)
            .push(rel_id.clone());

        self.inbound
            .entry(relationship.target_id.clone())
            .or_insert_with(Vec::new)
            .push(rel_id.clone());

        self.by_type
            .entry(relationship.relationship_type.clone())
            .or_insert_with(Vec::new)
            .push(rel_id.clone());

        if relationship.direction == RelationshipDirection::Bidirectional {
            self.bidirectional.insert(rel_id);
        }
    }

    pub fn remove_relationship(&mut self, relationship: &EntityRelationship) {
        let rel_id = &relationship.id;

        if let Some(outbound) = self.outbound.get_mut(&relationship.source_id) {
            outbound.retain(|id| id != rel_id);
        }

        if let Some(inbound) = self.inbound.get_mut(&relationship.target_id) {
            inbound.retain(|id| id != rel_id);
        }

        if let Some(type_rels) = self.by_type.get_mut(&relationship.relationship_type) {
            type_rels.retain(|id| id != rel_id);
        }

        self.bidirectional.remove(rel_id);
    }

    /// Get all outbound relationship IDs for an entity
    pub fn get_outbound(&self, entity_id: &str) -> Vec<String> {
        self.outbound.get(entity_id).cloned().unwrap_or_default()
    }

    /// Get all inbound relationship IDs for an entity
    pub fn get_inbound(&self, entity_id: &str) -> Vec<String> {
        self.inbound.get(entity_id).cloned().unwrap_or_default()
    }

    /// Get all relationship IDs for an entity (both directions)
    pub fn get_all_relationships(&self, entity_id: &str) -> Vec<String> {
        let mut all = self.get_outbound(entity_id);
        all.extend(self.get_inbound(entity_id));
        all.sort();
        all.dedup();
        all
    }

    /// Get relationships by type
    pub fn get_by_type(&self, rel_type: &EntityRelationType) -> Vec<String> {
        self.by_type.get(rel_type).cloned().unwrap_or_default()
    }

    /// Check if relationship is bidirectional
    pub fn is_bidirectional(&self, rel_id: &str) -> bool {
        self.bidirectional.contains(rel_id)
    }

    /// Clear all relationships from the index
    pub fn clear(&mut self) {
        self.outbound.clear();
        self.inbound.clear();
        self.by_type.clear();
        self.bidirectional.clear();
    }
}

/// Extended storage trait for relationship operations
pub trait RelationshipStorage: Storage {
    /// Store a relationship with index updates
    fn store_relationship(&mut self, relationship: &EntityRelationship) -> Result<(), EngramError>;

    /// Get a relationship by ID
    fn get_relationship(&self, id: &str) -> Result<Option<EntityRelationship>, EngramError>;

    /// Query relationships with filtering
    fn query_relationships(
        &self,
        filter: &RelationshipFilter,
    ) -> Result<Vec<EntityRelationship>, EngramError>;

    /// Get all relationships involving an entity (either direction)
    fn get_entity_relationships(
        &self,
        entity_id: &str,
    ) -> Result<Vec<EntityRelationship>, EngramError>;

    /// Get outbound relationships from an entity
    fn get_outbound_relationships(
        &self,
        entity_id: &str,
    ) -> Result<Vec<EntityRelationship>, EngramError>;

    /// Get inbound relationships to an entity
    fn get_inbound_relationships(
        &self,
        entity_id: &str,
    ) -> Result<Vec<EntityRelationship>, EngramError>;

    /// Find paths between entities
    fn find_paths(
        &self,
        source_id: &str,
        target_id: &str,
        algorithm: TraversalAlgorithm,
        max_depth: Option<usize>,
    ) -> Result<Vec<EntityPath>, EngramError>;

    /// Find shortest path between entities
    fn find_shortest_path(
        &self,
        source_id: &str,
        target_id: &str,
    ) -> Result<Option<EntityPath>, EngramError> {
        let paths = self.find_paths(source_id, target_id, TraversalAlgorithm::Dijkstra, None)?;
        Ok(paths
            .into_iter()
            .find(|p| p.path_type == PathType::Shortest))
    }

    /// Get all connected entities (graph traversal)
    fn get_connected_entities(
        &self,
        entity_id: &str,
        algorithm: TraversalAlgorithm,
        max_depth: Option<usize>,
    ) -> Result<Vec<String>, EngramError>;

    fn delete_relationship(&mut self, id: &str) -> Result<(), EngramError>;

    /// Get relationship index for direct access
    fn get_relationship_index(&self) -> Result<&RelationshipIndex, EngramError>;

    /// Rebuild relationship index from stored relationships
    fn rebuild_relationship_index(&mut self) -> Result<(), EngramError>;

    /// Validate relationship constraints before storing
    fn validate_relationship_constraints(
        &self,
        relationship: &EntityRelationship,
    ) -> Result<(), EngramError> {
        relationship
            .validate_entity()
            .map_err(|e| EngramError::Validation(e))?;

        let outbound_count = self
            .get_outbound_relationships(&relationship.source_id)?
            .len();
        let inbound_count = self
            .get_inbound_relationships(&relationship.target_id)?
            .len();

        if let Some(max_outbound) = relationship.constraints.max_outbound {
            if outbound_count >= max_outbound {
                return Err(EngramError::Validation(format!(
                    "Maximum outbound relationships ({}) exceeded for entity {}",
                    max_outbound, relationship.source_id
                )));
            }
        }

        if let Some(max_inbound) = relationship.constraints.max_inbound {
            if inbound_count >= max_inbound {
                return Err(EngramError::Validation(format!(
                    "Maximum inbound relationships ({}) exceeded for entity {}",
                    max_inbound, relationship.target_id
                )));
            }
        }

        if !relationship.constraints.allow_cycles {
            if self.would_create_cycle(relationship)? {
                return Err(EngramError::Validation(
                    "Relationship would create a cycle, which is not allowed".to_string(),
                ));
            }
        }

        Ok(())
    }

    /// Check if adding a relationship would create a cycle
    fn would_create_cycle(&self, relationship: &EntityRelationship) -> Result<bool, EngramError> {
        let paths = self.find_paths(
            &relationship.target_id,
            &relationship.source_id,
            TraversalAlgorithm::BreadthFirst,
            None,
        )?;
        Ok(!paths.is_empty())
    }

    /// Get relationship statistics
    fn get_relationship_stats(&self) -> Result<RelationshipStats, EngramError>;
}

/// Relationship storage statistics
#[derive(Debug, Clone)]
pub struct RelationshipStats {
    pub total_relationships: usize,
    pub relationships_by_type: HashMap<EntityRelationType, usize>,
    pub bidirectional_count: usize,
    pub average_connections_per_entity: f64,
    pub most_connected_entity: Option<(String, usize)>,
    pub relationship_density: f64,
}

/// State for Dijkstra's algorithm priority queue
#[derive(Debug, Clone)]
struct State {
    cost: i64, // Use i64 for ordering (multiply by 1000 to get precision)
    entity: String,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Eq for State {}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost)
    }
}

/// Graph analysis operations
pub struct GraphAnalyzer;

impl GraphAnalyzer {
    /// Perform breadth-first search
    pub fn bfs<S: RelationshipStorage>(
        storage: &S,
        start_entity: &str,
        target_entity: Option<&str>,
        max_depth: Option<usize>,
    ) -> Result<Vec<String>, EngramError> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut result = Vec::new();

        queue.push_back((start_entity.to_string(), 0));
        visited.insert(start_entity.to_string());

        while let Some((entity_id, depth)) = queue.pop_front() {
            result.push(entity_id.clone());

            if let Some(target) = target_entity {
                if entity_id == target {
                    break;
                }
            }

            if let Some(max_d) = max_depth {
                if depth >= max_d {
                    continue;
                }
            }

            let relationships = storage.get_outbound_relationships(&entity_id)?;
            for rel in relationships {
                if rel.active {
                    if !visited.contains(&rel.target_id) {
                        visited.insert(rel.target_id.clone());
                        queue.push_back((rel.target_id, depth + 1));
                    }
                }
            }

            let inbound = storage.get_inbound_relationships(&entity_id)?;
            for rel in inbound {
                if rel.active && rel.direction == RelationshipDirection::Bidirectional {
                    if !visited.contains(&rel.source_id) {
                        visited.insert(rel.source_id.clone());
                        queue.push_back((rel.source_id, depth + 1));
                    }
                }
            }
        }

        Ok(result)
    }

    /// Perform depth-first search
    pub fn dfs<S: RelationshipStorage>(
        storage: &S,
        start_entity: &str,
        target_entity: Option<&str>,
        max_depth: Option<usize>,
    ) -> Result<Vec<String>, EngramError> {
        let mut visited = HashSet::new();
        let mut result = Vec::new();

        Self::dfs_recursive(
            storage,
            start_entity,
            target_entity,
            max_depth,
            0,
            &mut visited,
            &mut result,
        )?;

        Ok(result)
    }

    fn dfs_recursive<S: RelationshipStorage>(
        storage: &S,
        entity_id: &str,
        target_entity: Option<&str>,
        max_depth: Option<usize>,
        current_depth: usize,
        visited: &mut HashSet<String>,
        result: &mut Vec<String>,
    ) -> Result<bool, EngramError> {
        if visited.contains(entity_id) {
            return Ok(false);
        }

        visited.insert(entity_id.to_string());
        result.push(entity_id.to_string());

        if let Some(target) = target_entity {
            if entity_id == target {
                return Ok(true);
            }
        }

        if let Some(max_d) = max_depth {
            if current_depth >= max_d {
                return Ok(false);
            }
        }

        let relationships = storage.get_outbound_relationships(entity_id)?;
        for rel in relationships {
            if rel.active {
                if Self::dfs_recursive(
                    storage,
                    &rel.target_id,
                    target_entity,
                    max_depth,
                    current_depth + 1,
                    visited,
                    result,
                )? {
                    return Ok(true);
                }
            }
        }

        let inbound = storage.get_inbound_relationships(entity_id)?;
        for rel in inbound {
            if rel.active && rel.direction == RelationshipDirection::Bidirectional {
                if Self::dfs_recursive(
                    storage,
                    &rel.source_id,
                    target_entity,
                    max_depth,
                    current_depth + 1,
                    visited,
                    result,
                )? {
                    return Ok(true);
                }
            }
        }

        Ok(false)
    }

    /// Find shortest path using Dijkstra's algorithm
    pub fn dijkstra<S: RelationshipStorage>(
        storage: &S,
        start_entity: &str,
        target_entity: &str,
    ) -> Result<Option<EntityPath>, EngramError> {
        use std::collections::BinaryHeap;

        let mut distances: HashMap<String, f64> = HashMap::new();
        let mut previous: HashMap<String, Option<(String, String)>> = HashMap::new();
        let mut visited = HashSet::new();
        let mut heap = BinaryHeap::new();

        distances.insert(start_entity.to_string(), 0.0);
        heap.push(Reverse(State {
            cost: 0,
            entity: start_entity.to_string(),
        }));

        while let Some(Reverse(State {
            cost: _,
            entity: current_entity,
        })) = heap.pop()
        {
            if visited.contains(&current_entity) {
                continue;
            }

            visited.insert(current_entity.clone());

            if current_entity == target_entity {
                break;
            }

            let current_dist = distances
                .get(&current_entity)
                .copied()
                .unwrap_or(f64::INFINITY);

            let relationships = storage.get_outbound_relationships(&current_entity)?;
            for rel in relationships {
                if !rel.active {
                    continue;
                }

                let next_entity = &rel.target_id;
                let weight = 1.0 - rel.strength.weight();
                let new_dist = current_dist + weight;

                let current_best = distances.get(next_entity).copied().unwrap_or(f64::INFINITY);
                if new_dist < current_best {
                    distances.insert(next_entity.clone(), new_dist);
                    previous.insert(
                        next_entity.clone(),
                        Some((current_entity.clone(), rel.id.clone())),
                    );
                    heap.push(Reverse(State {
                        cost: (new_dist * 1000.0) as i64,
                        entity: next_entity.clone(),
                    }));
                }
            }

            let inbound = storage.get_inbound_relationships(&current_entity)?;
            for rel in inbound {
                if !rel.active || rel.direction != RelationshipDirection::Bidirectional {
                    continue;
                }

                let next_entity = &rel.source_id;
                let weight = 1.0 - rel.strength.weight();
                let new_dist = current_dist + weight;

                let current_best = distances.get(next_entity).copied().unwrap_or(f64::INFINITY);
                if new_dist < current_best {
                    distances.insert(next_entity.clone(), new_dist);
                    previous.insert(
                        next_entity.clone(),
                        Some((current_entity.clone(), rel.id.clone())),
                    );
                    heap.push(Reverse(State {
                        cost: (new_dist * 1000.0) as i64,
                        entity: next_entity.clone(),
                    }));
                }
            }
        }

        if !distances.contains_key(target_entity) {
            return Ok(None);
        }

        let mut path_entities = Vec::new();
        let mut path_relationships = Vec::new();
        let mut current = target_entity;

        while let Some(Some((prev_entity, rel_id))) = previous.get(current) {
            path_entities.push(current.to_string());
            path_relationships.push(rel_id.clone());
            current = prev_entity;
        }
        path_entities.push(start_entity.to_string());

        path_entities.reverse();
        path_relationships.reverse();

        let total_weight = distances.get(target_entity).copied().unwrap_or(0.0);

        Ok(Some(EntityPath {
            entities: path_entities,
            relationships: path_relationships,
            total_weight,
            path_type: PathType::Shortest,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relationship_index() {
        let mut index = RelationshipIndex::new();

        let rel = EntityRelationship::new(
            "rel-001".to_string(),
            "agent".to_string(),
            "entity-1".to_string(),
            "task".to_string(),
            "entity-2".to_string(),
            "task".to_string(),
            EntityRelationType::DependsOn,
        );

        index.add_relationship(&rel);

        assert_eq!(index.get_outbound("entity-1"), vec!["rel-001"]);
        assert_eq!(index.get_inbound("entity-2"), vec!["rel-001"]);
        assert_eq!(
            index.get_by_type(&EntityRelationType::DependsOn),
            vec!["rel-001"]
        );
    }

    #[test]
    fn test_relationship_path() {
        let path = EntityPath {
            entities: vec![
                "entity-1".to_string(),
                "entity-2".to_string(),
                "entity-3".to_string(),
            ],
            relationships: vec!["rel-1".to_string(), "rel-2".to_string()],
            total_weight: 1.5,
            path_type: PathType::Shortest,
        };

        assert_eq!(path.entities.len(), 3);
        assert_eq!(path.relationships.len(), 2);
    }
}

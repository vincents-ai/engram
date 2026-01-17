use engram::{
    entities::{
        Entity, EntityRelationType, EntityRelationship, RelationshipDirection, RelationshipStrength,
    },
    storage::{GitStorage, RelationshipStorage, TraversalAlgorithm},
};
use std::fs;
use tempfile::TempDir;
use uuid::Uuid;

/// Integration tests for relationship management functionality
#[cfg(test)]
mod relationship_integration_tests {
    use super::*;

    fn setup_test_storage() -> (TempDir, GitStorage) {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let storage = GitStorage::new(temp_dir.path().to_str().unwrap(), "test-agent")
            .expect("Failed to create GitStorage");
        (temp_dir, storage)
    }

    #[test]
    fn test_create_and_retrieve_relationship() {
        let (_temp_dir, mut storage) = setup_test_storage();

        // Create a relationship
        let relationship = EntityRelationship::new(
            "task1".to_string(),
            "task".to_string(),
            "task2".to_string(),
            "task".to_string(),
            EntityRelationType::DependsOn,
            RelationshipDirection::Unidirectional,
            RelationshipStrength::Medium,
            "test-agent".to_string(),
        );

        // Store the relationship
        storage
            .store(&relationship)
            .expect("Failed to store relationship");

        // Retrieve and verify
        let retrieved = storage
            .get(&relationship.id, "relationship")
            .expect("Failed to retrieve relationship")
            .expect("Relationship not found");

        assert_eq!(retrieved.id(), &relationship.id);
    }

    #[test]
    fn test_list_relationships_with_filtering() {
        let (_temp_dir, mut storage) = setup_test_storage();

        // Create multiple relationships
        let rel1 = EntityRelationship::new(
            "project1".to_string(),
            "project".to_string(),
            "task1".to_string(),
            "task".to_string(),
            EntityRelationType::Contains,
            RelationshipDirection::Unidirectional,
            RelationshipStrength::Strong,
            "test-agent".to_string(),
        );

        let rel2 = EntityRelationship::new(
            "task1".to_string(),
            "task".to_string(),
            "task2".to_string(),
            "task".to_string(),
            EntityRelationType::DependsOn,
            RelationshipDirection::Unidirectional,
            RelationshipStrength::Medium,
            "test-agent".to_string(),
        );

        storage
            .store(&rel1)
            .expect("Failed to store relationship 1");
        storage
            .store(&rel2)
            .expect("Failed to store relationship 2");

        // Test listing all relationships
        let all_relationships = storage
            .list_all_relationships()
            .expect("Failed to list relationships");
        assert_eq!(all_relationships.len(), 2);

        // Test filtering by source entity
        let task1_relationships = storage
            .get_relationships_for_entity("task1")
            .expect("Failed to get relationships for task1");
        assert_eq!(task1_relationships.len(), 1);
        assert_eq!(
            task1_relationships[0].relationship_type,
            EntityRelationType::DependsOn
        );
    }

    #[test]
    fn test_bidirectional_relationships() {
        let (_temp_dir, mut storage) = setup_test_storage();

        let relationship = EntityRelationship::new(
            "component1".to_string(),
            "component".to_string(),
            "component2".to_string(),
            "component".to_string(),
            EntityRelationType::AssociatedWith,
            RelationshipDirection::Bidirectional,
            RelationshipStrength::Medium,
            "test-agent".to_string(),
        );

        storage
            .store(&relationship)
            .expect("Failed to store bidirectional relationship");

        // Verify both directions are accessible
        let comp1_relationships = storage
            .get_relationships_for_entity("component1")
            .expect("Failed to get relationships for component1");
        let comp2_relationships = storage
            .get_relationships_for_entity("component2")
            .expect("Failed to get relationships for component2");

        assert_eq!(comp1_relationships.len(), 1);
        assert_eq!(comp2_relationships.len(), 1);
    }

    #[test]
    fn test_path_finding_between_entities() {
        let (_temp_dir, mut storage) = setup_test_storage();

        // Create a chain: start -> middle -> end
        let rel1 = EntityRelationship::new(
            "start".to_string(),
            "task".to_string(),
            "middle".to_string(),
            "task".to_string(),
            EntityRelationType::DependsOn,
            RelationshipDirection::Unidirectional,
            RelationshipStrength::Medium,
            "test-agent".to_string(),
        );

        let rel2 = EntityRelationship::new(
            "middle".to_string(),
            "task".to_string(),
            "end".to_string(),
            "task".to_string(),
            EntityRelationType::DependsOn,
            RelationshipDirection::Unidirectional,
            RelationshipStrength::Medium,
            "test-agent".to_string(),
        );

        storage
            .store(&rel1)
            .expect("Failed to store relationship 1");
        storage
            .store(&rel2)
            .expect("Failed to store relationship 2");

        // Find path from start to end
        let paths = storage
            .find_paths("start", "end", TraversalAlgorithm::BreadthFirst, None)
            .expect("Failed to find paths");

        assert!(!paths.is_empty(), "Should find at least one path");
        assert_eq!(paths[0].entities, vec!["start", "middle", "end"]);
    }

    #[test]
    fn test_path_finding_no_connection() {
        let (_temp_dir, mut storage) = setup_test_storage();

        // Create isolated entities with no connection
        let rel1 = EntityRelationship::new(
            "isolated1".to_string(),
            "task".to_string(),
            "connected1".to_string(),
            "task".to_string(),
            EntityRelationType::DependsOn,
            RelationshipDirection::Unidirectional,
            RelationshipStrength::Medium,
            "test-agent".to_string(),
        );

        storage.store(&rel1).expect("Failed to store relationship");

        // Try to find path between isolated entities
        let paths = storage
            .find_paths(
                "isolated1",
                "isolated2",
                TraversalAlgorithm::BreadthFirst,
                None,
            )
            .expect("Failed to find paths");

        assert!(
            paths.is_empty(),
            "Should find no path between isolated entities"
        );
    }

    #[test]
    fn test_connected_entities_discovery() {
        let (_temp_dir, mut storage) = setup_test_storage();

        // Create hub with multiple connections
        let rel1 = EntityRelationship::new(
            "hub".to_string(),
            "project".to_string(),
            "task1".to_string(),
            "task".to_string(),
            EntityRelationType::Contains,
            RelationshipDirection::Unidirectional,
            RelationshipStrength::Strong,
            "test-agent".to_string(),
        );

        let rel2 = EntityRelationship::new(
            "hub".to_string(),
            "project".to_string(),
            "task2".to_string(),
            "task".to_string(),
            EntityRelationType::Contains,
            RelationshipDirection::Unidirectional,
            RelationshipStrength::Strong,
            "test-agent".to_string(),
        );

        let rel3 = EntityRelationship::new(
            "hub".to_string(),
            "project".to_string(),
            "doc1".to_string(),
            "document".to_string(),
            EntityRelationType::References,
            RelationshipDirection::Unidirectional,
            RelationshipStrength::Weak,
            "test-agent".to_string(),
        );

        storage
            .store(&rel1)
            .expect("Failed to store relationship 1");
        storage
            .store(&rel2)
            .expect("Failed to store relationship 2");
        storage
            .store(&rel3)
            .expect("Failed to store relationship 3");

        // Get connected entities
        let connected = storage
            .get_connected_entities("hub", TraversalAlgorithm::BreadthFirst, Some(1))
            .expect("Failed to get connected entities");

        assert_eq!(connected.len(), 3);
        assert!(connected.contains(&"task1".to_string()));
        assert!(connected.contains(&"task2".to_string()));
        assert!(connected.contains(&"doc1".to_string()));
    }

    #[test]
    fn test_relationship_statistics() {
        let (_temp_dir, mut storage) = setup_test_storage();

        // Create various types of relationships
        let rel1 = EntityRelationship::new(
            "project1".to_string(),
            "project".to_string(),
            "task1".to_string(),
            "task".to_string(),
            EntityRelationType::Contains,
            RelationshipDirection::Unidirectional,
            RelationshipStrength::Strong,
            "test-agent".to_string(),
        );

        let rel2 = EntityRelationship::new(
            "task1".to_string(),
            "task".to_string(),
            "task2".to_string(),
            "task".to_string(),
            EntityRelationType::DependsOn,
            RelationshipDirection::Unidirectional,
            RelationshipStrength::Medium,
            "test-agent".to_string(),
        );

        let rel3 = EntityRelationship::new(
            "comp1".to_string(),
            "component".to_string(),
            "comp2".to_string(),
            "component".to_string(),
            EntityRelationType::AssociatedWith,
            RelationshipDirection::Bidirectional,
            RelationshipStrength::Medium,
            "test-agent".to_string(),
        );

        storage
            .store(&rel1)
            .expect("Failed to store relationship 1");
        storage
            .store(&rel2)
            .expect("Failed to store relationship 2");
        storage
            .store(&rel3)
            .expect("Failed to store relationship 3");

        // Get statistics
        let stats = storage
            .get_relationship_stats()
            .expect("Failed to get statistics");

        assert_eq!(stats.total_relationships, 3);
        assert_eq!(stats.bidirectional_count, 1);
        assert!(stats
            .relationships_by_type
            .contains_key(&EntityRelationType::Contains));
        assert!(stats
            .relationships_by_type
            .contains_key(&EntityRelationType::DependsOn));
        assert!(stats
            .relationships_by_type
            .contains_key(&EntityRelationType::AssociatedWith));
    }

    #[test]
    fn test_relationship_deletion() {
        let (_temp_dir, mut storage) = setup_test_storage();

        let relationship = EntityRelationship::new(
            "source".to_string(),
            "task".to_string(),
            "target".to_string(),
            "task".to_string(),
            EntityRelationType::DependsOn,
            RelationshipDirection::Unidirectional,
            RelationshipStrength::Medium,
            "test-agent".to_string(),
        );

        let relationship_id = relationship.id.clone();
        storage
            .store(&relationship)
            .expect("Failed to store relationship");

        // Verify it exists
        assert!(storage
            .get(&relationship_id, "relationship")
            .unwrap()
            .is_some());

        // Delete it
        storage
            .delete(&relationship_id, "relationship")
            .expect("Failed to delete relationship");

        // Verify it's gone
        assert!(storage
            .get(&relationship_id, "relationship")
            .unwrap()
            .is_none());
    }

    #[test]
    fn test_different_traversal_algorithms() {
        let (_temp_dir, mut storage) = setup_test_storage();

        // Create a small graph: A -> B -> C, A -> C
        let rel1 = EntityRelationship::new(
            "A".to_string(),
            "task".to_string(),
            "B".to_string(),
            "task".to_string(),
            EntityRelationType::DependsOn,
            RelationshipDirection::Unidirectional,
            RelationshipStrength::Medium,
            "test-agent".to_string(),
        );

        let rel2 = EntityRelationship::new(
            "B".to_string(),
            "task".to_string(),
            "C".to_string(),
            "task".to_string(),
            EntityRelationType::DependsOn,
            RelationshipDirection::Unidirectional,
            RelationshipStrength::Medium,
            "test-agent".to_string(),
        );

        let rel3 = EntityRelationship::new(
            "A".to_string(),
            "task".to_string(),
            "C".to_string(),
            "task".to_string(),
            EntityRelationType::DependsOn,
            RelationshipDirection::Unidirectional,
            RelationshipStrength::Strong,
            "test-agent".to_string(),
        );

        storage
            .store(&rel1)
            .expect("Failed to store relationship 1");
        storage
            .store(&rel2)
            .expect("Failed to store relationship 2");
        storage
            .store(&rel3)
            .expect("Failed to store relationship 3");

        // Test different algorithms
        let bfs_paths = storage
            .find_paths("A", "C", TraversalAlgorithm::BreadthFirst, None)
            .expect("BFS failed");
        let dfs_paths = storage
            .find_paths("A", "C", TraversalAlgorithm::DepthFirst, None)
            .expect("DFS failed");
        let dijkstra_paths = storage
            .find_paths("A", "C", TraversalAlgorithm::Dijkstra, None)
            .expect("Dijkstra failed");

        assert!(!bfs_paths.is_empty());
        assert!(!dfs_paths.is_empty());
        assert!(!dijkstra_paths.is_empty());

        // All should find at least the direct path A -> C
        assert!(bfs_paths.iter().any(|p| p.entities == vec!["A", "C"]));
        assert!(dfs_paths.iter().any(|p| p.entities == vec!["A", "C"]));
        assert!(dijkstra_paths.iter().any(|p| p.entities == vec!["A", "C"]));
    }
}

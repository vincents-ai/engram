use engram::{
    entities::{EntityRelationType, EntityRelationship, RelationshipDirection},
    storage::{GitRefsStorage, RelationshipStorage, TraversalAlgorithm},
};
use tempfile::TempDir;

#[cfg(test)]
mod relationship_integration_tests {
    use super::*;

    fn setup_test_storage() -> (TempDir, GitRefsStorage) {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let storage = GitRefsStorage::new(temp_dir.path().to_str().unwrap(), "test-agent")
            .expect("Failed to create GitRefsStorage");
        (temp_dir, storage)
    }

    fn make_rel(
        source_id: &str,
        source_type: &str,
        target_id: &str,
        target_type: &str,
        rel_type: EntityRelationType,
    ) -> EntityRelationship {
        let mut rel = EntityRelationship::new(
            format!("{}-{}", source_id, target_id),
            "test-agent".to_string(),
            source_id.to_string(),
            source_type.to_string(),
            target_id.to_string(),
            target_type.to_string(),
            rel_type,
        );
        rel.direction = RelationshipDirection::Unidirectional;
        rel
    }

    #[test]
    fn test_create_and_retrieve_relationship() {
        let (_temp_dir, mut storage) = setup_test_storage();

        let rel = make_rel(
            "task1",
            "task",
            "task2",
            "task",
            EntityRelationType::DependsOn,
        );

        storage
            .store_relationship(&rel)
            .expect("Failed to store relationship");

        let retrieved = storage
            .get_relationship(&rel.id)
            .expect("Failed to retrieve relationship")
            .expect("Relationship not found");

        assert_eq!(retrieved.id, rel.id);
        assert_eq!(retrieved.source_id, "task1");
        assert_eq!(retrieved.target_id, "task2");
        assert_eq!(retrieved.relationship_type, EntityRelationType::DependsOn);
    }

    #[test]
    fn test_store_multiple_relationships() {
        let (_temp_dir, mut storage) = setup_test_storage();

        let rel1 = make_rel(
            "project1",
            "project",
            "task1",
            "task",
            EntityRelationType::Contains,
        );
        let rel2 = make_rel(
            "task1",
            "task",
            "task2",
            "task",
            EntityRelationType::DependsOn,
        );

        storage
            .store_relationship(&rel1)
            .expect("Failed to store rel1");
        storage
            .store_relationship(&rel2)
            .expect("Failed to store rel2");

        let r1 = storage
            .get_relationship(&rel1.id)
            .expect("Failed to get rel1")
            .expect("rel1 not found");
        let r2 = storage
            .get_relationship(&rel2.id)
            .expect("Failed to get rel2")
            .expect("rel2 not found");

        assert_eq!(r1.relationship_type, EntityRelationType::Contains);
        assert_eq!(r2.relationship_type, EntityRelationType::DependsOn);
    }

    #[test]
    fn test_bidirectional_relationship_persists() {
        let (_temp_dir, mut storage) = setup_test_storage();

        let mut rel = make_rel(
            "component1",
            "component",
            "component2",
            "component",
            EntityRelationType::AssociatedWith,
        );
        rel.direction = RelationshipDirection::Bidirectional;

        storage
            .store_relationship(&rel)
            .expect("Failed to store bidirectional relationship");

        let retrieved = storage
            .get_relationship(&rel.id)
            .expect("Failed to retrieve")
            .expect("Not found");

        assert_eq!(retrieved.direction, RelationshipDirection::Bidirectional);
    }

    #[test]
    fn test_entity_relationships_after_rebuild() {
        let (_temp_dir, mut storage) = setup_test_storage();

        let rel1 = make_rel(
            "project1",
            "project",
            "task1",
            "task",
            EntityRelationType::Contains,
        );
        let rel2 = make_rel(
            "task1",
            "task",
            "task2",
            "task",
            EntityRelationType::DependsOn,
        );

        storage
            .store_relationship(&rel1)
            .expect("Failed to store rel1");
        storage
            .store_relationship(&rel2)
            .expect("Failed to store rel2");

        storage
            .rebuild_relationship_index()
            .expect("Failed to rebuild index");

        let task1_rels = storage
            .get_entity_relationships("task1")
            .expect("Failed to get entity relationships");

        assert_eq!(task1_rels.len(), 2);
    }

    #[test]
    fn test_find_paths_is_stub() {
        let (_temp_dir, mut storage) = setup_test_storage();

        let rel1 = make_rel(
            "start",
            "task",
            "middle",
            "task",
            EntityRelationType::DependsOn,
        );
        let rel2 = make_rel(
            "middle",
            "task",
            "end",
            "task",
            EntityRelationType::DependsOn,
        );

        storage
            .store_relationship(&rel1)
            .expect("Failed to store rel1");
        storage
            .store_relationship(&rel2)
            .expect("Failed to store rel2");

        storage
            .rebuild_relationship_index()
            .expect("Failed to rebuild index");

        let paths = storage
            .find_paths("start", "end", TraversalAlgorithm::BreadthFirst, None)
            .expect("Failed to find paths");

        assert!(
            paths.is_empty(),
            "find_paths is currently a stub implementation"
        );
    }

    #[test]
    fn test_path_finding_no_connection() {
        let (_temp_dir, mut storage) = setup_test_storage();

        let rel1 = make_rel(
            "isolated1",
            "task",
            "connected1",
            "task",
            EntityRelationType::DependsOn,
        );
        storage
            .store_relationship(&rel1)
            .expect("Failed to store rel");

        storage
            .rebuild_relationship_index()
            .expect("Failed to rebuild index");

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

        let rel1 = make_rel(
            "hub",
            "project",
            "task1",
            "task",
            EntityRelationType::Contains,
        );
        let rel2 = make_rel(
            "hub",
            "project",
            "task2",
            "task",
            EntityRelationType::Contains,
        );
        let rel3 = make_rel(
            "hub",
            "project",
            "doc1",
            "document",
            EntityRelationType::References,
        );

        storage
            .store_relationship(&rel1)
            .expect("Failed to store rel1");
        storage
            .store_relationship(&rel2)
            .expect("Failed to store rel2");
        storage
            .store_relationship(&rel3)
            .expect("Failed to store rel3");

        storage
            .rebuild_relationship_index()
            .expect("Failed to rebuild index");

        let connected = storage
            .get_connected_entities("hub", TraversalAlgorithm::BreadthFirst, Some(1))
            .expect("Failed to get connected entities");

        assert_eq!(connected.len(), 4, "hub + 3 connected entities");
        assert!(connected.contains(&"hub".to_string()));
        assert!(connected.contains(&"task1".to_string()));
        assert!(connected.contains(&"task2".to_string()));
        assert!(connected.contains(&"doc1".to_string()));
    }

    #[test]
    fn test_relationship_deletion() {
        let (_temp_dir, mut storage) = setup_test_storage();

        let rel = make_rel(
            "source",
            "task",
            "target",
            "task",
            EntityRelationType::DependsOn,
        );
        let rel_id = rel.id.clone();

        storage
            .store_relationship(&rel)
            .expect("Failed to store relationship");

        assert!(storage.get_relationship(&rel_id).unwrap().is_some());

        storage
            .delete_relationship(&rel_id)
            .expect("Failed to delete relationship");

        assert!(storage.get_relationship(&rel_id).unwrap().is_none());
    }

    #[test]
    fn test_connected_entities_with_dfs() {
        let (_temp_dir, mut storage) = setup_test_storage();

        let rel1 = make_rel("A", "task", "B", "task", EntityRelationType::DependsOn);
        let rel2 = make_rel("B", "task", "C", "task", EntityRelationType::DependsOn);
        let rel3 = make_rel("A", "task", "C", "task", EntityRelationType::DependsOn);

        storage
            .store_relationship(&rel1)
            .expect("Failed to store rel1");
        storage
            .store_relationship(&rel2)
            .expect("Failed to store rel2");
        storage
            .store_relationship(&rel3)
            .expect("Failed to store rel3");

        storage
            .rebuild_relationship_index()
            .expect("Failed to rebuild index");

        let bfs = storage
            .get_connected_entities("A", TraversalAlgorithm::BreadthFirst, None)
            .expect("BFS failed");
        let dfs = storage
            .get_connected_entities("A", TraversalAlgorithm::DepthFirst, None)
            .expect("DFS failed");

        assert_eq!(bfs.len(), 3);
        assert_eq!(dfs.len(), 3);
        assert!(bfs.contains(&"B".to_string()));
        assert!(bfs.contains(&"C".to_string()));
        assert!(dfs.contains(&"B".to_string()));
        assert!(dfs.contains(&"C".to_string()));
    }

    #[test]
    fn test_outbound_inbound_relationships() {
        let (_temp_dir, mut storage) = setup_test_storage();

        let rel1 = make_rel(
            "parent",
            "task",
            "child1",
            "task",
            EntityRelationType::Contains,
        );
        let rel2 = make_rel(
            "parent",
            "task",
            "child2",
            "task",
            EntityRelationType::Contains,
        );

        storage
            .store_relationship(&rel1)
            .expect("Failed to store rel1");
        storage
            .store_relationship(&rel2)
            .expect("Failed to store rel2");

        storage
            .rebuild_relationship_index()
            .expect("Failed to rebuild index");

        let outbound = storage
            .get_outbound_relationships("parent")
            .expect("Failed to get outbound");
        let inbound = storage
            .get_inbound_relationships("child1")
            .expect("Failed to get inbound");

        assert_eq!(outbound.len(), 2);
        assert_eq!(inbound.len(), 1);
    }
}

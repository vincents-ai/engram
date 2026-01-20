//! Tests for Git refs storage implementation

use chrono::Utc;
use engram::entities::{EntityRelationType as RelationshipType, EntityRelationship, GenericEntity};
use engram::storage::{GitRefsStorage, QueryFilter, RelationshipStorage, Storage};
use serde_json::json;
use tempfile::TempDir;

fn create_test_storage() -> (TempDir, GitRefsStorage) {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let storage = GitRefsStorage::new(temp_dir.path().to_str().unwrap(), "test-agent")
        .expect("Failed to create GitRefsStorage");
    (temp_dir, storage)
}

fn create_test_task(id: &str, title: &str, status: &str) -> GenericEntity {
    GenericEntity {
        id: id.to_string(),
        entity_type: "task".to_string(),
        agent: "test-agent".to_string(),
        timestamp: Utc::now(),
        data: json!({
            "title": title,
            "status": status,
            "priority": "medium"
        }),
    }
}

fn create_test_context(id: &str, title: &str) -> GenericEntity {
    GenericEntity {
        id: id.to_string(),
        entity_type: "context".to_string(),
        agent: "test-agent".to_string(),
        timestamp: Utc::now(),
        data: json!({
            "title": title,
            "content": "Test context content"
        }),
    }
}

#[test]
fn test_git_refs_store_and_get() {
    let (_temp_dir, mut storage) = create_test_storage();

    let task = create_test_task("task-001", "Test Task", "todo");

    storage.store(&task).expect("Failed to store task");

    let retrieved = storage.get("task-001", "task").expect("Failed to get task");
    assert!(retrieved.is_some());

    let retrieved_task = retrieved.unwrap();
    assert_eq!(retrieved_task.id, "task-001");
    assert_eq!(retrieved_task.entity_type, "task");
    assert_eq!(retrieved_task.agent, "test-agent");
    assert_eq!(retrieved_task.data["title"], "Test Task");
}

#[test]
fn test_git_refs_delete() {
    let (_temp_dir, mut storage) = create_test_storage();

    let task = create_test_task("task-002", "Delete Test", "todo");

    storage.store(&task).expect("Failed to store task");

    let exists = storage
        .get("task-002", "task")
        .expect("Failed to check task")
        .is_some();
    assert!(exists);

    storage
        .delete("task-002", "task")
        .expect("Failed to delete task");

    let not_exists = storage
        .get("task-002", "task")
        .expect("Failed to check task")
        .is_none();
    assert!(not_exists);

    // Deleting non-existent entity should probably be Ok
    storage
        .delete("task-002", "task")
        .expect("Failed to delete non-existent task");
}

#[test]
fn test_git_refs_query_by_type() {
    let (_temp_dir, mut storage) = create_test_storage();

    let task1 = create_test_task("task-003", "Query Test 1", "todo");
    let task2 = create_test_task("task-004", "Query Test 2", "done");
    let context1 = create_test_context("context-001", "Test Context");

    storage.store(&task1).expect("Failed to store task1");
    storage.store(&task2).expect("Failed to store task2");
    storage.store(&context1).expect("Failed to store context1");

    let mut filter = QueryFilter::default();
    filter.entity_type = Some("task".to_string());

    let result = storage.query(&filter).expect("Failed to query tasks");
    assert_eq!(result.entities.len(), 2);
    assert_eq!(result.total_count, 2);

    for entity in &result.entities {
        assert_eq!(entity.entity_type, "task");
    }
}

#[test]
fn test_git_refs_query_by_agent() {
    let (_temp_dir, mut storage) = create_test_storage();

    let mut task1 = create_test_task("task-005", "Agent Test 1", "todo");
    let mut task2 = create_test_task("task-006", "Agent Test 2", "todo");
    task2.agent = "other-agent".to_string();

    storage.store(&task1).expect("Failed to store task1");
    storage.store(&task2).expect("Failed to store task2");

    let mut filter = QueryFilter::default();
    filter.agent = Some("test-agent".to_string());

    let result = storage.query(&filter).expect("Failed to query by agent");
    assert_eq!(result.entities.len(), 1);
    assert_eq!(result.entities[0].agent, "test-agent");
}

#[test]
fn test_git_refs_query_by_field() {
    let (_temp_dir, mut storage) = create_test_storage();

    let task1 = create_test_task("task-007", "Field Test 1", "todo");
    let task2 = create_test_task("task-008", "Field Test 2", "done");

    storage.store(&task1).expect("Failed to store task1");
    storage.store(&task2).expect("Failed to store task2");

    let mut filter = QueryFilter::default();
    filter
        .field_filters
        .insert("status".to_string(), json!("done"));

    let result = storage.query(&filter).expect("Failed to query by field");
    assert_eq!(result.entities.len(), 1);
    assert_eq!(result.entities[0].data["status"], "done");
}

#[test]
fn test_git_refs_get_all() {
    let (_temp_dir, mut storage) = create_test_storage();

    let task1 = create_test_task("task-009", "GetAll Test 1", "todo");
    let task2 = create_test_task("task-010", "GetAll Test 2", "done");
    let context1 = create_test_context("context-002", "GetAll Context");

    storage.store(&task1).expect("Failed to store task1");
    storage.store(&task2).expect("Failed to store task2");
    storage.store(&context1).expect("Failed to store context1");

    let all_tasks = storage.get_all("task").expect("Failed to get all tasks");
    assert_eq!(all_tasks.len(), 2);

    let all_contexts = storage
        .get_all("context")
        .expect("Failed to get all contexts");
    assert_eq!(all_contexts.len(), 1);
}

#[test]
fn test_git_refs_update() {
    let (_temp_dir, mut storage) = create_test_storage();

    let mut task = create_test_task("task-011", "Update Test", "todo");

    storage.store(&task).expect("Failed to store task");

    task.data["status"] = json!("in_progress");
    task.data["title"] = json!("Updated Task");

    storage.store(&task).expect("Failed to update task");

    let updated = storage
        .get("task-011", "task")
        .expect("Failed to get updated task")
        .unwrap();
    assert_eq!(updated.data["status"], "in_progress");
    assert_eq!(updated.data["title"], "Updated Task");
}

#[test]
fn test_git_refs_stats() {
    let (_temp_dir, mut storage) = create_test_storage();

    let task = create_test_task("task-012", "Stats Test", "todo");
    let context = create_test_context("context-003", "Stats Context");

    storage.store(&task).expect("Failed to store task");
    storage.store(&context).expect("Failed to store context");

    let stats = storage.get_stats().expect("Failed to get stats");
    assert_eq!(*stats.entities_by_type.get("task").unwrap_or(&0), 1);
    assert_eq!(*stats.entities_by_type.get("context").unwrap_or(&0), 1);
    assert_eq!(*stats.entities_by_type.get("reasoning").unwrap_or(&0), 0);
}

#[test]
fn test_git_refs_relationships() {
    let (_temp_dir, mut storage) = create_test_storage();

    let relationship = EntityRelationship::new(
        "rel-001".to_string(),
        "test-agent".to_string(),
        "task-013".to_string(),
        "task".to_string(),
        "context-004".to_string(),
        "context".to_string(),
        RelationshipType::References,
    );

    storage
        .store_relationship(&relationship)
        .expect("Failed to store relationship");

    let retrieved = storage
        .get_relationship("rel-001")
        .expect("Failed to get relationship");
    assert!(retrieved.is_some());

    let retrieved_rel = retrieved.unwrap();
    assert_eq!(retrieved_rel.id, "rel-001");
    assert_eq!(retrieved_rel.source_id, "task-013");
    assert_eq!(retrieved_rel.target_id, "context-004");
}

#[test]
fn test_git_refs_nonexistent_entity() {
    let (_temp_dir, storage) = create_test_storage();

    let result = storage
        .get("nonexistent-id", "task")
        .expect("Failed to query nonexistent entity");
    assert!(result.is_none());
}

#[test]
fn test_git_refs_pagination() {
    let (_temp_dir, mut storage) = create_test_storage();

    for i in 1..=10 {
        let task = create_test_task(
            &format!("task-{:03}", i),
            &format!("Pagination Test {}", i),
            "todo",
        );
        storage.store(&task).expect("Failed to store task");
    }

    let mut filter = QueryFilter::default();
    filter.entity_type = Some("task".to_string());
    filter.limit = Some(5);
    filter.offset = Some(0);

    let result = storage
        .query(&filter)
        .expect("Failed to query with pagination");
    assert_eq!(result.entities.len(), 5);
    assert_eq!(result.total_count, 10);

    filter.offset = Some(5);
    let result2 = storage.query(&filter).expect("Failed to query second page");
    assert_eq!(result2.entities.len(), 5);
    assert_eq!(result2.total_count, 10);
}

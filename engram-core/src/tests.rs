//! Tests for engram-core traits and types.

use crate::*;
use crate::entity_types::EntityRegistry;
use crate::storage_types;
use serde_json::json;

#[test]
fn generic_entity_from_value_complete() {
    let value = json!({
        "id": "test-123",
        "type": "task",
        "agent": "test-agent",
        "timestamp": "2025-01-15T10:30:00Z",
        "title": "Test task"
    });

    let entity = GenericEntity::from_value(value).unwrap();
    assert_eq!(entity.id, "test-123");
    assert_eq!(entity.entity_type, "task");
    assert_eq!(entity.agent, "test-agent");
}

#[test]
fn generic_entity_from_value_entity_type_alias() {
    // Should accept both "type" and "entity_type"
    let value = json!({
        "id": "test-456",
        "entity_type": "context",
        "agent": "bot"
    });

    let entity = GenericEntity::from_value(value).unwrap();
    assert_eq!(entity.entity_type, "context");
}

#[test]
fn generic_entity_from_value_missing_id() {
    let value = json!({
        "type": "task",
        "agent": "test"
    });

    let result = GenericEntity::from_value(value);
    assert!(result.is_err());
    match result.unwrap_err() {
        EngramError::Deserialization(msg) => assert!(msg.contains("id")),
        other => panic!("Expected Deserialization error, got: {:?}", other),
    }
}

#[test]
fn generic_entity_from_value_missing_type() {
    let value = json!({
        "id": "test-789",
        "agent": "test"
    });

    let result = GenericEntity::from_value(value);
    assert!(result.is_err());
    match result.unwrap_err() {
        EngramError::Deserialization(msg) => assert!(msg.contains("type")),
        other => panic!("Expected Deserialization error, got: {:?}", other),
    }
}

#[test]
fn generic_entity_from_value_defaults_agent() {
    let value = json!({
        "id": "test-default",
        "type": "knowledge"
    });

    let entity = GenericEntity::from_value(value).unwrap();
    assert_eq!(entity.agent, "unknown");
}

#[test]
fn generic_entity_from_value_defaults_timestamp() {
    let value = json!({
        "id": "test-ts",
        "type": "context"
    });

    let entity = GenericEntity::from_value(value).unwrap();
    // Timestamp should be roughly now (within last 10 seconds)
    let now = chrono::Utc::now();
    let diff = (now.timestamp() - entity.timestamp.timestamp()).abs();
    assert!(diff < 10, "Timestamp should be near now, but diff was {}s", diff);
}

#[test]
fn generic_entity_roundtrip_json() {
    let value = json!({
        "id": "roundtrip-1",
        "type": "reasoning",
        "agent": "test-agent",
        "timestamp": "2025-06-01T12:00:00Z",
        "conclusion": "It works"
    });

    let entity = GenericEntity::from_value(value.clone()).unwrap();

    // Serialize back
    let serialized = serde_json::to_string(&entity).unwrap();
    let deserialized: GenericEntity = serde_json::from_str(&serialized).unwrap();

    assert_eq!(deserialized.id, entity.id);
    assert_eq!(deserialized.entity_type, entity.entity_type);
    assert_eq!(deserialized.agent, entity.agent);
}

#[test]
fn entity_registry_register_and_check() {
    let mut registry = EntityRegistry::new();
    assert!(!registry.has_type("task"));

    // We can't register a concrete Entity easily in this test since
    // there are no concrete implementations in engram-core, but we
    // can verify the registry mechanics with a manual entry.
    registry.types.insert(
        "task".to_string(),
        Ok,
    );
    assert!(registry.has_type("task"));

    let names = registry.type_names();
    assert!(names.contains(&"task".to_string()));
}

#[test]
fn query_filter_default() {
    let filter = storage_types::QueryFilter::default();
    assert!(filter.entity_type.is_none());
    assert!(filter.agent.is_none());
    assert!(filter.tags.is_empty());
    assert!(filter.time_range.is_none());
    assert!(filter.limit.is_none());
    assert!(filter.offset.is_none());
}

#[test]
fn error_display_messages() {
    let err = EngramError::NotFound("task/123".into());
    assert!(err.to_string().contains("task/123"));

    let err = EngramError::Validation("bad data".into());
    assert!(err.to_string().contains("bad data"));

    let storage_err = StorageError::EntityNotFound("task".into(), "123".into());
    assert!(storage_err.to_string().contains("task"));
    assert!(storage_err.to_string().contains("123"));
}

#[test]
fn config_error_variants() {
    let err = ConfigError::Missing("api_key".into());
    assert!(err.to_string().contains("api_key"));

    let err = ConfigError::Invalid("bad format".into());
    assert!(err.to_string().contains("bad format"));
}

#[test]
fn storage_stats_serialization() {
    let stats = storage_types::StorageStats {
        total_entities: 42,
        entities_by_type: {
            let mut m = std::collections::HashMap::new();
            m.insert("task".to_string(), 20);
            m.insert("context".to_string(), 22);
            m
        },
        repo_size_bytes: 1024,
        last_commit: Some("abc123".to_string()),
    };

    let json = serde_json::to_string(&stats).unwrap();
    let back: storage_types::StorageStats = serde_json::from_str(&json).unwrap();
    assert_eq!(back.total_entities, 42);
    assert_eq!(back.repo_size_bytes, 1024);
}

#[test]
fn git_commit_serialization() {
    let commit = storage_types::GitCommit {
        id: "deadbeef".to_string(),
        author: "test".to_string(),
        message: "initial".to_string(),
        timestamp: chrono::Utc::now(),
        parents: vec!["parent1".to_string()],
    };

    let json = serde_json::to_string(&commit).unwrap();
    let back: storage_types::GitCommit = serde_json::from_str(&json).unwrap();
    assert_eq!(back.id, "deadbeef");
    assert_eq!(back.parents.len(), 1);
}

#[test]
fn time_range_and_sort_order() {
    let range = storage_types::TimeRange {
        start: chrono::Utc::now(),
        end: chrono::Utc::now(),
    };

    let json = serde_json::to_string(&range).unwrap();
    assert!(serde_json::from_str::<storage_types::TimeRange>(&json).is_ok());

    let sort = storage_types::SortOrder::NewestFirst;
    let json = serde_json::to_string(&sort).unwrap();
    assert!(json.contains("NewestFirst"));
}

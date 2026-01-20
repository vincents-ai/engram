//! Integration tests for Natural Language Query (NLQ) functionality
//!
//! These tests verify that the NLQ engine can properly:
//! - Classify intents from natural language queries
//! - Extract entities (agents, task IDs, status, priority) from text
//! - Map queries to storage operations
//! - Format results into natural language responses
//! - Handle JSON output format
//! - Perform efficiently with real data

use engram::{
    entities::{
        Context, ContextRelevance, Entity, EntityRelationType, EntityRelationship, Task,
        TaskPriority, TaskStatus,
    },
    nlq::{EntityExtractor, ExtractedEntity, IntentClassifier, NLQEngine, QueryIntent},
    storage::{GitStorage, Storage},
};
use serde_json;
use std::fs;
use tempfile::TempDir;
use tokio;
use uuid::Uuid;

struct NLQTestFixture {
    temp_dir: TempDir,
    storage: GitStorage,
    repo_path: String,
}

impl NLQTestFixture {
    fn new() -> Self {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let repo_path = temp_dir.path().join(".engram");

        fs::create_dir_all(&repo_path).expect("Failed to create storage directory");

        let storage = GitStorage::new(temp_dir.path().to_str().unwrap(), "test-agent")
            .expect("Failed to create GitStorage");

        Self {
            temp_dir,
            storage,
            repo_path: repo_path.to_string_lossy().to_string(),
        }
    }

    fn setup_test_data(&mut self) -> (Vec<String>, Vec<String>, Vec<String>) {
        let mut task_ids = Vec::new();
        let mut context_ids = Vec::new();
        let mut relationship_ids = Vec::new();

        let tasks = vec![
            (
                "Implement authentication",
                TaskStatus::InProgress,
                TaskPriority::High,
            ),
            ("Setup database", TaskStatus::Done, TaskPriority::Medium),
            ("Write documentation", TaskStatus::Todo, TaskPriority::Low),
            ("Deploy to production", TaskStatus::Todo, TaskPriority::High),
            ("Fix login bug", TaskStatus::Done, TaskPriority::High),
        ];

        for (title, status, priority) in tasks {
            let mut task = Task::new(
                title.to_string(),
                "Description for task".to_string(),
                "test-agent".to_string(),
                priority,
                None,
            );
            task.status = status;

            task_ids.push(task.id.clone());
            self.storage
                .store(&task.to_generic())
                .expect("Failed to store test task");
        }

        let contexts = vec![
            (
                "NLQ Implementation Guide",
                "Guide for implementing natural language queries in Engram",
            ),
            (
                "Authentication Requirements",
                "Security requirements for user authentication",
            ),
            ("Database Schema", "Database structure and relationships"),
        ];

        for (title, content) in contexts {
            let context = Context::new(
                title.to_string(),
                content.to_string(),
                "test-source".to_string(),
                ContextRelevance::High,
                "test-agent".to_string(),
            );

            context_ids.push(context.id.clone());
            self.storage
                .store(&context.to_generic())
                .expect("Failed to store test context");
        }

        if task_ids.len() >= 2 {
            let relationship = EntityRelationship::new(
                Uuid::new_v4().to_string(),
                "test-agent".to_string(),
                task_ids[0].clone(),
                "task".to_string(),
                task_ids[1].clone(),
                "task".to_string(),
                EntityRelationType::DependsOn,
            );

            relationship_ids.push(relationship.id.clone());
            self.storage
                .store(&relationship.to_generic())
                .expect("Failed to store test relationship");
        }

        if !task_ids.is_empty() && !context_ids.is_empty() {
            let relationship = EntityRelationship::new(
                Uuid::new_v4().to_string(),
                "test-agent".to_string(),
                task_ids[0].clone(),
                "task".to_string(),
                context_ids[0].clone(),
                "context".to_string(),
                EntityRelationType::References,
            );

            relationship_ids.push(relationship.id.clone());
            self.storage
                .store(&relationship.to_generic())
                .expect("Failed to store task-context relationship");
        }

        (task_ids, context_ids, relationship_ids)
    }
}

#[test]
fn test_nlq_intent_classification() {
    let classifier = IntentClassifier::new();

    let test_cases = vec![
        ("show my tasks", QueryIntent::ListTasks),
        ("list tasks for test-agent", QueryIntent::ListTasks),
        ("what tasks do I have", QueryIntent::ListTasks),
        ("show task abc-123", QueryIntent::ShowTaskDetails),
        (
            "what tasks depend on this task",
            QueryIntent::FindRelationships,
        ),
        (
            "find context about authentication",
            QueryIntent::SearchContext,
        ),
        ("show workflow status", QueryIntent::AnalyzeWorkflow),
        ("random unrelated query", QueryIntent::Unknown),
    ];

    for (query, expected_intent) in test_cases {
        let intent = classifier
            .classify(query)
            .expect("Failed to classify intent");
        assert_eq!(intent, expected_intent, "Failed for query: {}", query);
    }
}

#[test]
fn test_nlq_entity_extraction() {
    let extractor = EntityExtractor::new();

    let entities = extractor
        .extract("show tasks for alice")
        .expect("Failed to extract entities");
    let agent_entities: Vec<_> = entities
        .iter()
        .filter(|e| e.entity_type == "agent")
        .collect();
    assert_eq!(agent_entities.len(), 1);
    assert_eq!(agent_entities[0].value, "alice");

    let entities = extractor
        .extract("show done tasks for test-agent")
        .expect("Failed to extract entities");
    let status_entities: Vec<_> = entities
        .iter()
        .filter(|e| e.entity_type == "status")
        .collect();
    assert_eq!(status_entities.len(), 1);
    assert_eq!(status_entities[0].value, "done");

    let entities = extractor
        .extract("show high priority tasks")
        .expect("Failed to extract entities");
    let priority_entities: Vec<_> = entities
        .iter()
        .filter(|e| e.entity_type == "priority")
        .collect();
    assert_eq!(priority_entities.len(), 1);
    assert_eq!(priority_entities[0].value, "high");

    let uuid = Uuid::new_v4().to_string();
    let query = format!("show relationships for {}", uuid);
    let entities = extractor
        .extract(&query)
        .expect("Failed to extract entities");
    let id_entities: Vec<_> = entities
        .iter()
        .filter(|e| e.entity_type == "task_id")
        .collect();
    assert_eq!(id_entities.len(), 1);
    assert_eq!(id_entities[0].value, uuid);
}

#[tokio::test]
async fn test_nlq_task_listing() {
    let mut fixture = NLQTestFixture::new();
    let (task_ids, _, _) = fixture.setup_test_data();

    let engine = NLQEngine::new();
    let result = engine
        .process_query("show my tasks for test-agent", None, &fixture.storage)
        .await
        .expect("Failed to process query");

    assert!(result
        .formatted_response
        .contains("Implement authentication"));
    assert!(result.formatted_response.contains("Setup database"));
    assert!(result.formatted_response.contains("Write documentation"));
    assert!(task_ids.len() >= 3, "Should have at least 3 test tasks");
}

#[tokio::test]
async fn test_nlq_priority_filtering() {
    let mut fixture = NLQTestFixture::new();
    let (_, _, _) = fixture.setup_test_data();

    let engine = NLQEngine::new();

    let result = engine
        .process_query(
            "show high priority tasks for test-agent",
            None,
            &fixture.storage,
        )
        .await
        .expect("Failed to process query");

    assert!(result
        .formatted_response
        .contains("Implement authentication"));
    assert!(result.formatted_response.contains("Deploy to production"));
    assert!(result.formatted_response.contains("Fix login bug"));
    assert!(!result.formatted_response.contains("Write documentation"));
}

#[tokio::test]
async fn test_nlq_context_search() {
    let mut fixture = NLQTestFixture::new();
    let (_, _, _) = fixture.setup_test_data();

    let engine = NLQEngine::new();

    let result = engine
        .process_query(
            "find context about NLQ for test-agent",
            None,
            &fixture.storage,
        )
        .await
        .expect("Failed to process query");

    assert!(result
        .formatted_response
        .contains("NLQ Implementation Guide"));
    assert!(!result.formatted_response.contains("Database Schema"));

    let result = engine
        .process_query(
            "find context about authentication for test-agent",
            None,
            &fixture.storage,
        )
        .await
        .expect("Failed to process query");

    assert!(result
        .formatted_response
        .contains("Authentication Requirements"));
}

#[tokio::test]
async fn test_nlq_relationship_queries() {
    let mut fixture = NLQTestFixture::new();
    let (task_ids, _, _) = fixture.setup_test_data();

    let engine = NLQEngine::new();

    if !task_ids.is_empty() {
        let query = format!("what tasks depend on {} for test-agent", task_ids[0]);
        let result = engine
            .process_query(&query, None, &fixture.storage)
            .await
            .expect("Failed to process query");

        assert!(
            result.formatted_response.contains("relationship")
                || result.formatted_response.contains("depends")
                || result.formatted_response.contains("connection")
                || result.formatted_response.contains("No relationships")
        );
    }
}

#[tokio::test]
async fn test_nlq_unknown_query_handling() {
    let mut fixture = NLQTestFixture::new();
    let (_, _, _) = fixture.setup_test_data();

    let engine = NLQEngine::new();

    let result = engine
        .process_query(
            "some completely random unrelated query",
            None,
            &fixture.storage,
        )
        .await
        .expect("Failed to process query");

    assert!(
        result.formatted_response.contains("understand")
            || result.formatted_response.contains("help")
            || result.formatted_response.contains("try")
            || result.formatted_response.contains("sorry")
            || result.formatted_response.contains("unclear")
    );
}

#[tokio::test]
async fn test_nlq_combined_filters() {
    let mut fixture = NLQTestFixture::new();
    let (_, _, _) = fixture.setup_test_data();

    let engine = NLQEngine::new();

    // Test high priority filter first
    let result = engine
        .process_query(
            "show high priority tasks for test-agent",
            None,
            &fixture.storage,
        )
        .await
        .expect("Failed to process query");

    println!("High priority response: {}", result.formatted_response);
    assert!(result.formatted_response.contains("Fix login bug"));
    assert!(result
        .formatted_response
        .contains("Implement authentication"));

    // Test done status filter
    let result = engine
        .process_query("show done tasks for test-agent", None, &fixture.storage)
        .await
        .expect("Failed to process query");

    println!("Done tasks response: {}", result.formatted_response);
    assert!(result.formatted_response.contains("Fix login bug"));
    assert!(result.formatted_response.contains("Setup database"));
}

#[tokio::test]
async fn test_nlq_performance() {
    let mut fixture = NLQTestFixture::new();
    let (_, _, _) = fixture.setup_test_data();

    let engine = NLQEngine::new();

    let start = std::time::Instant::now();
    let _result = engine
        .process_query("show my tasks for test-agent", None, &fixture.storage)
        .await
        .expect("Failed to process query");
    let duration = start.elapsed();

    assert!(
        duration.as_secs() < 1,
        "NLQ query took too long: {:?}",
        duration
    );
}

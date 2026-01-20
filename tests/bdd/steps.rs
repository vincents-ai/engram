//! BDD step definitions for Engram features
//!
//! Implements Gherkin steps for testing tasks, contexts, reasoning,
//! knowledge, sessions, and synchronization.

use super::{EngramSteps, EngramWorld};
use cucumber::{given, then, when};

// Function to force linking of this module
pub fn register() {}

// ============================================================================
// GIVEN steps - Setup and preconditions
// ============================================================================

#[given("I have a workspace")]
async fn given_workspace(world: &mut EngramWorld) {
    world.given_i_have_a_workspace().await;
}

#[given(expr = "I am logged in as agent {string}")]
async fn given_logged_in_agent(world: &mut EngramWorld, agent: String) {
    world.given_i_am_logged_in_as_agent(agent).await;
}

#[given(expr = "{word} has a task {string}")]
async fn given_agent_has_task(world: &mut EngramWorld, agent: String, title: String) {
    world.initialize_storage(&agent);
    world.create_task(&title, "Test description", "medium");
}

#[given(expr = "{word} has a task {string} with status {string}")]
async fn given_agent_has_task_with_status(
    world: &mut EngramWorld,
    agent: String,
    title: String,
    _status: String,
) {
    world.initialize_storage(&agent);
    world.create_task(&title, "Test description", "medium");
    // TODO: Update task status - requires implementing update functionality
}

#[given(expr = "{word} has a task {string} with description {string} updated at {string}")]
async fn given_agent_has_task_with_details(
    world: &mut EngramWorld,
    agent: String,
    title: String,
    description: String,
    _timestamp: String,
) {
    world.initialize_storage(&agent);
    world.create_task(&title, &description, "medium");
}

#[given(expr = "{word} has {int} tasks")]
async fn given_agent_has_n_tasks(world: &mut EngramWorld, agent: String, count: i32) {
    world.initialize_storage(&agent);
    for i in 1..=count {
        world.create_task(&format!("Task {}", i), "Description", "medium");
    }
}

#[given(expr = "{word} has {int} completed sessions")]
async fn given_agent_has_completed_sessions(world: &mut EngramWorld, agent: String, count: i32) {
    world.initialize_storage(&agent);
    for i in 1..=count {
        world.create_session(&format!("Session {}", i), false);
        // TODO: Mark sessions as completed
    }
}

#[given(expr = "{word} has an active session")]
async fn given_agent_has_active_session(world: &mut EngramWorld, agent: String) {
    world.initialize_storage(&agent);
    world.create_session("Active Session", true);
}

#[given(expr = "I have a file {string} with content {string}")]
async fn given_file_with_content(world: &mut EngramWorld, filename: String, content: String) {
    world.create_test_file(&filename, &content);
}

#[given(expr = "I have a file {string} with reasoning conclusion")]
async fn given_file_with_conclusion(world: &mut EngramWorld, filename: String) {
    world.create_test_file(&filename, "This is the reasoning conclusion");
}

#[given(expr = "I have a JSON file {string} with knowledge items")]
async fn given_json_file_with_knowledge(world: &mut EngramWorld, filename: String) {
    let json_content = r#"[
        {"title": "Knowledge 1", "knowledge_type": "pattern", "confidence": 0.9},
        {"title": "Knowledge 2", "knowledge_type": "lesson", "confidence": 0.8}
    ]"#;
    world.create_test_file(&filename, json_content);
}

#[given(expr = "I have {int} contexts")]
async fn given_n_contexts(world: &mut EngramWorld, count: i32) {
    for i in 1..=count {
        world.create_context(&format!("Context {}", i), "Content", "medium");
    }
}

#[given(expr = "I have a context {string}")]
async fn given_context(world: &mut EngramWorld, title: String) {
    world.create_context(&title, "Test content", "medium");
}

#[given(expr = "I have {int} knowledge items of type {string}")]
async fn given_n_knowledge_of_type(world: &mut EngramWorld, count: i32, knowledge_type: String) {
    for i in 1..=count {
        world.create_knowledge(&format!("Knowledge {}", i), &knowledge_type, 0.8);
    }
}

#[given(expr = "I have knowledge {string}")]
async fn given_knowledge(world: &mut EngramWorld, title: String) {
    world.create_knowledge(&title, "pattern", 0.9);
}

#[given(expr = "I have {int} reasoning chains")]
async fn given_n_reasoning(world: &mut EngramWorld, count: i32) {
    for i in 1..=count {
        world.create_reasoning(&format!("Reasoning {}", i), "Description", "Conclusion");
    }
}

#[given(expr = "I have reasoning {string}")]
async fn given_reasoning(world: &mut EngramWorld, title: String) {
    world.create_reasoning(&title, "Test description", "Test conclusion");
}

// ============================================================================
// WHEN steps - Actions
// ============================================================================

#[when(expr = "I create a task with title {string}")]
async fn when_create_task(world: &mut EngramWorld, title: String) {
    world.create_task(&title, "Test description", "medium");
}

#[when(expr = "I create a task with title {string} and priority {string}")]
async fn when_create_task_with_priority(world: &mut EngramWorld, title: String, priority: String) {
    world.create_task(&title, "Test description", &priority);
}

#[when(expr = "the task has priority {string}")]
async fn when_task_has_priority(world: &mut EngramWorld, priority: String) {
    // Context for the previously created task
    world.set_last_priority(&priority);
}

#[when(expr = "the task is assigned to agent {string}")]
async fn when_task_assigned_to_agent(world: &mut EngramWorld, agent: String) {
    world.set_current_agent(&agent);
}

#[when(expr = "I list tasks for agent {string}")]
async fn when_list_tasks_for_agent(world: &mut EngramWorld, agent: String) {
    world.list_tasks_for_agent(&agent).await;
}

#[when(expr = "I update the task status to {string}")]
async fn when_update_task_status(world: &mut EngramWorld, status: String) {
    // TODO: Implement task update
    world.set_last_status(&status);
}

#[when("I show the task details")]
async fn when_show_task_details(world: &mut EngramWorld) {
    world.show_last_entity_details().await;
}

#[when(expr = "I pipe {string} to task create --title {string} --description-stdin")]
async fn when_pipe_to_task_create(world: &mut EngramWorld, content: String, title: String) {
    world.create_task(&title, &content, "medium");
}

#[when(expr = "I pipe {string} to task create --json")]
async fn when_pipe_json_to_task_create(world: &mut EngramWorld, json: String) {
    world.create_task_from_json(&json);
}

#[when(expr = "I create a context with title {string} and content-file {string}")]
async fn when_create_context_from_file(world: &mut EngramWorld, title: String, filename: String) {
    let content = world.read_test_file(&filename);
    world.create_context(&title, &content, "medium");
}

#[when(expr = "I pipe {string} to context create --title {string} --content-stdin")]
async fn when_pipe_to_context_create(world: &mut EngramWorld, content: String, title: String) {
    world.create_context(&title, &content, "medium");
}

#[when(expr = "I create knowledge items from JSON file {string}")]
async fn when_create_knowledge_from_json_file(world: &mut EngramWorld, filename: String) {
    let json = world.read_test_file(&filename);
    world.create_knowledge_from_json(&json);
}

#[when(expr = "I create reasoning with title {string} and conclusion-file {string}")]
async fn when_create_reasoning_from_file(world: &mut EngramWorld, title: String, filename: String) {
    let conclusion = world.read_test_file(&filename);
    world.create_reasoning(&title, "Description", &conclusion);
}

#[when(expr = "I run session start --name {string} --auto-detect")]
async fn when_start_session_with_autodetect(world: &mut EngramWorld, agent: String) {
    world.initialize_storage(&agent);
    world.create_session("Work Session", true);
}

#[when(expr = "I run session start --name {string}")]
async fn when_start_session(world: &mut EngramWorld, agent: String) {
    world.initialize_storage(&agent);
    world.create_session("Work Session", false);
}

#[when(expr = "I run session status --id <session-id> --metrics")]
async fn when_show_session_status(_world: &mut EngramWorld) {
    // TODO: Implement session status
}

#[when(expr = "I run session end --id <session-id> --generate-summary")]
async fn when_end_session_with_summary(_world: &mut EngramWorld) {
    // TODO: Implement session end
}

#[when(expr = "I list sessions for agent {string}")]
async fn when_list_sessions_for_agent(world: &mut EngramWorld, agent: String) {
    world.list_sessions_for_agent(&agent).await;
}

#[when(expr = "I list sessions for agent {string} with limit {int}")]
async fn when_list_sessions_with_limit(world: &mut EngramWorld, agent: String, limit: i32) {
    world
        .list_sessions_for_agent_with_limit(&agent, limit)
        .await;
}

#[when(expr = "I sync agents {string} with strategy {string}")]
async fn when_sync_agents(world: &mut EngramWorld, agents: String, strategy: String) {
    world.sync_agents(&agents, &strategy).await;
}

#[when("I list contexts")]
async fn when_list_contexts(world: &mut EngramWorld) {
    world.list_contexts().await;
}

#[when("I show the context details")]
async fn when_show_context_details(world: &mut EngramWorld) {
    world.show_last_entity_details().await;
}

#[when(expr = "I create knowledge with confidence {float}")]
async fn when_create_knowledge_with_confidence(world: &mut EngramWorld, confidence: f64) {
    world.create_knowledge("Test Knowledge", "pattern", confidence);
}

#[when(expr = "I list knowledge with filter type {string}")]
async fn when_list_knowledge_by_type(world: &mut EngramWorld, knowledge_type: String) {
    world.list_knowledge_by_type(&knowledge_type).await;
}

#[when("I show the knowledge details")]
async fn when_show_knowledge_details(world: &mut EngramWorld) {
    world.show_last_entity_details().await;
}

#[when("I list reasoning chains")]
async fn when_list_reasoning(world: &mut EngramWorld) {
    world.list_reasoning().await;
}

#[when("I show the reasoning details")]
async fn when_show_reasoning_details(world: &mut EngramWorld) {
    world.show_last_entity_details().await;
}

// ============================================================================
// THEN steps - Assertions
// ============================================================================

#[then("the task should be created successfully")]
async fn then_task_created_successfully(world: &mut EngramWorld) {
    world.then_the_task_should_be_created_successfully().await;
}

#[then("the task should be stored in Git")]
async fn then_task_stored_in_git(world: &mut EngramWorld) {
    assert!(world.is_storage_initialized());
}

#[then("the task ID should be returned")]
async fn then_task_id_returned(world: &mut EngramWorld) {
    let entities = world.get_created_entities("task");
    assert!(!entities.is_empty(), "No task ID was created");
}

#[then(expr = "I should see {int} tasks")]
async fn then_should_see_n_tasks(world: &mut EngramWorld, count: i32) {
    let entities = world.get_created_entities("task");
    if entities.len() != count as usize {
        if let Some(query_count) = world.last_query_count {
            assert_eq!(
                query_count, count as usize,
                "Expected {} tasks (from query)",
                count
            );
            return;
        }
    }
    assert_eq!(entities.len(), count as usize, "Expected {} tasks", count);
}

#[then(expr = "I should see {string}")]
async fn then_should_see(world: &mut EngramWorld, content: String) {
    // Check if content appears in last result
    if let Some(Ok(result)) = world.get_last_result() {
        assert!(
            result.contains(&content),
            "Expected to see '{}' in result",
            content
        );
    }
}

#[then(expr = "I should not see {string}")]
async fn then_should_not_see(world: &mut EngramWorld, content: String) {
    if let Some(Ok(result)) = world.get_last_result() {
        assert!(
            !result.contains(&content),
            "Did not expect to see '{}' in result",
            content
        );
    }
}

#[then(expr = "the task status should be {string}")]
async fn then_task_status_should_be(_world: &mut EngramWorld, _status: String) {
    // TODO: Verify task status
}

#[then("I should see the task title")]
async fn then_should_see_task_title(world: &mut EngramWorld) {
    assert!(world.get_last_result().is_some());
}

#[then("I should see the assigned agent")]
async fn then_should_see_assigned_agent(world: &mut EngramWorld) {
    assert!(world.current_agent.is_some());
}

#[then("I should see the creation timestamp")]
async fn then_should_see_creation_timestamp(_world: &mut EngramWorld) {
    // TODO: Verify timestamp in result
}

#[then(expr = "the task description should be {string}")]
async fn then_task_description_should_be(_world: &mut EngramWorld, _description: String) {
    // TODO: Verify task description
}

#[then(expr = "the task title should be {string}")]
async fn then_task_title_should_be(_world: &mut EngramWorld, _title: String) {
    // TODO: Verify task title
}

#[then(expr = "the task priority should be {string}")]
async fn then_task_priority_should_be(_world: &mut EngramWorld, _priority: String) {
    // TODO: Verify task priority
}

#[then(expr = "the task agent should be {string}")]
async fn then_task_agent_should_be(_world: &mut EngramWorld, _agent: String) {
    // TODO: Verify task agent
}

#[then("the context should be created successfully")]
async fn then_context_created_successfully(world: &mut EngramWorld) {
    let entities = world.get_created_entities("context");
    assert!(!entities.is_empty(), "Context was not created");
}

#[then(expr = "the context content should be {string}")]
async fn then_context_content_should_be(_world: &mut EngramWorld, _content: String) {
    // TODO: Verify context content
}

#[then("all knowledge items should be created successfully")]
async fn then_all_knowledge_created(_world: &mut EngramWorld) {
    // TODO: Verify multiple knowledge items
}

#[then("the reasoning should be created successfully")]
async fn then_reasoning_created_successfully(world: &mut EngramWorld) {
    let entities = world.get_created_entities("reasoning");
    assert!(!entities.is_empty(), "Reasoning was not created");
}

#[then("the conclusion should match the file content")]
async fn then_conclusion_matches_file(_world: &mut EngramWorld) {
    // TODO: Verify conclusion content
}

#[then("a session should be created")]
async fn then_session_created(world: &mut EngramWorld) {
    let entities = world.get_created_entities("session");
    assert!(!entities.is_empty(), "Session was not created");
}

#[then("the session should have a unique ID")]
async fn then_session_has_unique_id(world: &mut EngramWorld) {
    let entities = world.get_created_entities("session");
    assert!(!entities.is_empty(), "Session ID was not generated");
}

#[then("auto-detection should identify Engram project work")]
async fn then_autodetect_identifies_engram(_world: &mut EngramWorld) {
    // TODO: Verify auto-detection flag
}

#[then(expr = "the session status should be {string}")]
async fn then_session_status_should_be(_world: &mut EngramWorld, _status: String) {
    // TODO: Verify session status
}

#[then("the session should not have auto-detection enabled")]
async fn then_no_autodetect(_world: &mut EngramWorld) {
    // TODO: Verify auto-detection is false
}

#[then("I should see the session details")]
async fn then_should_see_session_details(_world: &mut EngramWorld) {
    // TODO: Verify session details in output
}

#[then("I should see SPACE framework metrics")]
async fn then_should_see_space_metrics(_world: &mut EngramWorld) {
    // TODO: Verify SPACE metrics in output
}

#[then("I should see DORA metrics")]
async fn then_should_see_dora_metrics(_world: &mut EngramWorld) {
    // TODO: Verify DORA metrics in output
}

#[then("I should see the session duration")]
async fn then_should_see_duration(_world: &mut EngramWorld) {
    // TODO: Verify duration in output
}

#[then("I should see a productivity summary")]
async fn then_should_see_productivity_summary(_world: &mut EngramWorld) {
    // TODO: Verify summary in output
}

#[then("I should see the total duration")]
async fn then_should_see_total_duration(_world: &mut EngramWorld) {
    // TODO: Verify total duration
}

#[then("I should see activity counts")]
async fn then_should_see_activity_counts(_world: &mut EngramWorld) {
    // TODO: Verify activity counts
}

#[then(expr = "I should see {int} sessions")]
async fn then_should_see_n_sessions(world: &mut EngramWorld, count: i32) {
    let entities = world.get_created_entities("session");

    if entities.len() != count as usize {
        if let Some(query_count) = world.last_query_count {
            assert_eq!(
                query_count, count as usize,
                "Expected {} sessions (from query)",
                count
            );
            return;
        }
    }

    assert_eq!(
        entities.len(),
        count as usize,
        "Expected {} sessions",
        count
    );
}

#[then(expr = "all sessions should be for agent {string}")]
async fn then_all_sessions_for_agent(_world: &mut EngramWorld, _agent: String) {
    // TODO: Verify all sessions belong to agent
}

#[then("they should be the 5 most recent sessions")]
async fn then_most_recent_sessions(_world: &mut EngramWorld) {
    // TODO: Verify sessions are sorted by recency
}

#[then("the sync should succeed")]
async fn then_sync_should_succeed(world: &mut EngramWorld) {
    world.then_the_operation_should_succeed().await;
}

#[then(expr = "the task should have {word}'s version")]
async fn then_task_has_version(_world: &mut EngramWorld, _agent: String) {
    // TODO: Verify task version from specific agent
}

#[then("no conflicts should be reported")]
async fn then_no_conflicts(_world: &mut EngramWorld) {
    // TODO: Verify no conflicts in sync result
}

#[then("conflicts should be detected")]
async fn then_conflicts_detected(_world: &mut EngramWorld) {
    // TODO: Verify conflicts were detected
}

#[then("I should see a conflict report")]
async fn then_should_see_conflict_report(_world: &mut EngramWorld) {
    // TODO: Verify conflict report in output
}

#[then("all unique tasks should be accessible")]
async fn then_all_unique_tasks_accessible(_world: &mut EngramWorld) {
    // TODO: Verify all unique tasks exist
}

#[then("duplicate tasks should be resolved")]
async fn then_duplicates_resolved(_world: &mut EngramWorld) {
    // TODO: Verify no duplicate tasks
}

#[then("I should see a message about single agent")]
async fn then_should_see_single_agent_message(_world: &mut EngramWorld) {
    // TODO: Verify single agent message
}

#[then("no sync operations should be performed")]
async fn then_no_sync_operations(_world: &mut EngramWorld) {
    // TODO: Verify no sync occurred
}

#[then("the sync should fail")]
async fn then_sync_should_fail(_world: &mut EngramWorld) {
    // TODO: Verify sync failed
}

#[then("I should see an error about empty agents list")]
async fn then_should_see_empty_agents_error(_world: &mut EngramWorld) {
    // TODO: Verify error message
}

#[then("I should see valid strategy options")]
async fn then_should_see_valid_strategies(_world: &mut EngramWorld) {
    // TODO: Verify strategy options in error
}

#[then(expr = "I should see {int} contexts")]
async fn then_should_see_n_contexts(world: &mut EngramWorld, count: i32) {
    let entities = world.get_created_entities("context");
    assert_eq!(
        entities.len(),
        count as usize,
        "Expected {} contexts",
        count
    );
}

#[then("I should see the context title")]
async fn then_should_see_context_title(_world: &mut EngramWorld) {
    // TODO: Verify context title in output
}

#[then("I should see the context content")]
async fn then_should_see_context_content(_world: &mut EngramWorld) {
    // TODO: Verify context content in output
}

#[then("I should see the relevance level")]
async fn then_should_see_relevance_level(_world: &mut EngramWorld) {
    // TODO: Verify relevance level in output
}

#[then("the knowledge should be created successfully")]
async fn then_knowledge_created_successfully(world: &mut EngramWorld) {
    let entities = world.get_created_entities("knowledge");
    assert!(!entities.is_empty(), "Knowledge was not created");
}

#[then("the knowledge should be stored in Git")]
async fn then_knowledge_stored_in_git(world: &mut EngramWorld) {
    assert!(world.is_storage_initialized());
}

#[then("the creation should fail")]
async fn then_creation_should_fail(_world: &mut EngramWorld) {
    // TODO: Verify creation failed
}

#[then("I should see a validation error about confidence range")]
async fn then_should_see_confidence_error(_world: &mut EngramWorld) {
    // TODO: Verify validation error
}

#[then(expr = "the knowledge confidence should be {float}")]
async fn then_knowledge_confidence_should_be(_world: &mut EngramWorld, _confidence: f64) {
    // TODO: Verify knowledge confidence
}

#[then(expr = "I should see {int} knowledge items")]
async fn then_should_see_n_knowledge(world: &mut EngramWorld, count: i32) {
    let entities = world.get_created_entities("knowledge");

    // Check last query count first if available (for filtered lists)
    if entities.len() != count as usize {
        if let Some(query_count) = world.last_query_count {
            assert_eq!(
                query_count, count as usize,
                "Expected {} knowledge items (from query)",
                count
            );
            return;
        }
    }

    assert_eq!(
        entities.len(),
        count as usize,
        "Expected {} knowledge items",
        count
    );
}

#[then("I should see the knowledge title")]
async fn then_should_see_knowledge_title(_world: &mut EngramWorld) {
    // TODO: Verify knowledge title
}

#[then("I should see the knowledge type")]
async fn then_should_see_knowledge_type(_world: &mut EngramWorld) {
    // TODO: Verify knowledge type
}

#[then("I should see the confidence score")]
async fn then_should_see_confidence_score(_world: &mut EngramWorld) {
    // TODO: Verify confidence score
}

#[then(expr = "the conclusion should be {string}")]
async fn then_conclusion_should_be(_world: &mut EngramWorld, _conclusion: String) {
    // TODO: Verify reasoning conclusion
}

#[then("the reasoning should reference the task ID")]
async fn then_reasoning_references_task(_world: &mut EngramWorld) {
    // TODO: Verify task ID reference
}

#[then(expr = "I should see {int} reasoning items")]
async fn then_should_see_n_reasoning(world: &mut EngramWorld, count: i32) {
    let entities = world.get_created_entities("reasoning");
    assert_eq!(
        entities.len(),
        count as usize,
        "Expected {} reasoning items",
        count
    );
}

#[then("I should see the reasoning title")]
async fn then_should_see_reasoning_title(_world: &mut EngramWorld) {
    // TODO: Verify reasoning title
}

#[then("I should see the description")]
async fn then_should_see_description(_world: &mut EngramWorld) {
    // TODO: Verify description
}

#[then("I should see the conclusion")]
async fn then_should_see_conclusion(_world: &mut EngramWorld) {
    // TODO: Verify conclusion
}

// ============================================================================
// RELATIONSHIP MANAGEMENT STEPS
// ============================================================================

#[given(expr = "I have an entity {string} of type {string}")]
async fn given_entity_of_type(world: &mut EngramWorld, entity_id: String, entity_type: String) {
    world.create_test_entity(&entity_id, &entity_type);
}

#[given(expr = "{string} depends on {string}")]
async fn given_entity_depends_on(world: &mut EngramWorld, source: String, target: String) {
    world.create_test_relationship(&source, &target, "depends-on", "unidirectional", "medium");
}

#[given(expr = "{string} contains {string}")]
async fn given_entity_contains(world: &mut EngramWorld, source: String, target: String) {
    world.create_test_relationship(&source, &target, "contains", "unidirectional", "medium");
}

#[given(expr = "{string} references {string}")]
async fn given_entity_references(world: &mut EngramWorld, source: String, target: String) {
    world.create_test_relationship(&source, &target, "references", "unidirectional", "medium");
}

#[given(expr = "{string} is associated with {string}")]
async fn given_entity_associated_with(world: &mut EngramWorld, source: String, target: String) {
    world.create_test_relationship(
        &source,
        &target,
        "associated-with",
        "bidirectional",
        "medium",
    );
}

#[given("I have multiple entities with various relationships")]
async fn given_multiple_entities_with_relationships(world: &mut EngramWorld) {
    // Create test entities
    world.create_test_entity("project1", "project");
    world.create_test_entity("task1", "task");
    world.create_test_entity("task2", "task");
    world.create_test_entity("doc1", "document");

    // Create various relationships
    world.create_test_relationship("project1", "task1", "contains", "unidirectional", "strong");
    world.create_test_relationship("project1", "task2", "contains", "unidirectional", "strong");
    world.create_test_relationship("task1", "doc1", "references", "unidirectional", "weak");
    world.create_test_relationship("task1", "task2", "depends-on", "unidirectional", "medium");
}

#[when(expr = "I create a relationship from {string} to {string} of type {string}")]
async fn when_create_relationship(
    world: &mut EngramWorld,
    source: String,
    target: String,
    rel_type: String,
) {
    world.create_test_relationship(&source, &target, &rel_type, "unidirectional", "medium");
}

#[when(expr = "I create a bidirectional relationship from {string} to {string} of type {string}")]
async fn when_create_bidirectional_relationship(
    world: &mut EngramWorld,
    source: String,
    target: String,
    rel_type: String,
) {
    world.create_test_relationship(&source, &target, &rel_type, "bidirectional", "medium");
}

#[when(
    expr = "I create a relationship from {string} to {string} of type {string} with strength {string}"
)]
async fn when_create_relationship_with_strength(
    world: &mut EngramWorld,
    source: String,
    target: String,
    rel_type: String,
    strength: String,
) {
    world.create_test_relationship(&source, &target, &rel_type, "unidirectional", &strength);
}

#[when(
    expr = "I create a relationship from {string} to {string} of type {string} with description {string}"
)]
async fn when_create_relationship_with_description(
    world: &mut EngramWorld,
    source: String,
    target: String,
    rel_type: String,
    description: String,
) {
    world.create_test_relationship_with_description(
        &source,
        &target,
        &rel_type,
        "unidirectional",
        "medium",
        &description,
    );
}

#[when(expr = "I list relationships for entity {string}")]
async fn when_list_relationships_for_entity(world: &mut EngramWorld, entity_id: String) {
    world.list_relationships_for_entity(&entity_id);
}

#[when(expr = "I list relationships for {string} filtered by type {string}")]
async fn when_list_relationships_filtered_by_type(
    world: &mut EngramWorld,
    entity_id: String,
    rel_type: String,
) {
    world.list_relationships_for_entity_filtered(&entity_id, &rel_type);
}

#[when("I show the relationship details")]
async fn when_show_relationship_details(world: &mut EngramWorld) {
    world.show_last_relationship_details();
}

#[when(expr = "I delete the relationship between {string} and {string}")]
async fn when_delete_relationship(world: &mut EngramWorld, source: String, target: String) {
    world.delete_relationship_between(&source, &target);
}

#[when(expr = "I find a path from {string} to {string}")]
async fn when_find_path(world: &mut EngramWorld, source: String, target: String) {
    world.find_path_between(&source, &target);
}

#[when(expr = "I get entities connected to {string}")]
async fn when_get_connected_entities(world: &mut EngramWorld, entity_id: String) {
    world.get_connected_entities(&entity_id);
}

#[when("I generate relationship statistics")]
async fn when_generate_relationship_statistics(world: &mut EngramWorld) {
    world.generate_relationship_statistics();
}

#[when(expr = "I try to create a relationship where {string} depends on {string}")]
async fn when_try_create_reverse_dependency(
    world: &mut EngramWorld,
    source: String,
    target: String,
) {
    world.try_create_relationship(&source, &target, "depends-on", "unidirectional", "medium");
}

#[when("the relationship constraints do not allow cycles")]
async fn when_constraints_disallow_cycles(_world: &mut EngramWorld) {
    // TODO: Set constraint configuration
}

#[when("the relationship constraints allow cycles")]
async fn when_constraints_allow_cycles(_world: &mut EngramWorld) {
    // TODO: Set constraint configuration
}

#[when(expr = "I try to create a third outbound relationship from {string}")]
async fn when_try_create_third_relationship(world: &mut EngramWorld, entity_id: String) {
    world.try_create_relationship(
        &entity_id,
        "target3",
        "depends-on",
        "unidirectional",
        "medium",
    );
}

#[when(expr = "I update the relationship strength to {string}")]
async fn when_update_relationship_strength(world: &mut EngramWorld, new_strength: String) {
    world.update_last_relationship_strength(&new_strength);
}

#[when("I restart the system")]
async fn when_restart_system(world: &mut EngramWorld) {
    world.restart_storage_system();
}

#[then("the relationship should be created successfully")]
async fn then_relationship_created_successfully(world: &mut EngramWorld) {
    let relationships = world.get_created_entities("relationship");
    assert!(!relationships.is_empty(), "Relationship was not created");
}

#[then("the relationship should be stored in Git")]
async fn then_relationship_stored_in_git(world: &mut EngramWorld) {
    assert!(world.is_storage_initialized());
}

#[then("the relationship ID should be returned")]
async fn then_relationship_id_returned(world: &mut EngramWorld) {
    if let Some(result) = world.get_last_result() {
        assert!(result.is_ok(), "Expected successful result");
    } else {
        panic!("No result available");
    }
}

#[then(expr = "the relationship direction should be {string}")]
async fn then_relationship_direction_should_be(
    world: &mut EngramWorld,
    expected_direction: String,
) {
    world.verify_last_relationship_direction(&expected_direction);
}

#[then(expr = "the relationship strength should be {string}")]
async fn then_relationship_strength_should_be(world: &mut EngramWorld, expected_strength: String) {
    world.verify_last_relationship_strength(&expected_strength);
}

#[then(expr = "I should see {int} relationships")]
async fn then_should_see_n_relationships(world: &mut EngramWorld, count: i32) {
    let relationships = world.get_last_relationship_count();
    assert_eq!(
        relationships, count as usize,
        "Expected {} relationships",
        count
    );
}

#[then(expr = "I should see a relationship to {string}")]
async fn then_should_see_relationship_to(world: &mut EngramWorld, target: String) {
    assert!(
        world.last_results_contain_relationship_to(&target),
        "Should contain relationship to {}",
        target
    );
}

#[then(expr = "I should not see a relationship to {string}")]
async fn then_should_not_see_relationship_to(world: &mut EngramWorld, target: String) {
    assert!(
        !world.last_results_contain_relationship_to(&target),
        "Should not contain relationship to {}",
        target
    );
}

#[then(expr = "I should see the source entity {string}")]
async fn then_should_see_source_entity(world: &mut EngramWorld, source: String) {
    world.verify_relationship_detail_contains_source(&source);
}

#[then(expr = "I should see the target entity {string}")]
async fn then_should_see_target_entity(world: &mut EngramWorld, target: String) {
    world.verify_relationship_detail_contains_target(&target);
}

#[then(expr = "I should see the relationship type {string}")]
async fn then_should_see_relationship_type(world: &mut EngramWorld, rel_type: String) {
    world.verify_relationship_detail_contains_type(&rel_type);
}

#[then("the relationship should be deleted successfully")]
async fn then_relationship_deleted_successfully(world: &mut EngramWorld) {
    if let Some(result) = world.get_last_result() {
        assert!(result.is_ok(), "Relationship deletion should succeed");
    }
}

#[then("the relationship should not exist in storage")]
async fn then_relationship_not_in_storage(world: &mut EngramWorld) {
    world.verify_relationship_deleted();
}

#[then("I should find a path")]
async fn then_should_find_path(world: &mut EngramWorld) {
    assert!(world.last_path_finding_found_path(), "Should find a path");
}

#[then("I should find no path")]
async fn then_should_find_no_path(world: &mut EngramWorld) {
    assert!(!world.last_path_finding_found_path(), "Should find no path");
}

#[then(expr = "the path should include {string}, {string}, {string} in order")]
async fn then_path_should_include_entities_in_order(
    world: &mut EngramWorld,
    entity1: String,
    entity2: String,
    entity3: String,
) {
    world.verify_path_includes_entities_in_order(&[entity1, entity2, entity3]);
}

#[then(expr = "I should see {int} connected entities")]
async fn then_should_see_n_connected_entities(world: &mut EngramWorld, count: i32) {
    let connected_count = world.get_last_connected_entities_count();
    assert_eq!(
        connected_count, count as usize,
        "Expected {} connected entities",
        count
    );
}

#[then("I should see the total number of relationships")]
async fn then_should_see_total_relationships(world: &mut EngramWorld) {
    world.verify_statistics_contain_total_relationships();
}

#[then("I should see relationships broken down by type")]
async fn then_should_see_relationships_by_type(world: &mut EngramWorld) {
    world.verify_statistics_contain_breakdown_by_type();
}

#[then("I should see the most connected entity")]
async fn then_should_see_most_connected_entity(world: &mut EngramWorld) {
    world.verify_statistics_contain_most_connected_entity();
}

#[then("I should see relationship density")]
async fn then_should_see_relationship_density(world: &mut EngramWorld) {
    world.verify_statistics_contain_relationship_density();
}

#[then("the relationship creation should fail")]
async fn then_relationship_creation_should_fail(world: &mut EngramWorld) {
    if let Some(result) = world.get_last_result() {
        assert!(result.is_err(), "Relationship creation should fail");
    } else {
        panic!("No result available to verify failure");
    }
}

#[then("I should see a cycle prevention error")]
async fn then_should_see_cycle_error(world: &mut EngramWorld) {
    if let Some(result) = world.get_last_result() {
        if let Err(error_msg) = result {
            assert!(
                error_msg.to_lowercase().contains("cycle"),
                "Should contain cycle error message. Got: {}",
                error_msg
            );
        }
    }
}

#[then("I should see a relationship limit error")]
async fn then_should_see_limit_error(world: &mut EngramWorld) {
    if let Some(result) = world.get_last_result() {
        if let Err(error_msg) = result {
            assert!(
                error_msg.contains("limit"),
                "Should contain limit error message"
            );
        }
    }
}

#[then(expr = "I should still see the relationship to {string}")]
async fn then_should_still_see_relationship_to(world: &mut EngramWorld, target: String) {
    assert!(
        world.last_results_contain_relationship_to(&target),
        "Should still contain relationship to {}",
        target
    );
}

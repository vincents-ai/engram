//! BDD tests for task workflow
use crate::bdd::EngramWorld;
use cucumber::{given, then, when};

#[given(expr = "{word} has a workflow {string} with stages {string}")]
async fn given_agent_has_workflow(
    world: &mut EngramWorld,
    agent: String,
    workflow_title: String,
    stages: String,
) {
    world.initialize_storage(&agent);
    let stage_list: Vec<&str> = stages.split(',').map(|s| s.trim()).collect();
    // This is a placeholder for creating a workflow entity
    // For now we'll just store the ID for later use
    world.create_test_workflow(&workflow_title, &stage_list);
}

#[when(expr = "I create a task with title {string} and workflow {string}")]
async fn when_create_task_with_workflow(
    world: &mut EngramWorld,
    title: String,
    workflow_name: String,
) {
    let workflow_id = world.get_workflow_id_by_name(&workflow_name);
    world.create_task_with_workflow(&title, "Description", "medium", workflow_id.as_deref());
}

#[then(expr = "the task should be in state {string}")]
async fn then_task_should_be_in_state(world: &mut EngramWorld, state: String) {
    world.verify_last_task_workflow_state(&state);
}

#[when(expr = "I transition the task to state {string}")]
async fn when_transition_task(world: &mut EngramWorld, state: String) {
    world.transition_last_task_to_state(&state);
}

#[then(expr = "the transition should be {string}")]
async fn then_transition_result(world: &mut EngramWorld, expected_result: String) {
    match expected_result.as_str() {
        "allowed" => assert!(
            world.last_operation_succeeded(),
            "Transition should be allowed"
        ),
        "denied" => assert!(
            !world.last_operation_succeeded(),
            "Transition should be denied"
        ),
        _ => panic!("Unknown expected result: {}", expected_result),
    }
}

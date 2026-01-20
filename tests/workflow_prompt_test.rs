use engram::entities::workflow::WorkflowState;

#[test]
fn test_workflow_state_with_prompts() {
    let json = r#"{
        "id": "state-1",
        "name": "Review",
        "state_type": "review",
        "description": "Code review",
        "is_final": false,
        "prompts": {
            "system": "You are a reviewer",
            "user": "Review task {{TASK_ID}}"
        }
    }"#;

    let state: WorkflowState = serde_json::from_str(json).unwrap();
    assert_eq!(state.prompts.unwrap().system.unwrap(), "You are a reviewer");
}

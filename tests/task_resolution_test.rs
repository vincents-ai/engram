use engram::cli::task::{create_task, resolve_task, update_task};
use engram::entities::{Entity, Task, TaskStatus};
use engram::storage::MemoryStorage;
use engram::Storage;

fn create_test_storage() -> MemoryStorage {
    MemoryStorage::new("default")
}

#[test]
fn test_resolve_blocked_task_with_message() {
    let mut storage = create_test_storage();

    // 1. Create a task
    create_task(
        &mut storage,
        Some("Blocked Task".to_string()),
        None,
        "medium",
        None,
        None,
        None,
        false,
        None,
        false,
        None,
        false,
        None,
    )
    .unwrap();

    let tasks = storage.query_by_agent("default", Some("task")).unwrap();
    let task_id = tasks[0].id.clone();

    // 2. Block the task
    update_task(
        &mut storage,
        &task_id,
        "blocked",
        None,
        Some("Missing credentials"),
    )
    .unwrap();

    // Verify blocked
    let task = Task::from_generic(storage.get(&task_id, "task").unwrap().unwrap()).unwrap();
    assert_eq!(task.status, TaskStatus::Blocked);
    assert_eq!(task.block_reason, Some("Missing credentials".to_string()));

    // 3. Resolve the task with a message
    resolve_task(
        &mut storage,
        &task_id,
        Some("Credentials provided in secure note"),
    )
    .unwrap();

    // 4. Verify resolved state
    let resolved_task =
        Task::from_generic(storage.get(&task_id, "task").unwrap().unwrap()).unwrap();

    // Should be InProgress
    assert_eq!(resolved_task.status, TaskStatus::InProgress);
    // Block reason should be cleared (Task::start clears it)
    assert!(resolved_task.block_reason.is_none());

    // Check metadata for resolution history
    let resolutions = resolved_task
        .metadata
        .get("resolutions")
        .unwrap()
        .as_array()
        .unwrap();
    assert_eq!(resolutions.len(), 1);

    let entry = &resolutions[0];
    assert_eq!(entry["resolution"], "Credentials provided in secure note");
    assert_eq!(entry["previous_block_reason"], "Missing credentials");
    assert!(entry["timestamp"].is_string());
}

#[test]
fn test_resolve_non_blocked_task() {
    let mut storage = create_test_storage();

    // 1. Create a task
    create_task(
        &mut storage,
        Some("Active Task".to_string()),
        None,
        "medium",
        None,
        None,
        None,
        false,
        None,
        false,
        None,
        false,
        None,
    )
    .unwrap();

    let tasks = storage.query_by_agent("default", Some("task")).unwrap();
    let task_id = tasks[0].id.clone();

    // 2. Try to resolve (should do nothing but succeed)
    let result = resolve_task(&mut storage, &task_id, Some("Trying to resolve"));

    assert!(result.is_ok());

    // Verify status hasn't changed (still Todo)
    let task = Task::from_generic(storage.get(&task_id, "task").unwrap().unwrap()).unwrap();
    assert_eq!(task.status, TaskStatus::Todo);
    // Metadata should be empty
    assert!(task.metadata.get("resolutions").is_none());
}

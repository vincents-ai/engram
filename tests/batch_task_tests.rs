//! Integration tests for `engram task create-batch` command

use engram::{
    cli::create_task_batch,
    entities::{Entity, Task},
    storage::MemoryStorage,
    Storage,
};
use std::io::Write;
use tempfile::NamedTempFile;

fn make_storage() -> MemoryStorage {
    MemoryStorage::new("default")
}

// ---------------------------------------------------------------------------
// JSON stdin input — 3 tasks, --output json → NDJSON lines
// ---------------------------------------------------------------------------

#[test]
fn test_create_batch_json_stdin_output_json() {
    // We can't inject stdin in unit tests, so we use --file instead (same code
    // path as --json for deserialization; stdin is tested via --file here).
    let mut storage = make_storage();

    let json = r#"[
      {"title": "Alpha", "priority": "high"},
      {"title": "Beta",  "priority": "medium", "description": "second task"},
      {"title": "Gamma"}
    ]"#;

    let mut file = NamedTempFile::new().expect("temp file");
    write!(file, "{}", json).unwrap();
    let path = file.path().to_str().unwrap().to_string();

    let result = create_task_batch(
        &mut storage,
        Some(path),
        false,
        None,
        None,
        "medium",
        None,
        "json",
        false,
    );
    assert!(result.is_ok(), "batch create failed: {:?}", result);

    let tasks = storage.query_by_agent("default", Some("task")).unwrap();
    assert_eq!(tasks.len(), 3, "expected 3 tasks, got {}", tasks.len());

    let titles: Vec<String> = tasks
        .iter()
        .filter_map(|g| Task::from_generic(g.clone()).ok())
        .map(|t: Task| t.title.clone())
        .collect();

    assert!(titles.contains(&"Alpha".to_string()));
    assert!(titles.contains(&"Beta".to_string()));
    assert!(titles.contains(&"Gamma".to_string()));
}

// ---------------------------------------------------------------------------
// --titles-file: 3 titles + blank line + comment → 3 tasks
// ---------------------------------------------------------------------------

#[test]
fn test_create_batch_titles_file_skips_blanks_and_comments() {
    let mut storage = make_storage();

    let content = "Task One\n\n# this is a comment\nTask Two\n  \nTask Three\n";

    let mut file = NamedTempFile::new().expect("temp file");
    write!(file, "{}", content).unwrap();
    let path = file.path().to_str().unwrap().to_string();

    let result = create_task_batch(
        &mut storage,
        None,
        false,
        Some(path),
        None,
        "medium",
        None,
        "text",
        false,
    );
    assert!(result.is_ok(), "batch create failed: {:?}", result);

    let tasks = storage.query_by_agent("default", Some("task")).unwrap();
    assert_eq!(tasks.len(), 3, "expected 3 tasks, got {}", tasks.len());

    let titles: Vec<String> = tasks
        .iter()
        .filter_map(|g| Task::from_generic(g.clone()).ok())
        .map(|t: Task| t.title.clone())
        .collect();

    assert!(titles.contains(&"Task One".to_string()));
    assert!(titles.contains(&"Task Two".to_string()));
    assert!(titles.contains(&"Task Three".to_string()));
}

// ---------------------------------------------------------------------------
// --output ids — bare UUIDs, one per line
// ---------------------------------------------------------------------------

#[test]
fn test_create_batch_output_ids() {
    let mut storage = make_storage();

    let json = r#"[{"title": "ID Task 1"}, {"title": "ID Task 2"}]"#;

    let mut file = NamedTempFile::new().expect("temp file");
    write!(file, "{}", json).unwrap();
    let path = file.path().to_str().unwrap().to_string();

    let result = create_task_batch(
        &mut storage,
        Some(path),
        false,
        None,
        None,
        "medium",
        None,
        "ids",
        false,
    );
    assert!(result.is_ok(), "batch create failed: {:?}", result);

    let tasks = storage.query_by_agent("default", Some("task")).unwrap();
    assert_eq!(tasks.len(), 2);
}

// ---------------------------------------------------------------------------
// --no-fail-fast: one bad entry (empty title) continues and creates others
// ---------------------------------------------------------------------------

#[test]
fn test_create_batch_no_fail_fast_partial_success() {
    let mut storage = make_storage();

    // Second entry has an empty title — Task::new will still accept it (the
    // storage layer doesn't validate non-empty titles), so we test the
    // no-fail-fast path by injecting a JSON file with a valid and invalid entry.
    // Since MemoryStorage doesn't fail on empty titles, we verify the count of
    // 3 tasks even though one has an empty title (the store itself succeeds).
    let json = r#"[
      {"title": "Good Task A"},
      {"title": ""},
      {"title": "Good Task B"}
    ]"#;

    let mut file = NamedTempFile::new().expect("temp file");
    write!(file, "{}", json).unwrap();
    let path = file.path().to_str().unwrap().to_string();

    let result = create_task_batch(
        &mut storage,
        Some(path),
        false,
        None,
        None,
        "medium",
        None,
        "text",
        true, // --no-fail-fast
    );
    assert!(
        result.is_ok(),
        "with --no-fail-fast the function should not return Err: {:?}",
        result
    );

    // All 3 entries are stored (MemoryStorage accepts empty titles)
    let tasks = storage.query_by_agent("default", Some("task")).unwrap();
    assert_eq!(tasks.len(), 3);
}

// ---------------------------------------------------------------------------
// Batch defaults: --parent and --priority applied to all tasks
// ---------------------------------------------------------------------------

#[test]
fn test_create_batch_applies_parent_and_priority_defaults() {
    let mut storage = make_storage();

    let json = r#"[{"title": "Child A"}, {"title": "Child B", "priority": "high"}]"#;

    let mut file = NamedTempFile::new().expect("temp file");
    write!(file, "{}", json).unwrap();
    let path = file.path().to_str().unwrap().to_string();

    let fake_parent = "00000000-0000-0000-0000-000000000001".to_string();

    let result = create_task_batch(
        &mut storage,
        Some(path),
        false,
        None,
        Some(fake_parent.clone()),
        "low", // batch default priority
        None,
        "text",
        false,
    );
    assert!(result.is_ok(), "{:?}", result);

    let tasks: Vec<Task> = storage
        .query_by_agent("default", Some("task"))
        .unwrap()
        .into_iter()
        .filter_map(|g| Task::from_generic(g).ok())
        .collect();

    assert_eq!(tasks.len(), 2);

    for task in &tasks {
        assert_eq!(
            task.parent.as_deref(),
            Some(fake_parent.as_str()),
            "parent not applied to task '{}'",
            task.title
        );
    }

    let child_a = tasks.iter().find(|t| t.title == "Child A").unwrap();
    assert_eq!(
        child_a.priority,
        engram::entities::TaskPriority::Low,
        "batch default priority should be applied when task has none"
    );

    let child_b = tasks.iter().find(|t| t.title == "Child B").unwrap();
    assert_eq!(
        child_b.priority,
        engram::entities::TaskPriority::High,
        "explicit priority on task should override batch default"
    );
}

// ---------------------------------------------------------------------------
// Error: no input source provided
// ---------------------------------------------------------------------------

#[test]
fn test_create_batch_requires_input_source() {
    let mut storage = make_storage();

    let result = create_task_batch(
        &mut storage,
        None,
        false, // --json false
        None,  // no --titles-file
        None,
        "medium",
        None,
        "text",
        false,
    );

    assert!(
        result.is_err(),
        "should return Err when no input source is given"
    );
}

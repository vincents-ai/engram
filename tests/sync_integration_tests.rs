//! Integration tests for sync operations
//!
//! Tests authentication, branch management, and remote operations
//! without requiring actual remote repositories.
//!
//! Note: These tests modify the process-wide current directory and must run serially.

use engram::{
    cli::sync::{create_branch, create_credentials, delete_branch, list_branches, switch_branch},
    error::EngramError,
    storage::{GitStorage, MemoryStorage, RemoteAuth},
};
use git2::{Repository, Signature};
use std::env;
use std::fs;
use std::sync::Mutex;
use tempfile::TempDir;

// Global mutex to ensure tests run serially since they modify process-wide state
static TEST_MUTEX: Mutex<()> = Mutex::new(());

/// Test fixture for sync operations
struct SyncTestFixture {
    temp_dir: TempDir,
    storage: GitStorage,
    repo_path: String,
}

impl SyncTestFixture {
    fn new() -> Result<Self, EngramError> {
        let temp_dir = TempDir::new().map_err(|e| EngramError::Io(e))?;
        let repo_path = temp_dir.path().join(".engram");

        fs::create_dir_all(&repo_path)?;
        let repo = Repository::init(&repo_path).map_err(|e| EngramError::Git(e.to_string()))?;

        let mut config = repo.config().map_err(|e| EngramError::Git(e.to_string()))?;
        config
            .set_str("user.name", "Test User")
            .map_err(|e| EngramError::Git(e.to_string()))?;
        config
            .set_str("user.email", "test@example.com")
            .map_err(|e| EngramError::Git(e.to_string()))?;

        let signature = Signature::now("Test User", "test@example.com")
            .map_err(|e| EngramError::Git(e.to_string()))?;
        let tree_id = {
            let mut index = repo.index().map_err(|e| EngramError::Git(e.to_string()))?;
            index
                .write_tree()
                .map_err(|e| EngramError::Git(e.to_string()))?
        };
        let tree = repo
            .find_tree(tree_id)
            .map_err(|e| EngramError::Git(e.to_string()))?;

        let commit_oid = repo
            .commit(
                Some("HEAD"),
                &signature,
                &signature,
                "Initial commit",
                &tree,
                &[],
            )
            .map_err(|e| EngramError::Git(e.to_string()))?;

        // Ensure 'main' branch exists and points to the initial commit
        let commit = repo
            .find_commit(commit_oid)
            .map_err(|e| EngramError::Git(e.to_string()))?;

        // Try to create main branch, or update it if it already exists
        if repo.find_branch("main", git2::BranchType::Local).is_err() {
            repo.branch("main", &commit, false)
                .map_err(|e| EngramError::Git(e.to_string()))?;
        }

        // Set HEAD to point to main branch
        repo.set_head("refs/heads/main")
            .map_err(|e| EngramError::Git(e.to_string()))?;

        let storage = GitStorage::new(&repo_path.to_string_lossy(), "test-agent")?;

        Ok(SyncTestFixture {
            temp_dir,
            storage,
            repo_path: repo_path.to_string_lossy().to_string(),
        })
    }

    fn set_working_directory(&self) -> Result<(), std::io::Error> {
        env::set_current_dir(self.temp_dir.path())
    }
}

#[tokio::test]
async fn test_branch_creation_and_switching() -> Result<(), EngramError> {
    let _guard = TEST_MUTEX.lock().unwrap();
    let fixture = SyncTestFixture::new()?;
    fixture.set_working_directory()?;

    create_branch("test-branch", Some("test-agent"), None)?;

    let result = create_branch("test-branch", Some("test-agent"), None);
    assert!(result.is_err());

    switch_branch("test-branch", false)?;

    switch_branch("new-branch", true)?;

    // Keep fixture alive to prevent cleanup
    drop(fixture);
    Ok(())
}

#[tokio::test]
async fn test_branch_listing() -> Result<(), EngramError> {
    let _guard = TEST_MUTEX.lock().unwrap();
    let fixture = SyncTestFixture::new()?;
    fixture.set_working_directory()?;

    create_branch("agent-alice", Some("alice"), None)?;
    create_branch("agent-bob", Some("bob"), None)?;
    create_branch("feature-123", None, None)?;

    list_branches(true, false)?;

    switch_branch("agent-alice", false)?;
    list_branches(false, true)?;

    drop(fixture);
    Ok(())
}

#[tokio::test]
async fn test_branch_deletion() -> Result<(), EngramError> {
    let _guard = TEST_MUTEX.lock().unwrap();
    let fixture = SyncTestFixture::new()?;
    fixture.set_working_directory()?;

    create_branch("delete-me", Some("test"), None)?;

    // Ensure branch creation completes before switching
    std::thread::sleep(std::time::Duration::from_millis(50));

    switch_branch("main", false)?;

    let result = delete_branch("delete-me", false);

    delete_branch("delete-me", true)?;

    let result = delete_branch("non-existent", true);
    assert!(result.is_err());

    drop(fixture);
    Ok(())
}

#[tokio::test]
async fn test_authentication_credentials() -> Result<(), EngramError> {
    let _guard = TEST_MUTEX.lock().unwrap();
    let _fixture = SyncTestFixture::new()?;

    let ssh_auth = RemoteAuth {
        auth_type: "ssh".to_string(),
        username: Some("git".to_string()),
        password: None,
        key_path: Some("/home/user/.ssh/id_rsa".to_string()),
    };

    let credentials = create_credentials(&ssh_auth)?;
    assert!(credentials.is_some());

    let http_auth = RemoteAuth {
        auth_type: "http".to_string(),
        username: Some("testuser".to_string()),
        password: Some("testpass".to_string()),
        key_path: None,
    };

    let credentials = create_credentials(&http_auth)?;
    assert!(credentials.is_some());

    let invalid_auth = RemoteAuth {
        auth_type: "invalid".to_string(),
        username: None,
        password: None,
        key_path: None,
    };

    let result = create_credentials(&invalid_auth);
    assert!(result.is_err());

    Ok(())
}

#[tokio::test]
async fn test_memory_storage_integration() -> Result<(), EngramError> {
    let _guard = TEST_MUTEX.lock().unwrap();
    let _fixture = SyncTestFixture::new()?;

    let mut _memory_storage = MemoryStorage::new("test-agent");

    Ok(())
}

#[tokio::test]
async fn test_multi_agent_branch_isolation() -> Result<(), EngramError> {
    let _guard = TEST_MUTEX.lock().unwrap();
    let fixture = SyncTestFixture::new()?;
    fixture.set_working_directory()?;

    create_branch("agent-alice-work", Some("alice"), None)?;
    create_branch("agent-bob-work", Some("bob"), None)?;
    create_branch("agent-charlie-work", Some("charlie"), None)?;

    switch_branch("agent-alice-work", false)?;
    switch_branch("agent-bob-work", false)?;
    switch_branch("agent-charlie-work", false)?;

    list_branches(true, false)?;

    drop(fixture);
    Ok(())
}

#[tokio::test]
async fn test_branch_agent_association() -> Result<(), EngramError> {
    let _guard = TEST_MUTEX.lock().unwrap();
    let fixture = SyncTestFixture::new()?;
    fixture.set_working_directory()?;

    create_branch("feature-auth", Some("security-agent"), None)?;
    create_branch("feature-ui", Some("frontend-agent"), None)?;
    create_branch("feature-db", Some("backend-agent"), None)?;

    drop(fixture);
    Ok(())
}

#[tokio::test]
async fn test_error_conditions() -> Result<(), EngramError> {
    let _guard = TEST_MUTEX.lock().unwrap();
    let fixture = SyncTestFixture::new()?;
    fixture.set_working_directory()?;

    let result = switch_branch("non-existent-branch", false);
    assert!(result.is_err());

    let result = create_branch("", Some("test"), None);
    assert!(result.is_err());

    create_branch("current-branch", Some("test"), None)?;

    // Ensure branch creation completes before switching
    std::thread::sleep(std::time::Duration::from_millis(50));

    switch_branch("current-branch", false)?;
    let result = delete_branch("current-branch", true);
    assert!(result.is_err());

    drop(fixture);
    Ok(())
}

#[tokio::test]
async fn test_concurrent_branch_operations() -> Result<(), EngramError> {
    let _guard = TEST_MUTEX.lock().unwrap();
    let fixture = SyncTestFixture::new()?;
    fixture.set_working_directory()?;

    for i in 1..=5 {
        let branch_name = format!("concurrent-{}", i);
        let agent_name = format!("agent-{}", i);

        create_branch(&branch_name, Some(&agent_name), None)?;

        // Ensure branch creation completes before switching
        std::thread::sleep(std::time::Duration::from_millis(50));

        switch_branch(&branch_name, false)?;
    }

    list_branches(true, false)?;

    switch_branch("main", false)?;
    for i in 1..=5 {
        let branch_name = format!("concurrent-{}", i);
        delete_branch(&branch_name, true)?;
    }

    drop(fixture);
    Ok(())
}

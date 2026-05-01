//! Migration utilities for converting between storage backends
//!
//! This module provides tools for migrating data from the dual-repository
//! architecture (.engram/ directory) to the Git refs storage architecture,
//! as well as fixing the triple-nesting serialisation bug.
//!
//! # Triple-nesting bug (BREAKING CHANGE to stored blob format)
//!
//! In early versions of engram the entity blob stored in git refs was
//! triple-nested: the actual entity fields ended up at
//! `data.entity.entity.entity.*` instead of being flat at `data.*`.
//!
//! The root cause was that `to_generic()` serialised the entity struct into a
//! `Value::Object` that was then stored *as-is* in the `data` field of the
//! `MemoryEntity` wrapper — adding an extra layer.  When this happened
//! repeatedly (e.g. through `EntityRegistry::create()`), nesting compounded.
//!
//! `git_refs_storage::flatten_data_map` now prevents new nesting at write
//! time.  `migrate_triple_nesting` below repairs blobs that were written
//! before the fix by delegating to `GitRefsStorage::flatten_nested_refs`.

use crate::error::EngramError;
use crate::storage::{memory_entity::MemoryEntity, FlattenRefsStats, GitRefsStorage, Storage};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Migration configuration and state
pub struct Migration {
    source_path: PathBuf,
    target_storage: GitRefsStorage,
    dry_run: bool,
    backup_only: bool,
}

/// Migration statistics
#[derive(Debug, Default)]
pub struct MigrationStats {
    pub entities_processed: usize,
    pub entities_migrated: usize,
    pub entities_failed: usize,
    pub entity_types: HashMap<String, usize>,
}

impl Migration {
    /// Create new migration instance
    pub fn new(
        workspace_path: &str,
        agent: &str,
        dry_run: bool,
        backup_only: bool,
    ) -> Result<Self, EngramError> {
        let source_path = PathBuf::from(workspace_path).join(".engram");
        let target_storage = GitRefsStorage::new(workspace_path, agent)?;

        Ok(Self {
            source_path,
            target_storage,
            dry_run,
            backup_only,
        })
    }

    /// Execute the migration from .engram/ to Git refs storage
    pub fn execute(&mut self) -> Result<MigrationStats, EngramError> {
        let mut stats = MigrationStats::default();

        if !self.source_path.exists() {
            return Ok(stats);
        }

        if self.backup_only {
            println!("💾 Creating backup only...");
            self.create_backup()?;
            return Ok(stats);
        }
        if self.dry_run {
            println!("📝 DRY RUN: No changes will be made");
        }

        let entity_dirs = self.discover_entity_directories()?;
        println!("📂 Found {} entity type directories", entity_dirs.len());

        for (entity_type, dir_path) in entity_dirs {
            println!("\n📁 Migrating {} entities...", entity_type);
            let type_stats = self.migrate_entity_type(&entity_type, &dir_path)?;

            stats.entities_processed += type_stats.entities_processed;
            stats.entities_migrated += type_stats.entities_migrated;
            stats.entities_failed += type_stats.entities_failed;
            stats
                .entity_types
                .insert(entity_type.clone(), type_stats.entities_migrated);

            println!(
                "   ✅ {}/{} {} entities migrated",
                type_stats.entities_migrated, type_stats.entities_processed, entity_type
            );
        }

        println!("\n🏁 Migration Summary:");
        println!("   📊 Total processed: {}", stats.entities_processed);
        println!("   ✅ Successfully migrated: {}", stats.entities_migrated);
        if stats.entities_failed > 0 {
            println!("   ❌ Failed: {}", stats.entities_failed);
        }

        if !self.dry_run && stats.entities_migrated > 0 {
            println!("\n💾 Creating backup of original .engram directory...");
            self.create_backup()?;
        }

        Ok(stats)
    }

    /// Discover entity type directories in .engram/
    fn discover_entity_directories(&self) -> Result<Vec<(String, PathBuf)>, EngramError> {
        let mut entity_dirs = Vec::new();

        let entries = fs::read_dir(&self.source_path).map_err(|e| {
            EngramError::NotFound(format!("Failed to read source directory: {}", e))
        })?;

        for entry in entries {
            let entry = entry.map_err(|e| {
                EngramError::InvalidOperation(format!("Failed to read directory entry: {}", e))
            })?;
            let path = entry.path();

            if path.is_dir() {
                let dir_name = path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or("")
                    .to_string();

                // Skip .git directory and other non-entity directories
                if !dir_name.starts_with('.')
                    && dir_name != "session"
                    && self.has_json_files(&path)?
                {
                    entity_dirs.push((dir_name, path));
                }
            }
        }

        entity_dirs.sort_by(|a, b| a.0.cmp(&b.0));
        Ok(entity_dirs)
    }

    /// Check if directory contains JSON files
    fn has_json_files(&self, dir_path: &Path) -> Result<bool, EngramError> {
        let entries = fs::read_dir(dir_path).map_err(|e| {
            EngramError::InvalidOperation(format!("Failed to read directory: {}", e))
        })?;

        for entry in entries {
            let entry = entry.map_err(|e| {
                EngramError::InvalidOperation(format!("Failed to read entry: {}", e))
            })?;
            if entry.path().extension().map_or(false, |ext| ext == "json") {
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// Migrate all entities of a specific type
    fn migrate_entity_type(
        &mut self,
        entity_type: &str,
        dir_path: &Path,
    ) -> Result<MigrationStats, EngramError> {
        let mut stats = MigrationStats::default();

        let entries = fs::read_dir(dir_path).map_err(|e| {
            EngramError::InvalidOperation(format!("Failed to read entity directory: {}", e))
        })?;

        for entry in entries {
            let entry = entry.map_err(|e| {
                EngramError::InvalidOperation(format!("Failed to read file entry: {}", e))
            })?;
            let path = entry.path();

            if path.extension().map_or(false, |ext| ext == "json") {
                stats.entities_processed += 1;

                match self.migrate_single_entity(entity_type, &path) {
                    Ok(_) => stats.entities_migrated += 1,
                    Err(e) => {
                        stats.entities_failed += 1;
                        eprintln!("   ⚠️  Failed to migrate {}: {}", path.display(), e);
                    }
                }
            }
        }

        Ok(stats)
    }

    /// Migrate a single entity file
    fn migrate_single_entity(
        &mut self,
        entity_type: &str,
        file_path: &Path,
    ) -> Result<(), EngramError> {
        // Read the MemoryEntity JSON file
        let content = fs::read_to_string(file_path)
            .map_err(|e| EngramError::InvalidOperation(format!("Failed to read file: {}", e)))?;

        let memory_entity: MemoryEntity = serde_json::from_str(&content)
            .map_err(|e| EngramError::Deserialization(e.to_string()))?;

        // Convert to GenericEntity format expected by Git refs storage
        let generic_entity = crate::entities::GenericEntity {
            id: memory_entity.id.clone(),
            entity_type: entity_type.to_string(),
            agent: memory_entity.agent.clone(),
            timestamp: memory_entity.timestamp,
            data: serde_json::to_value(&memory_entity.data)
                .map_err(|e| EngramError::Serialization(e))?,
        };

        if !self.dry_run {
            // Store in Git refs storage - just store the generic entity directly
            self.target_storage.store(&generic_entity)?;
        }

        Ok(())
    }

    /// Create backup of original .engram directory
    pub fn create_backup(&self) -> Result<(), EngramError> {
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let backup_path = self
            .source_path
            .parent()
            .unwrap()
            .join(format!(".engram_backup_{}", timestamp));

        self.copy_dir_all(&self.source_path, &backup_path)?;
        println!("   📦 Backup created at: {}", backup_path.display());
        Ok(())
    }

    /// Recursively copy directory
    fn copy_dir_all(&self, src: &Path, dst: &Path) -> Result<(), EngramError> {
        fs::create_dir_all(dst).map_err(|e| {
            EngramError::InvalidOperation(format!("Failed to create backup directory: {}", e))
        })?;

        for entry in fs::read_dir(src).map_err(|e| {
            EngramError::InvalidOperation(format!("Failed to read source directory: {}", e))
        })? {
            let entry = entry.map_err(|e| {
                EngramError::InvalidOperation(format!("Failed to read directory entry: {}", e))
            })?;
            let ty = entry.file_type().map_err(|e| {
                EngramError::InvalidOperation(format!("Failed to get file type: {}", e))
            })?;

            if ty.is_dir() {
                self.copy_dir_all(&entry.path(), &dst.join(entry.file_name()))?;
            } else {
                fs::copy(&entry.path(), &dst.join(entry.file_name())).map_err(|e| {
                    EngramError::InvalidOperation(format!("Failed to copy file: {}", e))
                })?;
            }
        }

        Ok(())
    }

    /// Validate that source data is ready for migration
    pub fn validate_migration_readiness(workspace_path: &str) -> Result<(), EngramError> {
        let engram_path = PathBuf::from(workspace_path).join(".engram");

        if !engram_path.exists() {
            return Err(EngramError::NotFound(
                "No .engram directory found. Nothing to migrate.".to_string(),
            ));
        }

        // Check for Git repository in target location
        let git_path = PathBuf::from(workspace_path).join(".git");
        if !git_path.exists() {
            return Err(EngramError::InvalidOperation(
                "No Git repository found. Git refs storage requires a Git repository.".to_string(),
            ));
        }

        // Basic validation of .engram structure
        let git_engram_path = engram_path.join(".git");
        if !git_engram_path.exists() {
            return Err(EngramError::InvalidOperation(
                ".engram directory is not a Git repository. Invalid source format.".to_string(),
            ));
        }

        println!("✅ Migration readiness validated");
        println!("   📁 Source: {}", engram_path.display());
        println!("   🎯 Target: Git refs in {}", git_path.display());

        Ok(())
    }
}

/// Report returned by [`migrate_triple_nesting`].
#[derive(Debug, Default)]
pub struct MigrationReport {
    /// Total entity refs inspected (workspace config and version sidecars excluded).
    pub refs_scanned: usize,
    /// Refs that were found to contain nested `data` blobs.
    pub nested_found: usize,
    /// Refs that were rewritten to a flat structure (always 0 in dry-run mode).
    pub refs_rewritten: usize,
}

impl From<FlattenRefsStats> for MigrationReport {
    fn from(s: FlattenRefsStats) -> Self {
        Self {
            refs_scanned: s.refs_scanned,
            nested_found: s.nested_found,
            refs_rewritten: s.refs_rewritten,
        }
    }
}

/// Scan all entity blobs in the git-ref store and flatten any that carry the
/// legacy triple-nesting bug.
///
/// # The bug
///
/// Early versions of engram stored entity data with up to three levels of
/// nesting inside the `data` field of the git blob:
///
/// ```json
/// { "data": { "data": { "data": { "title": "...", ... } } } }
/// ```
///
/// The fix in `git_refs_storage::flatten_data_map` prevents the bug for
/// **newly written** blobs.  This function repairs **existing** blobs that
/// were written before the fix.
///
/// # Detection
///
/// A blob is considered nested when its `data` field is a JSON object that
/// itself contains a `"data"` key whose value is also a JSON object.  The
/// detection is recursive: triple-, quadruple-, and deeper nesting is handled
/// by `flatten_data_map`.
///
/// # Idempotency
///
/// Already-flat blobs are left completely unchanged — the function writes a
/// new git blob only when the nesting is actually detected.  Running the
/// migration a second time is therefore safe and produces zero rewrites.
///
/// # Version sidecar refs
///
/// Refs under `refs/engram/*/v<N>/<uuid>` carry only metadata (version
/// number, timestamp, agent) and are never entity-data blobs.  They are
/// skipped entirely.
///
/// # Arguments
///
/// * `storage` — an initialised [`GitRefsStorage`] pointing at the workspace
///   to migrate.
/// * `dry_run` — when `true`, the function performs all detection but writes
///   nothing; `refs_rewritten` will be 0.
pub fn migrate_triple_nesting(
    storage: &GitRefsStorage,
    dry_run: bool,
) -> Result<MigrationReport, EngramError> {
    let stats = storage.flatten_nested_refs(dry_run)?;
    Ok(MigrationReport::from(stats))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::memory_entity::MemoryEntity;
    use std::collections::HashMap;

    fn setup_git_repo(dir: &std::path::Path) {
        gix::init(dir).unwrap();
        // Configure git identity for reflog operations (required on CI where no global config exists)
        let config_path = dir.join(".git").join("config");
        let config_content = std::fs::read_to_string(&config_path).unwrap_or_default();
        let new_content = format!(
            "{}\n[user]\n\tname = Test\n\temail = test@test.com\n",
            config_content.trim_end()
        );
        std::fs::write(&config_path, new_content).unwrap();
    }

    fn setup_engram_dir(dir: &std::path::Path) {
        let engram = dir.join(".engram");
        std::fs::create_dir_all(engram.join(".git")).unwrap();
    }

    fn create_valid_memory_entity_json(id: &str, entity_type: &str) -> String {
        let mut data = HashMap::new();
        data.insert("title".to_string(), serde_json::json!("Test Entity"));
        data.insert("value".to_string(), serde_json::json!(42));
        let entity = MemoryEntity::new(
            id.to_string(),
            entity_type.to_string(),
            "test-agent".to_string(),
            chrono::Utc::now(),
            data,
        );
        serde_json::to_string(&entity).unwrap()
    }

    #[test]
    fn test_migration_new() {
        let tmp = tempfile::TempDir::new().unwrap();
        let workspace = tmp.path().to_str().unwrap();
        setup_git_repo(tmp.path());

        let migration = Migration::new(workspace, "test-agent", false, false);
        assert!(migration.is_ok());
        let m = migration.unwrap();
        assert_eq!(
            m.source_path,
            std::path::PathBuf::from(workspace).join(".engram")
        );
        assert!(!m.dry_run);
        assert!(!m.backup_only);
    }

    #[test]
    fn test_migration_new_dry_run() {
        let tmp = tempfile::TempDir::new().unwrap();
        let workspace = tmp.path().to_str().unwrap();
        setup_git_repo(tmp.path());

        let migration = Migration::new(workspace, "test-agent", true, false).unwrap();
        assert!(migration.dry_run);
        assert!(!migration.backup_only);
    }

    #[test]
    fn test_migration_new_backup_only() {
        let tmp = tempfile::TempDir::new().unwrap();
        let workspace = tmp.path().to_str().unwrap();
        setup_git_repo(tmp.path());

        let migration = Migration::new(workspace, "test-agent", false, true).unwrap();
        assert!(!migration.dry_run);
        assert!(migration.backup_only);
    }

    #[test]
    fn test_execute_source_not_exists_after_new() {
        let tmp = tempfile::TempDir::new().unwrap();
        let workspace = tmp.path().to_str().unwrap();
        setup_git_repo(tmp.path());

        let mut migration = Migration::new(workspace, "test-agent", false, false).unwrap();
        let stats = migration.execute().unwrap();
        assert_eq!(stats.entities_processed, 0);
    }

    #[test]
    fn test_execute_empty_engram_dir() {
        let tmp = tempfile::TempDir::new().unwrap();
        let workspace = tmp.path().to_str().unwrap();
        setup_git_repo(tmp.path());
        setup_engram_dir(tmp.path());

        let mut migration = Migration::new(workspace, "test-agent", false, false).unwrap();
        let stats = migration.execute().unwrap();
        assert_eq!(stats.entities_processed, 0);
        assert_eq!(stats.entities_migrated, 0);
    }

    #[test]
    fn test_execute_backup_only() {
        let tmp = tempfile::TempDir::new().unwrap();
        let workspace = tmp.path().to_str().unwrap();
        setup_git_repo(tmp.path());
        setup_engram_dir(tmp.path());

        let mut migration = Migration::new(workspace, "test-agent", false, true).unwrap();
        let stats = migration.execute().unwrap();
        assert_eq!(stats.entities_processed, 0);
    }

    #[test]
    fn test_execute_dry_run_no_changes() {
        let tmp = tempfile::TempDir::new().unwrap();
        let workspace = tmp.path().to_str().unwrap();
        setup_git_repo(tmp.path());
        setup_engram_dir(tmp.path());

        let task_dir = tmp.path().join(".engram").join("task");
        std::fs::create_dir_all(&task_dir).unwrap();
        std::fs::write(
            task_dir.join("task-1.json"),
            create_valid_memory_entity_json("task-1", "task"),
        )
        .unwrap();

        let mut migration = Migration::new(workspace, "test-agent", true, false).unwrap();
        let stats = migration.execute().unwrap();
        assert_eq!(stats.entities_processed, 1);
        assert_eq!(stats.entities_migrated, 1);
    }

    #[test]
    fn test_execute_migrates_entities() {
        let tmp = tempfile::TempDir::new().unwrap();
        let workspace = tmp.path().to_str().unwrap();
        setup_git_repo(tmp.path());
        setup_engram_dir(tmp.path());

        let task_dir = tmp.path().join(".engram").join("task");
        std::fs::create_dir_all(&task_dir).unwrap();
        std::fs::write(
            task_dir.join("task-1.json"),
            create_valid_memory_entity_json("task-1", "task"),
        )
        .unwrap();
        std::fs::write(
            task_dir.join("task-2.json"),
            create_valid_memory_entity_json("task-2", "task"),
        )
        .unwrap();

        let mut migration = Migration::new(workspace, "test-agent", false, false).unwrap();
        let stats = migration.execute().unwrap();
        assert_eq!(stats.entities_processed, 2);
        assert_eq!(stats.entities_migrated, 2);
        assert_eq!(stats.entities_failed, 0);
        assert!(stats.entity_types.contains_key("task"));
    }

    #[test]
    fn test_execute_skips_hidden_dirs() {
        let tmp = tempfile::TempDir::new().unwrap();
        let workspace = tmp.path().to_str().unwrap();
        setup_git_repo(tmp.path());
        setup_engram_dir(tmp.path());

        let hidden_dir = tmp.path().join(".engram").join(".hidden");
        std::fs::create_dir_all(hidden_dir.join("ab")).unwrap();
        std::fs::write(
            hidden_dir.join("ab").join("entity.json"),
            create_valid_memory_entity_json("hidden-1", "hidden"),
        )
        .unwrap();

        let mut migration = Migration::new(workspace, "test-agent", false, false).unwrap();
        let stats = migration.execute().unwrap();
        assert_eq!(stats.entities_processed, 0);
    }

    #[test]
    fn test_execute_skips_session_dir() {
        let tmp = tempfile::TempDir::new().unwrap();
        let workspace = tmp.path().to_str().unwrap();
        setup_git_repo(tmp.path());
        setup_engram_dir(tmp.path());

        let session_dir = tmp.path().join(".engram").join("session");
        std::fs::create_dir_all(session_dir.join("ab")).unwrap();
        std::fs::write(
            session_dir.join("ab").join("entity.json"),
            create_valid_memory_entity_json("sess-1", "session"),
        )
        .unwrap();

        let mut migration = Migration::new(workspace, "test-agent", false, false).unwrap();
        let stats = migration.execute().unwrap();
        assert_eq!(stats.entities_processed, 0);
    }

    #[test]
    fn test_execute_skips_dirs_without_json() {
        let tmp = tempfile::TempDir::new().unwrap();
        let workspace = tmp.path().to_str().unwrap();
        setup_git_repo(tmp.path());
        setup_engram_dir(tmp.path());

        let notes_dir = tmp.path().join(".engram").join("notes");
        std::fs::create_dir_all(&notes_dir).unwrap();
        std::fs::write(notes_dir.join("readme.txt"), "not json").unwrap();

        let mut migration = Migration::new(workspace, "test-agent", false, false).unwrap();
        let stats = migration.execute().unwrap();
        assert_eq!(stats.entities_processed, 0);
    }

    #[test]
    fn test_execute_skips_non_json_files() {
        let tmp = tempfile::TempDir::new().unwrap();
        let workspace = tmp.path().to_str().unwrap();
        setup_git_repo(tmp.path());
        setup_engram_dir(tmp.path());

        let task_dir = tmp.path().join(".engram").join("task");
        std::fs::create_dir_all(&task_dir).unwrap();
        std::fs::write(
            task_dir.join("task-1.json"),
            create_valid_memory_entity_json("task-1", "task"),
        )
        .unwrap();
        std::fs::write(task_dir.join("notes.txt"), "not a json file").unwrap();

        let mut migration = Migration::new(workspace, "test-agent", false, false).unwrap();
        let stats = migration.execute().unwrap();
        assert_eq!(stats.entities_processed, 1);
    }

    #[test]
    fn test_execute_handles_corrupt_json() {
        let tmp = tempfile::TempDir::new().unwrap();
        let workspace = tmp.path().to_str().unwrap();
        setup_git_repo(tmp.path());
        setup_engram_dir(tmp.path());

        let task_dir = tmp.path().join(".engram").join("task");
        std::fs::create_dir_all(&task_dir).unwrap();
        std::fs::write(
            task_dir.join("good.json"),
            create_valid_memory_entity_json("good-1", "task"),
        )
        .unwrap();
        std::fs::write(task_dir.join("bad.json"), "{invalid json!!!").unwrap();

        let mut migration = Migration::new(workspace, "test-agent", false, false).unwrap();
        let stats = migration.execute().unwrap();
        assert_eq!(stats.entities_processed, 2);
        assert_eq!(stats.entities_migrated, 1);
        assert_eq!(stats.entities_failed, 1);
    }

    #[test]
    fn test_execute_nested_directories() {
        let tmp = tempfile::TempDir::new().unwrap();
        let workspace = tmp.path().to_str().unwrap();
        setup_git_repo(tmp.path());
        setup_engram_dir(tmp.path());

        let task_dir = tmp.path().join(".engram").join("task");
        let nested_dir = task_dir.join("nested");
        std::fs::create_dir_all(&nested_dir).unwrap();
        std::fs::write(
            task_dir.join("task-1.json"),
            create_valid_memory_entity_json("task-1", "task"),
        )
        .unwrap();
        std::fs::write(
            nested_dir.join("task-2.json"),
            create_valid_memory_entity_json("task-2", "task"),
        )
        .unwrap();

        let mut migration = Migration::new(workspace, "test-agent", false, false).unwrap();
        let stats = migration.execute().unwrap();
        assert_eq!(stats.entities_processed, 1);
        assert_eq!(stats.entities_migrated, 1);
    }

    #[test]
    fn test_execute_multiple_entity_types() {
        let tmp = tempfile::TempDir::new().unwrap();
        let workspace = tmp.path().to_str().unwrap();
        setup_git_repo(tmp.path());
        setup_engram_dir(tmp.path());

        let task_dir = tmp.path().join(".engram").join("task");
        let ctx_dir = tmp.path().join(".engram").join("context");
        std::fs::create_dir_all(&task_dir).unwrap();
        std::fs::create_dir_all(&ctx_dir).unwrap();
        std::fs::write(
            task_dir.join("t1.json"),
            create_valid_memory_entity_json("t1", "task"),
        )
        .unwrap();
        std::fs::write(
            ctx_dir.join("c1.json"),
            create_valid_memory_entity_json("c1", "context"),
        )
        .unwrap();

        let mut migration = Migration::new(workspace, "test-agent", false, false).unwrap();
        let stats = migration.execute().unwrap();
        assert_eq!(stats.entities_processed, 2);
        assert_eq!(stats.entities_migrated, 2);
        assert!(stats.entity_types.contains_key("task"));
        assert!(stats.entity_types.contains_key("context"));
    }

    #[test]
    fn test_create_backup() {
        let tmp = tempfile::TempDir::new().unwrap();
        let workspace = tmp.path().to_str().unwrap();
        setup_git_repo(tmp.path());
        setup_engram_dir(tmp.path());

        let task_dir = tmp.path().join(".engram").join("task");
        std::fs::create_dir_all(&task_dir).unwrap();
        std::fs::write(task_dir.join("t1.json"), "data").unwrap();

        let migration = Migration::new(workspace, "test-agent", false, false).unwrap();
        migration.create_backup().unwrap();

        let entries: Vec<_> = std::fs::read_dir(tmp.path())
            .unwrap()
            .filter_map(|e| e.ok())
            .collect();
        let backup_dirs: Vec<_> = entries
            .iter()
            .filter(|e| {
                e.file_name()
                    .to_str()
                    .unwrap_or("")
                    .starts_with(".engram_backup_")
            })
            .collect();
        assert_eq!(backup_dirs.len(), 1);

        let backup_task_dir = backup_dirs[0].path().join("task");
        assert!(backup_task_dir.exists());
        let backup_file = backup_task_dir.join("t1.json");
        assert!(backup_file.exists());
    }

    #[test]
    fn test_create_backup_nested() {
        let tmp = tempfile::TempDir::new().unwrap();
        let workspace = tmp.path().to_str().unwrap();
        setup_git_repo(tmp.path());
        setup_engram_dir(tmp.path());

        let nested = tmp.path().join(".engram").join("a").join("b");
        std::fs::create_dir_all(&nested).unwrap();
        std::fs::write(nested.join("file.json"), "nested data").unwrap();

        let migration = Migration::new(workspace, "test-agent", false, false).unwrap();
        migration.create_backup().unwrap();

        let entries: Vec<_> = std::fs::read_dir(tmp.path())
            .unwrap()
            .filter_map(|e| e.ok())
            .collect();
        let backup = entries
            .iter()
            .find(|e| {
                e.file_name()
                    .to_str()
                    .unwrap_or("")
                    .starts_with(".engram_backup_")
            })
            .unwrap();
        let backup_nested = backup.path().join("a").join("b").join("file.json");
        assert!(backup_nested.exists());
        let content = std::fs::read_to_string(&backup_nested).unwrap();
        assert_eq!(content, "nested data");
    }

    #[test]
    fn test_validate_migration_readiness_no_engram() {
        let tmp = tempfile::TempDir::new().unwrap();
        let workspace = tmp.path().to_str().unwrap();
        setup_git_repo(tmp.path());

        let result = Migration::validate_migration_readiness(workspace);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("No .engram directory found"));
    }

    #[test]
    fn test_validate_migration_readiness_no_git() {
        let tmp = tempfile::TempDir::new().unwrap();
        let workspace = tmp.path().to_str().unwrap();
        std::fs::create_dir_all(tmp.path().join(".engram")).unwrap();

        let result = Migration::validate_migration_readiness(workspace);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("No Git repository found"));
    }

    #[test]
    fn test_validate_migration_readiness_no_engram_git() {
        let tmp = tempfile::TempDir::new().unwrap();
        let workspace = tmp.path().to_str().unwrap();
        setup_git_repo(tmp.path());
        std::fs::create_dir_all(tmp.path().join(".engram")).unwrap();

        let result = Migration::validate_migration_readiness(workspace);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("not a Git repository"));
    }

    #[test]
    fn test_validate_migration_readiness_success() {
        let tmp = tempfile::TempDir::new().unwrap();
        let workspace = tmp.path().to_str().unwrap();
        setup_git_repo(tmp.path());
        setup_engram_dir(tmp.path());

        let result = Migration::validate_migration_readiness(workspace);
        assert!(result.is_ok());
    }

    #[test]
    fn test_migration_stats_default() {
        let stats = MigrationStats::default();
        assert_eq!(stats.entities_processed, 0);
        assert_eq!(stats.entities_migrated, 0);
        assert_eq!(stats.entities_failed, 0);
        assert!(stats.entity_types.is_empty());
    }

    #[test]
    fn test_discover_entity_directories_sorted() {
        let tmp = tempfile::TempDir::new().unwrap();
        let workspace = tmp.path().to_str().unwrap();
        setup_git_repo(tmp.path());
        setup_engram_dir(tmp.path());

        let z_dir = tmp.path().join(".engram").join("zebra");
        let a_dir = tmp.path().join(".engram").join("alpha");
        std::fs::create_dir_all(&z_dir).unwrap();
        std::fs::create_dir_all(&a_dir).unwrap();
        std::fs::write(z_dir.join("z1.json"), "data").unwrap();
        std::fs::write(a_dir.join("a1.json"), "data").unwrap();

        let migration = Migration::new(workspace, "test-agent", false, false).unwrap();
        let dirs = migration.discover_entity_directories().unwrap();
        assert_eq!(dirs.len(), 2);
        assert_eq!(dirs[0].0, "alpha");
        assert_eq!(dirs[1].0, "zebra");
    }

    // ── triple-nesting migration tests ───────────────────────────────────────

    /// Helper: write a raw blob JSON directly into a git ref (bypasses
    /// `store_entity_as_ref` so we can inject a pre-fix nested blob).
    fn write_raw_blob(repo: &gix::Repository, ref_name: &str, json: &str) {
        use gix::refs::transaction::{Change, LogChange, PreviousValue, RefEdit};
        use gix::refs::FullName;
        use gix::refs::Target;

        let blob_id = repo
            .write_object(&gix::objs::Blob {
                data: json.as_bytes().to_vec(),
            })
            .unwrap();
        repo.edit_reference(RefEdit {
            change: Change::Update {
                log: LogChange::default(),
                expected: PreviousValue::Any,
                new: Target::Object(blob_id.detach()),
            },
            name: FullName::try_from(ref_name).unwrap(),
            deref: false,
        })
        .unwrap();
    }

    /// Unit test: round-trip a Task through to_generic / store / get and
    /// assert that the retrieved data map is *flat* (no "data" key nested
    /// inside the "data" field of the stored blob).
    #[test]
    fn test_round_trip_produces_flat_structure() {
        use crate::entities::{Entity, Task, TaskPriority};
        use crate::storage::Storage;

        let tmp = tempfile::TempDir::new().unwrap();
        setup_git_repo(tmp.path());

        let mut storage =
            crate::storage::GitRefsStorage::new(tmp.path().to_str().unwrap(), "test-agent")
                .unwrap();

        let task = Task::new(
            "Flat task".to_string(),
            "Should be flat after round-trip".to_string(),
            "test-agent".to_string(),
            TaskPriority::High,
            None,
        );
        let task_id = task.id.clone();
        let generic = task.to_generic();
        storage.store(&generic).unwrap();

        let retrieved = storage.get(&task_id, "task").unwrap().unwrap();

        // The `data` field of the retrieved GenericEntity must NOT contain a
        // nested "data" key — that would indicate triple-nesting.
        assert!(
            !retrieved.data.get("data").map_or(false, |v| v.is_object()),
            "retrieved data must be flat — found nested 'data' key: {:?}",
            retrieved.data
        );

        // Entity fields must be accessible at the top level of `data`.
        assert_eq!(
            retrieved.data.get("title").and_then(|v| v.as_str()),
            Some("Flat task"),
            "title must be at data.title, not data.data.title"
        );
    }

    /// Test that `migrate_triple_nesting` correctly flattens a
    /// manually-constructed triple-nested blob.
    #[test]
    fn test_migrate_triple_nesting_flattens_nested_blob() {
        let tmp = tempfile::TempDir::new().unwrap();
        setup_git_repo(tmp.path());

        let storage =
            crate::storage::GitRefsStorage::new(tmp.path().to_str().unwrap(), "test-agent")
                .unwrap();
        let repo = gix::open(tmp.path()).unwrap();

        // Write a blob with triple nesting (data.data.data.*).
        // This simulates what was produced by the buggy code path.
        let triple_nested_json = r#"{
            "id": "triple-nested-1",
            "entity_type": "task",
            "agent": "test-agent",
            "timestamp": "2026-01-01T00:00:00Z",
            "data": {
                "data": {
                    "data": {
                        "title": "Triple Nested",
                        "status": "todo"
                    }
                }
            },
            "content_hash": "",
            "size_bytes": 0,
            "tags": [],
            "references": [],
            "metadata": {}
        }"#;
        write_raw_blob(
            &repo,
            "refs/engram/task/triple-nested-1",
            triple_nested_json,
        );

        let report = migrate_triple_nesting(&storage, false).unwrap();
        assert!(report.refs_scanned >= 1);
        assert_eq!(report.nested_found, 1, "should detect one nested ref");
        assert_eq!(report.refs_rewritten, 1, "should rewrite one ref");

        // Verify the blob is now flat.
        let r = repo
            .find_reference("refs/engram/task/triple-nested-1")
            .unwrap();
        let target_id = r.try_id().unwrap();
        let obj = repo.find_object(target_id).unwrap();
        let updated: serde_json::Value = serde_json::from_slice(&obj.data).unwrap();
        let data = updated.get("data").unwrap();

        // No more nesting.
        assert!(
            !data.get("data").map_or(false, |v| v.is_object()),
            "data must no longer contain a nested 'data' object"
        );
        // Entity fields now at top level.
        assert_eq!(
            data.get("title").and_then(|v| v.as_str()),
            Some("Triple Nested"),
            "title must be at data.title after migration"
        );
    }

    /// Test that `migrate_triple_nesting` leaves already-flat blobs unchanged
    /// (idempotency guarantee).
    #[test]
    fn test_migrate_triple_nesting_idempotent_on_flat_blobs() {
        use crate::entities::{Entity, Task, TaskPriority};
        use crate::storage::Storage;

        let tmp = tempfile::TempDir::new().unwrap();
        setup_git_repo(tmp.path());

        let mut storage =
            crate::storage::GitRefsStorage::new(tmp.path().to_str().unwrap(), "test-agent")
                .unwrap();

        // Store a correctly-written flat entity.
        let task = Task::new(
            "Already flat".to_string(),
            "No nesting here".to_string(),
            "test-agent".to_string(),
            TaskPriority::Low,
            None,
        );
        storage.store(&task.to_generic()).unwrap();

        // First pass — should find nothing to fix.
        let report1 = migrate_triple_nesting(&storage, false).unwrap();
        assert_eq!(
            report1.nested_found, 0,
            "already-flat blobs must not be detected as nested"
        );
        assert_eq!(report1.refs_rewritten, 0);

        // Second pass — still nothing.
        let report2 = migrate_triple_nesting(&storage, false).unwrap();
        assert_eq!(report2.nested_found, 0);
        assert_eq!(report2.refs_rewritten, 0);
    }

    /// Test that `migrate_triple_nesting` in dry-run mode detects but does
    /// not repair nested blobs.
    #[test]
    fn test_migrate_triple_nesting_dry_run() {
        let tmp = tempfile::TempDir::new().unwrap();
        setup_git_repo(tmp.path());

        let storage =
            crate::storage::GitRefsStorage::new(tmp.path().to_str().unwrap(), "test-agent")
                .unwrap();
        let repo = gix::open(tmp.path()).unwrap();

        let nested_json = r#"{
            "id": "dry-run-nested",
            "entity_type": "task",
            "agent": "test-agent",
            "timestamp": "2026-01-01T00:00:00Z",
            "data": {
                "data": {
                    "title": "Dry Run"
                }
            },
            "content_hash": "",
            "size_bytes": 0,
            "tags": [],
            "references": [],
            "metadata": {}
        }"#;
        write_raw_blob(&repo, "refs/engram/task/dry-run-nested", nested_json);

        let report = migrate_triple_nesting(&storage, true /* dry_run */).unwrap();
        assert_eq!(report.nested_found, 1);
        assert_eq!(
            report.refs_rewritten, 0,
            "dry-run must not rewrite any refs"
        );

        // Blob must remain nested after dry-run.
        let r = repo
            .find_reference("refs/engram/task/dry-run-nested")
            .unwrap();
        let target_id = r.try_id().unwrap();
        let obj = repo.find_object(target_id).unwrap();
        let still_nested: serde_json::Value = serde_json::from_slice(&obj.data).unwrap();
        assert_eq!(
            still_nested["data"]["data"]["title"],
            serde_json::json!("Dry Run"),
            "blob must remain nested after dry-run"
        );
    }
}

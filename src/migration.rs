//! Migration utilities for converting between storage backends
//!
//! This module provides tools for migrating data from the dual-repository
//! architecture (.engram/ directory) to the Git refs storage architecture.

use crate::error::EngramError;
use crate::storage::{memory_entity::MemoryEntity, GitStorage, Storage};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Migration configuration and state
pub struct Migration {
    source_path: PathBuf,
    target_storage: GitStorage,
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
        let target_storage = GitStorage::new(workspace_path, agent)?;

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
            return Err(EngramError::NotFound(format!(
                "Source migration path does not exist: {}",
                self.source_path.display()
            )));
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

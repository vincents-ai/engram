//! Perkeep CLI commands for backup and restore

use crate::error::EngramError;
use crate::perkeep::{EngramBackupMetadata, PerkeepClient, PerkeepConfig, SchemaObject};
use crate::storage::Storage;
use clap::Subcommand;
use serde_json::Value;

/// Perkeep commands
#[derive(Subcommand)]
pub enum PerkeepCommands {
    /// Backup entities to Perkeep server
    Backup {
        /// Backup all entities or specific type
        #[arg(long)]
        entity_type: Option<String>,

        /// Include entity relationships
        #[arg(long, default_value = "true")]
        include_relationships: bool,

        /// Backup description
        #[arg(long)]
        description: Option<String>,
    },

    /// Restore entities from Perkeep
    Restore {
        /// Backup blob reference to restore
        #[arg(long)]
        blobref: Option<String>,

        /// Restore to specific agent
        #[arg(long, short)]
        agent: Option<String>,

        /// Dry run (don't actually restore)
        #[arg(long)]
        dry_run: bool,
    },

    /// List available backups
    List {
        /// Show detailed information
        #[arg(long)]
        detailed: bool,
    },

    /// Check Perkeep server health
    Health,

    /// Configure Perkeep settings
    Config {
        /// Perkeep server URL
        #[arg(long, short)]
        server: Option<String>,

        /// Authentication token
        #[arg(long)]
        auth_token: Option<String>,

        /// Save to configuration file
        #[arg(long)]
        save: bool,
    },
}

/// Create a Perkeep backup
pub async fn perkeep_backup<S: Storage>(
    storage: &S,
    entity_type: Option<String>,
    include_relationships: bool,
    description: Option<String>,
) -> Result<(), EngramError> {
    let client = PerkeepClient::new(PerkeepConfig::default()).map_err(|e| {
        EngramError::InvalidOperation(format!("Failed to create Perkeep client: {}", e))
    })?;

    // Check server health
    if !client
        .health_check()
        .await
        .map_err(|e| EngramError::InvalidOperation(format!("Perkeep health check failed: {}", e)))?
    {
        return Err(EngramError::InvalidOperation(
            "Perkeep server is not available".to_string(),
        ));
    }

    println!("🔐 Connecting to Perkeep server...");
    println!("   Server: {}", client.server_url());

    // Query entities to backup
    let entity_types = match &entity_type {
        Some(t) => vec![t.clone()],
        None => vec![
            "task".to_string(),
            "context".to_string(),
            "reasoning".to_string(),
            "knowledge".to_string(),
            "session".to_string(),
        ],
    };

    let mut entity_blob_refs: std::collections::HashMap<String, String> =
        std::collections::HashMap::new();
    let mut total_size = 0u64;
    let mut entity_count = 0usize;

    println!("\n📦 Backing up entities...");

    for et in &entity_types {
        println!("   Processing {}...", et);

        let ids = storage.list_ids(et)?;

        for id in &ids {
            if let Ok(Some(entity)) = storage.get(&id, et) {
                let blob_data = serde_json::to_vec(&entity).map_err(|e| {
                    EngramError::InvalidOperation(format!("Failed to serialize entity: {}", e))
                })?;

                let blobref = client.upload_blob(&blob_data).await.map_err(|e| {
                    EngramError::InvalidOperation(format!("Failed to upload {} {}: {}", et, id, e))
                })?;

                entity_blob_refs.insert(format!("{}/{}", et, id), blobref.blobref.clone());
                total_size += blobref.size;
                entity_count += 1;
            }
        }

        println!("      ✓ {} entities", ids.len());
    }

    // Include relationships if requested
    if include_relationships {
        println!("   Processing relationships...");

        let rel_ids = storage.list_ids("relationship")?;

        for id in &rel_ids {
            if let Ok(Some(entity)) = storage.get(&id, "relationship") {
                let blob_data = serde_json::to_vec(&entity).map_err(|e| {
                    EngramError::InvalidOperation(format!(
                        "Failed to serialize relationship: {}",
                        e
                    ))
                })?;

                let blobref = client.upload_blob(&blob_data).await.map_err(|e| {
                    EngramError::InvalidOperation(format!(
                        "Failed to upload relationship {}: {}",
                        id, e
                    ))
                })?;

                entity_blob_refs.insert(format!("relationship/{}", id), blobref.blobref.clone());
                total_size += blobref.size;
            }
        }

        println!("      ✓ {} relationships", rel_ids.len());
    }

    // Create backup metadata
    let metadata = EngramBackupMetadata::new(
        entity_count,
        entity_types.clone(),
        entity_blob_refs,
        total_size,
        "default".to_string(),
    );

    // Upload metadata
    let metadata_schema = SchemaObject {
        camli_type: "engram.net/backup".to_string(),
        base_value_ref: None,
        file_name: Some(format!(
            "engram-backup-{}.json",
            chrono::Utc::now().format("%Y%m%d-%H%M%S")
        )),
        mime_type: Some("application/json".to_string()),
        size: Some(serde_json::to_vec(&metadata).unwrap().len() as u64),
        title: description.or(Some("Engram Backup".to_string())),
        description: Some(format!(
            "Engram backup containing {} entities",
            entity_count
        )),
        creation_time: Some(chrono::Utc::now().to_rfc3339()),
        custom_attributes: Some(
            serde_json::to_value(metadata.clone())
                .unwrap()
                .as_object()
                .unwrap()
                .clone()
                .into_iter()
                .collect(),
        ),
    };

    let metadata_blobref = client.upload_schema(&metadata_schema).await.map_err(|e| {
        EngramError::InvalidOperation(format!("Failed to upload backup metadata: {}", e))
    })?;

    println!("\n✅ Backup complete!");
    println!("   Entities backed up: {}", entity_count);
    println!("   Total size: {} bytes", total_size);
    println!("   Metadata blobref: {}", metadata_blobref.blobref);
    println!("\n💡 Use this blobref to restore later:");
    println!(
        "   engram perkeep restore --blobref {}",
        metadata_blobref.blobref
    );

    Ok(())
}

/// Restore entities from Perkeep
pub async fn perkeep_restore<S: Storage>(
    storage: &mut S,
    blobref: Option<String>,
    agent: Option<String>,
    dry_run: bool,
) -> Result<(), EngramError> {
    let client = PerkeepClient::new(PerkeepConfig::default()).map_err(|e| {
        EngramError::InvalidOperation(format!("Failed to create Perkeep client: {}", e))
    })?;

    // Check server health
    if !client
        .health_check()
        .await
        .map_err(|e| EngramError::InvalidOperation(format!("Perkeep health check failed: {}", e)))?
    {
        return Err(EngramError::InvalidOperation(
            "Perkeep server is not available".to_string(),
        ));
    }

    // Get backup blobref
    let blobref = match blobref {
        Some(ref b) => b.clone(),
        None => {
            // Find the most recent backup
            let matches = client
                .search_blobs("camliType:engram.net/backup")
                .await
                .map_err(|e| {
                    EngramError::InvalidOperation(format!("Failed to search backups: {}", e))
                })?;

            if matches.is_empty() {
                return Err(EngramError::NotFound(
                    "No backups found in Perkeep".to_string(),
                ));
            }

            matches[0].blobref.clone()
        }
    };

    println!("🔐 Restoring from Perkeep...");
    println!("   Backup blobref: {}", blobref);

    if dry_run {
        println!("\n�Dry run mode - no changes will be made");
    }

    // Fetch backup metadata
    let backup_data = client
        .fetch_blob(&blobref)
        .await
        .map_err(|e| EngramError::InvalidOperation(format!("Failed to fetch backup: {}", e)))?;

    let backup_data = match backup_data {
        Some(data) => data,
        None => {
            return Err(EngramError::NotFound(format!(
                "Backup blob not found: {}",
                blobref
            )));
        }
    };

    let metadata: EngramBackupMetadata = serde_json::from_slice(&backup_data).map_err(|e| {
        EngramError::InvalidOperation(format!("Failed to parse backup metadata: {}", e))
    })?;

    println!("\n📋 Backup Information:");
    println!("   Version: {}", metadata.version);
    println!("   Created: {}", metadata.timestamp);
    println!("   Entities: {}", metadata.entity_count);
    println!("   Total size: {} bytes", metadata.total_size);

    if dry_run {
        println!("\n🪧 Would restore {} entities:", metadata.entity_count);
        for (key, _) in metadata.entity_blob_refs.iter().take(10) {
            println!("   - {}", key);
        }
        if metadata.entity_blob_refs.len() > 10 {
            println!("   ... and {} more", metadata.entity_blob_refs.len() - 10);
        }
        return Ok(());
    }

    // Restore entities
    println!("\n📦 Restoring entities...");

    let mut restored_count = 0usize;

    for (entity_key, blobref) in &metadata.entity_blob_refs {
        if let Some(data) = client.fetch_blob(blobref).await.map_err(|e| {
            EngramError::InvalidOperation(format!("Failed to fetch {}: {}", entity_key, e))
        })? {
            let entity: Value = serde_json::from_slice(&data).map_err(|e| {
                EngramError::InvalidOperation(format!(
                    "Failed to deserialize {}: {}",
                    entity_key, e
                ))
            })?;

            let parts: Vec<&str> = entity_key.split('/').collect();
            if parts.len() >= 2 {
                let _entity_type = parts[0];
                let _entity_id = parts[1];

                let mut entity_obj = entity.as_object().unwrap().clone();
                if let Some(agent_name) = &agent {
                    entity_obj.insert("agent".to_string(), Value::String(agent_name.clone()));
                }

                let modified_entity = Value::Object(entity_obj);

                let entity = match crate::entities::GenericEntity::from_value(modified_entity) {
                    Ok(e) => e,
                    Err(e) => {
                        return Err(EngramError::InvalidOperation(format!(
                            "Failed to deserialize {}: {}",
                            entity_key, e
                        )));
                    }
                };

                storage.store(&entity).map_err(|e| {
                    EngramError::InvalidOperation(format!("Failed to store {}: {}", entity_key, e))
                })?;

                restored_count += 1;

                if restored_count % 10 == 0 {
                    println!("   Restored {} entities...", restored_count);
                }
            }
        }
    }

    println!("\n✅ Restore complete!");
    println!("   Entities restored: {}", restored_count);

    Ok(())
}

/// List available backups
pub async fn perkeep_list(detailed: bool) -> Result<(), EngramError> {
    let client = PerkeepClient::new(PerkeepConfig::default()).map_err(|e| {
        EngramError::InvalidOperation(format!("Failed to create Perkeep client: {}", e))
    })?;

    // Check server health
    if !client
        .health_check()
        .await
        .map_err(|e| EngramError::InvalidOperation(format!("Perkeep health check failed: {}", e)))?
    {
        return Err(EngramError::InvalidOperation(
            "Perkeep server is not available".to_string(),
        ));
    }

    // Search for backups
    let backups = client
        .search_blobs("camliType:engram.net/backup")
        .await
        .map_err(|e| EngramError::InvalidOperation(format!("Failed to search backups: {}", e)))?;

    if backups.is_empty() {
        println!("\n📭 No backups found in Perkeep.");
        return Ok(());
    }

    println!("\n📦 Available Backups:");
    println!("====================");

    for (i, backup) in backups.iter().enumerate() {
        println!("{}. {}", i + 1, backup.blobref);

        if detailed {
            if let Some(data) = client.fetch_blob(&backup.blobref).await.ok().flatten() {
                if let Ok(metadata) = serde_json::from_slice::<EngramBackupMetadata>(&data) {
                    println!("   Created: {}", metadata.timestamp);
                    println!("   Entities: {}", metadata.entity_count);
                    println!("   Size: {} bytes", metadata.total_size);
                }
            }
        }
    }

    Ok(())
}

/// Check Perkeep server health
pub async fn perkeep_health() -> Result<(), EngramError> {
    let client = PerkeepClient::new(PerkeepConfig::default()).map_err(|e| {
        EngramError::InvalidOperation(format!("Failed to create Perkeep client: {}", e))
    })?;

    let healthy = client
        .health_check()
        .await
        .map_err(|e| EngramError::InvalidOperation(format!("Health check failed: {}", e)))?;

    if healthy {
        println!("✅ Perkeep server is healthy");
        println!("   Server: {}", client.server_url());
    } else {
        return Err(EngramError::InvalidOperation(
            "Perkeep server is not responding".to_string(),
        ));
    }

    Ok(())
}

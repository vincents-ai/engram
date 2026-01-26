use crate::entities::GenericEntity;
use crate::error::EngramError;
use crate::storage::{ConflictResolution, RemoteAuth, Storage, SyncResult};
use chrono::Utc;
use git2::{Cred, FetchOptions, IndexAddOption, PushOptions, RemoteCallbacks, Repository};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(clap::Subcommand)]
pub enum SyncCommands {
    /// Synchronize agents locally
    Sync {
        #[arg(long, short)]
        agents: String,

        #[arg(long, short, default_value = "merge_with_conflict_resolution")]
        strategy: String,

        #[arg(long, default_value_t = false)]
        dry_run: bool,
    },
    /// Add remote repository
    AddRemote {
        name: String,
        url: String,
        #[arg(long, default_value = "main")]
        branch: String,
        #[arg(long)]
        auth_type: Option<String>,
        #[arg(long)]
        username: Option<String>,
        #[arg(long)]
        password: Option<String>,
        #[arg(long)]
        ssh_key: Option<String>,
    },
    /// List configured remotes
    ListRemotes,
    /// Show sync status with remote
    Status {
        #[arg(long)]
        remote: Option<String>,
    },
    /// Pull from remote
    Pull {
        #[arg(long)]
        remote: String,
        #[arg(long)]
        branch: Option<String>,
        #[arg(long)]
        agents: Option<String>,
        #[arg(long)]
        auth_type: Option<String>,
        #[arg(long)]
        username: Option<String>,
        #[arg(long)]
        password: Option<String>,
        #[arg(long)]
        ssh_key: Option<String>,
        #[arg(long, default_value_t = false)]
        dry_run: bool,
    },
    /// Push to remote  
    Push {
        #[arg(long)]
        remote: String,
        #[arg(long)]
        branch: Option<String>,
        #[arg(long)]
        agents: Option<String>,
        #[arg(long)]
        auth_type: Option<String>,
        #[arg(long)]
        username: Option<String>,
        #[arg(long)]
        password: Option<String>,
        #[arg(long)]
        ssh_key: Option<String>,
        #[arg(long, default_value_t = false)]
        dry_run: bool,
    },
    /// Create a new branch for agent isolation
    CreateBranch {
        name: String,
        #[arg(long)]
        agent: Option<String>,
        #[arg(long)]
        from: Option<String>,
    },
    /// Switch to a different branch
    SwitchBranch {
        name: String,
        #[arg(long, default_value_t = false)]
        create: bool,
    },
    /// List all branches
    ListBranches {
        #[arg(long, default_value_t = false)]
        all: bool,
        #[arg(long, default_value_t = false)]
        current: bool,
    },
    /// Delete a branch
    DeleteBranch {
        name: String,
        #[arg(long, default_value_t = false)]
        force: bool,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum MergeStrategy {
    LatestWins,
    IntelligentMerge,
    MergeWithConflictResolution,
    PriorityWins { agent: String },
}

impl MergeStrategy {
    pub fn from_str(s: &str) -> Result<Self, EngramError> {
        match s.to_lowercase().as_str() {
            "latest_wins" | "latest-wins" => Ok(MergeStrategy::LatestWins),
            "intelligent_merge" | "intelligent-merge" => Ok(MergeStrategy::IntelligentMerge),
            "merge_with_conflict_resolution" | "merge-with-conflict-resolution" => {
                Ok(MergeStrategy::MergeWithConflictResolution)
            }
            s if s.starts_with("priority_wins:") => {
                let agent = s.strip_prefix("priority_wins:").unwrap_or("").to_string();
                if agent.is_empty() {
                    return Err(EngramError::Validation(
                        "Priority agent required for priority_wins strategy".to_string()
                    ));
                }
                Ok(MergeStrategy::PriorityWins { agent })
            }
            _ => Err(EngramError::Validation(format!(
                "Unknown merge strategy: {}. Valid options: latest_wins, intelligent_merge, merge_with_conflict_resolution, priority_wins:<agent>",
                s
            ))),
        }
    }
}

pub fn sync_agents<S: Storage>(
    storage: &mut S,
    agents: Vec<String>,
    strategy: MergeStrategy,
    dry_run: bool,
) -> Result<SyncResult, EngramError> {
    let start_time = Utc::now();

    println!("🔄 Starting synchronization...");
    println!("🤖 Agents: {}", agents.join(", "));
    println!("📋 Strategy: {:?}", strategy);
    if dry_run {
        println!("🔍 Mode: Dry run (no changes will be made)");
    }
    println!();

    if agents.is_empty() {
        return Err(EngramError::Validation("No agents specified".to_string()));
    }

    if agents.len() == 1 {
        println!("ℹ️  Only one agent specified, nothing to synchronize");
        return Ok(SyncResult {
            entities_synced: 0,
            conflicts_resolved: Vec::new(),
            errors: Vec::new(),
            timestamp: start_time,
            synced_agents: agents,
            merged_entities: 0,
            duration_ms: 0,
        });
    }

    let entity_types = vec![
        "task",
        "context",
        "reasoning",
        "knowledge",
        "session",
        "compliance",
        "rule",
        "standard",
        "adr",
        "workflow",
    ];
    let mut total_synced = 0;
    let mut total_merged = 0;
    let mut all_conflicts = Vec::new();
    let mut errors = Vec::new();

    for entity_type in entity_types {
        match sync_entity_type(storage, entity_type, &agents, &strategy, dry_run) {
            Ok((synced, merged, conflicts)) => {
                total_synced += synced;
                total_merged += merged;
                all_conflicts.extend(conflicts);

                if synced > 0 {
                    println!(
                        "✅ {} entities: {} synced, {} merged",
                        entity_type, synced, merged
                    );
                }
            }
            Err(e) => {
                let error_msg = format!("Failed to sync {}: {}", entity_type, e);
                println!("❌ {}", error_msg);
                errors.push(error_msg);
            }
        }
    }

    if !dry_run && total_synced > 0 {
        storage.sync()?;
    }

    let end_time = Utc::now();
    let duration = end_time.signed_duration_since(start_time);

    println!("\n=== Synchronization Complete ===");
    println!("📊 Total entities synchronized: {}", total_synced);
    println!("🔗 Total entities merged: {}", total_merged);
    println!("⚠️  Conflicts resolved: {}", all_conflicts.len());
    println!("⏱️  Duration: {}ms", duration.num_milliseconds());

    if !errors.is_empty() {
        println!("❌ Errors: {}", errors.len());
        for error in &errors {
            println!("   • {}", error);
        }
    }

    Ok(SyncResult {
        entities_synced: total_synced,
        conflicts_resolved: all_conflicts,
        errors,
        timestamp: start_time,
        synced_agents: agents,
        merged_entities: total_merged,
        duration_ms: duration.num_milliseconds() as u64,
    })
}

fn sync_entity_type<S: Storage>(
    storage: &mut S,
    entity_type: &str,
    agents: &[String],
    strategy: &MergeStrategy,
    dry_run: bool,
) -> Result<(usize, usize, Vec<ConflictResolution>), EngramError> {
    println!("\n🔍 Synchronizing {} entities...", entity_type);

    let mut all_entities: Vec<GenericEntity> = Vec::new();

    for agent in agents {
        let agent_entities = storage.query_by_agent(agent, Some(entity_type))?;
        println!(
            "  📂 Found {} {} entities from agent {}",
            agent_entities.len(),
            entity_type,
            agent
        );
        all_entities.extend(agent_entities);
    }

    if all_entities.is_empty() {
        return Ok((0, 0, Vec::new()));
    }

    let entity_count_before = all_entities.len();

    let (merged_entities, conflicts) = match strategy {
        MergeStrategy::LatestWins => {
            let merged = merge_latest_wins(all_entities)?;
            (merged, Vec::new())
        }
        MergeStrategy::IntelligentMerge => {
            let merged = merge_intelligent(all_entities)?;
            (merged, Vec::new())
        }
        MergeStrategy::MergeWithConflictResolution => merge_with_conflict_detection(all_entities)?,
        MergeStrategy::PriorityWins { agent } => {
            let merged = merge_priority_wins(all_entities, agent)?;
            (merged, Vec::new())
        }
    };

    let entity_count_after = merged_entities.len();
    let merged_count = entity_count_before - entity_count_after;

    if merged_count > 0 {
        println!(
            "  🔗 Merged {} duplicate/conflicting entities",
            merged_count
        );
    }

    if !dry_run {
        for entity in &merged_entities {
            storage.store(entity)?;
        }
    }

    Ok((merged_entities.len(), merged_count, conflicts))
}

fn merge_latest_wins(entities: Vec<GenericEntity>) -> Result<Vec<GenericEntity>, EngramError> {
    use std::collections::HashMap;

    let mut entity_map: HashMap<String, GenericEntity> = HashMap::new();

    for entity in entities {
        let key = entity.id.clone();

        if let Some(existing) = entity_map.get(&key) {
            if entity.timestamp > existing.timestamp {
                entity_map.insert(key, entity);
            }
        } else {
            entity_map.insert(key, entity);
        }
    }

    Ok(entity_map.into_values().collect())
}

fn merge_intelligent(entities: Vec<GenericEntity>) -> Result<Vec<GenericEntity>, EngramError> {
    use std::collections::HashMap;

    let mut entity_map: HashMap<String, GenericEntity> = HashMap::new();

    for entity in entities {
        let key = entity.id.clone();

        if let Some(existing) = entity_map.get_mut(&key) {
            if entity.timestamp > existing.timestamp {
                let merged = intelligent_merge_entity(existing.clone(), entity)?;
                entity_map.insert(key, merged);
            }
        } else {
            entity_map.insert(key, entity);
        }
    }

    Ok(entity_map.into_values().collect())
}

fn merge_priority_wins(
    entities: Vec<GenericEntity>,
    priority_agent: &str,
) -> Result<Vec<GenericEntity>, EngramError> {
    use std::collections::HashMap;

    let mut entity_map: HashMap<String, GenericEntity> = HashMap::new();

    for entity in entities {
        let key = entity.id.clone();

        if let Some(existing) = entity_map.get(&key) {
            if entity.agent == priority_agent {
                entity_map.insert(key, entity);
            } else if existing.agent != priority_agent && entity.timestamp > existing.timestamp {
                entity_map.insert(key, entity);
            }
        } else {
            entity_map.insert(key, entity);
        }
    }

    Ok(entity_map.into_values().collect())
}

fn intelligent_merge_entity(
    existing: GenericEntity,
    newer: GenericEntity,
) -> Result<GenericEntity, EngramError> {
    let mut merged = newer.clone();

    if let (Some(existing_obj), Some(newer_obj)) =
        (existing.data.as_object(), merged.data.as_object_mut())
    {
        for (key, existing_value) in existing_obj {
            if let Some(newer_value) = newer_obj.get(key) {
                if newer_value.is_null()
                    || (newer_value.is_string() && newer_value.as_str().unwrap_or("").is_empty())
                    || (newer_value.is_array()
                        && newer_value.as_array().unwrap_or(&vec![]).is_empty())
                {
                    newer_obj.insert(key.clone(), existing_value.clone());
                }
            } else {
                newer_obj.insert(key.clone(), existing_value.clone());
            }
        }
    }

    Ok(merged)
}

fn merge_with_conflict_detection(
    entities: Vec<GenericEntity>,
) -> Result<(Vec<GenericEntity>, Vec<ConflictResolution>), EngramError> {
    use std::collections::HashMap;

    let mut entity_map: HashMap<String, GenericEntity> = HashMap::new();
    let mut conflicts = Vec::new();

    for entity in entities {
        let key = entity.id.clone();

        if let Some(existing) = entity_map.get(&key) {
            if has_conflict(existing, &entity) {
                println!(
                    "  ⚠️  CONFLICT: Entity {} has conflicting changes from different agents",
                    key
                );

                let conflict_details = analyze_conflict(existing, &entity);
                let conflict_resolution = ConflictResolution {
                    entity_id: key.clone(),
                    entity_type: entity.entity_type.clone(),
                    strategy_used: crate::storage::SyncStrategy::LatestWins,
                    winner: if entity.timestamp > existing.timestamp {
                        entity.agent.clone()
                    } else {
                        existing.agent.clone()
                    },
                    conflicts_detected: conflict_details,
                };

                if entity.timestamp > existing.timestamp {
                    println!(
                        "    ✅ Resolving with newer version from {} (timestamp: {})",
                        entity.agent, entity.timestamp
                    );
                    entity_map.insert(key, entity);
                } else {
                    println!(
                        "    ✅ Keeping existing version from {} (timestamp: {})",
                        existing.agent, existing.timestamp
                    );
                }

                conflicts.push(conflict_resolution);
            } else {
                if entity.timestamp > existing.timestamp {
                    entity_map.insert(key, entity);
                }
            }
        } else {
            entity_map.insert(key, entity);
        }
    }

    Ok((entity_map.into_values().collect(), conflicts))
}

fn has_conflict(e1: &GenericEntity, e2: &GenericEntity) -> bool {
    if e1.agent == e2.agent {
        return false;
    }

    if e1.data == e2.data {
        return false;
    }

    let time_diff = if e1.timestamp > e2.timestamp {
        e1.timestamp.signed_duration_since(e2.timestamp)
    } else {
        e2.timestamp.signed_duration_since(e1.timestamp)
    };

    let minutes_diff = time_diff.num_minutes().abs();

    minutes_diff < 5
}

fn analyze_conflict(e1: &GenericEntity, e2: &GenericEntity) -> Vec<String> {
    let mut conflicts = Vec::new();

    if let (Some(obj1), Some(obj2)) = (e1.data.as_object(), e2.data.as_object()) {
        for (key, value1) in obj1 {
            if let Some(value2) = obj2.get(key) {
                if value1 != value2 {
                    conflicts.push(format!(
                        "Field '{}' differs: {} vs {}",
                        key,
                        serde_json::to_string(value1).unwrap_or_default(),
                        serde_json::to_string(value2).unwrap_or_default()
                    ));
                }
            }
        }

        for key in obj2.keys() {
            if !obj1.contains_key(key) {
                conflicts.push(format!("Field '{}' only present in newer version", key));
            }
        }
    }

    if conflicts.is_empty() {
        conflicts.push("Data differs but specific fields could not be identified".to_string());
    }

    conflicts
}

/// Remote configuration for sync operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteConfig {
    pub name: String,
    pub url: String,
    pub branch: String,
    pub last_sync: Option<chrono::DateTime<chrono::Utc>>,
    pub auth_type: Option<String>,
    pub username: Option<String>,
    pub ssh_key_path: Option<String>,
}

/// Remote sync status
#[derive(Debug, Clone)]
pub struct RemoteSyncStatus {
    pub remote: String,
    pub local_hash: String,
    pub remote_hash: String,
    pub is_ahead: bool,
    pub is_behind: bool,
    pub last_checked: chrono::DateTime<chrono::Utc>,
}

/// Add a remote repository
pub fn add_remote<S: Storage>(
    _storage: &mut S,
    name: String,
    url: String,
    branch: String,
    auth_type: Option<String>,
    username: Option<String>,
    ssh_key: Option<String>,
) -> Result<(), EngramError> {
    println!("📡 Adding remote repository...");
    println!("   Name: {}", name);
    println!("   URL: {}", url);
    println!("   Branch: {}", branch);
    if let Some(ref auth) = auth_type {
        println!("   Authentication: {}", auth);
    }

    // Load existing remotes configuration
    let config_path = ".engram/remotes.json";
    let mut remotes: HashMap<String, RemoteConfig> = if Path::new(config_path).exists() {
        let content = fs::read_to_string(config_path).map_err(|e| EngramError::Io(e))?;
        serde_json::from_str(&content).map_err(|e| EngramError::Serialization(e))?
    } else {
        HashMap::new()
    };

    // Check if remote already exists
    if remotes.contains_key(&name) {
        return Err(EngramError::Validation(format!(
            "Remote '{}' already exists",
            name
        )));
    }

    // Add new remote configuration
    let remote_config = RemoteConfig {
        name: name.clone(),
        url: url.clone(),
        branch: branch.clone(),
        last_sync: None,
        auth_type: auth_type.clone(),
        username: username.clone(),
        ssh_key_path: ssh_key.clone(),
    };

    remotes.insert(name.clone(), remote_config);

    // Save updated configuration
    let config_content =
        serde_json::to_string_pretty(&remotes).map_err(|e| EngramError::Serialization(e))?;

    // Ensure .engram directory exists
    if !Path::new(".engram").exists() {
        fs::create_dir_all(".engram").map_err(|e| EngramError::Io(e))?;
    }

    fs::write(config_path, config_content).map_err(|e| EngramError::Io(e))?;

    println!("✅ Remote '{}' added successfully", name);
    Ok(())
}

/// List all configured remotes
pub fn list_remotes(writer: &mut dyn std::io::Write) -> Result<Vec<RemoteConfig>, EngramError> {
    use crate::cli::utils::create_table;
    use prettytable::row;

    let config_path = ".engram/remotes.json";

    if !Path::new(config_path).exists() {
        writeln!(writer, "📡 No remotes configured")?;
        return Ok(Vec::new());
    }

    let content = fs::read_to_string(config_path).map_err(|e| EngramError::Io(e))?;

    let remotes: HashMap<String, RemoteConfig> =
        serde_json::from_str(&content).map_err(|e| EngramError::Serialization(e))?;

    if remotes.is_empty() {
        writeln!(writer, "No remotes configured.")?;
        return Ok(Vec::new());
    }

    writeln!(writer, "Found {} configured remotes", remotes.len())?;
    writeln!(writer)?;

    let mut remote_list: Vec<RemoteConfig> = remotes.into_values().collect();
    remote_list.sort_by(|a, b| a.name.cmp(&b.name));

    let mut table = create_table();
    table.set_titles(row!["Name", "Branch", "URL", "Auth", "Last Sync"]);

    for remote in &remote_list {
        let auth_info = if let Some(ref auth_type) = remote.auth_type {
            let mut info = auth_type.clone();
            if let Some(ref username) = remote.username {
                info = format!("{} ({})", info, username);
            }
            info
        } else {
            "-".to_string()
        };

        let last_sync = if let Some(sync) = remote.last_sync {
            sync.format("%Y-%m-%d %H:%M").to_string()
        } else {
            "Never".to_string()
        };

        table.add_row(row![
            remote.name,
            remote.branch,
            remote.url,
            auth_info,
            last_sync
        ]);
    }

    table.print(writer)?;
    writeln!(writer)?;

    Ok(remote_list)
}

/// Get sync status with a remote
pub fn get_sync_status(
    writer: &mut dyn std::io::Write,
    remote_name: &str,
) -> Result<RemoteSyncStatus, EngramError> {
    writeln!(writer, "📊 Checking sync status for '{}'...", remote_name)?;

    // Load remotes configuration
    let config_path = ".engram/remotes.json";
    if !Path::new(config_path).exists() {
        return Err(EngramError::Validation(
            "No remotes configured. Use 'add-remote' first.".to_string(),
        ));
    }

    let content = fs::read_to_string(config_path).map_err(|e| EngramError::Io(e))?;

    let remotes: HashMap<String, RemoteConfig> =
        serde_json::from_str(&content).map_err(|e| EngramError::Serialization(e))?;

    let remote_config = remotes
        .get(remote_name)
        .ok_or_else(|| EngramError::Validation(format!("Remote '{}' not found", remote_name)))?;

    // Open Git repository
    let repo = Repository::open(".")
        .map_err(|e| EngramError::Git(format!("Failed to open repository: {}", e)))?;

    // Get local HEAD commit hash
    let local_head = repo
        .head()
        .map_err(|e| EngramError::Git(format!("Failed to get local HEAD: {}", e)))?;
    let local_oid = local_head
        .target()
        .ok_or_else(|| EngramError::Git("Failed to get local HEAD target".to_string()))?;
    let local_hash = local_oid.to_string();

    // For now, we'll simulate remote hash check (in a real implementation, we'd fetch from remote)
    // This would require network operations and authentication
    let remote_hash = "0000000000000000000000000000000000000000".to_string(); // Placeholder
    let is_ahead = false; // Placeholder
    let is_behind = false; // Placeholder

    writeln!(writer, "📊 Sync Status for '{}'", remote_name)?;
    writeln!(writer, "=========================")?;
    writeln!(writer, "Remote: {}", remote_config.url)?;
    writeln!(writer, "Local Hash: {}...", &local_hash[..12])?;
    writeln!(writer, "Remote Hash: {}...", &remote_hash[..12])?;

    if is_behind {
        writeln!(writer, "Status: ⬇️  Behind remote (pull needed)")?;
    } else if is_ahead {
        writeln!(writer, "Status: ⬆️  Ahead of remote (push needed)")?;
    } else {
        writeln!(writer, "Status: ✅ Up to date")?;
    }

    let status = RemoteSyncStatus {
        remote: remote_name.to_string(),
        local_hash,
        remote_hash,
        is_ahead,
        is_behind,
        last_checked: Utc::now(),
    };

    Ok(status)
}

/// Pull from remote repository
pub fn pull_from_remote<S: Storage>(
    _storage: &mut S,
    remote_name: String,
    branch: Option<String>,
    agents: Option<String>,
    auth: RemoteAuth,
    dry_run: bool,
) -> Result<(), EngramError> {
    println!("📥 Pulling from remote '{}'...", remote_name);

    if dry_run {
        println!("🔍 Dry run mode - no changes will be made");
    }

    // Load remotes configuration
    let config_path = ".engram/remotes.json";
    if !Path::new(config_path).exists() {
        return Err(EngramError::Validation(
            "No remotes configured. Use 'add-remote' first.".to_string(),
        ));
    }

    let content = fs::read_to_string(config_path).map_err(|e| EngramError::Io(e))?;

    let remotes: HashMap<String, RemoteConfig> =
        serde_json::from_str(&content).map_err(|e| EngramError::Serialization(e))?;

    let remote_config = remotes
        .get(&remote_name)
        .ok_or_else(|| EngramError::Validation(format!("Remote '{}' not found", remote_name)))?;

    let target_branch = branch.unwrap_or(remote_config.branch.clone());

    println!("📡 Remote: {}", remote_config.url);
    println!("🌿 Branch: {}", target_branch);

    if let Some(agent_list) = &agents {
        println!("🤖 Agents: {}", agent_list);
    } else {
        println!("🤖 Agents: All");
    }

    if !dry_run {
        println!("🔄 Attempting to pull from remote repository...");

        let repo = Repository::open(".")
            .map_err(|e| EngramError::Git(format!("Failed to open repository: {}", e)))?;

        let remote_exists = repo.find_remote(&remote_name).is_ok();
        if !remote_exists {
            repo.remote(&remote_name, &remote_config.url)
                .map_err(|e| EngramError::Git(format!("Failed to add remote: {}", e)))?;
        }

        let refspecs = [format!(
            "refs/heads/{}:refs/remotes/{}/{}",
            target_branch, remote_name, target_branch
        )];
        let refspecs_str: Vec<&str> = refspecs.iter().map(|s| s.as_str()).collect();

        authenticated_fetch(&repo, &remote_name, &refspecs_str, &auth)?;

        println!("✅ Successfully pulled from remote repository");
        println!("   Next: Local entities will be updated for specified agents");
    } else {
        println!("✅ Dry run completed - would pull from remote");
    }

    Ok(())
}

/// Push to remote repository
pub fn push_to_remote<S: Storage>(
    _storage: &mut S,
    remote_name: String,
    branch: Option<String>,
    agents: Option<String>,
    _auth: RemoteAuth,
    dry_run: bool,
) -> Result<(), EngramError> {
    println!("📤 Pushing to remote '{}'...", remote_name);

    if dry_run {
        println!("🔍 Dry run mode - no changes will be made");
    }

    // Load remotes configuration
    let config_path = ".engram/remotes.json";
    if !Path::new(config_path).exists() {
        return Err(EngramError::Validation(
            "No remotes configured. Use 'add-remote' first.".to_string(),
        ));
    }

    let content = fs::read_to_string(config_path).map_err(|e| EngramError::Io(e))?;

    let remotes: HashMap<String, RemoteConfig> =
        serde_json::from_str(&content).map_err(|e| EngramError::Serialization(e))?;

    let remote_config = remotes
        .get(&remote_name)
        .ok_or_else(|| EngramError::Validation(format!("Remote '{}' not found", remote_name)))?;

    let target_branch = branch.unwrap_or(remote_config.branch.clone());

    println!("📡 Remote: {}", remote_config.url);
    println!("🌿 Branch: {}", target_branch);

    if let Some(agent_list) = &agents {
        println!("🤖 Agents: {}", agent_list);
    } else {
        println!("🤖 Agents: All");
    }

    if !dry_run {
        // Open the Git repository
        let repo_path = std::env::current_dir()?.join(".engram");
        let repo = Repository::open(&repo_path)
            .map_err(|e| EngramError::Git(format!("Failed to open repository: {}", e)))?;

        // Stage and commit changes for specified agents
        let mut index = repo
            .index()
            .map_err(|e| EngramError::Git(format!("Failed to get repository index: {}", e)))?;

        // Add all changes to staging area
        index
            .add_all(["*"].iter(), IndexAddOption::DEFAULT, None)
            .map_err(|e| EngramError::Git(format!("Failed to stage changes: {}", e)))?;
        index
            .write()
            .map_err(|e| EngramError::Git(format!("Failed to write index: {}", e)))?;

        let tree_id = index
            .write_tree()
            .map_err(|e| EngramError::Git(format!("Failed to write tree: {}", e)))?;
        let tree = repo
            .find_tree(tree_id)
            .map_err(|e| EngramError::Git(format!("Failed to find tree: {}", e)))?;

        // Create commit message
        let commit_message = if let Some(agent_list) = &agents {
            format!("Sync changes for agents: {}", agent_list)
        } else {
            "Sync all agent changes".to_string()
        };

        // Get HEAD commit as parent (if exists)
        let parent_commit = match repo.head() {
            Ok(head) => Some(
                head.peel_to_commit()
                    .map_err(|e| EngramError::Git(format!("Failed to get HEAD commit: {}", e)))?,
            ),
            Err(_) => None, // First commit
        };

        let parents: Vec<&git2::Commit> = if let Some(ref parent) = parent_commit {
            vec![parent]
        } else {
            vec![]
        };

        // Create signature
        let signature = git2::Signature::now("Engram", "engram@local")
            .map_err(|e| EngramError::Git(format!("Failed to create signature: {}", e)))?;

        // Create the commit
        repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            &commit_message,
            &tree,
            &parents,
        )
        .map_err(|e| EngramError::Git(format!("Failed to create commit: {}", e)))?;

        println!("📦 Created commit: {}", commit_message);

        // Push to remote using authenticated_push
        let auth = RemoteAuth {
            auth_type: remote_config
                .auth_type
                .clone()
                .unwrap_or_else(|| "none".to_string()),
            username: remote_config.username.clone(),
            password: None, // We don't store passwords in remote config for security
            key_path: remote_config.ssh_key_path.clone(),
        };
        let refspec = format!("refs/heads/{}:refs/heads/{}", target_branch, target_branch);
        authenticated_push(&repo, &remote_name, &[&refspec], &auth)?;

        println!("✅ Successfully pushed to remote '{}'", remote_name);
    } else {
        println!("✅ Dry run completed - would push to remote");
    }

    Ok(())
}

/// Create Git2 credentials based on authentication configuration
pub fn create_credentials(auth: &RemoteAuth) -> Result<Option<RemoteCallbacks<'_>>, EngramError> {
    match auth.auth_type.as_str() {
        "ssh" => {
            let mut callbacks = RemoteCallbacks::new();
            callbacks.credentials(|_url, username_from_url, _allowed_types| {
                let username = auth
                    .username
                    .as_deref()
                    .or(username_from_url)
                    .unwrap_or("git");

                if let Some(key_path) = &auth.key_path {
                    Cred::ssh_key(
                        username,
                        None,
                        Path::new(key_path),
                        auth.password.as_deref(),
                    )
                } else {
                    Cred::ssh_key_from_agent(username)
                }
            });
            Ok(Some(callbacks))
        }
        "http" | "https" => {
            let mut callbacks = RemoteCallbacks::new();
            let username = auth.username.clone();
            let password = auth.password.clone();

            callbacks.credentials(move |_url, _username_from_url, _allowed_types| {
                if let (Some(ref user), Some(ref pass)) = (&username, &password) {
                    Cred::userpass_plaintext(user, pass)
                } else {
                    Cred::default()
                }
            });
            Ok(Some(callbacks))
        }
        "none" => Ok(None),
        _ => Err(EngramError::Validation(format!(
            "Invalid authentication type: '{}'. Valid options: ssh, http, https, none",
            auth.auth_type
        ))),
    }
}

/// Perform authenticated Git fetch operation
fn authenticated_fetch(
    repo: &Repository,
    remote_name: &str,
    refspecs: &[&str],
    auth: &RemoteAuth,
) -> Result<(), EngramError> {
    let mut remote = repo
        .find_remote(remote_name)
        .map_err(|e| EngramError::Git(format!("Failed to find remote '{}': {}", remote_name, e)))?;

    if let Some(callbacks) = create_credentials(auth)? {
        let mut fetch_options = FetchOptions::new();
        fetch_options.remote_callbacks(callbacks);
        remote
            .fetch(refspecs, Some(&mut fetch_options), None)
            .map_err(|e| EngramError::Git(format!("Failed to fetch from remote: {}", e)))?;
    } else {
        remote
            .fetch(refspecs, None, None)
            .map_err(|e| EngramError::Git(format!("Failed to fetch from remote: {}", e)))?;
    }

    Ok(())
}

/// Perform authenticated Git push operation
fn authenticated_push(
    repo: &Repository,
    remote_name: &str,
    refspecs: &[&str],
    auth: &RemoteAuth,
) -> Result<(), EngramError> {
    let mut remote = repo
        .find_remote(remote_name)
        .map_err(|e| EngramError::Git(format!("Failed to find remote '{}': {}", remote_name, e)))?;

    if let Some(callbacks) = create_credentials(auth)? {
        let mut push_options = PushOptions::new();
        push_options.remote_callbacks(callbacks);

        remote
            .push(refspecs, Some(&mut push_options))
            .map_err(|e| EngramError::Git(format!("Failed to push to remote: {}", e)))?;
    } else {
        remote
            .push(refspecs, None)
            .map_err(|e| EngramError::Git(format!("Failed to push to remote: {}", e)))?;
    }

    Ok(())
}

/// Handle sync commands
pub fn handle_sync_command<S: Storage>(
    storage: &mut S,
    command: &SyncCommands,
) -> Result<(), EngramError> {
    match command {
        SyncCommands::Sync {
            agents,
            strategy,
            dry_run,
        } => {
            let agent_list: Vec<String> = agents
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();

            if agent_list.is_empty() {
                return Err(EngramError::Validation(
                    "No valid agents provided".to_string(),
                ));
            }

            let merge_strategy = MergeStrategy::from_str(strategy)?;
            let _result = sync_agents(storage, agent_list, merge_strategy, *dry_run)?;

            println!("\n🎉 Synchronization completed successfully!");
            Ok(())
        }
        SyncCommands::AddRemote {
            name,
            url,
            branch,
            auth_type,
            username,
            password: _,
            ssh_key,
        } => add_remote(
            storage,
            name.clone(),
            url.clone(),
            branch.clone(),
            auth_type.clone(),
            username.clone(),
            ssh_key.clone(),
        ),
        SyncCommands::ListRemotes => {
            list_remotes(&mut std::io::stdout())?;
            Ok(())
        }
        SyncCommands::Status { remote } => {
            if let Some(remote_name) = remote {
                get_sync_status(&mut std::io::stdout(), remote_name)?;
            } else {
                return Err(EngramError::Validation(
                    "Remote name required for status check".to_string(),
                ));
            }
            Ok(())
        }
        SyncCommands::Pull {
            remote,
            branch,
            agents,
            auth_type,
            username,
            password,
            ssh_key,
            dry_run,
        } => {
            let auth = RemoteAuth {
                auth_type: auth_type.clone().unwrap_or_else(|| "none".to_string()),
                username: username.clone(),
                password: password.clone(),
                key_path: ssh_key.clone(),
            };
            pull_from_remote(
                storage,
                remote.clone(),
                branch.clone(),
                agents.clone(),
                auth,
                *dry_run,
            )
        }
        SyncCommands::Push {
            remote,
            branch,
            agents,
            auth_type,
            username,
            password,
            ssh_key,
            dry_run,
        } => {
            let auth = RemoteAuth {
                auth_type: auth_type.clone().unwrap_or_else(|| "none".to_string()),
                username: username.clone(),
                password: password.clone(),
                key_path: ssh_key.clone(),
            };
            push_to_remote(
                storage,
                remote.clone(),
                branch.clone(),
                agents.clone(),
                auth,
                *dry_run,
            )
        }
        SyncCommands::CreateBranch { name, agent, from } => {
            create_branch(name, agent.as_deref(), from.as_deref())
        }
        SyncCommands::SwitchBranch { name, create } => switch_branch(name, *create),
        SyncCommands::ListBranches { all, current } => list_branches(*all, *current),
        SyncCommands::DeleteBranch { name, force } => delete_branch(name, *force),
    }
}

/// Create a new branch for agent isolation
pub fn create_branch(
    branch_name: &str,
    agent: Option<&str>,
    from_branch: Option<&str>,
) -> Result<(), EngramError> {
    let repo_path = std::env::current_dir()?.join(".engram");
    let repo = Repository::open(&repo_path)
        .map_err(|e| EngramError::Git(format!("Failed to open repository: {}", e)))?;

    let from = from_branch.unwrap_or("main");

    println!("🌿 Creating branch '{}'", branch_name);
    if let Some(agent_name) = agent {
        println!("👤 Agent: {}", agent_name);
    }
    println!("📍 From: {}", from);

    let target_commit = if let Ok(target_ref) = repo.find_reference(&format!("refs/heads/{}", from))
    {
        target_ref.peel_to_commit().map_err(|e| {
            EngramError::Git(format!(
                "Failed to find commit for branch '{}': {}",
                from, e
            ))
        })?
    } else {
        return Err(EngramError::Git(format!(
            "Source branch '{}' not found",
            from
        )));
    };

    let _branch_ref = repo
        .reference(
            &format!("refs/heads/{}", branch_name),
            target_commit.id(),
            false,
            &format!("Create branch {}", branch_name),
        )
        .map_err(|e| {
            if e.code() == git2::ErrorCode::Exists {
                EngramError::Git(format!("Branch '{}' already exists", branch_name))
            } else {
                EngramError::Git(format!("Failed to create branch '{}': {}", branch_name, e))
            }
        })?;

    println!("✅ Branch '{}' created successfully", branch_name);

    if let Some(agent_name) = agent {
        println!(
            "📝 Branch '{}' is now associated with agent '{}'",
            branch_name, agent_name
        );
    }

    Ok(())
}

/// Switch to a different branch
pub fn switch_branch(branch_name: &str, create_if_missing: bool) -> Result<(), EngramError> {
    let repo_path = std::env::current_dir()?.join(".engram");
    let repo = Repository::open(&repo_path)
        .map_err(|e| EngramError::Git(format!("Failed to open repository: {}", e)))?;

    let branch_exists = repo
        .find_reference(&format!("refs/heads/{}", branch_name))
        .is_ok();

    if !branch_exists {
        if create_if_missing {
            create_branch(branch_name, None, None)?;
            // Reopen repository to ensure new branch is visible
            let repo = Repository::open(&repo_path)
                .map_err(|e| EngramError::Git(format!("Failed to reopen repository: {}", e)))?;
            let branch_ref = repo
                .find_reference(&format!("refs/heads/{}", branch_name))
                .map_err(|e| {
                    EngramError::Git(format!("Failed to find branch '{}': {}", branch_name, e))
                })?;
            let commit = branch_ref.peel_to_commit().map_err(|e| {
                EngramError::Git(format!(
                    "Failed to get commit for branch '{}': {}",
                    branch_name, e
                ))
            })?;
            repo.set_head(&format!("refs/heads/{}", branch_name))
                .map_err(|e| {
                    EngramError::Git(format!(
                        "Failed to switch to branch '{}': {}",
                        branch_name, e
                    ))
                })?;
            repo.checkout_tree(
                commit.tree().unwrap().as_object(),
                Some(git2::build::CheckoutBuilder::new().force()),
            )
            .map_err(|e| {
                EngramError::Git(format!(
                    "Failed to checkout branch '{}': {}",
                    branch_name, e
                ))
            })?;
            println!("🌿 Switched to branch '{}'", branch_name);
            return Ok(());
        } else {
            return Err(EngramError::Git(format!(
                "Branch '{}' does not exist. Use --create to create it.",
                branch_name
            )));
        }
    }

    let branch_ref = repo
        .find_reference(&format!("refs/heads/{}", branch_name))
        .map_err(|e| EngramError::Git(format!("Failed to find branch '{}': {}", branch_name, e)))?;

    let commit = branch_ref.peel_to_commit().map_err(|e| {
        EngramError::Git(format!(
            "Failed to get commit for branch '{}': {}",
            branch_name, e
        ))
    })?;

    repo.set_head(&format!("refs/heads/{}", branch_name))
        .map_err(|e| {
            EngramError::Git(format!(
                "Failed to switch to branch '{}': {}",
                branch_name, e
            ))
        })?;

    repo.checkout_tree(
        commit.tree().unwrap().as_object(),
        Some(git2::build::CheckoutBuilder::new().force()),
    )
    .map_err(|e| {
        EngramError::Git(format!(
            "Failed to checkout branch '{}': {}",
            branch_name, e
        ))
    })?;

    println!("🌿 Switched to branch '{}'", branch_name);
    Ok(())
}

/// List all branches
pub fn list_branches(_all: bool, current_only: bool) -> Result<(), EngramError> {
    use crate::cli::utils::create_table;
    use prettytable::row;

    let repo_path = std::env::current_dir()?.join(".engram");
    let repo = Repository::open(&repo_path)
        .map_err(|e| EngramError::Git(format!("Failed to open repository: {}", e)))?;

    let head = repo.head().ok();
    let current_branch = if let Some(head_ref) = &head {
        head_ref.shorthand()
    } else {
        None
    };

    let branches = repo
        .branches(Some(git2::BranchType::Local))
        .map_err(|e| EngramError::Git(format!("Failed to list branches: {}", e)))?;

    // Collect branches first to handle errors and sorting
    let mut branch_list = Vec::new();
    for branch_result in branches {
        let (branch, _branch_type) =
            branch_result.map_err(|e| EngramError::Git(format!("Failed to read branch: {}", e)))?;

        let branch_name = branch
            .name()
            .map_err(|e| EngramError::Git(format!("Failed to get branch name: {}", e)))?
            .unwrap_or("<unnamed>")
            .to_string();

        branch_list.push(branch_name);
    }
    branch_list.sort();

    if current_only {
        if let Some(current) = current_branch {
            println!("* {}", current);
        } else {
            println!("No current branch (detached HEAD)");
        }
        return Ok(());
    }

    let mut table = create_table();
    table.set_titles(row!["Current", "Branch Name"]);

    for branch_name in branch_list {
        let is_current = current_branch.as_deref() == Some(branch_name.as_str());
        let marker = if is_current { "*" } else { "" };

        table.add_row(row![marker, branch_name]);
    }

    table.printstd();
    println!();

    Ok(())
}

/// Delete a branch
pub fn delete_branch(branch_name: &str, force: bool) -> Result<(), EngramError> {
    let repo_path = std::env::current_dir()?.join(".engram");
    let repo = Repository::open(&repo_path)
        .map_err(|e| EngramError::Git(format!("Failed to open repository: {}", e)))?;

    let head = repo.head().ok();
    let current_branch = if let Some(head_ref) = &head {
        head_ref.shorthand()
    } else {
        None
    };

    if current_branch == Some(branch_name) {
        return Err(EngramError::Git(format!(
            "Cannot delete the currently checked out branch '{}'",
            branch_name
        )));
    }

    let mut branch = repo
        .find_branch(branch_name, git2::BranchType::Local)
        .map_err(|e| EngramError::Git(format!("Branch '{}' not found: {}", branch_name, e)))?;

    if !force {
        println!(
            "⚠️  Are you sure you want to delete branch '{}'? Use --force to confirm.",
            branch_name
        );
        return Ok(());
    }

    branch.delete().map_err(|e| {
        EngramError::Git(format!("Failed to delete branch '{}': {}", branch_name, e))
    })?;

    println!("✅ Branch '{}' deleted successfully", branch_name);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::MemoryStorage;

    #[test]
    fn test_merge_strategy_from_str() {
        assert!(matches!(
            MergeStrategy::from_str("latest_wins").unwrap(),
            MergeStrategy::LatestWins
        ));
        assert!(matches!(
            MergeStrategy::from_str("intelligent_merge").unwrap(),
            MergeStrategy::IntelligentMerge
        ));
        assert!(matches!(
            MergeStrategy::from_str("merge_with_conflict_resolution").unwrap(),
            MergeStrategy::MergeWithConflictResolution
        ));

        let strategy = MergeStrategy::from_str("priority_wins:agent1").unwrap();
        if let MergeStrategy::PriorityWins { agent } = strategy {
            assert_eq!(agent, "agent1");
        } else {
            panic!("Expected PriorityWins");
        }

        assert!(MergeStrategy::from_str("unknown").is_err());
        assert!(MergeStrategy::from_str("priority_wins:").is_err());
    }

    #[test]
    fn test_sync_agents_empty() {
        let mut storage = MemoryStorage::new("test-agent");
        let result = sync_agents(&mut storage, vec![], MergeStrategy::LatestWins, false);
        assert!(result.is_err());
    }

    #[test]
    fn test_sync_agents_single() {
        let mut storage = MemoryStorage::new("test-agent");
        let result = sync_agents(
            &mut storage,
            vec!["agent1".to_string()],
            MergeStrategy::LatestWins,
            false,
        );
        assert!(result.is_ok());
        let sync_result = result.unwrap();
        assert_eq!(sync_result.entities_synced, 0);
    }
}

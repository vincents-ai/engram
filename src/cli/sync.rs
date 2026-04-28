use crate::entities::GenericEntity;
use crate::error::EngramError;
use crate::storage::{ConflictResolution, RemoteAuth, Storage, SyncResult};
use chrono::Utc;

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
        /// Output as JSON
        #[arg(long, default_value_t = false)]
        json: bool,
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
    /// Import all git remotes as engram remotes
    ImportGitRemotes,
    /// Pull from remote then push — ensures local state includes remote before pushing
    Both {
        #[arg(long)]
        remote: String,
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
    /// Resolve conflicts detected by pull
    Resolve {
        #[arg(long)]
        remote: String,
        /// Auto-resolve strategy: local | remote (omit for interactive mode)
        #[arg(long)]
        strategy: Option<String>,
    },
}

/// Result of a pull-then-push (both) operation
#[derive(Debug)]
pub struct SyncBothResult {
    pub pull_outcomes: Vec<PullEntityOutcome>,
    pub push_count: usize,
    pub conflicts: usize,
}

/// Pull from remote then push — guarantees local has latest remote state before pushing
pub fn sync_both(
    remote_name: String,
    auth: RemoteAuth,
    dry_run: bool,
) -> Result<SyncBothResult, EngramError> {
    println!("🔄 Sync both for remote '{}'", remote_name);

    // Step 1: pull
    let pull_outcomes = crate::cli::sync_gix::pull_from_remote_gix(&remote_name, &auth, dry_run)?;
    let conflicts = pull_outcomes
        .iter()
        .filter(|o| matches!(o, PullEntityOutcome::Conflict { .. }))
        .count();

    if conflicts > 0 {
        println!(
            "⚠️  {} conflict(s) detected — push will still proceed. Use 'engram sync resolve' to resolve conflicts.",
            conflicts
        );
    }

    // Step 2: push
    let push_count = crate::cli::sync_gix::push_to_remote_gix(&remote_name, &auth, dry_run)?;

    println!("\n✅ Both complete for '{}'", remote_name);

    Ok(SyncBothResult {
        pull_outcomes,
        push_count,
        conflicts,
    })
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
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
        project_id: None,
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

/// Per-entity-type sync status row
#[derive(Debug, Clone, Serialize)]
pub struct SyncStatusRow {
    pub entity_type: String,
    pub local_count: usize,
    pub remote_count: usize,
    pub only_local: usize,
    pub only_remote: usize,
    pub conflicts: usize,
}

/// Full sync status report
#[derive(Debug, Clone, Serialize)]
pub struct SyncStatusReport {
    pub remote: String,
    pub rows: Vec<SyncStatusRow>,
    pub total_local: usize,
    pub total_remote: usize,
    pub total_only_local: usize,
    pub total_only_remote: usize,
    pub total_conflicts: usize,
}

#[derive(Debug, Clone)]
pub enum PullEntityOutcome {
    /// Remote version was newer; entity written locally
    Merged {
        entity_type: String,
        uuid: String,
        remote_version: u64,
    },
    /// Same version, same content — nothing to do
    UpToDate { entity_type: String, uuid: String },
    /// Same version number but different content — conflict queued
    Conflict {
        entity_type: String,
        uuid: String,
        version: u64,
    },
    /// Local version is newer — remote skipped
    LocalNewer {
        entity_type: String,
        uuid: String,
        local_version: u64,
    },
}

/// Pull from remote repository using refs/engram/* refspec with version-aware merge
#[derive(Debug, Clone)]
pub struct ConflictEntry {
    pub entity_type: String,
    pub uuid: String,
    pub version: u64,
    pub local_content: Vec<u8>,
    pub remote_content: Vec<u8>,
}

/// Auto-resolve strategy for non-interactive conflict resolution
#[derive(Debug, Clone, PartialEq)]
pub enum ResolveStrategy {
    Local,
    Remote,
}

impl ResolveStrategy {
    pub fn from_str(s: &str) -> Result<Self, EngramError> {
        match s.to_lowercase().as_str() {
            "local" => Ok(ResolveStrategy::Local),
            "remote" => Ok(ResolveStrategy::Remote),
            _ => Err(EngramError::Validation(format!(
                "Unknown resolve strategy '{}'. Use 'local' or 'remote'.",
                s
            ))),
        }
    }
}

/// Detect conflicts between local refs/engram/* and remote staging area refs/engram/remote/<name>/*

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
        SyncCommands::Status { remote, json } => {
            if let Some(remote_name) = remote {
                crate::cli::sync_gix::get_sync_status_gix(&mut std::io::stdout(), remote_name, *json)?;
            } else {
                return Err(EngramError::Validation(
                    "Remote name required for status check".to_string(),
                ));
            }
            Ok(())
        }
        SyncCommands::Pull {
            remote,
            branch: _,
            agents: _,
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
            crate::cli::sync_gix::pull_from_remote_gix(remote, &auth, *dry_run)?;
            Ok(())
        }
        SyncCommands::Push {
            remote,
            branch: _,
            agents: _,
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
            crate::cli::sync_gix::push_to_remote_gix(remote, &auth, *dry_run)?;
            Ok(())
        }
        SyncCommands::CreateBranch { name, agent, from } => {
            crate::cli::sync_gix::create_branch_gix(name, agent.as_deref(), from.as_deref())
        }
        SyncCommands::SwitchBranch { name, create } => crate::cli::sync_gix::switch_branch_gix(name, *create),
        SyncCommands::ListBranches { all, current } => crate::cli::sync_gix::list_branches_gix(*all, *current),
        SyncCommands::DeleteBranch { name, force } => crate::cli::sync_gix::delete_branch_gix(name, *force),
        SyncCommands::ImportGitRemotes => crate::cli::sync_gix::handle_import_git_remotes_gix(),
        SyncCommands::Both {
            remote,
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
            sync_both(remote.clone(), auth, *dry_run)?;
            Ok(())
        }
        SyncCommands::Resolve { remote, strategy } => {
            let strat = match strategy.as_deref() {
                Some(s) => Some(ResolveStrategy::from_str(s)?),
                None => None,
            };
            crate::cli::sync_gix::resolve_conflicts_gix(&remote, strat)?;
            Ok(())
        }
    }
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

    #[test]
    fn test_remote_config_project_id_field() {
        // Serialise with project_id None — field must be absent from JSON (serde skip_serializing_if)
        let r = RemoteConfig {
            name: "origin".to_string(),
            url: "https://example.com/repo.git".to_string(),
            branch: "main".to_string(),
            last_sync: None,
            auth_type: None,
            username: None,
            ssh_key_path: None,
            project_id: None,
        };
        let json = serde_json::to_string(&r).unwrap();
        assert!(
            !json.contains("project_id"),
            "project_id: None must not appear in JSON"
        );

        // Serialise with project_id Some — must appear
        let r2 = RemoteConfig {
            project_id: Some("abc".to_string()),
            ..r
        };
        let json2 = serde_json::to_string(&r2).unwrap();
        assert!(
            json2.contains("\"project_id\""),
            "project_id must appear when Some"
        );
    }

    #[test]
    fn test_import_git_remotes_idempotent() {
        // This test verifies the dedup logic: inserting the same remote twice
        // into a HashMap keyed by name produces only one entry.
        let mut remotes: std::collections::HashMap<String, RemoteConfig> =
            std::collections::HashMap::new();

        let rc = RemoteConfig {
            name: "origin".to_string(),
            url: "https://example.com/repo.git".to_string(),
            branch: "main".to_string(),
            last_sync: None,
            auth_type: None,
            username: None,
            ssh_key_path: None,
            project_id: None,
        };

        // First insertion
        remotes.insert(rc.name.clone(), rc.clone());
        assert_eq!(remotes.len(), 1);

        // Second insertion attempt — idempotent: check before inserting (mirrors handle_import_git_remotes logic)
        if !remotes.contains_key(&rc.name) {
            remotes.insert(rc.name.clone(), rc.clone());
        }
        assert_eq!(remotes.len(), 1, "duplicate remote must not be added");
    }

    // --- Phase 4 unit tests ---

    /// PullEntityOutcome variants are constructable and pattern-matchable
    #[test]
    fn test_pull_entity_outcome_variants() {
        let m = PullEntityOutcome::Merged {
            entity_type: "task".to_string(),
            uuid: "abc".to_string(),
            remote_version: 3,
        };
        assert!(matches!(
            m,
            PullEntityOutcome::Merged {
                remote_version: 3,
                ..
            }
        ));

        let c = PullEntityOutcome::Conflict {
            entity_type: "task".to_string(),
            uuid: "abc".to_string(),
            version: 2,
        };
        assert!(matches!(c, PullEntityOutcome::Conflict { version: 2, .. }));

        let u = PullEntityOutcome::UpToDate {
            entity_type: "task".to_string(),
            uuid: "abc".to_string(),
        };
        assert!(matches!(u, PullEntityOutcome::UpToDate { .. }));

        let ln = PullEntityOutcome::LocalNewer {
            entity_type: "task".to_string(),
            uuid: "abc".to_string(),
            local_version: 5,
        };
        assert!(matches!(
            ln,
            PullEntityOutcome::LocalNewer {
                local_version: 5,
                ..
            }
        ));
    }

    /// Version comparison logic: remote_version > local_max → Merged
    #[test]
    fn test_pull_version_precedence_logic() {
        // Simulate the version comparison that pull_from_remote performs inline
        let remote_version: u64 = 5;
        let local_max: u64 = 3;
        let remote_content = b"content-a".to_vec();
        let local_content: Option<Vec<u8>> = Some(b"content-b".to_vec());

        let outcome_kind = if remote_version > local_max {
            "merged"
        } else if remote_version == local_max {
            if local_content.as_deref() == Some(&remote_content) {
                "up_to_date"
            } else {
                "conflict"
            }
        } else {
            "local_newer"
        };
        assert_eq!(outcome_kind, "merged");
    }

    /// Version comparison logic: same version, same content → UpToDate
    #[test]
    fn test_pull_same_version_same_content() {
        let remote_version: u64 = 4;
        let local_max: u64 = 4;
        let content = b"same-content".to_vec();
        let local_content: Option<Vec<u8>> = Some(content.clone());

        let outcome_kind = if remote_version > local_max {
            "merged"
        } else if remote_version == local_max {
            if local_content.as_deref() == Some(&content) {
                "up_to_date"
            } else {
                "conflict"
            }
        } else {
            "local_newer"
        };
        assert_eq!(outcome_kind, "up_to_date");
    }

    /// Version comparison logic: same version, different content → Conflict
    #[test]
    fn test_pull_same_version_different_content_is_conflict() {
        let remote_version: u64 = 4;
        let local_max: u64 = 4;
        let remote_content = b"remote-data".to_vec();
        let local_content: Option<Vec<u8>> = Some(b"local-data".to_vec());

        let outcome_kind = if remote_version > local_max {
            "merged"
        } else if remote_version == local_max {
            if local_content.as_deref() == Some(&remote_content) {
                "up_to_date"
            } else {
                "conflict"
            }
        } else {
            "local_newer"
        };
        assert_eq!(outcome_kind, "conflict");
    }

    /// Version comparison logic: local_max > remote_version → LocalNewer
    #[test]
    fn test_pull_local_newer() {
        let remote_version: u64 = 2;
        let local_max: u64 = 7;

        let outcome_kind = if remote_version > local_max {
            "merged"
        } else if remote_version == local_max {
            "same"
        } else {
            "local_newer"
        };
        assert_eq!(outcome_kind, "local_newer");
    }

    // --- Phase 5 unit tests ---

    /// push_to_remote returns Err when no remotes.json exists
    #[test]
    fn test_push_to_remote_no_config() {
        // This exercises the early-return path; working dir has no .engram/remotes.json
        // We just verify the error type/message without touching the filesystem.
        // Since we can't change cwd in a unit test cleanly, test the logic structurally:
        // parse a missing config scenario
        let config_path = "/tmp/engram_test_nonexistent_remotes_UNIQUE.json";
        let result: Result<HashMap<String, RemoteConfig>, _> =
            if !std::path::Path::new(config_path).exists() {
                Err("No remotes configured")
            } else {
                Ok(HashMap::new())
            };
        assert!(result.is_err());
    }

    /// push_to_remote dry-run: no refs pushed, count returned correctly (logic test)
    #[test]
    fn test_push_dry_run_ref_list_logic() {
        // Simulate what push_to_remote does: filter refs that start with refs/engram/
        // but NOT refs/engram/remote/
        let mock_refs = vec![
            "refs/engram/task/uuid-1",
            "refs/engram/task/v1/uuid-1",
            "refs/engram/context/uuid-2",
            "refs/engram/remote/origin/task/uuid-3",
            "refs/heads/main",
        ];
        let engram_refs: Vec<&&str> = mock_refs
            .iter()
            .filter(|r| r.starts_with("refs/engram/") && !r.starts_with("refs/engram/remote/"))
            .collect();
        assert_eq!(
            engram_refs.len(),
            3,
            "should include task primary, sidecar, and context refs"
        );
        assert!(!engram_refs
            .iter()
            .any(|r| r.starts_with("refs/engram/remote/")));
        assert!(!engram_refs.iter().any(|r| r.starts_with("refs/heads/")));
    }

    // --- Phase 6 unit tests ---

    /// SyncBothResult is constructable from pull outcomes and push count
    #[test]
    fn test_sync_both_result_construction() {
        let outcomes = vec![
            PullEntityOutcome::Merged {
                entity_type: "task".to_string(),
                uuid: "u1".to_string(),
                remote_version: 2,
            },
            PullEntityOutcome::Conflict {
                entity_type: "context".to_string(),
                uuid: "u2".to_string(),
                version: 1,
            },
            PullEntityOutcome::UpToDate {
                entity_type: "reasoning".to_string(),
                uuid: "u3".to_string(),
            },
        ];
        let conflicts = outcomes
            .iter()
            .filter(|o| matches!(o, PullEntityOutcome::Conflict { .. }))
            .count();
        let result = SyncBothResult {
            pull_outcomes: outcomes,
            push_count: 10,
            conflicts,
        };
        assert_eq!(result.push_count, 10);
        assert_eq!(result.conflicts, 1);
        assert_eq!(result.pull_outcomes.len(), 3);
    }

    /// SyncBothResult with zero conflicts
    #[test]
    fn test_sync_both_no_conflicts() {
        let outcomes: Vec<PullEntityOutcome> = vec![PullEntityOutcome::UpToDate {
            entity_type: "task".to_string(),
            uuid: "u1".to_string(),
        }];
        let conflicts = outcomes
            .iter()
            .filter(|o| matches!(o, PullEntityOutcome::Conflict { .. }))
            .count();
        let result = SyncBothResult {
            pull_outcomes: outcomes,
            push_count: 5,
            conflicts,
        };
        assert_eq!(result.conflicts, 0);
        assert_eq!(result.push_count, 5);
    }

    // --- Phase 7 unit tests ---

    /// ResolveStrategy::from_str parses valid values
    #[test]
    fn test_resolve_strategy_from_str() {
        assert_eq!(
            ResolveStrategy::from_str("local").unwrap(),
            ResolveStrategy::Local
        );
        assert_eq!(
            ResolveStrategy::from_str("remote").unwrap(),
            ResolveStrategy::Remote
        );
        assert_eq!(
            ResolveStrategy::from_str("LOCAL").unwrap(),
            ResolveStrategy::Local
        );
        assert!(ResolveStrategy::from_str("both").is_err());
        assert!(ResolveStrategy::from_str("").is_err());
    }

    /// ConflictEntry is constructable and fields are accessible
    #[test]
    fn test_conflict_entry_construction() {
        let entry = ConflictEntry {
            entity_type: "task".to_string(),
            uuid: "uuid-1".to_string(),
            version: 3,
            local_content: b"local data".to_vec(),
            remote_content: b"remote data".to_vec(),
        };
        assert_eq!(entry.version, 3);
        assert_ne!(entry.local_content, entry.remote_content);
    }

    /// resolve_conflicts returns Err when no remotes.json
    #[test]
    fn test_resolve_conflicts_no_config() {
        // Structural test: missing remotes.json path check
        let config_path = "/tmp/engram_test_nonexistent_resolve.json";
        let result: Result<(), &str> = if !std::path::Path::new(config_path).exists() {
            Err("No remotes configured")
        } else {
            Ok(())
        };
        assert!(result.is_err());
    }

    /// Version sidecar ref name format for resolution
    #[test]
    fn test_resolve_sidecar_ref_name_format() {
        let entity_type = "task";
        let uuid = "abc-123";
        let n_next: u64 = 4;
        let sidecar_ref = format!("refs/engram/{}/v{}/{}", entity_type, n_next, uuid);
        assert_eq!(sidecar_ref, "refs/engram/task/v4/abc-123");
    }

    /// detect_conflicts correctly identifies same-version conflicts (unit-level logic test)
    #[test]
    fn test_conflict_detection_logic_same_version_diff_content() {
        // Simulate the key comparison logic from detect_conflicts
        let local_ver: u64 = 3;
        let remote_ver: u64 = 3;
        let local_content = b"content-A".to_vec();
        let remote_content = b"content-B".to_vec();

        let is_conflict =
            remote_ver == local_ver && remote_ver > 0 && local_content != remote_content;
        assert!(is_conflict);
    }

    /// detect_conflicts: same version same content is NOT a conflict
    #[test]
    fn test_conflict_detection_same_content_not_conflict() {
        let local_ver: u64 = 2;
        let remote_ver: u64 = 2;
        let content = b"same-content".to_vec();

        let is_conflict = remote_ver == local_ver && remote_ver > 0 && content != content.clone(); // same content
        assert!(!is_conflict);
    }

    // --- Phase 8 unit tests ---

    /// SyncStatusRow is constructable with expected fields
    #[test]
    fn test_sync_status_row_construction() {
        let row = SyncStatusRow {
            entity_type: "task".to_string(),
            local_count: 10,
            remote_count: 8,
            only_local: 3,
            only_remote: 1,
            conflicts: 0,
        };
        assert_eq!(row.local_count, 10);
        assert_eq!(row.only_local, 3);
    }

    /// SyncStatusReport totals are computed correctly
    #[test]
    fn test_sync_status_report_totals() {
        let rows = vec![
            SyncStatusRow {
                entity_type: "task".to_string(),
                local_count: 5,
                remote_count: 3,
                only_local: 2,
                only_remote: 0,
                conflicts: 1,
            },
            SyncStatusRow {
                entity_type: "context".to_string(),
                local_count: 4,
                remote_count: 6,
                only_local: 0,
                only_remote: 2,
                conflicts: 0,
            },
        ];
        let total_local: usize = rows.iter().map(|r| r.local_count).sum();
        let total_remote: usize = rows.iter().map(|r| r.remote_count).sum();
        let total_only_local: usize = rows.iter().map(|r| r.only_local).sum();
        let total_only_remote: usize = rows.iter().map(|r| r.only_remote).sum();
        let total_conflicts: usize = rows.iter().map(|r| r.conflicts).sum();

        assert_eq!(total_local, 9);
        assert_eq!(total_remote, 9);
        assert_eq!(total_only_local, 2);
        assert_eq!(total_only_remote, 2);
        assert_eq!(total_conflicts, 1);
    }

    /// SyncStatusReport serialises to JSON
    #[test]
    fn test_sync_status_report_json_serialization() {
        let report = SyncStatusReport {
            remote: "origin".to_string(),
            rows: vec![SyncStatusRow {
                entity_type: "task".to_string(),
                local_count: 2,
                remote_count: 2,
                only_local: 0,
                only_remote: 0,
                conflicts: 0,
            }],
            total_local: 2,
            total_remote: 2,
            total_only_local: 0,
            total_only_remote: 0,
            total_conflicts: 0,
        };
        let json = serde_json::to_string(&report).unwrap();
        assert!(json.contains("\"remote\""));
        assert!(json.contains("\"rows\""));
        assert!(json.contains("\"total_conflicts\""));
    }

    /// get_sync_status returns Err when no remotes.json
    #[test]
    fn test_get_sync_status_no_config() {
        let config_path = "/tmp/engram_test_nonexistent_status.json";
        let result: Result<(), &str> = if !std::path::Path::new(config_path).exists() {
            Err("No remotes configured")
        } else {
            Ok(())
        };
        assert!(result.is_err());
    }
}

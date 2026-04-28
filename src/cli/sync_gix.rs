//! Gix-based replacements for git2 operations in sync.rs
//!
//! This module provides the same functionality as the git2 calls in sync.rs
//! but using gix APIs exclusively. Functions are migrated here one at a time
//! as we remove the git2 dependency.

use crate::error::EngramError;

/// Open the engram git repo (the .engram directory)
fn open_engram_repo() -> Result<gix::Repository, EngramError> {
    let repo_path = std::env::current_dir()
        .map_err(|e| EngramError::Io(e))?
        .join(".engram");
    gix::open(&repo_path).map_err(|e| EngramError::Git(format!("Failed to open repository: {}", e)))
}

/// Open the workspace git repo (current directory)
fn open_workspace_repo() -> Result<gix::Repository, EngramError> {
    let cwd = std::env::current_dir().map_err(|e| EngramError::Io(e))?;
    gix::open(&cwd).map_err(|e| EngramError::Git(format!("Failed to open repository: {}", e)))
}

/// List all refs as (name, oid_string) pairs using gix
pub fn list_all_refs(repo: &gix::Repository) -> Result<Vec<(String, String)>, EngramError> {
    let refs_platform = repo
        .references()
        .map_err(|e| EngramError::Git(format!("Failed to list references: {}", e)))?;
    let all_refs = refs_platform
        .all()
        .map_err(|e| EngramError::Git(format!("Failed to iterate references: {}", e)))?;

    let mut result = Vec::new();
    for r_result in all_refs {
        let r = match r_result {
            Ok(r) => r,
            Err(_) => continue,
        };
        let name = String::from_utf8_lossy(r.name().as_bstr()).to_string();
        if let Some(id) = r.try_id() {
            result.push((name, id.to_string()));
        }
    }
    Ok(result)
}

/// Get the current branch name (short form, without refs/heads/ prefix)
pub fn current_branch(repo: &gix::Repository) -> Option<String> {
    match repo.head_ref() {
        Ok(Some(r)) => Some(r.name().shorten().to_string()),
        _ => None,
    }
}

/// Read a blob from the repo by OID string
fn read_blob(repo: &gix::Repository, oid_str: &str) -> Result<Vec<u8>, EngramError> {
    let oid = gix::ObjectId::from_hex(oid_str.as_bytes())
        .map_err(|e| EngramError::Git(format!("Invalid OID '{}': {}", oid_str, e)))?;
    let obj = repo
        .find_object(oid)
        .map_err(|e| EngramError::Git(format!("Failed to find object '{}': {}", oid_str, e)))?;
    match obj.kind {
        gix::object::Kind::Blob => Ok(obj.data.clone()),
        other => Err(EngramError::Git(format!(
            "Expected blob, got {:?}",
            other
        ))),
    }
}

/// Write a blob to the repo and return its OID string
fn write_blob(repo: &gix::Repository, data: &[u8]) -> Result<String, EngramError> {
    let oid = repo
        .write_blob(data)
        .map_err(|e| EngramError::Git(format!("Failed to write blob: {}", e)))?;
    Ok(oid.to_string())
}

/// Update or create a reference pointing to an OID
fn set_ref(
    repo: &gix::Repository,
    ref_name: &str,
    oid_str: &str,
    create: bool,
    message: &str,
) -> Result<(), EngramError> {
    use gix::refs::FullName;
    use gix::refs::Target;
    use gix::refs::transaction::{Change, LogChange, PreviousValue, RefEdit};

    let name = FullName::try_from(ref_name)
        .map_err(|e| EngramError::Git(format!("Invalid ref name '{}': {}", ref_name, e)))?;
    let oid = gix::ObjectId::from_hex(oid_str.as_bytes())
        .map_err(|e| EngramError::Git(format!("Invalid OID '{}': {}", oid_str, e)))?;

    let expected = if create {
        PreviousValue::MustNotExist
    } else {
        PreviousValue::Any
    };

    repo.edit_reference(RefEdit {
        change: Change::Update {
            log: LogChange {
                mode: gix::refs::transaction::RefLog::AndReference,
                force_create_reflog: false,
                message: message.into(),
            },
            expected,
            new: Target::Object(oid.into()),
        },
        name,
        deref: false,
    })
    .map_err(|e| EngramError::Git(format!("Failed to write ref '{}': {}", ref_name, e)))?;

    Ok(())
}

use std::collections::HashMap;

/// Re-export sync.rs types
pub use super::sync::{ConflictEntry, SyncStatusReport, SyncStatusRow};

/// Detect conflicts between local and remote engram refs
pub fn detect_conflicts_gix(
    repo: &gix::Repository,
    remote_name: &str,
) -> Result<Vec<ConflictEntry>, EngramError> {
    let remote_prefix = format!("refs/engram/remote/{}/", remote_name);
    let sidecar_segment = "/v";

    let all_refs = list_all_refs(repo)?;

    // Build local max-version map
    let mut local_max_version: HashMap<(String, String), u64> = HashMap::new();
    for (name, _) in &all_refs {
        if !name.starts_with("refs/engram/") || name.starts_with("refs/engram/remote/") {
            continue;
        }
        let after = &name["refs/engram/".len()..];
        if let Some(v_pos) = after.find("/v") {
            let entity_type = &after[..v_pos];
            let rest = &after[v_pos + 2..];
            if let Some(slash_pos) = rest.find('/') {
                let version_str = &rest[..slash_pos];
                let uuid = &rest[slash_pos + 1..];
                if let Ok(n) = version_str.parse::<u64>() {
                    let key = (entity_type.to_string(), uuid.to_string());
                    let entry = local_max_version.entry(key).or_insert(0);
                    if n > *entry {
                        *entry = n;
                    }
                }
            }
        }
    }

    // Build remote max-version map
    let mut remote_max_version: HashMap<(String, String), u64> = HashMap::new();
    for (name, _) in &all_refs {
        if !name.starts_with(&remote_prefix) {
            continue;
        }
        let after = &name[remote_prefix.len()..];
        if let Some(v_pos) = after.find("/v") {
            let entity_type = &after[..v_pos];
            let rest = &after[v_pos + 2..];
            if let Some(slash_pos) = rest.find('/') {
                let version_str = &rest[..slash_pos];
                let uuid = &rest[slash_pos + 1..];
                if let Ok(n) = version_str.parse::<u64>() {
                    let key = (entity_type.to_string(), uuid.to_string());
                    let entry = remote_max_version.entry(key).or_insert(0);
                    if n > *entry {
                        *entry = n;
                    }
                }
            }
        }
    }

    let mut conflicts = Vec::new();

    for (ref_name, remote_oid) in &all_refs {
        if !ref_name.starts_with(&remote_prefix) {
            continue;
        }
        let after = &ref_name[remote_prefix.len()..];
        if after.contains(sidecar_segment) || after.starts_with("config/") {
            continue;
        }
        let slash_pos = match after.find('/') {
            Some(p) => p,
            None => continue,
        };
        let entity_type = &after[..slash_pos];
        let uuid = &after[slash_pos + 1..];
        if uuid.contains('/') {
            continue;
        }

        let key = (entity_type.to_string(), uuid.to_string());
        let remote_ver = *remote_max_version.get(&key).unwrap_or(&0);
        let local_ver = *local_max_version.get(&key).unwrap_or(&0);

        if remote_ver != local_ver || remote_ver == 0 {
            continue;
        }

        let local_ref_name = format!("refs/engram/{}/{}", entity_type, uuid);
        let local_oid = match all_refs.iter().find(|(n, _)| n == &local_ref_name) {
            Some((_, oid)) => oid.clone(),
            None => continue,
        };
        let local_content = match read_blob(repo, &local_oid) {
            Ok(c) => c,
            Err(_) => continue,
        };
        let remote_content = match read_blob(repo, remote_oid) {
            Ok(c) => c,
            Err(_) => continue,
        };

        if local_content != remote_content {
            conflicts.push(ConflictEntry {
                entity_type: entity_type.to_string(),
                uuid: uuid.to_string(),
                version: remote_ver,
                local_content,
                remote_content,
            });
        }
    }

    Ok(conflicts)
}

/// Get sync status report using gix
pub fn get_sync_status_gix(
    writer: &mut dyn std::io::Write,
    remote_name: &str,
    output_json: bool,
) -> Result<SyncStatusReport, EngramError> {
    let config_path = ".engram/remotes.json";
    if !Path::new(config_path).exists() {
        return Err(EngramError::Validation(
            "No remotes configured. Use 'add-remote' first.".to_string(),
        ));
    }

    let content = fs::read_to_string(config_path).map_err(|e| EngramError::Io(e))?;
    let remotes: HashMap<String, serde_json::Value> =
        serde_json::from_str(&content).map_err(|e| EngramError::Serialization(e))?;
    let _remote_config = remotes
        .get(remote_name)
        .ok_or_else(|| EngramError::Validation(format!("Remote '{}' not found", remote_name)))?;

    let repo = open_workspace_repo()?;
    let remote_prefix = format!("refs/engram/remote/{}/", remote_name);
    let sidecar_segment = "/v";

    let all_refs = list_all_refs(&repo)?;

    let mut local_uuids: HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut remote_uuids: HashMap<String, HashMap<String, String>> = HashMap::new();

    for (ref_name, oid) in &all_refs {
        if ref_name.starts_with("refs/engram/") && !ref_name.starts_with("refs/engram/remote/") {
            let after = &ref_name["refs/engram/".len()..];
            if after.contains(sidecar_segment) || after.starts_with("config/") {
                continue;
            }
            let slash_pos = match after.find('/') {
                Some(p) => p,
                None => continue,
            };
            let entity_type = &after[..slash_pos];
            let uuid = &after[slash_pos + 1..];
            if uuid.contains('/') {
                continue;
            }
            local_uuids
                .entry(entity_type.to_string())
                .or_default()
                .insert(uuid.to_string(), oid.clone());
        } else if ref_name.starts_with(&remote_prefix) {
            let after = &ref_name[remote_prefix.len()..];
            if after.contains(sidecar_segment) || after.starts_with("config/") {
                continue;
            }
            let slash_pos = match after.find('/') {
                Some(p) => p,
                None => continue,
            };
            let entity_type = &after[..slash_pos];
            let uuid = &after[slash_pos + 1..];
            if uuid.contains('/') {
                continue;
            }
            remote_uuids
                .entry(entity_type.to_string())
                .or_default()
                .insert(uuid.to_string(), oid.clone());
        }
    }

    let mut entity_types: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    for k in local_uuids.keys() {
        entity_types.insert(k.clone());
    }
    for k in remote_uuids.keys() {
        entity_types.insert(k.clone());
    }

    let conflicts_all = detect_conflicts_gix(&repo, remote_name)?;
    let mut conflicts_by_type: HashMap<String, usize> = HashMap::new();
    for c in &conflicts_all {
        *conflicts_by_type.entry(c.entity_type.clone()).or_insert(0) += 1;
    }

    let mut rows: Vec<SyncStatusRow> = Vec::new();
    for entity_type in &entity_types {
        let local_map = local_uuids.get(entity_type).cloned().unwrap_or_default();
        let remote_map = remote_uuids.get(entity_type).cloned().unwrap_or_default();

        let only_local = local_map
            .keys()
            .filter(|u| !remote_map.contains_key(*u))
            .count();
        let only_remote = remote_map
            .keys()
            .filter(|u| !local_map.contains_key(*u))
            .count();
        let conflicts = *conflicts_by_type.get(entity_type).unwrap_or(&0);

        rows.push(SyncStatusRow {
            entity_type: entity_type.clone(),
            local_count: local_map.len(),
            remote_count: remote_map.len(),
            only_local,
            only_remote,
            conflicts,
        });
    }

    let total_local: usize = rows.iter().map(|r| r.local_count).sum();
    let total_remote: usize = rows.iter().map(|r| r.remote_count).sum();
    let total_only_local: usize = rows.iter().map(|r| r.only_local).sum();
    let total_only_remote: usize = rows.iter().map(|r| r.only_remote).sum();
    let total_conflicts: usize = rows.iter().map(|r| r.conflicts).sum();

    let report = SyncStatusReport {
        remote: remote_name.to_string(),
        rows,
        total_local,
        total_remote,
        total_only_local,
        total_only_remote,
        total_conflicts,
    };

    if output_json {
        let json = serde_json::to_string_pretty(&report)
            .map_err(|e| EngramError::Serialization(e))?;
        writeln!(writer, "{}", json)?;
    } else {
        writeln!(writer, "Sync status — remote '{}'", remote_name)?;
        writeln!(writer, "{:-<70}", "")?;
        writeln!(
            writer,
            "{:<16} {:>8} {:>8} {:>12} {:>13} {:>10}",
            "ENTITY TYPE", "LOCAL", "REMOTE", "ONLY LOCAL", "ONLY REMOTE", "CONFLICTS"
        )?;
        writeln!(writer, "{:-<70}", "")?;
        for row in &report.rows {
            writeln!(
                writer,
                "{:<16} {:>8} {:>8} {:>12} {:>13} {:>10}",
                row.entity_type,
                row.local_count,
                row.remote_count,
                row.only_local,
                row.only_remote,
                row.conflicts,
            )?;
        }
        writeln!(writer, "{:-<70}", "")?;
        writeln!(
            writer,
            "{:<16} {:>8} {:>8} {:>12} {:>13} {:>10}",
            "TOTAL",
            report.total_local,
            report.total_remote,
            report.total_only_local,
            report.total_only_remote,
            report.total_conflicts,
        )?;

        if report.total_conflicts > 0 {
            writeln!(
                writer,
                "\n{} conflict(s) detected — run 'engram sync resolve --remote {}' to resolve.",
                report.total_conflicts, remote_name
            )?;
        } else if report.total_only_local > 0 && report.total_only_remote == 0 {
            writeln!(
                writer,
                "\nLocal is ahead — consider 'engram sync push --remote {}'.",
                remote_name
            )?;
        } else if report.total_only_remote > 0 && report.total_only_local == 0 {
            writeln!(
                writer,
                "\nRemote has new entities — consider 'engram sync pull --remote {}'.",
                remote_name
            )?;
        } else if report.total_only_remote > 0 || report.total_only_local > 0 {
            writeln!(
                writer,
                "\nDivergence detected — consider 'engram sync both --remote {}'.",
                remote_name
            )?;
        } else {
            writeln!(writer, "\nIn sync.")?;
        }
    }

    Ok(report)
}

use std::fs;
use std::path::Path;

/// List local branches (refs/heads/*) using gix
pub fn list_branches_gix(_all: bool, current_only: bool) -> Result<(), EngramError> {
    use crate::cli::utils::{create_table, truncate};
    use prettytable::row;

    let repo = open_engram_repo()?;
    let current = current_branch(&repo);

    let all_refs = list_all_refs(&repo)?;
    let mut branch_list: Vec<String> = all_refs
        .iter()
        .filter(|(name, _)| name.starts_with("refs/heads/"))
        .map(|(name, _)| name["refs/heads/".len()..].to_string())
        .collect();
    branch_list.sort();

    if current_only {
        if let Some(cur) = &current {
            println!("* {}", cur);
        } else {
            println!("No current branch (detached HEAD)");
        }
        return Ok(());
    }

    let mut table = create_table();
    table.set_titles(row!["Current", "Branch Name"]);

    for branch_name in &branch_list {
        let is_current = current.as_deref() == Some(branch_name.as_str());
        let marker = if is_current { "*" } else { "" };
        table.add_row(row![marker, truncate(branch_name, 40)]);
    }
    table.printstd();
    println!();

    Ok(())
}

/// Delete a branch (refs/heads/<name>) using gix
pub fn delete_branch_gix(branch_name: &str, force: bool) -> Result<(), EngramError> {
    let repo = open_engram_repo()?;

    let current = current_branch(&repo);
    if current.as_deref() == Some(branch_name) {
        return Err(EngramError::Git(format!(
            "Cannot delete the currently checked out branch '{}'",
            branch_name
        )));
    }

    if !force {
        println!(
            "⚠️  Are you sure you want to delete branch '{}'? Use --force to confirm.",
            branch_name
        );
        return Ok(());
    }

    use gix::refs::FullName;
    use gix::refs::transaction::{Change, PreviousValue, RefEdit, RefLog};

    let ref_name = FullName::try_from(format!("refs/heads/{}", branch_name))
        .map_err(|e| EngramError::Git(format!("Invalid branch name: {}", e)))?;

    repo.edit_reference(RefEdit {
        change: Change::Delete {
            expected: PreviousValue::MustExist,
            log: RefLog::AndReference,
        },
        name: ref_name,
        deref: false,
    })
    .map_err(|e| EngramError::Git(format!("Failed to delete branch '{}': {}", branch_name, e)))?;

    println!("✅ Branch '{}' deleted successfully", branch_name);
    Ok(())
}

/// Resolve conflicts using gix — strategy: Some("local") or Some("remote") for non-interactive
pub fn resolve_conflicts_gix(
    remote_name: &str,
    strategy: Option<super::sync::ResolveStrategy>,
) -> Result<usize, EngramError> {
    let config_path = ".engram/remotes.json";
    if !Path::new(config_path).exists() {
        return Err(EngramError::Validation(
            "No remotes configured. Use 'add-remote' first.".to_string(),
        ));
    }

    let content = fs::read_to_string(config_path).map_err(|e| EngramError::Io(e))?;
    let remotes: HashMap<String, serde_json::Value> =
        serde_json::from_str(&content).map_err(|e| EngramError::Serialization(e))?;
    let _remote_config = remotes
        .get(remote_name)
        .ok_or_else(|| EngramError::Validation(format!("Remote '{}' not found", remote_name)))?;

    let repo = open_workspace_repo()?;
    let conflicts = detect_conflicts_gix(&repo, remote_name)?;

    if conflicts.is_empty() {
        println!("No conflicts to resolve for remote '{}'.", remote_name);
        return Ok(0);
    }

    println!(
        "Found {} conflict(s) for remote '{}':",
        conflicts.len(),
        remote_name
    );
    let mut resolved = 0;

    for conflict in &conflicts {
        println!();
        println!("CONFLICT  {}/{}", conflict.entity_type, conflict.uuid);
        println!("  Version: {}", conflict.version);

        let local_preview = String::from_utf8_lossy(
            &conflict.local_content[..conflict.local_content.len().min(200)],
        );
        let remote_preview = String::from_utf8_lossy(
            &conflict.remote_content[..conflict.remote_content.len().min(200)],
        );
        println!("  Local  : {}", local_preview);
        println!("  Remote : {}", remote_preview);

        let winner_content: &[u8] = match &strategy {
            Some(super::sync::ResolveStrategy::Local) => {
                println!("  -> auto: keeping local");
                &conflict.local_content
            }
            Some(super::sync::ResolveStrategy::Remote) => {
                println!("  -> auto: using remote");
                &conflict.remote_content
            }
            None => {
                println!("  Choose: [l]ocal / [r]emote / [s]kip");
                let mut input = String::new();
                std::io::stdin()
                    .read_line(&mut input)
                    .map_err(|e| EngramError::Io(e))?;
                match input.trim().to_lowercase().as_str() {
                    "l" | "local" => {
                        println!("  -> keeping local");
                        &conflict.local_content
                    }
                    "r" | "remote" => {
                        println!("  -> using remote");
                        &conflict.remote_content
                    }
                    _ => {
                        println!("  -> skipped");
                        continue;
                    }
                }
            }
        };

        // Write winner blob and update primary ref
        let blob_oid = write_blob(&repo, winner_content)?;

        let local_ref_name = format!("refs/engram/{}/{}", conflict.entity_type, conflict.uuid);
        set_ref(
            &repo,
            &local_ref_name,
            &blob_oid,
            false,
            &format!(
                "resolve conflict {}/{} v{}",
                conflict.entity_type, conflict.uuid, conflict.version
            ),
        )?;

        // Write new version sidecar at N+1
        let n_next = conflict.version + 1;
        let sidecar_json = serde_json::json!({
            "entity_type": conflict.entity_type,
            "uuid": conflict.uuid,
            "version": n_next,
            "resolved_from_conflict": true,
            "remote": remote_name,
            "created_at": chrono::Utc::now().to_rfc3339(),
        });
        let sidecar_blob = write_blob(&repo, sidecar_json.to_string().as_bytes())?;
        let sidecar_ref = format!(
            "refs/engram/{}/v{}/{}",
            conflict.entity_type, n_next, conflict.uuid
        );
        set_ref(
            &repo,
            &sidecar_ref,
            &sidecar_blob,
            true,
            &format!(
                "resolve sidecar v{} {}/{}",
                n_next, conflict.entity_type, conflict.uuid
            ),
        )?;

        resolved += 1;
    }

    println!();
    println!("Resolved {}/{} conflict(s).", resolved, conflicts.len());
    Ok(resolved)
}

/// Set HEAD to point to a symbolic reference (branch)
fn set_head_symbolic(
    repo: &gix::Repository,
    target_ref: &str,
) -> Result<(), EngramError> {
    use gix::refs::FullName;
    use gix::refs::Target;
    use gix::refs::transaction::{Change, LogChange, PreviousValue, RefEdit};

    let name = FullName::try_from("HEAD")
        .map_err(|e| EngramError::Git(format!("Invalid HEAD ref: {}", e)))?;
    let target = FullName::try_from(target_ref.to_string())
        .map_err(|e| EngramError::Git(format!("Invalid target ref '{}': {}", target_ref, e)))?;

    repo.edit_reference(RefEdit {
        change: Change::Update {
            log: LogChange {
                mode: gix::refs::transaction::RefLog::AndReference,
                force_create_reflog: false,
                message: format!("switch to {}", target_ref).into(),
            },
            expected: PreviousValue::Any,
            new: Target::Symbolic(target),
        },
        name,
        deref: false,
    })
    .map_err(|e| EngramError::Git(format!("Failed to set HEAD: {}", e)))?;

    Ok(())
}

/// Import git remotes from the workspace repo into engram remotes.json
pub fn handle_import_git_remotes_gix() -> Result<(), EngramError> {
    use super::sync::RemoteConfig;

    let repo = open_workspace_repo()?;

    let remote_names: Vec<_> = repo
        .remote_names()
        .into_iter()
        .map(|name| name.to_string())
        .collect();

    let config_path = ".engram/remotes.json";
    let mut imported = 0usize;
    let mut skipped = 0usize;

    for name in &remote_names {
        let remote = match repo.find_remote(name.as_str()) {
            Ok(r) => r,
            Err(_) => {
                skipped += 1;
                continue;
            }
        };

        let url = match remote.url(gix::remote::Direction::Fetch) {
            Some(u) => u.to_bstring().to_string(),
            None => {
                skipped += 1;
                continue;
            }
        };

        if url.is_empty() {
            skipped += 1;
            continue;
        }

        let mut remotes: HashMap<String, RemoteConfig> = if Path::new(config_path).exists() {
            let content = fs::read_to_string(config_path).map_err(|e| EngramError::Io(e))?;
            serde_json::from_str(&content).map_err(|e| EngramError::Serialization(e))?
        } else {
            HashMap::new()
        };

        if remotes.contains_key(name) {
            skipped += 1;
            continue;
        }

        let remote_config = RemoteConfig {
            name: name.to_string(),
            url: url.clone(),
            branch: "main".to_string(),
            last_sync: None,
            auth_type: None,
            username: None,
            ssh_key_path: None,
            project_id: None,
        };

        remotes.insert(name.to_string(), remote_config);

        let config_content =
            serde_json::to_string_pretty(&remotes).map_err(|e| EngramError::Serialization(e))?;

        if !Path::new(".engram").exists() {
            fs::create_dir_all(".engram").map_err(|e| EngramError::Io(e))?;
        }

        fs::write(config_path, config_content).map_err(|e| EngramError::Io(e))?;

        println!("📡 Imported remote '{}' ({})", name, url);
        imported += 1;
    }

    println!(
        "\n✅ Import complete: {} imported, {} skipped (already existed or no URL)",
        imported, skipped
    );

    Ok(())
}

/// Create a branch in the engram repo using gix
pub fn create_branch_gix(
    name: &str,
    agent: Option<&str>,
    from: Option<&str>,
) -> Result<(), EngramError> {
    let repo = open_engram_repo()?;

    // Determine the starting point
    let start_ref = if let Some(from_name) = from {
        format!("refs/heads/{}", from_name)
    } else {
        "HEAD".to_string()
    };

    // Resolve the starting point to an OID
    let start_oid = if start_ref == "HEAD" {
        repo.head_id()
            .map_err(|e| EngramError::Git(format!("Failed to resolve HEAD: {}", e)))?
            .to_string()
    } else {
        let ref_name = gix::refs::FullName::try_from(start_ref.clone())
            .map_err(|e| EngramError::Git(format!("Invalid ref '{}': {}", start_ref, e)))?;
        repo.find_reference(&ref_name)
            .map_err(|e| EngramError::Git(format!("Failed to find ref '{}': {}", start_ref, e)))?
            .try_id()
            .ok_or_else(|| EngramError::Git(format!("Ref '{}' has no target", start_ref)))?
            .to_string()
    };

    let new_ref = format!("refs/heads/{}", name);
    let message = format!("create branch '{}'{}",
        name,
        agent.map(|a| format!(" (agent: {})", a)).unwrap_or_default()
    );

    set_ref(&repo, &new_ref, &start_oid, true, &message)?;

    // Switch HEAD to the new branch
    set_head_symbolic(&repo, &format!("refs/heads/{}", name))?;

    println!("✅ Created and switched to branch '{}'", name);
    Ok(())
}

/// Switch to a branch in the engram repo using gix
pub fn switch_branch_gix(name: &str, create_if_missing: bool) -> Result<(), EngramError> {
    let repo = open_engram_repo()?;

    let branch_ref = format!("refs/heads/{}", name);
    let ref_name = gix::refs::FullName::try_from(branch_ref.clone())
        .map_err(|e| EngramError::Git(format!("Invalid branch name '{}': {}", name, e)))?;

    // Check if branch exists
    let exists = repo.find_reference(&ref_name).is_ok();

    if !exists && !create_if_missing {
        return Err(EngramError::Git(format!(
            "Branch '{}' does not exist. Use --create to create it.",
            name
        )));
    }

    if !exists && create_if_missing {
        // Create from HEAD
        let start_oid = repo.head_id()
            .map_err(|e| EngramError::Git(format!("Failed to resolve HEAD: {}", e)))?
            .to_string();
        set_ref(&repo, &branch_ref, &start_oid, true, &format!("create branch '{}' via switch", name))?;
    }

    // Switch HEAD
    set_head_symbolic(&repo, &format!("refs/heads/{}", name))?;

    println!("✅ Switched to branch '{}'", name);
    Ok(())
}

/// Set up russh environment variables from RemoteAuth for gix transport
fn set_russh_env(auth: &crate::storage::RemoteAuth) {
    if auth.auth_type == "ssh" {
        if let Some(ref key_path) = auth.key_path {
            std::env::set_var("ENGRAM_SSH_KEY", key_path);
        }
        if let Some(ref password) = auth.password {
            std::env::set_var("ENGRAM_SSH_KEY_PASSWORD", password);
        }
        std::env::remove_var("ENGRAM_SSH_ACCEPT_UNKNOWN");
    }
}

/// Perform a gix-based fetch of refs/engram/* from a remote
fn gix_fetch(
    repo: &gix::Repository,
    remote_url: &str,
    auth: &crate::storage::RemoteAuth,
    refspec_str: &str,
) -> Result<(), EngramError> {
    use gix::remote::Direction;

    // Set up russh env for SSH auth
    set_russh_env(auth);

    // Create a temporary remote from URL with our custom refspec
    let remote = repo.remote_at(remote_url)
        .map_err(|e| EngramError::Git(format!("Failed to create remote for '{}': {:?}", remote_url, e)))?;

    let refspec: gix::refspec::RefSpec = gix::refspec::parse(
        refspec_str.into(),
        gix::refspec::parse::Operation::Fetch,
    ).map_err(|e| EngramError::Git(format!("Invalid refspec '{}': {:?}", refspec_str, e)))?
    .into();

    let ref_map_opts = gix::remote::ref_map::Options {
        prefix_from_spec_as_filter_on_remote: true,
        handshake_parameters: Vec::new(),
        extra_refspecs: vec![refspec],
    };

    let connection = remote.connect(Direction::Fetch)
        .map_err(|e| EngramError::Git(format!("Failed to connect to '{}': {:?}", remote_url, e)))?;

    let prepare = connection.prepare_fetch(gix::progress::Discard, ref_map_opts)
        .map_err(|e| EngramError::Git(format!("Failed to prepare fetch from '{}': {:?}", remote_url, e)))?;

    let should_interrupt = std::sync::atomic::AtomicBool::new(false);
    let _outcome = prepare.receive(gix::progress::Discard, &should_interrupt)
        .map_err(|e| EngramError::Git(format!("Failed to fetch from '{}': {:?}", remote_url, e)))?;

    Ok(())
}

/// Pull from remote using gix transport
pub fn pull_from_remote_gix(
    remote_name: &str,
    auth: &crate::storage::RemoteAuth,
    dry_run: bool,
) -> Result<Vec<super::sync::PullEntityOutcome>, EngramError> {
    use super::sync::{PullEntityOutcome, RemoteConfig};

    println!("📥 Pulling from remote '{}'...", remote_name);
    if dry_run {
        println!("   (dry-run — no local changes will be written)");
    }

    let config_path = ".engram/remotes.json";
    if !Path::new(config_path).exists() {
        return Err(EngramError::Validation(
            "No remotes configured. Use 'add-remote' first.".to_string(),
        ));
    }

    let content = fs::read_to_string(config_path).map_err(|e| EngramError::Io(e))?;
    let mut remotes: HashMap<String, RemoteConfig> =
        serde_json::from_str(&content).map_err(|e| EngramError::Serialization(e))?;

    let remote_config = remotes
        .get(remote_name)
        .ok_or_else(|| EngramError::Validation(format!("Remote '{}' not found", remote_name)))?
        .clone();

    println!("📡 Remote URL: {}", remote_config.url);

    let repo = open_workspace_repo()?;

    // Fetch refs/engram/* into refs/engram/remote/<name>/*
    let refspec = format!("+refs/engram/*:refs/engram/remote/{}/*", remote_name);
    gix_fetch(&repo, &remote_config.url, auth, &refspec)?;
    println!("   Fetch complete.");

    // --- Version-aware merge (same logic as git2 version, using gix helpers) ---
    let remote_prefix = format!("refs/engram/remote/{}/", remote_name);
    let sidecar_segment = "/v";
    let mut outcomes: Vec<PullEntityOutcome> = Vec::new();

    let all_refs = list_all_refs(&repo)?;

    // Build local max-version map
    let mut local_max_version: HashMap<(String, String), u64> = HashMap::new();
    for (name, _) in &all_refs {
        if !name.starts_with("refs/engram/") || name.starts_with("refs/engram/remote/") {
            continue;
        }
        let after = &name["refs/engram/".len()..];
        if let Some(v_pos) = after.find("/v") {
            let entity_type = &after[..v_pos];
            let rest = &after[v_pos + 2..];
            if let Some(slash_pos) = rest.find('/') {
                let version_str = &rest[..slash_pos];
                let uuid = &rest[slash_pos + 1..];
                if let Ok(n) = version_str.parse::<u64>() {
                    let key = (entity_type.to_string(), uuid.to_string());
                    let entry = local_max_version.entry(key).or_insert(0);
                    if n > *entry {
                        *entry = n;
                    }
                }
            }
        }
    }

    // Check for workspace config project_id update
    let workspace_config_ref = format!("refs/engram/remote/{}/config/workspace", remote_name);
    let mut new_project_id: Option<String> = None;
    if let Some((_name, oid)) = all_refs.iter().find(|(n, _)| n == &workspace_config_ref) {
        if let Ok(data) = read_blob(&repo, oid) {
            if let Ok(json) = serde_json::from_slice::<serde_json::Value>(&data) {
                if let Some(pid) = json.get("project_id").and_then(|v| v.as_str()) {
                    if remote_config.project_id.as_deref() != Some(pid) {
                        new_project_id = Some(pid.to_string());
                    }
                }
            }
        }
    }

    // Iterate remote primary entity refs
    for (ref_name, remote_oid) in &all_refs {
        if !ref_name.starts_with(&remote_prefix) {
            continue;
        }
        let after_prefix = &ref_name[remote_prefix.len()..];

        if after_prefix.contains(sidecar_segment) || after_prefix.starts_with("config/") {
            continue;
        }

        let slash_pos = match after_prefix.find('/') {
            Some(p) => p,
            None => continue,
        };
        let entity_type = &after_prefix[..slash_pos];
        let uuid = &after_prefix[slash_pos + 1..];
        if uuid.contains('/') {
            continue;
        }

        // Determine remote version from remote sidecar
        let remote_sidecar_prefix = format!("refs/engram/remote/{}/{}/v", remote_name, entity_type);
        let remote_sidecar_suffix = format!("/{}", uuid);
        let remote_version: u64 = all_refs
            .iter()
            .filter(|(n, _)| {
                n.starts_with(&remote_sidecar_prefix) && n.ends_with(&remote_sidecar_suffix)
            })
            .filter_map(|(n, _)| {
                let after = &n[remote_sidecar_prefix.len()..];
                let version_part = &after[..after.len() - remote_sidecar_suffix.len()];
                version_part.parse::<u64>().ok()
            })
            .max()
            .unwrap_or(0);

        let key = (entity_type.to_string(), uuid.to_string());
        let local_max = *local_max_version.get(&key).unwrap_or(&0);

        let remote_content = match read_blob(&repo, remote_oid) {
            Ok(c) => c,
            Err(_) => continue,
        };

        let local_ref_name = format!("refs/engram/{}/{}", entity_type, uuid);
        let local_content: Option<Vec<u8>> = all_refs
            .iter()
            .find(|(n, _)| n == &local_ref_name)
            .and_then(|(_, oid)| read_blob(&repo, oid).ok());

        let outcome = if remote_version > local_max {
            if !dry_run {
                set_ref(&repo, &local_ref_name, remote_oid, false,
                    &format!("pull: merge {} {} v{} from {}", entity_type, uuid, remote_version, remote_name))?;
            }
            PullEntityOutcome::Merged {
                entity_type: entity_type.to_string(),
                uuid: uuid.to_string(),
                remote_version,
            }
        } else if remote_version == local_max && remote_version > 0 {
            if local_content.as_deref() == Some(&remote_content) {
                PullEntityOutcome::UpToDate {
                    entity_type: entity_type.to_string(),
                    uuid: uuid.to_string(),
                }
            } else {
                PullEntityOutcome::Conflict {
                    entity_type: entity_type.to_string(),
                    uuid: uuid.to_string(),
                    version: remote_version,
                }
            }
        } else if local_max > remote_version {
            PullEntityOutcome::LocalNewer {
                entity_type: entity_type.to_string(),
                uuid: uuid.to_string(),
                local_version: local_max,
            }
        } else {
            // Both 0 — new from remote
            if !dry_run {
                set_ref(&repo, &local_ref_name, remote_oid, false,
                    &format!("pull: new {} {} from {}", entity_type, uuid, remote_name))?;
            }
            PullEntityOutcome::Merged {
                entity_type: entity_type.to_string(),
                uuid: uuid.to_string(),
                remote_version: 0,
            }
        };

        outcomes.push(outcome);
    }

    // Update project_id
    if let Some(pid) = new_project_id {
        if !dry_run {
            if let Some(cfg) = remotes.get_mut(remote_name) {
                cfg.project_id = Some(pid.clone());
                let config_content = serde_json::to_string_pretty(&remotes)
                    .map_err(|e| EngramError::Serialization(e))?;
                fs::write(config_path, config_content).map_err(|e| EngramError::Io(e))?;
                println!("   Updated remote project_id: {}", &pid[..16]);
            }
        }
    }

    // Print summary
    let merged = outcomes.iter().filter(|o| matches!(o, PullEntityOutcome::Merged { .. })).count();
    let up_to_date = outcomes.iter().filter(|o| matches!(o, PullEntityOutcome::UpToDate { .. })).count();
    let conflicts = outcomes.iter().filter(|o| matches!(o, PullEntityOutcome::Conflict { .. })).count();
    let local_newer = outcomes.iter().filter(|o| matches!(o, PullEntityOutcome::LocalNewer { .. })).count();

    println!();
    println!("Pull summary for '{}':", remote_name);
    println!("  Merged (remote newer): {}", merged);
    println!("  Up to date:            {}", up_to_date);
    println!("  Conflicts:             {}", conflicts);
    println!("  Skipped (local newer): {}", local_newer);

    for o in &outcomes {
        if let PullEntityOutcome::Conflict { entity_type, uuid, version } = o {
            println!("  CONFLICT {}/{} at v{} — use 'engram sync resolve' to resolve", entity_type, uuid, version);
        }
    }

    if dry_run {
        println!("(dry-run: no changes written)");
    }

    Ok(outcomes)
}

/// Push to remote using gix transport + pack generation
pub fn push_to_remote_gix(
    remote_name: &str,
    auth: &crate::storage::RemoteAuth,
    dry_run: bool,
) -> Result<usize, EngramError> {
    use super::sync::RemoteConfig;

    println!("📤 Pushing to remote '{}'...", remote_name);
    if dry_run {
        println!("   (dry-run — no refs will be pushed)");
    }

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

    println!("📡 Remote URL: {}", remote_config.url);

    let repo = open_workspace_repo()?;

    // Collect all local refs/engram/* refs (excluding refs/engram/remote/*)
    let all_refs = list_all_refs(&repo)?;
    let local_engram_refs: Vec<(String, String)> = all_refs
        .iter()
        .filter(|(name, _)| name.starts_with("refs/engram/") && !name.starts_with("refs/engram/remote/"))
        .cloned()
        .collect();

    if local_engram_refs.is_empty() {
        println!("   No refs/engram/* refs found locally — nothing to push.");
        return Ok(0);
    }

    println!(
        "   Found {} local engram refs to push.",
        local_engram_refs.len()
    );

    if dry_run {
        for (r, _) in &local_engram_refs {
            println!("   would push: {}", r);
        }
        println!("(dry-run: no refs pushed)");
        return Ok(local_engram_refs.len());
    }

    // Generate pack from all referenced objects
    let oids: Vec<gix::ObjectId> = local_engram_refs
        .iter()
        .filter_map(|(_, oid)| gix::ObjectId::from_hex(oid.as_bytes()).ok())
        .collect();
    let pack_data = repo.pack_from_objects(&oids)
        .map_err(|e| EngramError::Git(format!("Failed to generate pack: {}", e)))?;

    // Set up russh env for SSH auth
    set_russh_env(auth);

    // Connect to remote and push
    let remote = repo.remote_at(remote_config.url.as_str())
        .map_err(|e| EngramError::Git(format!("Failed to create remote: {:?}", e)))?;

    let connection = remote.connect(gix::remote::Direction::Push)
        .map_err(|e| EngramError::Git(format!("Failed to connect to '{}': {:?}", remote_config.url, e)))?;

    // Build ref updates — all are force-push (old_id = zero for create)
    let zero_id = gix::ObjectId::null(repo.object_hash());
    let ref_updates: Vec<gix::remote::push::RefUpdate> = local_engram_refs
        .iter()
        .map(|(name, new_oid)| {
            let new_id = gix::ObjectId::from_hex(new_oid.as_bytes())
                .unwrap_or_else(|_| zero_id.clone());
            gix::remote::push::RefUpdate {
                old_id: zero_id.clone(),
                new_id,
                name: name.as_bytes().into(),
            }
        })
        .collect();

    let _outcome = connection.push(ref_updates, &pack_data)
        .map_err(|e| EngramError::Git(format!("Push failed: {:?}", e)))?;

    println!(
        "✅ Pushed {} engram refs to '{}'",
        local_engram_refs.len(),
        remote_name
    );
    Ok(local_engram_refs.len())
}

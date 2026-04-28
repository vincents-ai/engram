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
#[allow(dead_code)]
fn open_workspace_repo() -> Result<gix::Repository, EngramError> {
    let cwd = std::env::current_dir().map_err(|e| EngramError::Io(e))?;
    gix::open(&cwd).map_err(|e| EngramError::Git(format!("Failed to open repository: {}", e)))
}

/// List all refs as (name, oid) pairs using gix
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

    // Delete the ref
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

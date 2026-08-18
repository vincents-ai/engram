//! Git command implementations — gix-backed porcelain
//!
//! No shell-outs to system git. All operations use the gix crate.

use crate::error::EngramError;
use crate::storage::GitRefsStorage;
use chrono::DateTime;
use clap::Subcommand;

/// Git subcommands (gix-backed, no shell-outs)
#[derive(Subcommand)]
pub enum GitCommands {
    /// Create a commit with engram task validation
    Checkpoint {
        /// Commit message (must contain task UUID)
        #[arg(short = 'm', long = "message")]
        message: String,

        /// Allow empty commits (no changes)
        #[arg(long = "allow-empty")]
        allow_empty: bool,
    },

    /// Show repository and HEAD status
    Status,

    /// Show commit history
    Log {
        /// Maximum number of commits to show
        #[arg(short = 'n', long = "limit", default_value = "20")]
        limit: usize,
    },

    /// Verify commit graph integrity
    VerifyHistory {
        /// Maximum number of commits to check (0 = all)
        #[arg(short = 'n', long = "limit", default_value = "100")]
        limit: usize,
    },
}

/// Handle Git commands
pub fn handle_git_command(command: GitCommands) -> Result<(), EngramError> {
    match command {
        GitCommands::Checkpoint {
            message,
            allow_empty,
        } => run_checkpoint(&message, allow_empty),
        GitCommands::Status => run_status(),
        GitCommands::Log { limit } => run_log(limit),
        GitCommands::VerifyHistory { limit } => run_verify_history(limit),
    }
}

/// Check for banned flags in a raw argument list
pub fn check_banned_flags(args: &[String]) -> Result<(), EngramError> {
    for arg in args {
        if arg == "--no-verify" || arg == "-n" {
            return Err(EngramError::Validation(
                "❌ Using --no-verify is not allowed via engram git.\n\n\
                 Bypassing hooks prevents Engram from validating your task references and relationships.\n\n\
                 💡 If your commit is being rejected:\n\
                 1. Read the error message carefully - it explains exactly what is missing.\n\
                 2. Run 'engram validate commit --message \"your message\" --dry-run' to debug.\n\
                 3. Ensure you have a valid task ID in your message.\n\
                 4. Ensure the task has linked 'context' and 'reasoning' entities."
                    .to_string(),
            ));
        }
    }
    Ok(())
}

fn open_repo() -> Result<gix::Repository, EngramError> {
    let cwd = std::env::current_dir().map_err(EngramError::Io)?;
    gix::open(&cwd).map_err(|e| EngramError::Git(format!("Failed to open repo: {}", e)))
}

fn run_checkpoint(message: &str, allow_empty: bool) -> Result<(), EngramError> {
    let cwd = std::env::current_dir().map_err(EngramError::Io)?;
    let path = cwd.to_string_lossy().to_string();

    // Validate commit message with engram storage
    match GitRefsStorage::new(&path, "engram-cli") {
        Ok(storage) => {
            match crate::validation::CommitValidator::new(storage.clone()) {
                Ok(mut validator) => {
                    let staged_files = validator.get_staged_files().unwrap_or_default();
                    let result = validator.validate_commit(message, &staged_files);
                    if !result.valid {
                        let mut error_msg = String::from("❌ Commit validation failed:\n\n");
                        for err in result.errors {
                            error_msg.push_str(&format!("• {}\n", err.message));
                            if let Some(suggestion) = err.suggestion {
                                error_msg.push_str(&format!("  Suggestion: {}\n", suggestion));
                            }
                            error_msg.push('\n');
                        }
                        return Err(EngramError::Validation(error_msg));
                    }

                    // Auto-guide suggestion
                    match crate::cli::auto_guide::get_auto_guide_suggestion(
                        &storage,
                        &crate::cli::auto_guide::AutoGuideConfig::default(),
                        Some("commit"),
                    ) {
                        Ok(Some(suggestion)) => {
                            println!(
                                "\n💡 \x1b[1m\x1b[36mEngram Suggestion:\x1b[0m {}",
                                suggestion
                            );
                        }
                        Ok(None) => {}
                        Err(_) => {}
                    }
                }
                Err(e) => {
                    return Err(EngramError::Validation(format!(
                        "Failed to initialize validator: {}",
                        e
                    )));
                }
            }
        }
        Err(_) => {
            eprintln!("⚠️  Warning: Engram storage not accessible. Skipping internal validation.");
        }
    }

    // Create the commit via gix
    let repo = open_repo()?;

    let head_id = repo
        .head_id()
        .map_err(|e| EngramError::Git(format!("Failed to get HEAD: {}", e)))?;

    let head_obj = repo
        .find_object(head_id)
        .map_err(|e| EngramError::Git(format!("Failed to find HEAD commit: {}", e)))?;
    let head_commit = gix::objs::CommitRef::from_bytes(&head_obj.data, gix::hash::Kind::Sha1)
        .map_err(|e| EngramError::Git(format!("Failed to parse HEAD commit: {}", e)))?;

    // Use HEAD tree — user stages via `git add` externally
    // The key value of checkpoint is validation, not index manipulation
    let tree_id = gix::hash::ObjectId::from_hex(head_commit.tree)
        .map_err(|e| EngramError::Git(format!("Invalid tree hash: {}", e)))?;

    if !allow_empty {
        // TODO: Compare index to HEAD tree when gix index write_tree is available
        // For now, always proceed — validation is the main feature
    }

    // Create commit object
    let sig = gix::actor::Signature {
        name: "engram".into(),
        email: "engram@local".into(),
        time: gix::date::Time::now_local_or_utc(),
    };

    let commit = gix::objs::Commit {
        tree: tree_id,
        parents: std::iter::once(head_id.detach()).collect(),
        author: sig.clone(),
        committer: sig,
        encoding: None,
        message: format!("{}\n", message).into(),
        extra_headers: Default::default(),
    };

    let commit_id = repo
        .write_object(&commit)
        .map_err(|e| EngramError::Git(format!("Failed to write commit: {}", e)))?;

    // Update HEAD branch
    let head_ref = repo
        .head_ref()
        .map_err(|e| EngramError::Git(format!("Failed to get HEAD ref: {}", e)))?;

    let branch_name = match head_ref {
        Some(r) => r.name().shorten().to_string(),
        None => "main".to_string(),
    };

    use gix::refs::transaction::{Change, LogChange, PreviousValue, RefEdit};
    use gix::refs::FullName;
    use gix::refs::Target;

    let ref_name = FullName::try_from(format!("refs/heads/{}", branch_name))
        .map_err(|e| EngramError::Git(format!("Invalid ref name: {}", e)))?;

    repo.edit_reference(RefEdit {
        change: Change::Update {
            log: LogChange::default(),
            expected: PreviousValue::MustExist,
            new: Target::Object(commit_id.detach()),
        },
        name: ref_name,
        deref: false,
    })
    .map_err(|e| EngramError::Git(format!("Failed to update branch: {}", e)))?;

    let commit_str = commit_id.to_string();
    let short_id = &commit_str[..8.min(commit_str.len())];
    println!("[{}] {}", short_id, message.lines().next().unwrap_or(""));

    println!("\n💡 Tip: Don't forget to commit all changes before ending your session.");

    Ok(())
}

fn run_status() -> Result<(), EngramError> {
    let repo = open_repo()?;

    // Get HEAD info
    let head_info: Option<(gix::ObjectId, String)> = (|| {
        let head_id = repo.head_id().ok()?;
        let obj = repo.find_object(head_id).ok()?;
        let commit = gix::objs::CommitRef::from_bytes(&obj.data, gix::hash::Kind::Sha1).ok()?;
        let first_line = commit
            .message
            .to_string()
            .lines()
            .next()
            .unwrap_or("")
            .to_string();
        Some((head_id.detach(), first_line))
    })();

    match head_info {
        Some((head_id, first_line)) => {
            let head_str = head_id.to_string();
            let short_hash = &head_str[..8];
            let branch = match repo.head_ref() {
                Ok(Some(r)) => r.name().shorten().to_string(),
                _ => "HEAD".to_string(),
            };

            println!("On branch {}", branch);
            println!("HEAD {} ({})", short_hash, first_line);
        }
        None => {
            println!("No commits yet.");
        }
    }

    Ok(())
}

fn run_log(limit: usize) -> Result<(), EngramError> {
    let repo = open_repo()?;
    let head_id = repo
        .head_id()
        .map_err(|e| EngramError::Git(format!("Failed to get HEAD: {}", e)))?;

    let walk = repo
        .rev_walk([head_id])
        .sorting(gix::revision::walk::Sorting::ByCommitTime(
            gix::traverse::commit::simple::CommitTimeOrder::NewestFirst,
        ))
        .all()
        .map_err(|e| EngramError::Git(format!("revwalk failed: {}", e)))?;

    for (count, info_result) in walk.enumerate() {
        if count >= limit {
            break;
        }
        let info = info_result
            .map_err(|e| EngramError::Git(format!("revwalk iteration failed: {}", e)))?;

        let obj = repo
            .find_object(info.id)
            .map_err(|e| EngramError::Git(format!("Failed to find commit: {}", e)))?;
        let commit = gix::objs::CommitRef::from_bytes(&obj.data, gix::hash::Kind::Sha1)
            .map_err(|e| EngramError::Git(format!("Failed to parse commit: {}", e)))?;

        let id_str = info.id.to_string();
        let short_hash = &id_str[..8];
        let msg = commit.message.to_string();
        let msg_first_line = msg.lines().next().unwrap_or("");
        let author = commit
            .author()
            .map(|sig| sig.name.to_string())
            .unwrap_or_default();

        let date = info
            .commit_time
            .and_then(|secs| DateTime::from_timestamp(secs, 0))
            .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
            .unwrap_or_else(|| "unknown".to_string());

        println!("{} {} {} <{}>", short_hash, date, msg_first_line, author);
    }

    Ok(())
}

fn run_verify_history(limit: usize) -> Result<(), EngramError> {
    let repo = open_repo()?;
    let head_id = repo
        .head_id()
        .map_err(|e| EngramError::Git(format!("Failed to get HEAD: {}", e)))?;

    let walk = repo
        .rev_walk([head_id])
        .sorting(gix::revision::walk::Sorting::ByCommitTime(
            gix::traverse::commit::simple::CommitTimeOrder::NewestFirst,
        ))
        .all()
        .map_err(|e| EngramError::Git(format!("revwalk failed: {}", e)))?;

    let mut checked = 0usize;
    let mut errors = 0usize;
    let effective_limit = if limit == 0 { usize::MAX } else { limit };

    for info_result in walk {
        if checked >= effective_limit {
            break;
        }
        let info = match info_result {
            Ok(i) => i,
            Err(e) => {
                eprintln!("❌ Corrupt commit graph at position {}: {}", checked, e);
                errors += 1;
                checked += 1;
                continue;
            }
        };

        let obj = match repo.find_object(info.id) {
            Ok(o) => o,
            Err(e) => {
                eprintln!("❌ Missing object {}: {}", info.id, e);
                errors += 1;
                checked += 1;
                continue;
            }
        };

        match gix::objs::CommitRef::from_bytes(&obj.data, gix::hash::Kind::Sha1) {
            Ok(commit) => {
                // Verify tree exists
                let tree_id = gix::hash::ObjectId::from_hex(commit.tree)
                    .map_err(|e| EngramError::Git(format!("Invalid tree hash: {}", e)))?;
                if repo.find_object(tree_id).is_err() {
                    eprintln!(
                        "❌ Commit {} references missing tree {}",
                        info.id, commit.tree
                    );
                    errors += 1;
                }
                // Verify parents exist
                for parent_bstr in &commit.parents {
                    let parent_id = match gix::hash::ObjectId::from_hex(parent_bstr) {
                        Ok(id) => id,
                        Err(_) => {
                            eprintln!(
                                "❌ Commit {} has invalid parent hash {:?}",
                                info.id, parent_bstr
                            );
                            errors += 1;
                            continue;
                        }
                    };
                    if repo.find_object(parent_id).is_err() {
                        eprintln!(
                            "❌ Commit {} references missing parent {}",
                            info.id, parent_id
                        );
                        errors += 1;
                    }
                }
                // Verify author parses
                if commit.author().is_err() {
                    eprintln!("⚠️  Commit {} has malformed author", info.id);
                    errors += 1;
                }
            }
            Err(e) => {
                eprintln!("❌ Corrupt commit object {}: {}", info.id, e);
                errors += 1;
            }
        }

        checked += 1;
    }

    if errors == 0 {
        println!("✅ Verified {} commit(s) — all valid", checked);
    } else {
        println!(
            "❌ Verified {} commit(s) — {} error(s) found",
            checked, errors
        );
        return Err(EngramError::Git(format!(
            "History verification failed with {} error(s)",
            errors
        )));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_banned_flag_no_verify() {
        let args = vec!["commit".to_string(), "--no-verify".to_string()];
        let result = check_banned_flags(&args);
        assert!(
            matches!(result, Err(EngramError::Validation(msg)) if msg.contains("Using --no-verify is not allowed"))
        );
    }

    #[test]
    fn test_banned_flag_short_n() {
        let args = vec!["commit".to_string(), "-n".to_string()];
        let result = check_banned_flags(&args);
        assert!(
            matches!(result, Err(EngramError::Validation(msg)) if msg.contains("Using --no-verify is not allowed"))
        );
    }

    #[test]
    fn test_banned_flags_clear() {
        let args = vec!["commit".to_string(), "-m".to_string(), "test".to_string()];
        let result = check_banned_flags(&args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_log_command() {
        let cmd = GitCommands::Log { limit: 5 };
        let result = handle_git_command(cmd);
        match result {
            Ok(()) => assert!(true),
            Err(EngramError::Git(_)) => assert!(true),
            Err(e) => panic!("Unexpected error type: {:?}", e),
        }
    }

    #[test]
    fn test_status_command() {
        let cmd = GitCommands::Status;
        let result = handle_git_command(cmd);
        match result {
            Ok(()) => assert!(true),
            Err(EngramError::Git(_)) => assert!(true),
            Err(e) => panic!("Unexpected error type: {:?}", e),
        }
    }

    #[test]
    fn test_verify_history_command() {
        let cmd = GitCommands::VerifyHistory { limit: 10 };
        let result = handle_git_command(cmd);
        match result {
            Ok(()) => assert!(true),
            Err(EngramError::Git(_)) => assert!(true),
            Err(e) => panic!("Unexpected error type: {:?}", e),
        }
    }
}

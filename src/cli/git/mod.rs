//! Git command implementations

use crate::error::EngramError;
use clap::Subcommand;
use std::process::Command;

/// Git commands
#[derive(Subcommand)]
pub enum GitCommands {
    /// Run git command without --no-verify
    #[command(external_subcommand)]
    External(Vec<String>),
}

/// Handle Git commands
pub fn handle_git_command(args: Vec<String>) -> Result<(), EngramError> {
    if args.is_empty() {
        return Err(EngramError::Validation(
            "No git command provided".to_string(),
        ));
    }

    // Check for banned flags
    for arg in &args {
        if arg == "--no-verify" || arg == "-n" {
            // Note: -n is short for --no-verify in some contexts, though git commit uses it for --no-verify?
            // git commit -n is indeed --no-verify.
            // Let's be strict about --no-verify.
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

    // Handle commit command specifically to force validation
    if let Some(cmd) = args.first() {
        if cmd == "commit" {
            // Find message index
            let mut message = String::new();
            if let Some(idx) = args
                .iter()
                .position(|arg| arg == "-m" || arg == "--message")
            {
                if idx + 1 < args.len() {
                    message = args[idx + 1].clone();
                }
            }

            // If we have a message, run validation
            if !message.is_empty() {
                // Instantiate storage and validator to check the commit message
                // This is a safety measure to ensure validation runs even without hooks
                let current_dir = std::env::current_dir()
                    .map_err(|e| EngramError::Io(e))?
                    .to_string_lossy()
                    .to_string();

                // Debug print
                // println!("DEBUG: Validating commit message: '{}'", message);

                // We try to initialize storage. If it fails (e.g. not an engram repo yet),
                // we might want to warn or skip. But assuming 'engram git' is used in an engram repo.
                match crate::storage::GitRefsStorage::new(&current_dir, "engram-cli") {
                    Ok(storage) => {
                        match crate::validation::CommitValidator::new(storage.clone()) {
                            Ok(mut validator) => {
                                // Get staged files for validation
                                let staged_files = validator.get_staged_files().unwrap_or_default();

                                let result = validator.validate_commit(&message, &staged_files);

                                if !result.valid {
                                    // Format the errors nicely
                                    let mut error_msg =
                                        String::from("❌ Commit validation failed:\n\n");
                                    for err in result.errors {
                                        error_msg.push_str(&format!("• {}\n", err.message));
                                        if let Some(suggestion) = err.suggestion {
                                            error_msg.push_str(&format!(
                                                "  Suggestion: {}\n",
                                                suggestion
                                            ));
                                        }
                                        error_msg.push('\n');
                                    }

                                    return Err(EngramError::Validation(error_msg));
                                }

                                // Validation passed
                                // Check for auto-guide suggestions
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
                                    Err(_) => {
                                        // Silently fail to not disrupt flow
                                    }
                                }
                            }
                            Err(e) => {
                                // If we can't create validator, that's a problem but maybe not blocking?
                                // Let's log it but maybe proceed if it's just a config issue?
                                // No, safely fail.
                                return Err(EngramError::Validation(format!(
                                    "Failed to initialize validator: {}",
                                    e
                                )));
                            }
                        }
                    }
                    Err(_) => {
                        // If we can't initialize storage, it might not be a repo yet or other issues.
                        // We'll proceed with git command but warn.
                        eprintln!("⚠️  Warning: Engram storage not accessible. Skipping internal validation.");
                    }
                }
            }
        }
    }

    // Construct and execute the git command
    let status = Command::new("git")
        .args(&args)
        .status()
        .map_err(|e| EngramError::Io(e))?;

    if !status.success() {
        return Err(EngramError::Git(format!(
            "Git command failed with status: {}",
            status
        )));
    }

    // specific post-command logic
    if let Some(subcmd) = args.first() {
        if subcmd == "add" || subcmd == "commit" {
            // Check status to remind user of remaining changes
            let status_output = Command::new("git")
                .args(&["status", "-s"])
                .output()
                .map_err(EngramError::Io)?;

            if !status_output.stdout.is_empty() {
                println!("\n📦 Remaining unstaged/untracked files:");
                let output_str = String::from_utf8_lossy(&status_output.stdout);
                for line in output_str.lines() {
                    println!("  {}", line);
                }
                println!(
                    "\n💡 Tip: Don't forget to commit all changes before ending your session."
                );
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_git_no_args() {
        let args = vec![];
        let result = handle_git_command(args);
        assert!(
            matches!(result, Err(EngramError::Validation(msg)) if msg == "No git command provided")
        );
    }

    #[test]
    fn test_git_banned_flag_no_verify() {
        let args = vec!["commit".to_string(), "--no-verify".to_string()];
        let result = handle_git_command(args);
        assert!(
            matches!(result, Err(EngramError::Validation(msg)) if msg.contains("Using --no-verify is not allowed"))
        );
    }

    #[test]
    fn test_git_banned_flag_short_n() {
        let args = vec!["commit".to_string(), "-n".to_string()];
        let result = handle_git_command(args);
        assert!(
            matches!(result, Err(EngramError::Validation(msg)) if msg.contains("Using --no-verify is not allowed"))
        );
    }

    #[test]
    fn test_git_status_execution() {
        // Exercises the execution path for a read-only command
        let args = vec!["status".to_string()];
        let result = handle_git_command(args);
        // Should succeed in a repo, or fail with Git error if not (which is also a handled path)
        match result {
            Ok(_) => assert!(true),
            Err(EngramError::Git(_)) => assert!(true),
            Err(e) => panic!("Unexpected error type: {:?}", e),
        }
    }

    #[test]
    fn test_git_command_failure() {
        // Exercises the error handling path for failed commands
        let args = vec![
            "checkout".to_string(),
            "non-existent-branch-xyz-123".to_string(),
        ];
        let result = handle_git_command(args);
        assert!(matches!(result, Err(EngramError::Git(_))));
    }

    #[test]
    fn test_git_add_post_command_logic() {
        // Exercises the post-command logic (lines 58+) by simulating an 'add' command.
        // We use --dry-run to avoid actually staging files during the test.
        let args = vec!["add".to_string(), "--dry-run".to_string(), ".".to_string()];
        let result = handle_git_command(args);

        match result {
            Ok(_) => assert!(true),
            Err(EngramError::Git(_)) => assert!(true), // Acceptable if git fails
            Err(e) => panic!("Unexpected error type: {:?}", e),
        }
    }
}

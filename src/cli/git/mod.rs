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

    // Check if it is a commit command and ensure we aren't bypassing hooks implicitly if that were possible (it's mostly --no-verify)

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
}

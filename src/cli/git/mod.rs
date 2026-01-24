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
            return Err(EngramError::Validation("Using --no-verify is not allowed via engram git. Please use standard git if you must bypass hooks.".to_string()));
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

    Ok(())
}

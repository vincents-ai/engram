//! Validation command implementations

use crate::error::EngramError;
use crate::storage::{RelationshipStorage, Storage};
use crate::validation::{CommitValidator, HookManager};
use clap::Subcommand;

/// Validation commands
#[derive(Debug, Subcommand)]
pub enum ValidationCommands {
    /// Validate a commit
    Commit {
        /// Commit message to validate
        #[arg(long, short)]
        message: String,

        /// Dry run (don't require actual git repo)
        #[arg(long)]
        dry_run: bool,
    },
    /// Manage git hooks
    Hook {
        #[command(subcommand)]
        command: HookCommands,
    },
    /// Check validation setup
    Check,
}

/// Hook management commands
#[derive(Debug, Subcommand)]
pub enum HookCommands {
    /// Install pre-commit hook
    Install,
    /// Uninstall pre-commit hook
    Uninstall,
    /// Show hook status
    Status,
}

/// Handle validation commands
pub fn handle_validation_command<S: Storage + RelationshipStorage>(
    command: ValidationCommands,
    storage: S,
) -> Result<(), EngramError> {
    match command {
        ValidationCommands::Commit { message, dry_run } => {
            handle_commit_validation(storage, &message, dry_run)?;
        }
        ValidationCommands::Hook { command } => {
            handle_hook_command(storage, command)?;
        }
        ValidationCommands::Check => {
            handle_check_command(storage)?;
        }
    }
    Ok(())
}

/// Handle commit validation
fn handle_commit_validation<S: Storage + RelationshipStorage>(
    storage: S,
    message: &str,
    dry_run: bool,
) -> Result<(), EngramError> {
    let mut validator = CommitValidator::new(storage)?;

    let staged_files = if dry_run {
        vec![]
    } else {
        validator.get_staged_files()?
    };

    let result = validator.validate_commit(message, &staged_files);

    if result.valid {
        println!("✅ Validation passed");
        if !result.task_id.as_ref().map_or(true, |id| id == "exempt") {
            println!("📋 Task ID: {}", result.task_id.unwrap());
        }
    } else {
        println!("❌ Validation failed");
        for error in result.errors {
            println!("  • {}", error.message);
            if let Some(suggestion) = error.suggestion {
                println!("    💡 {}", suggestion);
            }
        }

        std::process::exit(1);
    }

    Ok(())
}

/// Handle hook management commands
fn handle_hook_command<S: Storage + RelationshipStorage>(
    _storage: S,
    command: HookCommands,
) -> Result<(), EngramError> {
    let git_dir = ".";
    let mut hook_manager = HookManager::new(git_dir)?;

    match command {
        HookCommands::Install => {
            hook_manager.install()?;
            println!("✅ Hook installed successfully");
        }
        HookCommands::Uninstall => {
            hook_manager.uninstall()?;
            println!("✅ Hook uninstalled successfully");
        }
        HookCommands::Status => {
            hook_manager.show_status()?;
        }
    }

    Ok(())
}

/// Handle check command
fn handle_check_command<S: Storage + RelationshipStorage>(storage: S) -> Result<(), EngramError> {
    let mut validator = CommitValidator::new(storage)?;
    let git_dir = ".";
    let hook_manager = HookManager::new(git_dir)?;

    let status = hook_manager.verify_setup()?;

    if status.is_healthy() {
        println!("🎉 All validation systems are working correctly!");
    } else {
        println!("⚠️  Issues found:");
        for issue in status.get_issues() {
            println!("  • {}", issue);
        }
        println!("\nRun 'engram validation hook install' to fix setup issues.");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_command_parsing() {
        // Test basic command structure
        let _cmd = ValidationCommands::Commit {
            message: "test".to_string(),
            dry_run: false,
        };
    }
}

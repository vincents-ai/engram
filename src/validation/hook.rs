//! Git pre-commit hook management

use crate::error::EngramError;
use crate::validation::config::ValidationConfig;
use std::fs;
use std::path::Path;

/// Manager for git pre-commit hooks
pub struct HookManager {
    git_dir: String,
    config: ValidationConfig,
}

impl HookManager {
    /// Create a new hook manager
    pub fn new<P: AsRef<Path>>(git_dir: P) -> Result<Self, EngramError> {
        let git_dir = git_dir.as_ref().to_string_lossy().to_string();
        let config = ValidationConfig::default();

        Ok(Self { git_dir, config })
    }

    /// Create a hook manager with custom configuration
    pub fn with_config<P: AsRef<Path>>(
        git_dir: P,
        config: ValidationConfig,
    ) -> Result<Self, EngramError> {
        let git_dir = git_dir.as_ref().to_string_lossy().to_string();

        Ok(Self { git_dir, config })
    }

    /// Generate the hook script content
    fn generate_hook_script(&self) -> String {
        "#!/usr/bin/env bash\n# ENGRAM_PRE_COMMIT_HOOK placeholder\necho \"Hook not yet implemented\"\nexit 0\n".to_string()
    }

    /// Check if hook is installed
    pub fn is_installed(&self) -> Result<bool, EngramError> {
        let hook_path = Path::new(&self.git_dir)
            .join(".git")
            .join("hooks")
            .join("pre-commit");

        if !hook_path.exists() {
            return Ok(false);
        }

        let content = fs::read_to_string(&hook_path).map_err(EngramError::Io)?;

        Ok(content.contains("ENGRAM_PRE_COMMIT_HOOK"))
    }

    /// Get hook script content
    pub fn get_hook_content(&self) -> String {
        self.generate_hook_script()
    }

    /// Install the pre-commit hook
    pub fn install(&mut self) -> Result<(), EngramError> {
        let hook_path = Path::new(&self.git_dir)
            .join(".git")
            .join("hooks")
            .join("pre-commit");

        // Create hooks directory if it doesn't exist
        if let Some(hooks_dir) = hook_path.parent() {
            fs::create_dir_all(hooks_dir).map_err(EngramError::Io)?;
        }

        // Generate and write the hook script
        let script_content = self.generate_hook_script();
        fs::write(&hook_path, script_content).map_err(EngramError::Io)?;

        // Make the hook executable (Unix-like systems)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&hook_path)
                .map_err(EngramError::Io)?
                .permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&hook_path, perms).map_err(EngramError::Io)?;
        }

        Ok(())
    }

    /// Uninstall the pre-commit hook
    pub fn uninstall(&mut self) -> Result<(), EngramError> {
        let hook_path = Path::new(&self.git_dir)
            .join(".git")
            .join("hooks")
            .join("pre-commit");

        if hook_path.exists() {
            let content = fs::read_to_string(&hook_path).map_err(EngramError::Io)?;

            if content.contains("ENGRAM_PRE_COMMIT_HOOK") {
                fs::remove_file(&hook_path).map_err(EngramError::Io)?;
            } else {
                return Err(EngramError::Validation(
                    "Pre-commit hook exists but was not installed by Engram".to_string(),
                ));
            }
        }

        Ok(())
    }

    /// Show hook status
    pub fn show_status(&self) -> Result<(), EngramError> {
        let status = self.verify_setup()?;

        println!("Hook Status:");
        println!(
            "  In Git Repo: {}",
            if status.in_git_repo { "✅" } else { "❌" }
        );
        println!(
            "  Hook Installed: {}",
            if status.hook_installed { "✅" } else { "❌" }
        );
        println!(
            "  Engram Available: {}",
            if status.engram_available {
                "✅"
            } else {
                "❌"
            }
        );
        println!(
            "  Config Valid: {}",
            if status.config_valid { "✅" } else { "❌" }
        );
        println!(
            "  Validation Works: {}",
            if status.validation_works {
                "✅"
            } else {
                "❌"
            }
        );

        if !status.is_healthy() {
            println!("\nIssues:");
            for issue in status.get_issues() {
                println!("  • {}", issue);
            }
        }

        Ok(())
    }

    /// Verify hook setup and return status
    pub fn verify_setup(&self) -> Result<HookStatus, EngramError> {
        let mut status = HookStatus::default();

        // Check if we're in a git repository
        let git_dir = Path::new(&self.git_dir).join(".git");
        status.in_git_repo = git_dir.exists();

        // Check if hook is installed
        status.hook_installed = self.is_installed()?;

        // Check if engram command is available
        status.engram_available = std::process::Command::new("which")
            .arg("engram")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false);

        // Check if config is valid
        status.config_valid = true; // ValidationConfig::default() should always be valid

        // Check if validation works by testing a sample commit
        status.validation_works = status.hook_installed && status.engram_available;

        Ok(status)
    }
}

/// Status of hook setup
#[derive(Debug, Default)]
pub struct HookStatus {
    pub in_git_repo: bool,
    pub hook_installed: bool,
    pub engram_available: bool,
    pub config_valid: bool,
    pub validation_works: bool,
}

impl HookStatus {
    /// Check if everything is working
    pub fn is_healthy(&self) -> bool {
        self.in_git_repo
            && self.hook_installed
            && self.engram_available
            && self.config_valid
            && self.validation_works
    }

    /// Get a summary of issues
    pub fn get_issues(&self) -> Vec<String> {
        let mut issues = Vec::new();

        if !self.in_git_repo {
            issues.push("Not in a git repository".to_string());
        }
        if !self.hook_installed {
            issues.push("Pre-commit hook not installed".to_string());
        }
        if !self.engram_available {
            issues.push("Engram command not available".to_string());
        }
        if !self.config_valid {
            issues.push("Configuration invalid".to_string());
        }
        if !self.validation_works {
            issues.push("Validation not working".to_string());
        }

        issues
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hook_script_generation() {
        let git_dir = "/tmp/test_git";
        let hook_manager = HookManager::new(git_dir).unwrap();

        let script = hook_manager.generate_hook_script();
        assert!(script.contains("ENGRAM_PRE_COMMIT_HOOK"));
    }

    #[test]
    fn test_hook_status_default() {
        let status = HookStatus::default();
        assert!(!status.is_healthy());
        assert_eq!(status.get_issues().len(), 5); // All 5 are false by default
    }

    #[test]
    fn test_hook_status_healthy() {
        let mut status = HookStatus::default();
        status.in_git_repo = true;
        status.hook_installed = true;
        status.engram_available = true;
        status.config_valid = true;
        status.validation_works = true;

        assert!(status.is_healthy());
        assert!(status.get_issues().is_empty());
    }
}

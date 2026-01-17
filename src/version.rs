use std::env;
use std::process::Command;

/// Version information for Engram with compile-time and runtime fallback detection
#[derive(Debug, Clone)]
pub struct BuildInfo {
    pub package_version: String,
    pub commit_sha: String,
    pub commit_date: String,
    pub build_timestamp: String,
}

impl BuildInfo {
    pub fn get() -> Self {
        Self {
            package_version: env::var("CARGO_PKG_VERSION").unwrap_or_default(),
            commit_sha: env::var("ENGRAM_COMMIT_SHA").unwrap_or_else(|_| get_runtime_git_sha()),
            commit_date: env::var("ENGRAM_COMMIT_DATE").unwrap_or_else(|_| get_runtime_git_date()),
            build_timestamp: env::var("ENGRAM_BUILD_TIMESTAMP")
                .unwrap_or_else(|_| get_current_timestamp()),
        }
    }

    pub fn version_string(&self) -> String {
        if self.commit_sha != "unknown" && self.commit_sha.len() >= 8 {
            format!(
                "{} ({} {})",
                self.package_version,
                &self.commit_sha[..8],
                self.commit_date
            )
        } else {
            self.package_version.clone()
        }
    }
}

fn get_runtime_git_sha() -> String {
    run_git_command(&["log", "-1", "--format=%H"]).unwrap_or_default()
}

fn get_runtime_git_date() -> String {
    run_git_command(&["log", "-1", "--format=%ci"])
        .map(|s| s.split_whitespace().nth(1).unwrap_or_default().to_string())
        .unwrap_or_default()
}

fn run_git_command(args: &[&str]) -> Option<String> {
    match Command::new("git").args(args).output() {
        Ok(output) if output.status.success() => String::from_utf8_lossy(&output.stdout)
            .trim()
            .to_string()
            .into(),
        _ => None,
    }
}

fn get_current_timestamp() -> String {
    run_date_command(&["+%Y-%m-%dT%H:%M:%SZ"]).unwrap_or_default()
}

fn run_date_command(args: &[&str]) -> Option<String> {
    match Command::new("date").args(args).output() {
        Ok(output) => String::from_utf8_lossy(&output.stdout)
            .trim()
            .to_string()
            .into(),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_info_always_has_package_version() {
        let info = BuildInfo::get();
        assert!(!info.package_version.is_empty());
    }

    #[test]
    fn build_info_always_has_timestamp() {
        let info = BuildInfo::get();
        assert!(!info.build_timestamp.is_empty());
    }
}

impl BuildInfo {
    /// Get build info from environment variables set by build.rs
    /// Fallback to runtime detection if not available
    pub fn get() -> Self {
        Self {
            package_version: env::var("CARGO_PKG_VERSION")
                .unwrap_or_else(|_| "unknown".to_string()),
            commit_sha: env::var("ENGRAM_COMMIT_SHA").unwrap_or_else(|_| get_runtime_git_info().0),
            commit_date: env::var("ENGRAM_COMMIT_DATE")
                .unwrap_or_else(|_| get_runtime_git_info().1),
            build_timestamp: env::var("ENGRAM_BUILD_TIMESTAMP")
                .unwrap_or_else(|_| get_current_timestamp()),
        }
    }

    /// Get full version string with commit info if available
    pub fn version_string(&self) -> String {
        if self.commit_sha != "unknown" && self.commit_sha.len() >= 8 {
            format!(
                "{} ({} {})",
                self.package_version,
                &self.commit_sha[..8],
                self.commit_date
            )
        } else {
            self.package_version.clone()
        }
    }
}

/// Fallback: Get git information at runtime (used when build.rs not executed)
fn get_runtime_git_info() -> (String, String) {
    match Command::new("git")
        .args(&["log", "-1", "--format=%H %ci"])
        .output()
    {
        Ok(output) if output.status.success() => {
            let git_info = String::from_utf8_lossy(&output.stdout);
            let parts: Vec<&str> = git_info.split_whitespace().collect();
            if parts.len() >= 2 {
                (parts[0].to_string(), parts[1].to_string())
            } else {
                ("unknown".to_string(), "unknown".to_string())
            }
        }
        Err(_) => ("unknown".to_string(), "unknown".to_string()),
    }
}

/// Fallback: Get current timestamp
fn get_current_timestamp() -> String {
    match Command::new("date").args(&["+%Y-%m-%dT%H:%M:%SZ"]).output() {
        Ok(output) => String::from_utf8_lossy(&output.stdout).trim().to_string(),
        Err(_) => "unknown".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_info_fallback() {
        let info = BuildInfo::get();
        // Should always return something, even if git fails
        assert!(!info.package_version.is_empty());
        assert!(!info.build_timestamp.is_empty());
    }
}

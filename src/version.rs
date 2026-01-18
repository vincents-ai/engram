use std::env;
use std::process::Command;

/// Version information for Engram with git tag-based release management
#[derive(Debug, Clone)]
pub struct BuildInfo {
    pub package_version: String,
    pub git_tag: String,
    pub commit_sha: String,
    pub commit_date: String,
    pub build_timestamp: String,
    pub is_tagged_release: bool,
}

impl BuildInfo {
    pub fn get() -> Self {
        Self {
            package_version: env::var("CARGO_PKG_VERSION")
                .unwrap_or_else(|_| "unknown".to_string()),
            git_tag: env::var("ENGRAM_GIT_TAG").unwrap_or_else(|_| get_runtime_git_tag()),
            commit_sha: env::var("ENGRAM_COMMIT_SHA").unwrap_or_else(|_| get_runtime_git_sha()),
            commit_date: env::var("ENGRAM_COMMIT_DATE").unwrap_or_else(|_| get_runtime_git_date()),
            build_timestamp: env::var("ENGRAM_BUILD_TIMESTAMP")
                .unwrap_or_else(|_| get_current_timestamp()),
            is_tagged_release: env::var("ENGRAM_IS_TAGGED_RELEASE")
                .map(|val| val == "true")
                .unwrap_or(false),
        }
    }

    pub fn version_string(&self) -> String {
        if self.is_tagged_release {
            self.package_version.clone() // Clean version for tagged releases
        } else if !self.commit_sha.is_empty() && self.commit_sha.len() >= 8 {
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

fn get_runtime_git_tag() -> String {
    run_git_command(&["describe", "--tags", "--abbrev=0", "--exact-match"]).unwrap_or_default()
}

fn get_runtime_git_sha() -> String {
    run_git_command(&["log", "-1", "--format=%H"]).unwrap_or_else(|| "unknown".to_string())
}

fn get_runtime_git_date() -> String {
    run_git_command(&["log", "-1", "--format=%ci"])
        .map(|output| {
            output
                .split_whitespace()
                .next()
                .unwrap_or("unknown")
                .to_string()
        })
        .unwrap_or_else(|| "unknown".to_string())
}

fn run_git_command(args: &[&str]) -> Option<String> {
    match Command::new("git").args(args).output() {
        Ok(output) if output.status.success() => {
            let result = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if result.is_empty() {
                None
            } else {
                Some(result)
            }
        }
        _ => None,
    }
}

fn get_current_timestamp() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    format!("{}", duration.as_secs())
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
    fn build_info_version_string_format() {
        let info = BuildInfo::get();
        let version = info.version_string();
        assert!(!version.is_empty());
        // Should contain either just version or version with commit info
        assert!(version.contains(&info.package_version));
    }
}

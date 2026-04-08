use crate::entities::ResourceLimits;
use crate::feedback::{FeedbackStatus, StructuredFeedback};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreflightCheckResult {
    pub name: String,
    pub status: PreflightStatus,
    pub message: String,
    pub detail: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PreflightStatus {
    Pass,
    Warn,
    Fail,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreflightReport {
    pub checks: Vec<PreflightCheckResult>,
    pub overall_status: PreflightStatus,
}

impl StructuredFeedback for PreflightReport {
    fn to_json(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }

    fn summary(&self) -> String {
        let passed = self
            .checks
            .iter()
            .filter(|c| c.status == PreflightStatus::Pass)
            .count();
        let warned = self
            .checks
            .iter()
            .filter(|c| c.status == PreflightStatus::Warn)
            .count();
        let failed = self
            .checks
            .iter()
            .filter(|c| c.status == PreflightStatus::Fail)
            .count();
        let total = self.checks.len();
        format!(
            "Pre-flight: {}/{} passed, {} warning(s), {} failure(s)",
            passed, total, warned, failed
        )
    }

    fn status_code(&self) -> FeedbackStatus {
        match self.overall_status {
            PreflightStatus::Pass => FeedbackStatus::Success,
            PreflightStatus::Warn => FeedbackStatus::Warning,
            PreflightStatus::Fail => FeedbackStatus::Failed,
        }
    }
}

pub fn run_preflight_checks(
    workspace_dir: &Path,
    resource_limits: Option<&ResourceLimits>,
) -> PreflightReport {
    let mut checks = Vec::new();

    checks.push(check_resource_limits(resource_limits));
    checks.push(check_workspace_exists(workspace_dir));
    checks.push(check_workspace_writable(workspace_dir));
    checks.push(check_git_repo_clean(workspace_dir));
    checks.push(check_git_available());
    checks.push(check_engram_storage(workspace_dir));

    let overall_status = if checks.iter().any(|c| c.status == PreflightStatus::Fail) {
        PreflightStatus::Fail
    } else if checks.iter().any(|c| c.status == PreflightStatus::Warn) {
        PreflightStatus::Warn
    } else {
        PreflightStatus::Pass
    };

    PreflightReport {
        checks,
        overall_status,
    }
}

pub fn check_resource_limits(limits: Option<&ResourceLimits>) -> PreflightCheckResult {
    match limits {
        None => PreflightCheckResult {
            name: "resource_limits".into(),
            status: PreflightStatus::Warn,
            message: "No resource limits configured; using defaults".into(),
            detail: Some("Set memory, CPU, and disk limits via sandbox configuration".into()),
        },
        Some(limits) => {
            if limits.max_memory_mb < 64 {
                return PreflightCheckResult {
                    name: "resource_limits".into(),
                    status: PreflightStatus::Warn,
                    message: format!("Memory limit {}MB is very low", limits.max_memory_mb),
                    detail: Some("Consider increasing max_memory_mb for reliable operation".into()),
                };
            }
            if limits.max_cpu_percentage < 10 {
                return PreflightCheckResult {
                    name: "resource_limits".into(),
                    status: PreflightStatus::Warn,
                    message: format!("CPU limit {}% is very low", limits.max_cpu_percentage),
                    detail: Some("Consider increasing max_cpu_percentage".into()),
                };
            }
            PreflightCheckResult {
                name: "resource_limits".into(),
                status: PreflightStatus::Pass,
                message: "Resource limits configured".into(),
                detail: Some(format!(
                    "memory={}MB, cpu={}%, disk={}MB",
                    limits.max_memory_mb, limits.max_cpu_percentage, limits.max_disk_space_mb
                )),
            }
        }
    }
}

pub fn check_workspace_exists(workspace_dir: &Path) -> PreflightCheckResult {
    if workspace_dir.exists() {
        PreflightCheckResult {
            name: "workspace_exists".into(),
            status: PreflightStatus::Pass,
            message: "Workspace directory exists".into(),
            detail: Some(workspace_dir.display().to_string()),
        }
    } else {
        PreflightCheckResult {
            name: "workspace_exists".into(),
            status: PreflightStatus::Fail,
            message: "Workspace directory does not exist".into(),
            detail: Some(workspace_dir.display().to_string()),
        }
    }
}

pub fn check_workspace_writable(workspace_dir: &Path) -> PreflightCheckResult {
    let probe = workspace_dir.join(".engram_preflight_probe");
    match std::fs::write(&probe, b"probe") {
        Ok(()) => {
            let _ = std::fs::remove_file(&probe);
            PreflightCheckResult {
                name: "workspace_writable".into(),
                status: PreflightStatus::Pass,
                message: "Workspace directory is writable".into(),
                detail: None,
            }
        }
        Err(e) => PreflightCheckResult {
            name: "workspace_writable".into(),
            status: PreflightStatus::Fail,
            message: "Workspace directory is not writable".into(),
            detail: Some(e.to_string()),
        },
    }
}

pub fn check_git_repo_clean(workspace_dir: &Path) -> PreflightCheckResult {
    let output = std::process::Command::new("git")
        .args([
            "-C",
            &workspace_dir.display().to_string(),
            "status",
            "--porcelain",
        ])
        .output();

    match output {
        Ok(output) if output.status.success() => {
            let dirty = String::from_utf8_lossy(&output.stdout);
            let lines: Vec<&str> = dirty.lines().filter(|l| !l.is_empty()).collect();
            if lines.is_empty() {
                PreflightCheckResult {
                    name: "git_clean".into(),
                    status: PreflightStatus::Pass,
                    message: "Git repository is clean".into(),
                    detail: None,
                }
            } else {
                PreflightCheckResult {
                    name: "git_clean".into(),
                    status: PreflightStatus::Warn,
                    message: format!("Git repository has {} uncommitted change(s)", lines.len()),
                    detail: Some(format!("{} files changed", lines.len())),
                }
            }
        }
        Ok(output) => PreflightCheckResult {
            name: "git_clean".into(),
            status: PreflightStatus::Warn,
            message: "Could not check git status".into(),
            detail: Some(String::from_utf8_lossy(&output.stderr).trim().to_string()),
        },
        Err(e) => PreflightCheckResult {
            name: "git_clean".into(),
            status: PreflightStatus::Warn,
            message: "Could not run git".into(),
            detail: Some(e.to_string()),
        },
    }
}

pub fn check_git_available() -> PreflightCheckResult {
    match std::process::Command::new("git")
        .args(["--version"])
        .output()
    {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
            PreflightCheckResult {
                name: "git_available".into(),
                status: PreflightStatus::Pass,
                message: "Git is available".into(),
                detail: Some(version),
            }
        }
        Ok(output) => PreflightCheckResult {
            name: "git_available".into(),
            status: PreflightStatus::Fail,
            message: "Git is not available or broken".into(),
            detail: Some(String::from_utf8_lossy(&output.stderr).trim().to_string()),
        },
        Err(e) => PreflightCheckResult {
            name: "git_available".into(),
            status: PreflightStatus::Fail,
            message: "Git not found on PATH".into(),
            detail: Some(e.to_string()),
        },
    }
}

pub fn check_engram_storage(workspace_dir: &Path) -> PreflightCheckResult {
    let engram_dir = workspace_dir.join(".engram");
    if engram_dir.exists() {
        let refs_dir = engram_dir.join("refs");
        if refs_dir.exists() {
            PreflightCheckResult {
                name: "engram_storage".into(),
                status: PreflightStatus::Pass,
                message: "Engram storage is accessible".into(),
                detail: Some(engram_dir.display().to_string()),
            }
        } else {
            PreflightCheckResult {
                name: "engram_storage".into(),
                status: PreflightStatus::Warn,
                message: ".engram directory exists but refs/ is missing".into(),
                detail: Some("Run 'engram setup workspace' to initialize storage".into()),
            }
        }
    } else {
        PreflightCheckResult {
            name: "engram_storage".into(),
            status: PreflightStatus::Fail,
            message: ".engram directory not found".into(),
            detail: Some("Run 'engram setup workspace' to initialize storage".into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn default_limits() -> ResourceLimits {
        ResourceLimits {
            max_memory_mb: 512,
            max_cpu_percentage: 50,
            max_disk_space_mb: 1024,
            max_execution_time_minutes: 30,
            max_concurrent_operations: 10,
            max_file_size_mb: 50,
            max_network_requests_per_minute: 30,
        }
    }

    #[test]
    fn test_check_resource_limits_none() {
        let result = check_resource_limits(None);
        assert_eq!(result.status, PreflightStatus::Warn);
        assert_eq!(result.name, "resource_limits");
    }

    #[test]
    fn test_check_resource_limits_valid() {
        let limits = default_limits();
        let result = check_resource_limits(Some(&limits));
        assert_eq!(result.status, PreflightStatus::Pass);
        assert!(result.detail.is_some());
    }

    #[test]
    fn test_check_resource_limits_low_memory() {
        let mut limits = default_limits();
        limits.max_memory_mb = 16;
        let result = check_resource_limits(Some(&limits));
        assert_eq!(result.status, PreflightStatus::Warn);
    }

    #[test]
    fn test_check_resource_limits_low_cpu() {
        let mut limits = default_limits();
        limits.max_cpu_percentage = 5;
        let result = check_resource_limits(Some(&limits));
        assert_eq!(result.status, PreflightStatus::Warn);
    }

    #[test]
    fn test_check_workspace_exists_present() {
        let dir = tempfile::tempdir().unwrap();
        let result = check_workspace_exists(dir.path());
        assert_eq!(result.status, PreflightStatus::Pass);
    }

    #[test]
    fn test_check_workspace_exists_missing() {
        let dir = Path::new("/tmp/engram_preflight_nonexistent_12345");
        let result = check_workspace_exists(dir);
        assert_eq!(result.status, PreflightStatus::Fail);
    }

    #[test]
    fn test_check_workspace_writable() {
        let dir = tempfile::tempdir().unwrap();
        let result = check_workspace_writable(dir.path());
        assert_eq!(result.status, PreflightStatus::Pass);
    }

    #[test]
    fn test_check_workspace_not_writable() {
        let result = check_workspace_writable(Path::new("/proc/1"));
        assert_eq!(result.status, PreflightStatus::Fail);
    }

    #[test]
    fn test_check_git_available() {
        let result = check_git_available();
        assert_eq!(result.status, PreflightStatus::Pass);
        assert!(result.detail.unwrap().contains("git"));
    }

    #[test]
    fn test_check_git_repo_clean() {
        let dir = tempfile::tempdir().unwrap();
        std::process::Command::new("git")
            .args(["init"])
            .current_dir(dir.path())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .ok();
        let result = check_git_repo_clean(dir.path());
        assert!(matches!(
            result.status,
            PreflightStatus::Pass | PreflightStatus::Warn
        ));
    }

    #[test]
    fn test_check_git_repo_not_a_repo() {
        let dir = tempfile::tempdir().unwrap();
        let result = check_git_repo_clean(dir.path());
        assert_eq!(result.status, PreflightStatus::Warn);
    }

    #[test]
    fn test_check_engram_storage_missing() {
        let dir = tempfile::tempdir().unwrap();
        let result = check_engram_storage(dir.path());
        assert_eq!(result.status, PreflightStatus::Fail);
    }

    #[test]
    fn test_check_engram_storage_partial() {
        let dir = tempfile::tempdir().unwrap();
        fs::create_dir(dir.path().join(".engram")).unwrap();
        let result = check_engram_storage(dir.path());
        assert_eq!(result.status, PreflightStatus::Warn);
    }

    #[test]
    fn test_check_engram_storage_valid() {
        let dir = tempfile::tempdir().unwrap();
        fs::create_dir_all(dir.path().join(".engram/refs")).unwrap();
        let result = check_engram_storage(dir.path());
        assert_eq!(result.status, PreflightStatus::Pass);
    }

    #[test]
    fn test_run_preflight_all_pass() {
        let dir = tempfile::tempdir().unwrap();
        fs::create_dir_all(dir.path().join(".engram/refs")).unwrap();
        std::process::Command::new("git")
            .args(["init"])
            .current_dir(dir.path())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .ok();
        let limits = default_limits();
        let report = run_preflight_checks(dir.path(), Some(&limits));
        assert_eq!(report.overall_status, PreflightStatus::Pass);
        assert_eq!(report.checks.len(), 6);
    }

    #[test]
    fn test_run_preflight_has_failures() {
        let dir = tempfile::tempdir().unwrap();
        let report = run_preflight_checks(dir.path(), None);
        assert!(matches!(
            report.overall_status,
            PreflightStatus::Fail | PreflightStatus::Warn
        ));
    }

    #[test]
    fn test_preflight_report_structured_feedback() {
        let report = PreflightReport {
            checks: vec![PreflightCheckResult {
                name: "test".into(),
                status: PreflightStatus::Pass,
                message: "ok".into(),
                detail: None,
            }],
            overall_status: PreflightStatus::Pass,
        };
        assert_eq!(report.status_code(), FeedbackStatus::Success);
        assert!(report.summary().contains("1/1 passed"));
        let json = report.to_json();
        assert_eq!(json["overall_status"], "pass");
        assert!(report.to_pretty_json().contains("overall_status"));
    }

    #[test]
    fn test_preflight_report_warning_status() {
        let report = PreflightReport {
            checks: vec![PreflightCheckResult {
                name: "t".into(),
                status: PreflightStatus::Warn,
                message: "w".into(),
                detail: None,
            }],
            overall_status: PreflightStatus::Warn,
        };
        assert_eq!(report.status_code(), FeedbackStatus::Warning);
    }

    #[test]
    fn test_preflight_report_failed_status() {
        let report = PreflightReport {
            checks: vec![PreflightCheckResult {
                name: "t".into(),
                status: PreflightStatus::Fail,
                message: "f".into(),
                detail: None,
            }],
            overall_status: PreflightStatus::Fail,
        };
        assert_eq!(report.status_code(), FeedbackStatus::Failed);
    }
}

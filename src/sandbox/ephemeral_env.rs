use crate::error::EngramError;
use crate::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::{Command, Stdio};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NixSandboxConfig {
    pub enabled: bool,
    pub packages: Vec<String>,
    pub nixpkgs_rev: Option<String>,
    pub timeout_seconds: u64,
    pub fallback_to_direct: bool,
}

impl Default for NixSandboxConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            packages: Vec::new(),
            nixpkgs_rev: None,
            timeout_seconds: 300,
            fallback_to_direct: true,
        }
    }
}

impl NixSandboxConfig {
    pub fn with_packages(packages: Vec<String>) -> Self {
        Self {
            enabled: true,
            packages,
            nixpkgs_rev: None,
            timeout_seconds: 300,
            fallback_to_direct: true,
        }
    }
}

pub struct NixSandbox {
    config: NixSandboxConfig,
    nix_available: bool,
}

#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
    pub sandbox_used: bool,
}

impl NixSandbox {
    pub fn new(config: NixSandboxConfig) -> Self {
        let nix_available = Self::check_nix_available();
        Self {
            config,
            nix_available,
        }
    }

    fn check_nix_available() -> bool {
        Command::new("nix")
            .arg("--version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    }

    pub fn is_available(&self) -> bool {
        self.config.enabled && self.nix_available
    }

    pub fn execute(
        &self,
        command: &str,
        args: &[String],
        working_directory: Option<&str>,
        environment: &HashMap<String, String>,
        timeout: Option<Duration>,
    ) -> Result<ExecutionResult> {
        let effective_timeout = timeout.unwrap_or(Duration::from_secs(self.config.timeout_seconds));

        if !self.is_available() {
            if self.config.fallback_to_direct {
                tracing::warn!(
                    "Nix sandbox not available (enabled={}, nix_found={}), falling back to direct execution",
                    self.config.enabled,
                    self.nix_available
                );
                return self.execute_direct(
                    command,
                    args,
                    working_directory,
                    environment,
                    effective_timeout,
                );
            }
            return Err(EngramError::Validation(
                "Nix sandbox is enabled but nix is not available on this system".to_string(),
            ));
        }

        self.execute_nix(
            command,
            args,
            working_directory,
            environment,
            effective_timeout,
        )
    }

    fn execute_nix(
        &self,
        command: &str,
        args: &[String],
        working_directory: Option<&str>,
        environment: &HashMap<String, String>,
        timeout: Duration,
    ) -> Result<ExecutionResult> {
        let mut nix_args = Vec::new();

        if let Some(ref rev) = self.config.nixpkgs_rev {
            nix_args.extend_from_slice(&[
                "-I".to_string(),
                format!(
                    "nixpkgs=https://github.com/NixOS/nixpkgs/archive/{}.tar.gz",
                    rev
                ),
            ]);
        }

        nix_args.push("-p".to_string());
        for pkg in &self.config.packages {
            nix_args.push(pkg.clone());
        }

        let full_command = if args.is_empty() {
            command.to_string()
        } else {
            format!("{} {}", command, args.join(" "))
        };

        nix_args.push("--run".to_string());
        nix_args.push(full_command);

        let mut cmd = Command::new("nix-shell");
        cmd.args(&nix_args);

        if let Some(cwd) = working_directory {
            cmd.current_dir(cwd);
        }

        for (key, value) in environment {
            cmd.env(key, value);
        }

        cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

        let child = cmd
            .spawn()
            .map_err(|e| EngramError::Validation(format!("Failed to spawn nix-shell: {}", e)))?;

        let output = wait_with_timeout(child, timeout)
            .map_err(|e| EngramError::Validation(format!("Nix sandbox execution failed: {}", e)))?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let exit_code = output.status.code().unwrap_or(-1);
        let success = output.status.success();

        Ok(ExecutionResult {
            success,
            stdout,
            stderr,
            exit_code,
            sandbox_used: true,
        })
    }

    fn execute_direct(
        &self,
        command: &str,
        args: &[String],
        working_directory: Option<&str>,
        environment: &HashMap<String, String>,
        timeout: Duration,
    ) -> Result<ExecutionResult> {
        let mut cmd = Command::new(command);
        cmd.args(args);

        if let Some(cwd) = working_directory {
            cmd.current_dir(cwd);
        }

        for (key, value) in environment {
            cmd.env(key, value);
        }

        cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

        let child = cmd.spawn().map_err(|e| {
            EngramError::Validation(format!("Failed to spawn command '{}': {}", command, e))
        })?;

        let output = wait_with_timeout(child, timeout)
            .map_err(|e| EngramError::Validation(format!("Command execution failed: {}", e)))?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let exit_code = output.status.code().unwrap_or(-1);
        let success = output.status.success();

        Ok(ExecutionResult {
            success,
            stdout,
            stderr,
            exit_code,
            sandbox_used: false,
        })
    }
}

fn wait_with_timeout(
    child: std::process::Child,
    timeout: Duration,
) -> Result<std::process::Output> {
    use std::sync::mpsc;
    use std::thread;

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let result = child.wait_with_output();
        let _ = tx.send(result);
    });

    match rx.recv_timeout(timeout) {
        Ok(Ok(output)) => Ok(output),
        Ok(Err(e)) => Err(EngramError::Validation(format!(
            "Failed to get command output: {}",
            e
        ))),
        Err(_) => Err(EngramError::Validation(format!(
            "Command timed out after {:?}",
            timeout
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn env() -> HashMap<String, String> {
        HashMap::new()
    }

    #[test]
    fn test_default_config() {
        let config = NixSandboxConfig::default();
        assert!(!config.enabled);
        assert!(config.packages.is_empty());
        assert!(config.nixpkgs_rev.is_none());
        assert_eq!(config.timeout_seconds, 300);
        assert!(config.fallback_to_direct);
    }

    #[test]
    fn test_with_packages() {
        let config = NixSandboxConfig::with_packages(vec!["jq".to_string(), "curl".to_string()]);
        assert!(config.enabled);
        assert_eq!(config.packages, vec!["jq", "curl"]);
        assert!(config.fallback_to_direct);
    }

    #[test]
    fn test_nix_sandbox_not_available_when_disabled() {
        let sandbox = NixSandbox::new(NixSandboxConfig::default());
        assert!(!sandbox.is_available());
    }

    #[test]
    fn test_fallback_to_direct_when_nix_unavailable() {
        let config = NixSandboxConfig::with_packages(vec!["jq".to_string()]);
        let sandbox = NixSandbox::new(config);
        let result = sandbox.execute(
            "echo",
            &[("hello".to_string())],
            None,
            &env(),
            Some(Duration::from_secs(10)),
        );
        assert!(result.is_ok());
        let r = result.unwrap();
        assert!(r.success);
        assert!(r.stdout.contains("hello"));
    }

    #[test]
    fn test_fallback_refuses_when_configured() {
        let config = NixSandboxConfig {
            enabled: true,
            packages: vec!["jq".to_string()],
            nixpkgs_rev: None,
            timeout_seconds: 300,
            fallback_to_direct: false,
        };
        let sandbox = NixSandbox::new(config);
        if !sandbox.nix_available {
            let result = sandbox.execute("echo", &[], None, &env(), None);
            assert!(result.is_err());
        }
    }

    #[test]
    fn test_nix_shell_if_available() {
        let config = NixSandboxConfig::with_packages(vec!["coreutils".to_string()]);
        let sandbox = NixSandbox::new(config);
        if sandbox.is_available() {
            let result = sandbox.execute(
                "echo",
                &[("nix-test".to_string())],
                None,
                &env(),
                Some(Duration::from_secs(120)),
            );
            assert!(result.is_ok());
            let r = result.unwrap();
            assert!(r.success);
            assert!(r.stdout.contains("nix-test"));
            assert!(r.sandbox_used);
        }
    }

    #[test]
    fn test_execution_result_sandbox_flag() {
        let config = NixSandboxConfig::default();
        let sandbox = NixSandbox::new(config);
        let result = sandbox.execute("echo", &[("direct".to_string())], None, &env(), None);
        let r = result.unwrap();
        assert!(!r.sandbox_used);
    }

    #[test]
    fn test_timeout_propagation() {
        let config = NixSandboxConfig::default();
        let sandbox = NixSandbox::new(config);
        let result = sandbox.execute(
            "echo",
            &[("quick".to_string())],
            None,
            &env(),
            Some(Duration::from_secs(5)),
        );
        assert!(result.is_ok());
    }
}

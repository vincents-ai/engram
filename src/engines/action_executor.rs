//! Action Executor for Workflow Actions
//!
//! Executes various types of actions that can be triggered during workflow transitions,
//! including external commands, notifications, and custom actions.

use crate::error::EngramError;
use crate::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::{Command, Stdio};
use std::time::Duration;

/// Action execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionResult {
    pub success: bool,
    pub message: String,
    pub output: Option<String>,
    pub error: Option<String>,
    pub exit_code: Option<i32>,
    pub metadata: HashMap<String, String>,
}

/// Action type enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ActionType {
    ExternalCommand,
    Notification,
    UpdateEntity,
    Custom,
}

/// Action parameters for external command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalCommandParams {
    pub command: String,
    pub args: Vec<String>,
    pub working_directory: Option<String>,
    pub environment: HashMap<String, String>,
    pub timeout_seconds: Option<u64>,
    pub capture_output: bool,
}

/// Action executor
pub struct ActionExecutor {
    default_timeout: Duration,
    allow_external_commands: bool,
}

impl ActionExecutor {
    /// Create a new action executor
    pub fn new(allow_external_commands: bool) -> Self {
        Self {
            default_timeout: Duration::from_secs(300), // 5 minutes default
            allow_external_commands,
        }
    }

    /// Execute an action based on type and parameters
    pub fn execute_action(
        &self,
        action_type: &str,
        parameters: &HashMap<String, serde_json::Value>,
    ) -> Result<ActionResult> {
        match action_type {
            "external_command" => self.execute_external_command(parameters),
            "notification" => self.execute_notification(parameters),
            "update_entity" => self.execute_update_entity(parameters),
            _ => Err(EngramError::Validation(format!(
                "Unknown action type: {}",
                action_type
            ))),
        }
    }

    /// Execute an external command
    fn execute_external_command(
        &self,
        parameters: &HashMap<String, serde_json::Value>,
    ) -> Result<ActionResult> {
        if !self.allow_external_commands {
            return Err(EngramError::Validation(
                "External commands are disabled".to_string(),
            ));
        }

        // Parse command parameters
        let command = parameters
            .get("command")
            .and_then(|v| v.as_str())
            .ok_or_else(|| EngramError::Validation("Missing 'command' parameter".to_string()))?;

        let args: Vec<String> = parameters
            .get("args")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();

        let working_directory = parameters
            .get("working_directory")
            .and_then(|v| v.as_str())
            .map(String::from);

        let environment: HashMap<String, String> = parameters
            .get("environment")
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                    .collect()
            })
            .unwrap_or_default();

        let timeout_seconds = parameters
            .get("timeout_seconds")
            .and_then(|v| v.as_u64())
            .unwrap_or(300);

        let capture_output = parameters
            .get("capture_output")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);

        // Execute the command
        self.run_command(
            command,
            &args,
            working_directory.as_deref(),
            &environment,
            Duration::from_secs(timeout_seconds),
            capture_output,
        )
    }

    /// Run a command with the specified parameters
    fn run_command(
        &self,
        command: &str,
        args: &[String],
        working_directory: Option<&str>,
        environment: &HashMap<String, String>,
        timeout: Duration,
        capture_output: bool,
    ) -> Result<ActionResult> {
        let mut cmd = Command::new(command);
        cmd.args(args);

        if let Some(working_dir) = working_directory {
            cmd.current_dir(working_dir);
        }

        for (key, value) in environment {
            cmd.env(key, value);
        }

        if capture_output {
            cmd.stdout(Stdio::piped()).stderr(Stdio::piped());
        }

        let child = cmd.spawn().map_err(|e| {
            EngramError::Validation(format!("Failed to spawn command '{}': {}", command, e))
        })?;

        let output = self
            .wait_for_output_with_timeout(child, timeout)
            .map_err(|e| EngramError::Validation(format!("Command execution failed: {}", e)))?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let exit_code = output.status.code().unwrap_or(-1);
        let success = output.status.success();

        Ok(ActionResult {
            success,
            message: if success {
                format!("Command '{}' executed successfully", command)
            } else {
                format!("Command '{}' failed with exit code {}", command, exit_code)
            },
            output: if stdout.is_empty() {
                None
            } else {
                Some(stdout)
            },
            error: if stderr.is_empty() {
                None
            } else {
                Some(stderr)
            },
            exit_code: Some(exit_code),
            metadata: HashMap::new(),
        })
    }

    /// Wait for command output with timeout
    fn wait_for_output_with_timeout(
        &self,
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

    /// Execute a notification action
    fn execute_notification(
        &self,
        parameters: &HashMap<String, serde_json::Value>,
    ) -> Result<ActionResult> {
        let message = parameters
            .get("message")
            .and_then(|v| v.as_str())
            .ok_or_else(|| EngramError::Validation("Missing 'message' parameter".to_string()))?;

        // For now, just log the notification
        // In the future, this could send emails, Slack messages, etc.
        tracing::info!("Workflow notification: {}", message);

        Ok(ActionResult {
            success: true,
            message: format!("Notification sent: {}", message),
            output: None,
            error: None,
            exit_code: None,
            metadata: HashMap::new(),
        })
    }

    /// Execute an entity update action
    fn execute_update_entity(
        &self,
        parameters: &HashMap<String, serde_json::Value>,
    ) -> Result<ActionResult> {
        let entity_id = parameters
            .get("entity_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| EngramError::Validation("Missing 'entity_id' parameter".to_string()))?;

        let entity_type = parameters
            .get("entity_type")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                EngramError::Validation("Missing 'entity_type' parameter".to_string())
            })?;

        // For now, just acknowledge the update
        // In the future, this would actually update the entity in storage
        tracing::info!(
            "Update entity action: {} (type: {})",
            entity_id,
            entity_type
        );

        Ok(ActionResult {
            success: true,
            message: format!("Entity {} updated", entity_id),
            output: None,
            error: None,
            exit_code: None,
            metadata: HashMap::new(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_external_command_success() {
        let executor = ActionExecutor::new(true);
        let mut params = HashMap::new();
        params.insert(
            "command".to_string(),
            serde_json::Value::String("echo".to_string()),
        );
        params.insert("args".to_string(), serde_json::json!(["hello", "world"]));
        params.insert("capture_output".to_string(), serde_json::json!(true));

        let result = executor.execute_action("external_command", &params);
        assert!(result.is_ok());
        let action_result = result.unwrap();
        assert!(action_result.success);
        assert!(action_result.output.is_some());
        assert!(action_result.output.unwrap().contains("hello world"));
    }

    #[test]
    fn test_external_command_disabled() {
        let executor = ActionExecutor::new(false);
        let mut params = HashMap::new();
        params.insert(
            "command".to_string(),
            serde_json::Value::String("echo".to_string()),
        );

        let result = executor.execute_action("external_command", &params);
        assert!(result.is_err());
    }

    #[test]
    fn test_notification_action() {
        let executor = ActionExecutor::new(true);
        let mut params = HashMap::new();
        params.insert(
            "message".to_string(),
            serde_json::Value::String("Test notification".to_string()),
        );

        let result = executor.execute_action("notification", &params);
        assert!(result.is_ok());
        let action_result = result.unwrap();
        assert!(action_result.success);
        assert!(action_result.message.contains("Test notification"));
    }

    #[test]
    fn test_unknown_action_type() {
        let executor = ActionExecutor::new(true);
        let params = HashMap::new();

        let result = executor.execute_action("unknown_action", &params);
        assert!(result.is_err());
    }
}

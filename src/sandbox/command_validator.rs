#![allow(clippy::needless_borrows_for_generic_args)]

use crate::entities::{
    BuiltinCommandType, CommandFilter, CommandPattern, DangerousPattern, ParameterRestriction,
    RiskLevel,
};
use crate::sandbox::{CommandValidationResult, SandboxRequest, SandboxResult};
use regex::Regex;
use std::collections::HashMap;

pub struct CommandValidator {
    compiled_patterns: HashMap<String, Regex>,
}

impl CommandValidator {
    pub fn new() -> Self {
        Self {
            compiled_patterns: HashMap::new(),
        }
    }

    pub async fn validate_command(
        &mut self,
        request: &SandboxRequest,
        filter: &CommandFilter,
    ) -> SandboxResult<CommandValidationResult> {
        if filter.whitelist_mode {
            return self.validate_whitelist(request, filter).await;
        } else {
            return self.validate_blacklist(request, filter).await;
        }
    }

    async fn validate_whitelist(
        &mut self,
        request: &SandboxRequest,
        filter: &CommandFilter,
    ) -> SandboxResult<CommandValidationResult> {
        if self.matches_any_pattern(&request.operation, &filter.allowed_commands)? {
            return self.check_parameters(request, filter).await;
        }

        Ok(CommandValidationResult::Block(format!(
            "Command '{}' not in allowed list",
            request.operation
        )))
    }

    async fn validate_blacklist(
        &mut self,
        request: &SandboxRequest,
        filter: &CommandFilter,
    ) -> SandboxResult<CommandValidationResult> {
        if self.matches_any_pattern(&request.operation, &filter.forbidden_commands)? {
            return Ok(CommandValidationResult::Block(format!(
                "Command '{}' is explicitly forbidden",
                request.operation
            )));
        }

        if self.matches_dangerous_pattern(request, filter).await? {
            return Ok(CommandValidationResult::RequiresApproval);
        }

        self.check_parameters(request, filter).await
    }

    fn matches_any_pattern(
        &mut self,
        command: &str,
        patterns: &[CommandPattern],
    ) -> SandboxResult<bool> {
        for pattern in patterns {
            if self.matches_pattern(command, pattern)? {
                return Ok(true);
            }
        }
        Ok(false)
    }

    fn matches_pattern(&mut self, command: &str, pattern: &CommandPattern) -> SandboxResult<bool> {
        match pattern {
            CommandPattern::Exact {
                command: pattern_cmd,
            } => Ok(command == pattern_cmd),
            CommandPattern::Prefix { prefix } => Ok(command.starts_with(prefix)),
            CommandPattern::Regex {
                pattern: regex_pattern,
            } => self.matches_regex_string(command, regex_pattern),
            CommandPattern::Builtin { command_type } => {
                self.matches_builtin_command(command, command_type)
            }
        }
    }

    fn matches_builtin_command(
        &self,
        command: &str,
        command_type: &BuiltinCommandType,
    ) -> SandboxResult<bool> {
        match command_type {
            BuiltinCommandType::Git => Ok(command.starts_with("git_")),
            BuiltinCommandType::Cargo => Ok(command.starts_with("cargo_")),
            BuiltinCommandType::Engram => Ok(command.starts_with("engram_")),
            BuiltinCommandType::FileSystem => Ok(matches!(
                command,
                "read_file" | "write_file" | "delete_file" | "list_files" | "create_file"
            )),
            BuiltinCommandType::Network => Ok(matches!(
                command,
                "network_request" | "http_get" | "http_post" | "download_file"
            )),
            BuiltinCommandType::System => Ok(matches!(
                command,
                "execute_command" | "system_info" | "process_list"
            )),
        }
    }

    async fn check_parameters(
        &mut self,
        request: &SandboxRequest,
        filter: &CommandFilter,
    ) -> SandboxResult<CommandValidationResult> {
        if let Some(restriction) = filter.parameter_restrictions.get(&request.operation) {
            for (param_name, param_value) in request
                .parameters
                .as_object()
                .unwrap_or(&serde_json::Map::new())
            {
                if let Err(error_msg) = self.validate_parameter(param_value, restriction) {
                    return Ok(CommandValidationResult::Block(format!(
                        "Parameter '{}': {}",
                        param_name, error_msg
                    )));
                }
            }
        }

        Ok(CommandValidationResult::Allow)
    }

    fn validate_parameter(
        &mut self,
        value: &serde_json::Value,
        restriction: &ParameterRestriction,
    ) -> Result<(), String> {
        let value_str = match value.as_str() {
            Some(s) => s,
            None => &serde_json::to_string(value).unwrap_or_default(),
        };

        if !restriction.allowed_values.is_empty()
            && !restriction.allowed_values.contains(&value_str.to_string())
        {
            return Err(format!("Value '{}' not in allowed list", value_str));
        }

        if restriction
            .forbidden_values
            .contains(&value_str.to_string())
        {
            return Err(format!("Value '{}' is forbidden", value_str));
        }

        if let Some(max_len) = restriction.max_length {
            if value_str.len() > max_len {
                return Err(format!(
                    "Value length {} exceeds maximum {}",
                    value_str.len(),
                    max_len
                ));
            }
        }

        if let Some(pattern) = &restriction.pattern_validation {
            if !self
                .matches_regex_string(value_str, pattern)
                .unwrap_or(false)
            {
                return Err("Value does not match required pattern".to_string());
            }
        }

        Ok(())
    }

    async fn matches_dangerous_pattern(
        &mut self,
        request: &SandboxRequest,
        filter: &CommandFilter,
    ) -> SandboxResult<bool> {
        for dangerous_pattern in &filter.dangerous_patterns {
            if self.check_dangerous_pattern(request, dangerous_pattern)? {
                return Ok(true);
            }
        }

        Ok(false)
    }

    fn check_dangerous_pattern(
        &mut self,
        request: &SandboxRequest,
        pattern: &DangerousPattern,
    ) -> SandboxResult<bool> {
        if request.operation.contains(&pattern.pattern) {
            return Ok(true);
        }

        if let Some(command_str) = request.parameters.get("command") {
            if let Some(cmd) = command_str.as_str() {
                if cmd.contains(&pattern.pattern) {
                    return Ok(true);
                }
            }
        }

        if self.matches_regex_string(
            &serde_json::to_string(request).unwrap_or_default(),
            &pattern.pattern,
        )? {
            return Ok(true);
        }

        Ok(false)
    }

    fn matches_regex_string(&mut self, text: &str, pattern: &str) -> SandboxResult<bool> {
        let regex = if let Some(cached_regex) = self.compiled_patterns.get(pattern) {
            cached_regex
        } else {
            match Regex::new(pattern) {
                Ok(regex) => {
                    self.compiled_patterns.insert(pattern.to_string(), regex);
                    self.compiled_patterns.get(pattern).unwrap()
                }
                Err(_) => return Ok(false),
            }
        };

        Ok(regex.is_match(text))
    }

    pub fn clear_pattern_cache(&mut self) {
        self.compiled_patterns.clear();
    }

    pub fn add_pattern(&mut self, pattern: String) -> Result<(), regex::Error> {
        let regex = Regex::new(&pattern)?;
        self.compiled_patterns.insert(pattern, regex);
        Ok(())
    }

    pub fn remove_pattern(&mut self, pattern: &str) {
        self.compiled_patterns.remove(pattern);
    }

    pub fn validate_command_syntax(command: &str) -> bool {
        if command.is_empty() {
            return false;
        }

        let dangerous_keywords = [
            "rm -rf",
            "sudo",
            "chmod 777",
            "eval",
            "exec",
            "> /dev/",
            "dd if=",
            "mkfs",
            "fdisk",
            ":(){:|:&};:",
        ];

        for keyword in &dangerous_keywords {
            if command.contains(keyword) {
                return false;
            }
        }

        true
    }

    pub fn estimate_command_risk(command: &str) -> RiskLevel {
        if command.is_empty() {
            return RiskLevel::Low;
        }

        let high_risk_patterns = ["rm", "del", "format", "sudo", "chmod", "chown"];
        let medium_risk_patterns = ["cp", "mv", "mkdir", "rmdir", "curl", "wget"];

        for pattern in &high_risk_patterns {
            if command.contains(pattern) {
                return RiskLevel::High;
            }
        }

        for pattern in &medium_risk_patterns {
            if command.contains(pattern) {
                return RiskLevel::Medium;
            }
        }

        RiskLevel::Low
    }
}

impl Default for CommandValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use serde_json::json;

    #[tokio::test]
    async fn test_validate_whitelist_allowed() {
        let mut validator = CommandValidator::new();
        let request = SandboxRequest {
            agent_id: "test-agent".to_string(),
            operation: "ls".to_string(),
            resource_type: "command".to_string(),
            parameters: json!({}),
            timestamp: Utc::now(),
            session_id: Some("session-1".to_string()),
        };

        let filter = CommandFilter {
            whitelist_mode: true,
            allowed_commands: vec![CommandPattern::Exact {
                command: "ls".to_string(),
            }],
            ..Default::default()
        };

        let result = validator.validate_command(&request, &filter).await.unwrap();
        assert!(matches!(result, CommandValidationResult::Allow));
    }

    #[tokio::test]
    async fn test_validate_whitelist_blocked() {
        let mut validator = CommandValidator::new();
        let request = SandboxRequest {
            agent_id: "test-agent".to_string(),
            operation: "rm".to_string(),
            resource_type: "command".to_string(),
            parameters: json!({}),
            timestamp: Utc::now(),
            session_id: Some("session-1".to_string()),
        };

        let filter = CommandFilter {
            whitelist_mode: true,
            allowed_commands: vec![CommandPattern::Exact {
                command: "ls".to_string(),
            }],
            ..Default::default()
        };

        let result = validator.validate_command(&request, &filter).await.unwrap();
        assert!(matches!(result, CommandValidationResult::Block(_)));
    }

    #[test]
    fn test_risk_estimation() {
        assert!(matches!(
            CommandValidator::estimate_command_risk("rm -rf /"),
            RiskLevel::High
        ));
        assert!(matches!(
            CommandValidator::estimate_command_risk("curl http://example.com"),
            RiskLevel::Medium
        ));
        assert!(matches!(
            CommandValidator::estimate_command_risk("echo hello"),
            RiskLevel::Low
        ));
    }

    #[test]
    fn test_validate_syntax() {
        assert!(CommandValidator::validate_command_syntax("ls -la"));
        assert!(!CommandValidator::validate_command_syntax("rm -rf /"));
        assert!(!CommandValidator::validate_command_syntax(
            "command > /dev/sda"
        ));
    }

    #[test]
    fn test_validate_syntax_empty() {
        assert!(!CommandValidator::validate_command_syntax(""));
    }

    #[test]
    fn test_validate_syntax_all_keywords() {
        assert!(CommandValidator::validate_command_syntax("echo hello"));
        assert!(!CommandValidator::validate_command_syntax("sudo apt"));
        assert!(!CommandValidator::validate_command_syntax("chmod 777 f"));
        assert!(!CommandValidator::validate_command_syntax("eval x"));
        assert!(!CommandValidator::validate_command_syntax("exec /bin/bash"));
        assert!(!CommandValidator::validate_command_syntax(
            "echo > /dev/null"
        ));
        assert!(!CommandValidator::validate_command_syntax(
            "dd if=/dev/zero"
        ));
        assert!(!CommandValidator::validate_command_syntax("mkfs /dev/sda"));
        assert!(!CommandValidator::validate_command_syntax("fdisk /dev/sda"));
        assert!(!CommandValidator::validate_command_syntax(":(){:|:&};:"));
    }

    #[test]
    fn test_risk_empty() {
        assert!(matches!(
            CommandValidator::estimate_command_risk(""),
            RiskLevel::Low
        ));
    }

    #[test]
    fn test_risk_all() {
        assert!(matches!(
            CommandValidator::estimate_command_risk("sudo rm"),
            RiskLevel::High
        ));
        assert!(matches!(
            CommandValidator::estimate_command_risk("format c:"),
            RiskLevel::High
        ));
        assert!(matches!(
            CommandValidator::estimate_command_risk("chown root"),
            RiskLevel::High
        ));
        assert!(matches!(
            CommandValidator::estimate_command_risk("del f"),
            RiskLevel::High
        ));
        assert!(matches!(
            CommandValidator::estimate_command_risk("rmdir d"),
            RiskLevel::High
        ));
        assert!(matches!(
            CommandValidator::estimate_command_risk("cp a b"),
            RiskLevel::Medium
        ));
        assert!(matches!(
            CommandValidator::estimate_command_risk("mv a b"),
            RiskLevel::Medium
        ));
        assert!(matches!(
            CommandValidator::estimate_command_risk("mkdir d"),
            RiskLevel::Medium
        ));
        assert!(matches!(
            CommandValidator::estimate_command_risk("curl x"),
            RiskLevel::Medium
        ));
        assert!(matches!(
            CommandValidator::estimate_command_risk("wget x"),
            RiskLevel::Medium
        ));
        assert!(matches!(
            CommandValidator::estimate_command_risk("echo hello"),
            RiskLevel::Low
        ));
    }

    #[test]
    fn test_default() {
        let _ = CommandValidator::default();
    }

    #[tokio::test]
    async fn test_blacklist_forbidden() {
        let mut v = CommandValidator::new();
        let req = SandboxRequest {
            agent_id: "a".into(),
            operation: "sudo".into(),
            resource_type: "c".into(),
            parameters: json!({}),
            timestamp: Utc::now(),
            session_id: None,
        };
        let f = CommandFilter {
            whitelist_mode: false,
            forbidden_commands: vec![CommandPattern::Prefix {
                prefix: "sudo".into(),
            }],
            ..Default::default()
        };
        assert!(matches!(
            v.validate_command(&req, &f).await.unwrap(),
            CommandValidationResult::Block(_)
        ));
    }

    #[tokio::test]
    async fn test_blacklist_allowed() {
        let mut v = CommandValidator::new();
        let req = SandboxRequest {
            agent_id: "a".into(),
            operation: "safe".into(),
            resource_type: "c".into(),
            parameters: json!({}),
            timestamp: Utc::now(),
            session_id: None,
        };
        let f = CommandFilter {
            whitelist_mode: false,
            forbidden_commands: vec![CommandPattern::Prefix {
                prefix: "sudo".into(),
            }],
            ..Default::default()
        };
        assert!(matches!(
            v.validate_command(&req, &f).await.unwrap(),
            CommandValidationResult::Allow
        ));
    }

    #[tokio::test]
    async fn test_dangerous_pattern_requires_approval() {
        let mut v = CommandValidator::new();
        let req = SandboxRequest {
            agent_id: "a".into(),
            operation: "cmd".into(),
            resource_type: "c".into(),
            parameters: json!({"command": "rm -rf /tmp"}),
            timestamp: Utc::now(),
            session_id: None,
        };
        let f = CommandFilter {
            whitelist_mode: false,
            dangerous_patterns: vec![DangerousPattern {
                pattern: "rm -rf".into(),
                description: "d".into(),
                risk_level: RiskLevel::High,
                auto_block: false,
            }],
            ..Default::default()
        };
        assert!(matches!(
            v.validate_command(&req, &f).await.unwrap(),
            CommandValidationResult::RequiresApproval
        ));
    }

    #[tokio::test]
    async fn test_dangerous_pattern_in_operation() {
        let mut v = CommandValidator::new();
        let req = SandboxRequest {
            agent_id: "a".into(),
            operation: "rm -rf x".into(),
            resource_type: "c".into(),
            parameters: json!({}),
            timestamp: Utc::now(),
            session_id: None,
        };
        let f = CommandFilter {
            whitelist_mode: false,
            dangerous_patterns: vec![DangerousPattern {
                pattern: "rm -rf".into(),
                description: "d".into(),
                risk_level: RiskLevel::High,
                auto_block: false,
            }],
            ..Default::default()
        };
        assert!(matches!(
            v.validate_command(&req, &f).await.unwrap(),
            CommandValidationResult::RequiresApproval
        ));
    }

    #[tokio::test]
    async fn test_prefix_pattern() {
        let mut v = CommandValidator::new();
        let req = SandboxRequest {
            agent_id: "a".into(),
            operation: "git_push".into(),
            resource_type: "c".into(),
            parameters: json!({}),
            timestamp: Utc::now(),
            session_id: None,
        };
        let f = CommandFilter {
            whitelist_mode: true,
            allowed_commands: vec![CommandPattern::Prefix {
                prefix: "git_".into(),
            }],
            ..Default::default()
        };
        assert!(matches!(
            v.validate_command(&req, &f).await.unwrap(),
            CommandValidationResult::Allow
        ));
    }

    #[tokio::test]
    async fn test_regex_pattern() {
        let mut v = CommandValidator::new();
        let req = SandboxRequest {
            agent_id: "a".into(),
            operation: "test_123".into(),
            resource_type: "c".into(),
            parameters: json!({}),
            timestamp: Utc::now(),
            session_id: None,
        };
        let f = CommandFilter {
            whitelist_mode: true,
            allowed_commands: vec![CommandPattern::Regex {
                pattern: r"test_\d+".into(),
            }],
            ..Default::default()
        };
        assert!(matches!(
            v.validate_command(&req, &f).await.unwrap(),
            CommandValidationResult::Allow
        ));
    }

    #[tokio::test]
    async fn test_regex_no_match() {
        let mut v = CommandValidator::new();
        let req = SandboxRequest {
            agent_id: "a".into(),
            operation: "abc".into(),
            resource_type: "c".into(),
            parameters: json!({}),
            timestamp: Utc::now(),
            session_id: None,
        };
        let f = CommandFilter {
            whitelist_mode: true,
            allowed_commands: vec![CommandPattern::Regex {
                pattern: r"^\d+$".into(),
            }],
            ..Default::default()
        };
        assert!(matches!(
            v.validate_command(&req, &f).await.unwrap(),
            CommandValidationResult::Block(_)
        ));
    }

    #[tokio::test]
    async fn test_builtin_git() {
        let mut v = CommandValidator::new();
        let f = CommandFilter {
            whitelist_mode: true,
            allowed_commands: vec![CommandPattern::Builtin {
                command_type: BuiltinCommandType::Git,
            }],
            ..Default::default()
        };
        let req = SandboxRequest {
            agent_id: "a".into(),
            operation: "git_status".into(),
            resource_type: "c".into(),
            parameters: json!({}),
            timestamp: Utc::now(),
            session_id: None,
        };
        assert!(matches!(
            v.validate_command(&req, &f).await.unwrap(),
            CommandValidationResult::Allow
        ));
    }

    #[tokio::test]
    async fn test_builtin_cargo() {
        let mut v = CommandValidator::new();
        let f = CommandFilter {
            whitelist_mode: true,
            allowed_commands: vec![CommandPattern::Builtin {
                command_type: BuiltinCommandType::Cargo,
            }],
            ..Default::default()
        };
        let req = SandboxRequest {
            agent_id: "a".into(),
            operation: "cargo_build".into(),
            resource_type: "c".into(),
            parameters: json!({}),
            timestamp: Utc::now(),
            session_id: None,
        };
        assert!(matches!(
            v.validate_command(&req, &f).await.unwrap(),
            CommandValidationResult::Allow
        ));
    }

    #[tokio::test]
    async fn test_builtin_filesystem() {
        let mut v = CommandValidator::new();
        let f = CommandFilter {
            whitelist_mode: true,
            allowed_commands: vec![CommandPattern::Builtin {
                command_type: BuiltinCommandType::FileSystem,
            }],
            ..Default::default()
        };
        for cmd in &[
            "read_file",
            "write_file",
            "delete_file",
            "list_files",
            "create_file",
        ] {
            let req = SandboxRequest {
                agent_id: "a".into(),
                operation: cmd.to_string(),
                resource_type: "c".into(),
                parameters: json!({}),
                timestamp: Utc::now(),
                session_id: None,
            };
            assert!(
                matches!(
                    v.validate_command(&req, &f).await.unwrap(),
                    CommandValidationResult::Allow
                ),
                "{}",
                cmd
            );
        }
    }

    #[tokio::test]
    async fn test_builtin_network() {
        let mut v = CommandValidator::new();
        let f = CommandFilter {
            whitelist_mode: true,
            allowed_commands: vec![CommandPattern::Builtin {
                command_type: BuiltinCommandType::Network,
            }],
            ..Default::default()
        };
        for cmd in &["network_request", "http_get", "http_post", "download_file"] {
            let req = SandboxRequest {
                agent_id: "a".into(),
                operation: cmd.to_string(),
                resource_type: "c".into(),
                parameters: json!({}),
                timestamp: Utc::now(),
                session_id: None,
            };
            assert!(
                matches!(
                    v.validate_command(&req, &f).await.unwrap(),
                    CommandValidationResult::Allow
                ),
                "{}",
                cmd
            );
        }
    }

    #[tokio::test]
    async fn test_builtin_system() {
        let mut v = CommandValidator::new();
        let f = CommandFilter {
            whitelist_mode: true,
            allowed_commands: vec![CommandPattern::Builtin {
                command_type: BuiltinCommandType::System,
            }],
            ..Default::default()
        };
        for cmd in &["execute_command", "system_info", "process_list"] {
            let req = SandboxRequest {
                agent_id: "a".into(),
                operation: cmd.to_string(),
                resource_type: "c".into(),
                parameters: json!({}),
                timestamp: Utc::now(),
                session_id: None,
            };
            assert!(
                matches!(
                    v.validate_command(&req, &f).await.unwrap(),
                    CommandValidationResult::Allow
                ),
                "{}",
                cmd
            );
        }
    }

    #[tokio::test]
    async fn test_builtin_engram() {
        let mut v = CommandValidator::new();
        let f = CommandFilter {
            whitelist_mode: true,
            allowed_commands: vec![CommandPattern::Builtin {
                command_type: BuiltinCommandType::Engram,
            }],
            ..Default::default()
        };
        let req = SandboxRequest {
            agent_id: "a".into(),
            operation: "engram_ask".into(),
            resource_type: "c".into(),
            parameters: json!({}),
            timestamp: Utc::now(),
            session_id: None,
        };
        assert!(matches!(
            v.validate_command(&req, &f).await.unwrap(),
            CommandValidationResult::Allow
        ));
    }

    #[tokio::test]
    async fn test_param_allowed_values() {
        let mut v = CommandValidator::new();
        let req = SandboxRequest {
            agent_id: "a".into(),
            operation: "deploy".into(),
            resource_type: "c".into(),
            parameters: json!({"env": "staging"}),
            timestamp: Utc::now(),
            session_id: None,
        };
        let mut r = HashMap::new();
        r.insert(
            "deploy".into(),
            ParameterRestriction {
                allowed_values: vec!["staging".into(), "production".into()],
                forbidden_values: vec![],
                max_length: None,
                pattern_validation: None,
            },
        );
        let f = CommandFilter {
            whitelist_mode: false,
            parameter_restrictions: r,
            ..Default::default()
        };
        assert!(matches!(
            v.validate_command(&req, &f).await.unwrap(),
            CommandValidationResult::Allow
        ));
    }

    #[tokio::test]
    async fn test_param_forbidden_values() {
        let mut v = CommandValidator::new();
        let req = SandboxRequest {
            agent_id: "a".into(),
            operation: "deploy".into(),
            resource_type: "c".into(),
            parameters: json!({"env": "production"}),
            timestamp: Utc::now(),
            session_id: None,
        };
        let mut r = HashMap::new();
        r.insert(
            "deploy".into(),
            ParameterRestriction {
                allowed_values: vec![],
                forbidden_values: vec!["production".into()],
                max_length: None,
                pattern_validation: None,
            },
        );
        let f = CommandFilter {
            whitelist_mode: false,
            parameter_restrictions: r,
            ..Default::default()
        };
        assert!(matches!(
            v.validate_command(&req, &f).await.unwrap(),
            CommandValidationResult::Block(_)
        ));
    }

    #[tokio::test]
    async fn test_param_max_length() {
        let mut v = CommandValidator::new();
        let req = SandboxRequest {
            agent_id: "a".into(),
            operation: "set_name".into(),
            resource_type: "c".into(),
            parameters: json!({"name": "this_is_very_long_name_exceeds"}),
            timestamp: Utc::now(),
            session_id: None,
        };
        let mut r = HashMap::new();
        r.insert(
            "set_name".into(),
            ParameterRestriction {
                allowed_values: vec![],
                forbidden_values: vec![],
                max_length: Some(10),
                pattern_validation: None,
            },
        );
        let f = CommandFilter {
            whitelist_mode: false,
            parameter_restrictions: r,
            ..Default::default()
        };
        assert!(matches!(
            v.validate_command(&req, &f).await.unwrap(),
            CommandValidationResult::Block(_)
        ));
    }

    #[tokio::test]
    async fn test_param_pattern_invalid() {
        let mut v = CommandValidator::new();
        let req = SandboxRequest {
            agent_id: "a".into(),
            operation: "set_email".into(),
            resource_type: "c".into(),
            parameters: json!({"email": "invalid-email"}),
            timestamp: Utc::now(),
            session_id: None,
        };
        let mut r = HashMap::new();
        r.insert(
            "set_email".into(),
            ParameterRestriction {
                allowed_values: vec![],
                forbidden_values: vec![],
                max_length: None,
                pattern_validation: Some(r"^[^@]+@[^@]+\.[^@]+$".into()),
            },
        );
        let f = CommandFilter {
            whitelist_mode: false,
            parameter_restrictions: r,
            ..Default::default()
        };
        assert!(matches!(
            v.validate_command(&req, &f).await.unwrap(),
            CommandValidationResult::Block(_)
        ));
    }

    #[tokio::test]
    async fn test_param_pattern_valid() {
        let mut v = CommandValidator::new();
        let req = SandboxRequest {
            agent_id: "a".into(),
            operation: "set_email".into(),
            resource_type: "c".into(),
            parameters: json!({"email": "user@example.com"}),
            timestamp: Utc::now(),
            session_id: None,
        };
        let mut r = HashMap::new();
        r.insert(
            "set_email".into(),
            ParameterRestriction {
                allowed_values: vec![],
                forbidden_values: vec![],
                max_length: None,
                pattern_validation: Some(r"^[^@]+@[^@]+\.[^@]+$".into()),
            },
        );
        let f = CommandFilter {
            whitelist_mode: false,
            parameter_restrictions: r,
            ..Default::default()
        };
        assert!(matches!(
            v.validate_command(&req, &f).await.unwrap(),
            CommandValidationResult::Allow
        ));
    }

    #[test]
    fn test_cache_ops() {
        let mut v = CommandValidator::new();
        v.add_pattern(r"\d+".into()).unwrap();
        assert!(v.compiled_patterns.contains_key(r"\d+"));
        v.remove_pattern(r"\d+");
        assert!(!v.compiled_patterns.contains_key(r"\d+"));
        v.add_pattern(r"[a-z]+".into()).unwrap();
        v.clear_pattern_cache();
        assert!(v.compiled_patterns.is_empty());
    }

    #[test]
    fn test_add_pattern_invalid() {
        assert!(CommandValidator::new()
            .add_pattern(r"[invalid(".into())
            .is_err());
    }

    #[tokio::test]
    async fn test_dangerous_regex_match() {
        let mut v = CommandValidator::new();
        let req = SandboxRequest {
            agent_id: "test.agent".into(),
            operation: "safe".into(),
            resource_type: "c".into(),
            parameters: json!({"cmd": "hello"}),
            timestamp: Utc::now(),
            session_id: None,
        };
        let f = CommandFilter {
            whitelist_mode: false,
            dangerous_patterns: vec![DangerousPattern {
                pattern: r"test\.agent".into(),
                description: "d".into(),
                risk_level: RiskLevel::Medium,
                auto_block: false,
            }],
            ..Default::default()
        };
        assert!(matches!(
            v.validate_command(&req, &f).await.unwrap(),
            CommandValidationResult::RequiresApproval
        ));
    }

    #[tokio::test]
    async fn test_no_dangerous_patterns() {
        let mut v = CommandValidator::new();
        let req = SandboxRequest {
            agent_id: "a".into(),
            operation: "safe".into(),
            resource_type: "c".into(),
            parameters: json!({}),
            timestamp: Utc::now(),
            session_id: None,
        };
        let f = CommandFilter {
            whitelist_mode: false,
            dangerous_patterns: vec![DangerousPattern {
                pattern: "nonexistent_xyz".into(),
                description: "d".into(),
                risk_level: RiskLevel::Low,
                auto_block: false,
            }],
            ..Default::default()
        };
        assert!(matches!(
            v.validate_command(&req, &f).await.unwrap(),
            CommandValidationResult::Allow
        ));
    }

    #[tokio::test]
    async fn test_param_non_string_value() {
        let mut v = CommandValidator::new();
        let req = SandboxRequest {
            agent_id: "a".into(),
            operation: "set_count".into(),
            resource_type: "c".into(),
            parameters: json!({"count": 42}),
            timestamp: Utc::now(),
            session_id: None,
        };
        let mut r = HashMap::new();
        r.insert(
            "set_count".into(),
            ParameterRestriction {
                allowed_values: vec!["42".into()],
                forbidden_values: vec![],
                max_length: None,
                pattern_validation: None,
            },
        );
        let f = CommandFilter {
            whitelist_mode: false,
            parameter_restrictions: r,
            ..Default::default()
        };
        assert!(matches!(
            v.validate_command(&req, &f).await.unwrap(),
            CommandValidationResult::Allow
        ));
    }
}

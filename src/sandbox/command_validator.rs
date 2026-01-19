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

        if !restriction.allowed_values.is_empty() {
            if !restriction.allowed_values.contains(&value_str.to_string()) {
                return Err(format!("Value '{}' not in allowed list", value_str));
            }
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

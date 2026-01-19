use crate::entities::{FileOperation, NetworkPolicy, PermissionSet};
use crate::sandbox::{SandboxError, SandboxRequest, SandboxResult};
use std::collections::HashMap;

pub struct PermissionEngine {
    cached_permissions: HashMap<String, PermissionSet>,
}

impl PermissionEngine {
    pub fn new() -> Self {
        Self {
            cached_permissions: HashMap::new(),
        }
    }

    pub async fn validate_operation(
        &mut self,
        request: &SandboxRequest,
        permissions: &PermissionSet,
    ) -> SandboxResult<()> {
        match request.operation.as_str() {
            "read_file" | "list_files" => {
                if !permissions
                    .allowed_file_operations
                    .contains(&FileOperation::Read)
                {
                    return Err(SandboxError::PermissionDenied(
                        "File read operations not permitted".to_string(),
                    ));
                }
            }
            "write_file" | "create_file" | "modify_file" => {
                if !permissions
                    .allowed_file_operations
                    .contains(&FileOperation::Write)
                {
                    return Err(SandboxError::PermissionDenied(
                        "File write operations not permitted".to_string(),
                    ));
                }
            }
            "delete_file" | "move_file" => {
                if !permissions
                    .allowed_file_operations
                    .contains(&FileOperation::Delete)
                {
                    return Err(SandboxError::PermissionDenied(
                        "File deletion operations not permitted".to_string(),
                    ));
                }
            }
            "execute_command" => {
                // Check if command is allowed via command permissions
                let command_name = request
                    .parameters
                    .get("command")
                    .and_then(|v| v.as_str())
                    .unwrap_or(&request.operation);

                if !self.is_command_allowed(command_name, permissions) {
                    return Err(SandboxError::PermissionDenied(
                        "Command execution not permitted".to_string(),
                    ));
                }
            }
            "network_request" => match permissions.network_access {
                NetworkPolicy::Denied => {
                    return Err(SandboxError::PermissionDenied(
                        "Network access not permitted".to_string(),
                    ));
                }
                NetworkPolicy::InternalOnly => {
                    if let Some(url) = request.parameters.get("url") {
                        if let Some(url_str) = url.as_str() {
                            if !self.is_internal_url(url_str) {
                                return Err(SandboxError::PermissionDenied(
                                    "External network access not permitted".to_string(),
                                ));
                            }
                        }
                    }
                }
                NetworkPolicy::AllowedWithMonitoring | NetworkPolicy::Unrestricted => {}
            },
            "create_workflow" => {
                if !permissions.workflow_permissions.can_create_workflows {
                    return Err(SandboxError::PermissionDenied(
                        "Workflow creation not permitted".to_string(),
                    ));
                }
            }
            "modify_workflow" => {
                if !permissions.workflow_permissions.can_modify_workflows {
                    return Err(SandboxError::PermissionDenied(
                        "Workflow modification not permitted".to_string(),
                    ));
                }
            }
            "execute_workflow" => {
                if !permissions.workflow_permissions.can_execute_workflows {
                    return Err(SandboxError::PermissionDenied(
                        "Workflow execution not permitted".to_string(),
                    ));
                }

                if let Some(workflow_type) = request.parameters.get("workflow_type") {
                    if let Some(workflow_str) = workflow_type.as_str() {
                        if permissions
                            .workflow_permissions
                            .restricted_workflow_types
                            .contains(&workflow_str.to_string())
                        {
                            return Err(SandboxError::PermissionDenied(format!(
                                "Workflow type '{}' is restricted",
                                workflow_str
                            )));
                        }
                    }
                }
            }
            _ => {
                return Err(SandboxError::PermissionDenied(format!(
                    "Unknown operation: {}",
                    request.operation
                )));
            }
        }

        Ok(())
    }

    fn is_command_allowed(&self, command: &str, permissions: &PermissionSet) -> bool {
        // Check if command matches any allowed command patterns
        permissions
            .allowed_commands
            .iter()
            .any(|cmd_perm| match &cmd_perm.pattern {
                crate::entities::CommandPattern::Exact {
                    command: allowed_cmd,
                } => command == allowed_cmd,
                crate::entities::CommandPattern::Prefix { prefix } => command.starts_with(prefix),
                crate::entities::CommandPattern::Regex { pattern } => {
                    if let Ok(regex) = regex::Regex::new(pattern) {
                        regex.is_match(command)
                    } else {
                        false
                    }
                }
                crate::entities::CommandPattern::Builtin { command_type } => {
                    self.matches_builtin_command_type(command, command_type)
                }
            })
    }

    fn matches_builtin_command_type(
        &self,
        command: &str,
        command_type: &crate::entities::BuiltinCommandType,
    ) -> bool {
        use crate::entities::BuiltinCommandType;
        match command_type {
            BuiltinCommandType::Git => command.starts_with("git"),
            BuiltinCommandType::Cargo => command.starts_with("cargo"),
            BuiltinCommandType::Engram => command.starts_with("engram"),
            BuiltinCommandType::FileSystem => matches!(
                command,
                "read_file" | "write_file" | "delete_file" | "list_files" | "create_file"
            ),
            BuiltinCommandType::Network => matches!(
                command,
                "network_request" | "http_get" | "http_post" | "download_file"
            ),
            BuiltinCommandType::System => {
                matches!(command, "execute_command" | "system_info" | "process_list")
            }
        }
    }

    fn is_internal_url(&self, url: &str) -> bool {
        // Simple string-based internal URL check without url crate dependency
        url.starts_with("http://127.")
            || url.starts_with("https://127.")
            || url.starts_with("http://192.168.")
            || url.starts_with("https://192.168.")
            || url.starts_with("http://10.")
            || url.starts_with("https://10.")
            || url.starts_with("http://localhost")
            || url.starts_with("https://localhost")
            || url.contains(".local")
    }

    pub fn cache_permissions(&mut self, agent_id: String, permissions: PermissionSet) {
        self.cached_permissions.insert(agent_id, permissions);
    }

    pub fn clear_cache(&mut self) {
        self.cached_permissions.clear();
    }

    pub fn get_cached_permissions(&self, agent_id: &str) -> Option<&PermissionSet> {
        self.cached_permissions.get(agent_id)
    }
}

impl Default for PermissionEngine {
    fn default() -> Self {
        Self::new()
    }
}

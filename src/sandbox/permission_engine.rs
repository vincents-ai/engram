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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::{CommandPattern, CommandPermission, RiskLevel, WorkflowPermissions};

    fn create_test_permissions() -> PermissionSet {
        let mut allowed_file_ops = Vec::new(); // Changed to Vec
        allowed_file_ops.push(FileOperation::Read);

        PermissionSet {
            allowed_commands: vec![
                CommandPermission {
                    pattern: CommandPattern::Exact {
                        command: "echo".to_string(),
                    },
                    description: "echo command".to_string(), // Added description
                    risk_level: crate::entities::RiskLevel::Low, // Added risk_level
                },
                CommandPattern::Prefix {
                    prefix: "git".to_string(),
                }
                .into(),
            ],
            allowed_file_operations: allowed_file_ops,
            network_access: NetworkPolicy::InternalOnly,
            workflow_permissions: WorkflowPermissions {
                can_create_workflows: true,
                can_modify_workflows: false,
                can_execute_workflows: true,
                restricted_workflow_types: vec!["dangerous_workflow".to_string()], // Changed to Vec
            },
            forbidden_paths: vec![],          // Added forbidden_paths
            quality_gate_permissions: vec![], // Added quality_gate_permissions
        }
    }

    #[tokio::test]
    async fn test_validate_file_operations() {
        let mut engine = PermissionEngine::new();
        let permissions = create_test_permissions();

        // Test allowed read
        let req_read = SandboxRequest {
            operation: "read_file".to_string(),
            parameters: serde_json::Value::Object(serde_json::Map::new()), // Changed to Value
            agent_id: "test_agent".to_string(),
            resource_type: "file".to_string(),
            session_id: Some("session_1".to_string()), // Wrapped in Some
            timestamp: chrono::Utc::now(),             // Added timestamp
        };
        assert!(engine
            .validate_operation(&req_read, &permissions)
            .await
            .is_ok());

        // Test denied write
        let req_write = SandboxRequest {
            operation: "write_file".to_string(),
            parameters: serde_json::Value::Object(serde_json::Map::new()), // Changed to Value
            agent_id: "test_agent".to_string(),
            resource_type: "file".to_string(),
            session_id: Some("session_1".to_string()), // Wrapped in Some
            timestamp: chrono::Utc::now(),             // Added timestamp
        };
        assert!(engine
            .validate_operation(&req_write, &permissions)
            .await
            .is_err());
    }

    #[tokio::test]
    async fn test_validate_command_execution() {
        let mut engine = PermissionEngine::new();
        let permissions = create_test_permissions();

        // Exact match
        let mut params = serde_json::Map::new(); // Changed to Map
        params.insert("command".to_string(), serde_json::json!("echo hello"));
        let _req_echo = SandboxRequest {
            operation: "execute_command".to_string(),
            parameters: serde_json::Value::Object(params), // Wrapped in Value
            agent_id: "test_agent".to_string(),
            resource_type: "system".to_string(),
            session_id: Some("session_1".to_string()), // Wrapped in Some
            timestamp: chrono::Utc::now(),             // Added timestamp
        };

        // "echo" matches exact "echo", but "echo hello" doesn't match exact "echo".
        // The implementation checks `regex.is_match(command)` or `command == allowed_cmd`.

        // Let's test a prefix one that works.
        let mut params_git = serde_json::Map::new(); // Changed to Map
        params_git.insert("command".to_string(), serde_json::json!("git status"));
        let req_git = SandboxRequest {
            operation: "execute_command".to_string(),
            parameters: serde_json::Value::Object(params_git), // Wrapped in Value
            agent_id: "test_agent".to_string(),
            resource_type: "system".to_string(),
            session_id: Some("session_1".to_string()), // Wrapped in Some
            timestamp: chrono::Utc::now(),             // Added timestamp
        };
        assert!(engine
            .validate_operation(&req_git, &permissions)
            .await
            .is_ok());

        // Test denied command
        let mut params_rm = serde_json::Map::new(); // Changed to Map
        params_rm.insert("command".to_string(), serde_json::json!("rm -rf /"));
        let req_rm = SandboxRequest {
            operation: "execute_command".to_string(),
            parameters: serde_json::Value::Object(params_rm), // Wrapped in Value
            agent_id: "test_agent".to_string(),
            resource_type: "system".to_string(),
            session_id: Some("session_1".to_string()), // Wrapped in Some
            timestamp: chrono::Utc::now(),             // Added timestamp
        };
        assert!(engine
            .validate_operation(&req_rm, &permissions)
            .await
            .is_err());
    }

    #[tokio::test]
    async fn test_network_policy_internal_only() {
        let mut engine = PermissionEngine::new();
        let permissions = create_test_permissions(); // Has InternalOnly

        // Localhost allowed
        let mut params_local = serde_json::Map::new(); // Changed to Map
        params_local.insert(
            "url".to_string(),
            serde_json::json!("http://localhost:8080"),
        );
        let req_local = SandboxRequest {
            operation: "network_request".to_string(),
            parameters: serde_json::Value::Object(params_local), // Wrapped in Value
            agent_id: "test_agent".to_string(),
            resource_type: "network".to_string(),
            session_id: Some("session_1".to_string()), // Wrapped in Some
            timestamp: chrono::Utc::now(),             // Added timestamp
        };
        assert!(engine
            .validate_operation(&req_local, &permissions)
            .await
            .is_ok());

        // External denied
        let mut params_ext = serde_json::Map::new(); // Changed to Map
        params_ext.insert("url".to_string(), serde_json::json!("https://google.com"));
        let req_ext = SandboxRequest {
            operation: "network_request".to_string(),
            parameters: serde_json::Value::Object(params_ext), // Wrapped in Value
            agent_id: "test_agent".to_string(),
            resource_type: "network".to_string(),
            session_id: Some("session_1".to_string()), // Wrapped in Some
            timestamp: chrono::Utc::now(),             // Added timestamp
        };
        assert!(engine
            .validate_operation(&req_ext, &permissions)
            .await
            .is_err());
    }

    #[tokio::test]
    async fn test_workflow_permissions() {
        let mut engine = PermissionEngine::new();
        let permissions = create_test_permissions();

        // Create workflow allowed
        let req_create = SandboxRequest {
            operation: "create_workflow".to_string(),
            parameters: serde_json::Value::Object(serde_json::Map::new()), // Changed to Value
            agent_id: "test_agent".to_string(),
            resource_type: "workflow".to_string(),
            session_id: Some("session_1".to_string()), // Wrapped in Some
            timestamp: chrono::Utc::now(),             // Added timestamp
        };
        assert!(engine
            .validate_operation(&req_create, &permissions)
            .await
            .is_ok());

        // Modify workflow denied
        let req_modify = SandboxRequest {
            operation: "modify_workflow".to_string(),
            parameters: serde_json::Value::Object(serde_json::Map::new()), // Changed to Value
            agent_id: "test_agent".to_string(),
            resource_type: "workflow".to_string(),
            session_id: Some("session_1".to_string()), // Wrapped in Some
            timestamp: chrono::Utc::now(),             // Added timestamp
        };
        assert!(engine
            .validate_operation(&req_modify, &permissions)
            .await
            .is_err());

        // Execute restricted workflow type
        let mut params_restricted = serde_json::Map::new(); // Changed to Map
        params_restricted.insert(
            "workflow_type".to_string(),
            serde_json::json!("dangerous_workflow"),
        );
        let req_exec_restricted = SandboxRequest {
            operation: "execute_workflow".to_string(),
            parameters: serde_json::Value::Object(params_restricted), // Wrapped in Value
            agent_id: "test_agent".to_string(),
            resource_type: "workflow".to_string(),
            session_id: Some("session_1".to_string()), // Wrapped in Some
            timestamp: chrono::Utc::now(),             // Added timestamp
        };
        assert!(engine
            .validate_operation(&req_exec_restricted, &permissions)
            .await
            .is_err());
    }

    #[test]
    fn test_caching() {
        let mut engine = PermissionEngine::new();
        let permissions = create_test_permissions();
        let agent_id = "agent_007".to_string();

        engine.cache_permissions(agent_id.clone(), permissions);
        assert!(engine.get_cached_permissions(&agent_id).is_some());

        engine.clear_cache();
        assert!(engine.get_cached_permissions(&agent_id).is_none());
    }

    #[test]
    fn test_default() {
        let _ = PermissionEngine::default();
    }

    #[tokio::test]
    async fn test_list_files() {
        let mut e = PermissionEngine::new();
        let p = create_test_permissions();
        let req = SandboxRequest {
            operation: "list_files".into(),
            parameters: serde_json::Value::Object(serde_json::Map::new()),
            agent_id: "t".into(),
            resource_type: "f".into(),
            session_id: None,
            timestamp: chrono::Utc::now(),
        };
        assert!(e.validate_operation(&req, &p).await.is_ok());
    }

    #[tokio::test]
    async fn test_all_file_ops_with_perms() {
        let mut e = PermissionEngine::new();
        let mut p = create_test_permissions();
        p.allowed_file_operations = vec![
            FileOperation::Read,
            FileOperation::Write,
            FileOperation::Delete,
        ];
        for op in &[
            "read_file",
            "list_files",
            "write_file",
            "create_file",
            "modify_file",
            "delete_file",
            "move_file",
        ] {
            let req = SandboxRequest {
                operation: (*op).into(),
                parameters: serde_json::Value::Object(serde_json::Map::new()),
                agent_id: "t".into(),
                resource_type: "f".into(),
                session_id: None,
                timestamp: chrono::Utc::now(),
            };
            assert!(e.validate_operation(&req, &p).await.is_ok(), "{}", op);
        }
    }

    #[tokio::test]
    async fn test_all_file_ops_denied() {
        let mut e = PermissionEngine::new();
        let p = PermissionSet {
            allowed_commands: vec![],
            forbidden_paths: vec![],
            allowed_file_operations: vec![],
            network_access: NetworkPolicy::Denied,
            quality_gate_permissions: vec![],
            workflow_permissions: WorkflowPermissions {
                can_create_workflows: false,
                can_modify_workflows: false,
                can_execute_workflows: false,
                restricted_workflow_types: vec![],
            },
        };
        for op in &[
            "read_file",
            "write_file",
            "create_file",
            "modify_file",
            "delete_file",
            "move_file",
        ] {
            let req = SandboxRequest {
                operation: (*op).into(),
                parameters: serde_json::Value::Object(serde_json::Map::new()),
                agent_id: "t".into(),
                resource_type: "f".into(),
                session_id: None,
                timestamp: chrono::Utc::now(),
            };
            assert!(e.validate_operation(&req, &p).await.is_err(), "{}", op);
        }
    }

    #[tokio::test]
    async fn test_network_denied() {
        let mut e = PermissionEngine::new();
        let mut p = create_test_permissions();
        p.network_access = NetworkPolicy::Denied;
        let req = SandboxRequest {
            operation: "network_request".into(),
            parameters: serde_json::Value::Object(serde_json::Map::new()),
            agent_id: "t".into(),
            resource_type: "n".into(),
            session_id: None,
            timestamp: chrono::Utc::now(),
        };
        assert!(e.validate_operation(&req, &p).await.is_err());
    }

    #[tokio::test]
    async fn test_network_monitoring() {
        let mut e = PermissionEngine::new();
        let mut p = create_test_permissions();
        p.network_access = NetworkPolicy::AllowedWithMonitoring;
        let req = SandboxRequest {
            operation: "network_request".into(),
            parameters: serde_json::Value::Object(serde_json::Map::new()),
            agent_id: "t".into(),
            resource_type: "n".into(),
            session_id: None,
            timestamp: chrono::Utc::now(),
        };
        assert!(e.validate_operation(&req, &p).await.is_ok());
    }

    #[tokio::test]
    async fn test_network_unrestricted() {
        let mut e = PermissionEngine::new();
        let mut p = create_test_permissions();
        p.network_access = NetworkPolicy::Unrestricted;
        let req = SandboxRequest {
            operation: "network_request".into(),
            parameters: serde_json::Value::Object(serde_json::Map::new()),
            agent_id: "t".into(),
            resource_type: "n".into(),
            session_id: None,
            timestamp: chrono::Utc::now(),
        };
        assert!(e.validate_operation(&req, &p).await.is_ok());
    }

    #[tokio::test]
    async fn test_internal_no_url() {
        let mut e = PermissionEngine::new();
        let p = create_test_permissions();
        let req = SandboxRequest {
            operation: "network_request".into(),
            parameters: serde_json::Value::Object(serde_json::Map::new()),
            agent_id: "t".into(),
            resource_type: "n".into(),
            session_id: None,
            timestamp: chrono::Utc::now(),
        };
        assert!(e.validate_operation(&req, &p).await.is_ok());
    }

    #[tokio::test]
    async fn test_internal_non_string_url() {
        let mut e = PermissionEngine::new();
        let p = create_test_permissions();
        let mut params = serde_json::Map::new();
        params.insert("url".into(), serde_json::json!(12345));
        let req = SandboxRequest {
            operation: "network_request".into(),
            parameters: serde_json::Value::Object(params),
            agent_id: "t".into(),
            resource_type: "n".into(),
            session_id: None,
            timestamp: chrono::Utc::now(),
        };
        assert!(e.validate_operation(&req, &p).await.is_ok());
    }

    #[tokio::test]
    async fn test_unknown_operation() {
        let mut e = PermissionEngine::new();
        let p = create_test_permissions();
        let req = SandboxRequest {
            operation: "completely_unknown".into(),
            parameters: serde_json::Value::Object(serde_json::Map::new()),
            agent_id: "t".into(),
            resource_type: "o".into(),
            session_id: None,
            timestamp: chrono::Utc::now(),
        };
        assert!(
            matches!(e.validate_operation(&req, &p).await, Err(SandboxError::PermissionDenied(m)) if m.contains("Unknown operation"))
        );
    }

    #[tokio::test]
    async fn test_exec_no_cmd_param() {
        let mut e = PermissionEngine::new();
        let p = create_test_permissions();
        let req = SandboxRequest {
            operation: "execute_command".into(),
            parameters: serde_json::Value::Object(serde_json::Map::new()),
            agent_id: "t".into(),
            resource_type: "s".into(),
            session_id: None,
            timestamp: chrono::Utc::now(),
        };
        assert!(e.validate_operation(&req, &p).await.is_err());
    }

    #[tokio::test]
    async fn test_wf_create_denied() {
        let mut e = PermissionEngine::new();
        let mut p = create_test_permissions();
        p.workflow_permissions.can_create_workflows = false;
        let req = SandboxRequest {
            operation: "create_workflow".into(),
            parameters: serde_json::Value::Object(serde_json::Map::new()),
            agent_id: "t".into(),
            resource_type: "w".into(),
            session_id: None,
            timestamp: chrono::Utc::now(),
        };
        assert!(e.validate_operation(&req, &p).await.is_err());
    }

    #[tokio::test]
    async fn test_wf_modify_allowed() {
        let mut e = PermissionEngine::new();
        let mut p = create_test_permissions();
        p.workflow_permissions.can_modify_workflows = true;
        let req = SandboxRequest {
            operation: "modify_workflow".into(),
            parameters: serde_json::Value::Object(serde_json::Map::new()),
            agent_id: "t".into(),
            resource_type: "w".into(),
            session_id: None,
            timestamp: chrono::Utc::now(),
        };
        assert!(e.validate_operation(&req, &p).await.is_ok());
    }

    #[tokio::test]
    async fn test_wf_exec_allowed() {
        let mut e = PermissionEngine::new();
        let p = create_test_permissions();
        let mut params = serde_json::Map::new();
        params.insert("workflow_type".into(), serde_json::json!("safe_workflow"));
        let req = SandboxRequest {
            operation: "execute_workflow".into(),
            parameters: serde_json::Value::Object(params),
            agent_id: "t".into(),
            resource_type: "w".into(),
            session_id: None,
            timestamp: chrono::Utc::now(),
        };
        assert!(e.validate_operation(&req, &p).await.is_ok());
    }

    #[tokio::test]
    async fn test_wf_exec_no_type() {
        let mut e = PermissionEngine::new();
        let p = create_test_permissions();
        let req = SandboxRequest {
            operation: "execute_workflow".into(),
            parameters: serde_json::Value::Object(serde_json::Map::new()),
            agent_id: "t".into(),
            resource_type: "w".into(),
            session_id: None,
            timestamp: chrono::Utc::now(),
        };
        assert!(e.validate_operation(&req, &p).await.is_ok());
    }

    #[tokio::test]
    async fn test_wf_exec_denied() {
        let mut e = PermissionEngine::new();
        let mut p = create_test_permissions();
        p.workflow_permissions.can_execute_workflows = false;
        let req = SandboxRequest {
            operation: "execute_workflow".into(),
            parameters: serde_json::Value::Object(serde_json::Map::new()),
            agent_id: "t".into(),
            resource_type: "w".into(),
            session_id: None,
            timestamp: chrono::Utc::now(),
        };
        assert!(e.validate_operation(&req, &p).await.is_err());
    }

    #[tokio::test]
    async fn test_wf_type_non_string() {
        let mut e = PermissionEngine::new();
        let p = create_test_permissions();
        let mut params = serde_json::Map::new();
        params.insert("workflow_type".into(), serde_json::json!(12345));
        let req = SandboxRequest {
            operation: "execute_workflow".into(),
            parameters: serde_json::Value::Object(params),
            agent_id: "t".into(),
            resource_type: "w".into(),
            session_id: None,
            timestamp: chrono::Utc::now(),
        };
        assert!(e.validate_operation(&req, &p).await.is_ok());
    }

    #[test]
    fn test_is_internal_url() {
        let e = PermissionEngine::new();
        assert!(e.is_internal_url("http://127.0.0.1"));
        assert!(e.is_internal_url("https://127.0.0.1"));
        assert!(e.is_internal_url("http://192.168.1.1"));
        assert!(e.is_internal_url("https://192.168.1.1"));
        assert!(e.is_internal_url("http://10.0.0.1"));
        assert!(e.is_internal_url("https://10.0.0.1"));
        assert!(e.is_internal_url("http://localhost:8080"));
        assert!(e.is_internal_url("https://localhost"));
        assert!(e.is_internal_url("http://my.local"));
        assert!(!e.is_internal_url("https://google.com"));
        assert!(!e.is_internal_url("http://example.com"));
        assert!(!e.is_internal_url("https://8.8.8.8"));
    }

    #[test]
    fn test_cmd_allowed_exact() {
        let e = PermissionEngine::new();
        let p = create_test_permissions();
        assert!(e.is_command_allowed("echo", &p));
        assert!(!e.is_command_allowed("ls", &p));
    }

    #[test]
    fn test_cmd_allowed_prefix() {
        let e = PermissionEngine::new();
        let p = create_test_permissions();
        assert!(e.is_command_allowed("git status", &p));
        assert!(!e.is_command_allowed("svn status", &p));
    }

    #[test]
    fn test_cmd_allowed_regex() {
        let e = PermissionEngine::new();
        let mut p = create_test_permissions();
        p.allowed_commands.push(CommandPermission {
            pattern: CommandPattern::Regex {
                pattern: r"^test_\w+$".into(),
            },
            description: "t".into(),
            risk_level: RiskLevel::Low,
        });
        assert!(e.is_command_allowed("test_hello", &p));
        assert!(!e.is_command_allowed("test-", &p));
    }

    #[test]
    fn test_cmd_allowed_invalid_regex() {
        let e = PermissionEngine::new();
        let mut p = create_test_permissions();
        p.allowed_commands.push(CommandPermission {
            pattern: CommandPattern::Regex {
                pattern: r"[invalid(".into(),
            },
            description: "t".into(),
            risk_level: RiskLevel::Low,
        });
        assert!(!e.is_command_allowed("anything", &p));
    }

    #[test]
    fn test_builtin_cmd_type() {
        let e = PermissionEngine::new();
        use crate::entities::BuiltinCommandType;
        assert!(e.matches_builtin_command_type("git status", &BuiltinCommandType::Git));
        assert!(e.matches_builtin_command_type("cargo build", &BuiltinCommandType::Cargo));
        assert!(e.matches_builtin_command_type("engram ask", &BuiltinCommandType::Engram));
        assert!(e.matches_builtin_command_type("read_file", &BuiltinCommandType::FileSystem));
        assert!(e.matches_builtin_command_type("network_request", &BuiltinCommandType::Network));
        assert!(e.matches_builtin_command_type("execute_command", &BuiltinCommandType::System));
        assert!(!e.matches_builtin_command_type("svn", &BuiltinCommandType::Git));
        assert!(!e.matches_builtin_command_type("npm", &BuiltinCommandType::Cargo));
        assert!(!e.matches_builtin_command_type("read_dir", &BuiltinCommandType::FileSystem));
        assert!(!e.matches_builtin_command_type("ftp", &BuiltinCommandType::Network));
        assert!(!e.matches_builtin_command_type("run", &BuiltinCommandType::System));
    }

    #[tokio::test]
    async fn test_internal_urls_pass() {
        let mut e = PermissionEngine::new();
        let p = create_test_permissions();
        for url in &[
            "http://127.0.0.1:8080",
            "http://localhost:3000",
            "http://192.168.1.100",
            "http://10.0.0.5",
            "http://s.local",
        ] {
            let mut params = serde_json::Map::new();
            params.insert("url".into(), serde_json::json!(url));
            let req = SandboxRequest {
                operation: "network_request".into(),
                parameters: serde_json::Value::Object(params),
                agent_id: "t".into(),
                resource_type: "n".into(),
                session_id: None,
                timestamp: chrono::Utc::now(),
            };
            assert!(e.validate_operation(&req, &p).await.is_ok(), "{}", url);
        }
        for url in &["https://google.com", "http://example.org"] {
            let mut params = serde_json::Map::new();
            params.insert("url".into(), serde_json::json!(url));
            let req = SandboxRequest {
                operation: "network_request".into(),
                parameters: serde_json::Value::Object(params),
                agent_id: "t".into(),
                resource_type: "n".into(),
                session_id: None,
                timestamp: chrono::Utc::now(),
            };
            assert!(e.validate_operation(&req, &p).await.is_err(), "{}", url);
        }
    }
}

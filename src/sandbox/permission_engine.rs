use crate::entities::PermissionSet;
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
                if !permissions.can_read_files {
                    return Err(SandboxError::PermissionDenied(
                        "File read operations not permitted".to_string(),
                    ));
                }
            }
            "write_file" | "create_file" | "modify_file" => {
                if !permissions.can_write_files {
                    return Err(SandboxError::PermissionDenied(
                        "File write operations not permitted".to_string(),
                    ));
                }
            }
            "delete_file" | "move_file" => {
                if !permissions.can_delete_files {
                    return Err(SandboxError::PermissionDenied(
                        "File deletion operations not permitted".to_string(),
                    ));
                }
            }
            "execute_command" => {
                if !permissions.can_execute_commands {
                    return Err(SandboxError::PermissionDenied(
                        "Command execution not permitted".to_string(),
                    ));
                }
            }
            "network_request" => {
                if !permissions.can_access_network {
                    return Err(SandboxError::PermissionDenied(
                        "Network access not permitted".to_string(),
                    ));
                }
            }
            "modify_entity" => {
                if !permissions.can_modify_entities {
                    return Err(SandboxError::PermissionDenied(
                        "Entity modification not permitted".to_string(),
                    ));
                }
            }
            "delete_entity" => {
                if !permissions.can_delete_entities {
                    return Err(SandboxError::PermissionDenied(
                        "Entity deletion not permitted".to_string(),
                    ));
                }
            }
            "create_workflow" => {
                if !permissions.can_create_workflows {
                    return Err(SandboxError::PermissionDenied(
                        "Workflow creation not permitted".to_string(),
                    ));
                }
            }
            "modify_workflow" => {
                if !permissions.can_modify_workflows {
                    return Err(SandboxError::PermissionDenied(
                        "Workflow modification not permitted".to_string(),
                    ));
                }
            }
            "execute_workflow" => {
                if !permissions.can_execute_workflows {
                    return Err(SandboxError::PermissionDenied(
                        "Workflow execution not permitted".to_string(),
                    ));
                }

                if let Some(workflow_type) = request.parameters.get("workflow_type") {
                    if let Some(workflow_str) = workflow_type.as_str() {
                        if permissions
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

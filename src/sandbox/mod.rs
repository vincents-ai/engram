//! Agent Sandbox System
//!
//! Provides safety and security constraints for agent operations through:
//! - Permission validation
//! - Resource monitoring
//! - Command filtering
//! - Escalation handling

pub mod command_validator;
pub mod escalation_handler;
pub mod permission_engine;
pub mod resource_monitor;

use crate::entities::agent_sandbox::OperationType;
use crate::entities::{
    AgentSandbox, Entity, EscalationOperationType, EscalationPriority, EscalationRequest,
    OperationContext, SandboxLevel,
};
use crate::storage::Storage;
use chrono::{DateTime, Duration as ChronoDuration, Utc};
use std::collections::HashMap;
use std::time::Instant;
use thiserror::Error;

pub use command_validator::CommandValidator;
pub use escalation_handler::{EscalationHandler, EscalationStatistics};
pub use permission_engine::PermissionEngine;
pub use resource_monitor::ResourceMonitor;

/// Errors that can occur during sandbox operations
#[derive(Error, Debug)]
pub enum SandboxError {
    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Resource limit exceeded: {0}")]
    ResourceLimitExceeded(String),

    #[error("Command not allowed: {0}")]
    CommandBlocked(String),

    #[error("Escalation required: {0}")]
    EscalationRequired(String),

    #[error("Invalid sandbox configuration: {0}")]
    InvalidConfig(String),

    #[error("Storage error: {0}")]
    StorageError(String),
}

/// Result type for sandbox operations
pub type SandboxResult<T> = Result<T, SandboxError>;

/// Request context for sandbox validation
#[derive(Debug, Clone, serde::Serialize)]
pub struct SandboxRequest {
    pub agent_id: String,
    pub operation: String,
    pub resource_type: String,
    pub parameters: serde_json::Value,
    pub timestamp: DateTime<Utc>,
    pub session_id: Option<String>,
}

/// Response from sandbox validation
#[derive(Debug, Clone)]
pub enum SandboxResponse {
    /// Operation is allowed to proceed
    Allow {
        conditions: Vec<String>,
        monitoring_required: bool,
    },

    /// Operation is denied
    Deny {
        reason: String,
        suggestion: Option<String>,
    },

    /// Operation requires escalation to human
    Escalate {
        reason: String,
        escalation_id: String,
        timeout: chrono::Duration,
    },

    /// Operation is deferred for later evaluation
    Defer {
        reason: String,
        retry_after: chrono::Duration,
    },
}

/// Main sandbox engine that orchestrates validation
pub struct SandboxEngine {
    permission_engine: PermissionEngine,
    resource_monitor: ResourceMonitor,
    command_validator: CommandValidator,
    storage: Box<dyn Storage>,
    start_time: Instant,
}

impl SandboxEngine {
    /// Create a new sandbox engine with the provided storage backend
    pub fn new(storage: Box<dyn Storage>) -> Self {
        Self {
            permission_engine: PermissionEngine::new(),
            resource_monitor: ResourceMonitor::new(),
            command_validator: CommandValidator::new(),
            storage,
            start_time: Instant::now(),
        }
    }

    /// Validate a sandbox request against all constraints
    pub async fn validate_request(
        &mut self,
        request: SandboxRequest,
    ) -> SandboxResult<SandboxResponse> {
        // Get sandbox configuration for the agent
        let sandbox = self.get_agent_sandbox(&request.agent_id).await?;

        // Step 1: Permission validation
        if let Err(e) = self
            .permission_engine
            .validate_operation(&request, &sandbox.permissions)
            .await
        {
            return Ok(SandboxResponse::Deny {
                reason: format!("Permission denied: {}", e),
                suggestion: Some(
                    "Request elevated permissions or contact administrator".to_string(),
                ),
            });
        }

        // Step 2: Resource limits validation
        if let Err(e) = self
            .resource_monitor
            .check_limits(&request.agent_id, &request, &sandbox.resource_limits)
            .await
        {
            return Ok(SandboxResponse::Deny {
                reason: format!("Resource limit exceeded: {}", e),
                suggestion: Some("Reduce resource usage or request higher limits".to_string()),
            });
        }

        // Step 3: Command filtering
        match self
            .command_validator
            .validate_command(&request, &sandbox.command_filter)
            .await?
        {
            CommandValidationResult::Allow => {}
            CommandValidationResult::Block(reason) => {
                return Ok(SandboxResponse::Deny {
                    reason: format!("Command blocked: {}", reason),
                    suggestion: Some("Use alternative commands or request permission".to_string()),
                });
            }
            CommandValidationResult::RequiresApproval => {
                // Check escalation policy
                if sandbox
                    .escalation_policy
                    .require_human_approval
                    .iter()
                    .any(|op_type| self.matches_operation_type(&request.operation, op_type))
                {
                    let escalation_id = self.create_escalation_request(&request, &sandbox).await?;
                    return Ok(SandboxResponse::Escalate {
                        reason: "Operation requires human approval".to_string(),
                        escalation_id,
                        timeout: ChronoDuration::from_std(
                            sandbox.escalation_policy.escalation_timeout,
                        )
                        .unwrap_or(ChronoDuration::minutes(10)),
                    });
                }
            }
        }

        // Step 4: Check if monitoring is required
        let monitoring_required = self.requires_monitoring(&request, &sandbox);

        // Operation is allowed
        Ok(SandboxResponse::Allow {
            conditions: self.get_operation_conditions(&request, &sandbox),
            monitoring_required,
        })
    }

    /// Get sandbox configuration for an agent
    async fn get_agent_sandbox(&mut self, agent_id: &str) -> SandboxResult<AgentSandbox> {
        // Try to find existing sandbox for this agent
        let entity_ids: Vec<String> = self
            .storage
            .list_ids("agent_sandbox")
            .map_err(|e| SandboxError::StorageError(e.to_string()))?;

        for entity_id in entity_ids {
            if let Ok(Some(entity)) = self.storage.get(&entity_id, "agent_sandbox") {
                if let Ok(sandbox) = AgentSandbox::from_generic(entity) {
                    if sandbox.agent_id == agent_id {
                        return Ok(sandbox);
                    }
                }
            }
        }

        // No sandbox found, create default based on agent type
        self.create_default_sandbox(agent_id).await
    }

    /// Create default sandbox configuration for an agent
    async fn create_default_sandbox(&mut self, agent_id: &str) -> SandboxResult<AgentSandbox> {
        let sandbox = AgentSandbox::new(
            agent_id.to_string(),
            SandboxLevel::Standard, // Default level
            "system".to_string(),   // Created by system
            "default".to_string(),  // Default agent
        );

        // Store the new sandbox
        self.storage
            .store(&sandbox.to_generic())
            .map_err(|e| SandboxError::StorageError(e.to_string()))?;

        Ok(sandbox)
    }

    /// Create an escalation request for manual approval
    async fn create_escalation_request(
        &mut self,
        request: &SandboxRequest,
        sandbox: &AgentSandbox,
    ) -> SandboxResult<String> {
        let operation_type = self.infer_escalation_operation_type(&request.operation);
        let priority = self.infer_escalation_priority(sandbox, &request.operation);

        let operation_context = OperationContext {
            operation: request.operation.clone(),
            parameters: match request.parameters.as_object() {
                Some(obj) => obj.iter().map(|(k, v)| (k.clone(), v.clone())).collect(),
                None => HashMap::new(),
            },
            resource: Some(request.resource_type.clone()),
            block_reason: format!(
                "Operation blocked by sandbox level {:?}",
                sandbox.sandbox_level
            ),
            alternatives: self.suggest_alternatives(&request.operation),
            risk_assessment: Some(self.assess_risk(&request.operation)),
        };

        let mut escalation = EscalationRequest::new(
            request.agent_id.clone(),
            operation_type,
            operation_context,
            format!(
                "Agent {} requests permission for operation: {}",
                request.agent_id, request.operation
            ),
            priority,
            "default".to_string(),
        );

        escalation.session_id = request.session_id.clone();

        let generic_entity = escalation.to_generic();
        let escalation_id = escalation.id.clone();

        self.storage.store(&generic_entity).map_err(|e| {
            SandboxError::StorageError(format!("Failed to store escalation: {}", e))
        })?;

        Ok(escalation_id)
    }

    fn infer_escalation_operation_type(&self, operation: &str) -> EscalationOperationType {
        match operation {
            op if op.contains("file") || op.contains("File") => {
                EscalationOperationType::FileSystemAccess
            }
            op if op.contains("network") || op.contains("Network") => {
                EscalationOperationType::NetworkAccess
            }
            op if op.contains("command") || op.contains("execute") => {
                EscalationOperationType::CommandExecution
            }
            op if op.contains("workflow") || op.contains("Workflow") => {
                EscalationOperationType::WorkflowModification
            }
            op if op.contains("quality_gate") => EscalationOperationType::QualityGateOverride,
            op if op.contains("resource") || op.contains("limit") => {
                EscalationOperationType::ResourceLimitIncrease
            }
            _ => EscalationOperationType::Custom(operation.to_string()),
        }
    }

    fn infer_escalation_priority(
        &self,
        sandbox: &AgentSandbox,
        operation: &str,
    ) -> EscalationPriority {
        let high_risk_ops = ["delete", "remove", "execute", "command"];
        let is_high_risk = high_risk_ops.iter().any(|risk| operation.contains(risk));

        match (&sandbox.sandbox_level, is_high_risk) {
            (SandboxLevel::Training, _) => EscalationPriority::Low,
            (SandboxLevel::Isolated, true) => EscalationPriority::High,
            (SandboxLevel::Isolated, false) => EscalationPriority::Normal,
            (SandboxLevel::Restricted, true) => EscalationPriority::High,
            (SandboxLevel::Restricted, false) => EscalationPriority::Normal,
            (_, true) => EscalationPriority::High,
            (_, false) => EscalationPriority::Normal,
        }
    }

    fn suggest_alternatives(&self, operation: &str) -> Vec<String> {
        match operation {
            "file_delete" => vec![
                "Use file_move to archive instead".to_string(),
                "Request permission for specific file patterns".to_string(),
            ],
            "network_request" => vec![
                "Use internal API endpoints if available".to_string(),
                "Request proxy configuration".to_string(),
            ],
            "execute_command" => vec![
                "Use engram built-in operations".to_string(),
                "Request specific command whitelist".to_string(),
            ],
            _ => vec!["Contact administrator for guidance".to_string()],
        }
    }

    fn assess_risk(&self, operation: &str) -> String {
        match operation {
            op if op.contains("delete") || op.contains("remove") => {
                "High risk: Irreversible data modification".to_string()
            }
            op if op.contains("network") => {
                "Medium risk: External communication and data exfiltration potential".to_string()
            }
            op if op.contains("execute") || op.contains("command") => {
                "High risk: Arbitrary code execution".to_string()
            }
            op if op.contains("write") || op.contains("modify") => {
                "Medium risk: Data modification".to_string()
            }
            _ => "Low risk: Read-only or safe operation".to_string(),
        }
    }

    /// Check if an operation requires monitoring
    fn requires_monitoring(&self, request: &SandboxRequest, sandbox: &AgentSandbox) -> bool {
        // High-risk operations always require monitoring
        let high_risk_operations = [
            "file_write",
            "file_delete",
            "network_request",
            "execute_command",
            "modify_entity",
            "delete_entity",
        ];

        if high_risk_operations.contains(&request.operation.as_str()) {
            return true;
        }

        // Restricted and isolated levels require more monitoring
        matches!(
            sandbox.sandbox_level,
            SandboxLevel::Restricted | SandboxLevel::Isolated
        )
    }

    /// Get operation-specific conditions
    fn get_operation_conditions(
        &self,
        request: &SandboxRequest,
        sandbox: &AgentSandbox,
    ) -> Vec<String> {
        let mut conditions = Vec::new();

        // Add level-specific conditions
        match sandbox.sandbox_level {
            SandboxLevel::Training => {
                conditions.push("Operation will be logged for training purposes".to_string());
                conditions.push("No persistent changes will be made".to_string());
            }
            SandboxLevel::Restricted => {
                conditions.push("Operation will be closely monitored".to_string());
                conditions.push("Automatic rollback available".to_string());
            }
            SandboxLevel::Isolated => {
                conditions.push("Operation isolated from other agents".to_string());
                conditions.push("No external network access".to_string());
            }
            _ => {}
        }

        // Add operation-specific conditions
        match request.operation.as_str() {
            "file_write" => {
                conditions.push("File changes will be versioned".to_string());
            }
            "network_request" => {
                conditions.push("Network traffic will be logged".to_string());
            }
            _ => {}
        }

        conditions
    }

    /// Update sandbox configuration
    pub async fn update_sandbox(
        &mut self,
        agent_id: &str,
        level: SandboxLevel,
        _updated_by: &str,
    ) -> SandboxResult<()> {
        let mut sandbox = self.get_agent_sandbox(agent_id).await?;

        sandbox.sandbox_level = level;
        sandbox.last_modified = Utc::now();

        self.storage
            .store(&sandbox.to_generic())
            .map_err(|e| SandboxError::StorageError(e.to_string()))?;

        Ok(())
    }

    /// Record a violation for monitoring and analytics
    pub async fn record_violation(
        &mut self,
        agent_id: &str,
        violation_type: &str,
        description: &str,
    ) -> SandboxResult<()> {
        let mut sandbox = self.get_agent_sandbox(agent_id).await?;

        sandbox.violation_count += 1;
        sandbox.last_modified = Utc::now();

        // Add to metadata
        let violation_entry = serde_json::json!({
            "type": violation_type,
            "description": description,
            "timestamp": Utc::now(),
        });

        if let Some(violations) = sandbox.metadata.get_mut("violations") {
            if let Some(array) = violations.as_array_mut() {
                array.push(violation_entry);
            }
        } else {
            sandbox.metadata.insert(
                "violations".to_string(),
                serde_json::json!([violation_entry]),
            );
        }

        self.storage
            .store(&sandbox.to_generic())
            .map_err(|e| SandboxError::StorageError(e.to_string()))?;

        Ok(())
    }

    /// Get sandbox statistics for monitoring
    pub async fn get_sandbox_stats(&mut self, agent_id: &str) -> SandboxResult<SandboxStats> {
        let sandbox = self.get_agent_sandbox(agent_id).await?;

        Ok(SandboxStats {
            agent_id: sandbox.agent_id.clone(),
            sandbox_level: sandbox.sandbox_level.clone(),
            violation_count: sandbox.violation_count,
            created_at: sandbox.created_at,
            last_modified: sandbox.last_modified,
            uptime: self.start_time.elapsed(),
        })
    }

    /// Helper method to match operation string to OperationType
    fn matches_operation_type(&self, operation: &str, op_type: &OperationType) -> bool {
        use OperationType::*;
        match op_type {
            FileWrite => {
                operation == "file_write" || operation == "write_file" || operation == "create_file"
            }
            FileDelete => operation == "file_delete" || operation == "delete_file",
            CommandExecution => operation == "execute_command",
            NetworkAccess => operation == "network_request",
            ConfigChange => operation == "config_change",
            DatabaseOperation => operation == "database_operation",
            SystemFileAccess => operation == "system_file_access",
            PrivilegedOperation => operation == "privileged_operation",
        }
    }
}

/// Command validation result
#[derive(Debug, Clone)]
pub enum CommandValidationResult {
    Allow,
    Block(String),
    RequiresApproval,
}

/// Sandbox statistics for monitoring
#[derive(Debug, Clone)]
pub struct SandboxStats {
    pub agent_id: String,
    pub sandbox_level: SandboxLevel,
    pub violation_count: u32,
    pub created_at: DateTime<Utc>,
    pub last_modified: DateTime<Utc>,
    pub uptime: std::time::Duration,
}

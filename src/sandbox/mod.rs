//! Agent Sandbox System
//!
//! Provides safety and security constraints for agent operations through:
//! - Permission validation
//! - Resource monitoring
//! - Command filtering
//! - Escalation handling

pub mod command_validator;
pub mod permission_engine;
pub mod resource_monitor;

use crate::entities::{AgentSandbox, SandboxLevel};
use crate::storage::Storage;
use chrono::{DateTime, Utc};
use std::time::Instant;
use thiserror::Error;

pub use command_validator::CommandValidator;
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
#[derive(Debug, Clone)]
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
                    .contains(&request.operation)
                {
                    let escalation_id = self.create_escalation_request(&request, &sandbox).await?;
                    return Ok(SandboxResponse::Escalate {
                        reason: "Operation requires human approval".to_string(),
                        escalation_id,
                        timeout: sandbox.escalation_policy.escalation_timeout.into(),
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
        let sandboxes = self
            .storage
            .list_entities("agent_sandbox")
            .map_err(|e| SandboxError::StorageError(e.to_string()))?;

        for entity in sandboxes {
            if let Ok(sandbox) = AgentSandbox::from_generic(entity) {
                if sandbox.agent_id == agent_id {
                    return Ok(sandbox);
                }
            }
        }

        // No sandbox found, create default based on agent type
        self.create_default_sandbox(agent_id).await
    }

    /// Create default sandbox configuration for an agent
    async fn create_default_sandbox(&mut self, agent_id: &str) -> SandboxResult<AgentSandbox> {
        let sandbox = AgentSandbox::new_with_level(
            agent_id.to_string(),
            SandboxLevel::Standard, // Default level
            "system".to_string(),   // Created by system
            "default".to_string(),  // Default agent
        );

        // Store the new sandbox
        self.storage
            .store(&sandbox)
            .map_err(|e| SandboxError::StorageError(e.to_string()))?;

        Ok(sandbox)
    }

    /// Create an escalation request for manual approval
    async fn create_escalation_request(
        &mut self,
        request: &SandboxRequest,
        sandbox: &AgentSandbox,
    ) -> SandboxResult<String> {
        // Generate unique escalation ID
        let escalation_id = format!(
            "esc_{}_{}",
            request.agent_id,
            chrono::Utc::now().timestamp_millis()
        );

        // TODO: Create escalation entity and store it
        // This will be implemented when we create the escalation entities

        Ok(escalation_id)
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
        updated_by: &str,
    ) -> SandboxResult<()> {
        let mut sandbox = self.get_agent_sandbox(agent_id).await?;

        sandbox.sandbox_level = level;
        sandbox.last_modified = Utc::now();

        self.storage
            .store(&sandbox)
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
            .store(&sandbox)
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

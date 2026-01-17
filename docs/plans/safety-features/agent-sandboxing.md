# Agent Sandboxing System for Engram

**Date**: 2026-01-17
**Priority**: High
**Phase**: 1 - Safety & Reliability

## Overview

Implement a comprehensive sandboxing system that provides granular access control, resource limits, and security isolation for LLM agents, ensuring safe operation in production environments.

## Architecture

### Core Components

1. **Permission Engine** - Manages agent access rights and capabilities
2. **Resource Monitor** - Tracks and enforces resource usage limits
3. **Command Filter** - Validates and restricts agent operations
4. **Escalation System** - Handles permission elevation requests
5. **Isolation Manager** - Provides execution environment isolation

### Entity Design

```rust
// src/entities/agent_sandbox.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSandbox {
    pub agent_id: String,
    pub sandbox_level: SandboxLevel,
    pub permissions: PermissionSet,
    pub resource_limits: ResourceLimits,
    pub command_filter: CommandFilter,
    pub escalation_policy: EscalationPolicy,
    pub created_by: String,
    pub created_at: DateTime<Utc>,
    pub last_modified: DateTime<Utc>,
    pub violation_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SandboxLevel {
    Unrestricted,  // Full system access
    Standard,      // Normal development access
    Restricted,    // Limited access, requires approval for sensitive operations
    Isolated,      // Heavily restricted, read-only for most operations
    Training,      // Safe environment for learning agents
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionSet {
    pub allowed_commands: Vec<CommandPermission>,
    pub forbidden_paths: Vec<PathRestriction>,
    pub allowed_file_operations: Vec<FileOperation>,
    pub network_access: NetworkPolicy,
    pub quality_gate_permissions: Vec<QualityGatePermission>,
    pub workflow_permissions: WorkflowPermissions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    pub max_memory_mb: u64,
    pub max_cpu_percentage: u32,
    pub max_disk_space_mb: u64,
    pub max_execution_time_minutes: u32,
    pub max_concurrent_operations: u32,
    pub max_file_size_mb: u64,
    pub max_network_requests_per_minute: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandFilter {
    pub whitelist_mode: bool,
    pub allowed_commands: Vec<CommandPattern>,
    pub forbidden_commands: Vec<CommandPattern>,
    pub parameter_restrictions: HashMap<String, ParameterRestriction>,
    pub dangerous_patterns: Vec<DangerousPattern>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommandPattern {
    Exact { command: String },
    Prefix { prefix: String },
    Regex { pattern: String },
    Builtin { command_type: BuiltinCommandType },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscalationPolicy {
    pub auto_approve_safe_operations: bool,
    pub require_human_approval: Vec<OperationType>,
    pub escalation_timeout: Duration,
    pub fallback_action: FallbackAction,
    pub notification_channels: Vec<String>,
}
```

### Sandbox Engine

```rust
// src/sandbox/mod.rs
pub struct SandboxEngine {
    permission_engine: PermissionEngine,
    resource_monitor: ResourceMonitor,
    command_validator: CommandValidator,
    escalation_handler: EscalationHandler,
    audit_logger: AuditLogger,
}

impl SandboxEngine {
    pub async fn validate_operation(&self,
        agent_id: &str,
        operation: &AgentOperation
    ) -> Result<ValidationResult> {
        // 1. Get agent sandbox configuration
        let sandbox = self.get_agent_sandbox(agent_id).await?;
        
        // 2. Check permissions
        let permission_check = self.permission_engine
            .check_permission(&sandbox.permissions, operation).await?;
        
        if !permission_check.allowed {
            return Ok(ValidationResult::Denied {
                reason: permission_check.denial_reason,
                escalation_available: permission_check.can_escalate,
            });
        }
        
        // 3. Validate resource limits
        let resource_check = self.resource_monitor
            .check_resource_availability(&sandbox.resource_limits, operation).await?;
        
        if !resource_check.sufficient {
            return Ok(ValidationResult::ResourceExceeded {
                exceeded_limits: resource_check.exceeded_limits,
                suggested_action: resource_check.suggestion,
            });
        }
        
        // 4. Filter and validate command
        let command_check = self.command_validator
            .validate_command(&sandbox.command_filter, operation).await?;
        
        if !command_check.safe {
            return Ok(ValidationResult::UnsafeCommand {
                risk_factors: command_check.risk_factors,
                mitigation_options: command_check.mitigations,
            });
        }
        
        // 5. Log successful validation
        self.audit_logger.log_operation_approved(agent_id, operation).await?;
        
        Ok(ValidationResult::Approved {
            conditions: command_check.conditions,
            monitoring_required: resource_check.monitoring_required,
        })
    }
    
    pub async fn execute_with_sandbox(&self,
        agent_id: &str,
        operation: &AgentOperation
    ) -> Result<ExecutionResult> {
        // 1. Validate operation
        let validation = self.validate_operation(agent_id, operation).await?;
        
        match validation {
            ValidationResult::Approved { conditions, monitoring_required } => {
                // 2. Set up isolated execution environment
                let execution_context = self.create_execution_context(
                    agent_id, 
                    operation, 
                    &conditions
                ).await?;
                
                // 3. Execute with monitoring
                let result = if monitoring_required {
                    self.execute_with_monitoring(execution_context).await?
                } else {
                    self.execute_direct(execution_context).await?
                };
                
                // 4. Update resource usage tracking
                self.resource_monitor.update_usage(agent_id, &result).await?;
                
                Ok(result)
            },
            ValidationResult::Denied { reason, escalation_available } => {
                if escalation_available {
                    self.handle_escalation_request(agent_id, operation, reason).await
                } else {
                    Err(EngramError::OperationDenied { reason })
                }
            },
            // Handle other validation results...
            _ => Err(EngramError::OperationBlocked)
        }
    }
}

impl PermissionEngine {
    async fn check_permission(&self, 
        permissions: &PermissionSet,
        operation: &AgentOperation
    ) -> Result<PermissionCheckResult> {
        match operation {
            AgentOperation::FileOperation { path, operation_type } => {
                self.check_file_permission(permissions, path, operation_type).await
            },
            AgentOperation::Command { command, args } => {
                self.check_command_permission(permissions, command, args).await
            },
            AgentOperation::NetworkRequest { url, method } => {
                self.check_network_permission(permissions, url, method).await
            },
            AgentOperation::QualityGate { gate_name } => {
                self.check_quality_gate_permission(permissions, gate_name).await
            },
            // ... other operation types
        }
    }
    
    async fn check_file_permission(&self,
        permissions: &PermissionSet,
        path: &str,
        operation_type: &FileOperationType
    ) -> Result<PermissionCheckResult> {
        // Check forbidden paths
        for restriction in &permissions.forbidden_paths {
            if self.path_matches_restriction(path, restriction) {
                return Ok(PermissionCheckResult {
                    allowed: false,
                    denial_reason: format!("Path {} is forbidden", path),
                    can_escalate: restriction.escalation_allowed,
                });
            }
        }
        
        // Check allowed file operations
        if !permissions.allowed_file_operations.contains(operation_type) {
            return Ok(PermissionCheckResult {
                allowed: false,
                denial_reason: format!("File operation {:?} not permitted", operation_type),
                can_escalate: true,
            });
        }
        
        Ok(PermissionCheckResult {
            allowed: true,
            denial_reason: String::new(),
            can_escalate: false,
        })
    }
}
```

## Predefined Sandbox Levels

```yaml
# .engram/sandbox-levels.yaml
sandbox_levels:
  training:
    description: "Safe environment for learning agents"
    permissions:
      allowed_commands:
        - pattern: "cargo check"
        - pattern: "cargo test --lib"
        - pattern: "git status"
        - pattern: "git log"
      forbidden_paths:
        - "/etc/*"
        - "/home/*/.ssh/*"
        - "/home/*/.aws/*"
        - "*/src/security/*"
      allowed_file_operations: ["read", "write_temp"]
      network_access: "denied"
    resource_limits:
      max_memory_mb: 512
      max_cpu_percentage: 25
      max_execution_time_minutes: 10
    escalation_policy:
      auto_approve_safe_operations: false
      require_human_approval: ["file_write", "command_execution"]
      
  restricted:
    description: "Limited access with approval for sensitive operations"
    permissions:
      allowed_commands:
        - pattern: "cargo *"
        - pattern: "git *"
        - pattern: "engram task *"
        - pattern: "engram context *"
      forbidden_paths:
        - "/etc/*"
        - "*/migrations/*"
        - "*/Cargo.toml"
      allowed_file_operations: ["read", "write_non_config"]
      network_access: "internal_only"
    resource_limits:
      max_memory_mb: 2048
      max_cpu_percentage: 50
      max_execution_time_minutes: 30
    escalation_policy:
      auto_approve_safe_operations: true
      require_human_approval: ["config_change", "database_operation"]
      
  standard:
    description: "Normal development access"
    permissions:
      allowed_commands:
        - pattern: "*"
      forbidden_commands:
        - pattern: "rm -rf /*"
        - pattern: "sudo *"
        - pattern: "dd *"
      forbidden_paths:
        - "/etc/*"
        - "/root/*"
        - "/home/*/.ssh/id_*"
      allowed_file_operations: ["read", "write", "create", "delete_non_system"]
      network_access: "allowed_with_monitoring"
    resource_limits:
      max_memory_mb: 4096
      max_cpu_percentage: 75
      max_execution_time_minutes: 60
    escalation_policy:
      auto_approve_safe_operations: true
      require_human_approval: ["system_file_access", "privileged_operation"]
```

## CLI Commands

```bash
# Sandbox management
engram sandbox create --agent alice --level restricted     # Create sandbox
engram sandbox update --agent alice --level standard       # Update sandbox level
engram sandbox show --agent alice                         # Show sandbox config
engram sandbox delete --agent alice                       # Remove sandbox

# Permission management
engram sandbox allow --agent alice --command "cargo build"   # Allow specific command
engram sandbox forbid --agent alice --path "/etc/*"         # Forbid path access
engram sandbox grant --agent alice --operation file_write   # Grant operation
engram sandbox revoke --agent alice --operation network     # Revoke permission

# Resource limits
engram sandbox limit --agent alice --memory 2GB             # Set memory limit
engram sandbox limit --agent alice --cpu 50%                # Set CPU limit
engram sandbox limit --agent alice --time 30min             # Set time limit
engram sandbox monitor --agent alice                        # Monitor resource usage

# Escalation management
engram escalation pending                                    # Show pending requests
engram escalation approve --request req-123                 # Approve escalation
engram escalation deny --request req-123 --reason "unsafe"  # Deny escalation
engram escalation policy --agent alice --auto-approve safe  # Set escalation policy

# Sandbox monitoring
engram sandbox violations --agent alice                     # Show violation history
engram sandbox audit --agent alice --period 7d             # Audit sandbox activity
engram sandbox stats --all-agents                          # Usage statistics
engram sandbox health-check                                # System health check
```

## Integration with Task Execution

```rust
// Enhanced task execution with sandboxing
impl TaskExecutor {
    pub async fn execute_sandboxed_task(&mut self, task: &Task) -> Result<TaskResult> {
        let agent_id = &task.agent;
        
        // 1. Check if agent has sandbox
        let sandbox = self.sandbox_engine.get_agent_sandbox(agent_id).await?;
        
        // 2. Plan task operations
        let planned_operations = self.plan_task_operations(task).await?;
        
        // 3. Pre-validate all operations
        for operation in &planned_operations {
            let validation = self.sandbox_engine
                .validate_operation(agent_id, operation).await?;
            
            if !validation.is_approved() {
                return self.handle_blocked_operation(task, operation, validation).await;
            }
        }
        
        // 4. Execute task with sandbox monitoring
        let execution_context = SandboxedExecutionContext {
            task: task.clone(),
            sandbox: sandbox.clone(),
            operations: planned_operations,
        };
        
        self.execute_in_sandbox(execution_context).await
    }
    
    async fn handle_blocked_operation(&self,
        task: &Task,
        operation: &AgentOperation,
        validation: ValidationResult
    ) -> Result<TaskResult> {
        match validation {
            ValidationResult::Denied { reason, escalation_available } => {
                if escalation_available {
                    // Create escalation request
                    let escalation_id = self.create_escalation_request(
                        &task.agent,
                        operation,
                        &reason
                    ).await?;
                    
                    TaskResult::blocked(format!(
                        "Operation requires approval. Escalation created: {}", 
                        escalation_id
                    ))
                } else {
                    TaskResult::failed(format!("Operation denied: {}", reason))
                }
            },
            ValidationResult::ResourceExceeded { exceeded_limits, .. } => {
                TaskResult::blocked(format!(
                    "Resource limits exceeded: {:?}", 
                    exceeded_limits
                ))
            },
            // ... handle other validation results
        }
    }
}
```

## Example Scenarios

### Scenario 1: Restricted Agent Attempting Unsafe Operation
```bash
$ engram task execute auth-refactor --agent junior-agent
→ Analyzing task operations...
❌ Operation blocked: File write to /src/security/auth.rs
   Reason: Path forbidden for restricted agent
   Available actions:
   1. Request escalation (requires supervisor approval)
   2. Modify task scope to avoid restricted paths
   3. Assign to unrestricted agent

$ engram escalation request --operation file_write \
  --path "/src/security/auth.rs" --reason "auth refactoring task"
✓ Escalation request created: ESC-2026-001
   Notification sent to: security-team@company.com
   Estimated approval time: 15 minutes
```

### Scenario 2: Resource Limit Exceeded
```bash
$ engram sandbox monitor --agent alice --real-time
→ Agent alice resource usage:
   Memory: 1.8GB / 2.0GB (90% - Warning)
   CPU: 45% / 50% 
   Runtime: 25min / 30min
   
⚠️  WARNING: Memory usage approaching limit

$ engram task execute large-test-suite --agent alice
❌ Task execution blocked
   Reason: Insufficient memory (requires 1.5GB, only 0.2GB available)
   Suggestions:
   1. Wait for current operations to complete (5min estimated)
   2. Increase memory limit for this agent
   3. Split task into smaller operations
```

### Scenario 3: Successful Escalation
```bash
$ engram escalation approve ESC-2026-001 --approver security-lead
✓ Escalation approved
   Agent: junior-agent
   Operation: file_write to /src/security/auth.rs
   Temporary permission granted (expires in 2 hours)
   
→ Resuming blocked task: auth-refactor
✓ Task completed successfully with elevated permissions
   Automatic permission revocation in: 1h 45min
```

## File Structure

```
src/
├── sandbox/
│   ├── mod.rs                  # Main sandbox engine
│   ├── permission_engine.rs    # Permission validation
│   ├── resource_monitor.rs     # Resource usage tracking
│   ├── command_validator.rs    # Command filtering and validation
│   ├── escalation_handler.rs   # Permission escalation logic
│   └── isolation_manager.rs    # Execution environment isolation
├── entities/
│   ├── agent_sandbox.rs        # Sandbox configuration entities
│   ├── escalation_request.rs   # Escalation tracking entities
│   └── sandbox_violation.rs    # Violation logging entities
└── cli/
    ├── sandbox.rs              # Sandbox management commands
    └── escalation.rs           # Escalation management commands
```

## Success Metrics

1. **Security**: Zero unauthorized system access incidents
2. **Usability**: 95% of operations complete without escalation
3. **Performance**: <50ms overhead for sandbox validation
4. **Escalation Efficiency**: 90% of escalations resolved within SLA
5. **Resource Optimization**: 80% improvement in resource utilization

## Integration Points

- Extends existing agent identification system
- Integrates with task execution pipeline
- Uses existing audit and logging infrastructure
- Compatible with current CLI patterns
- Leverages existing storage and entity systems

## Future Enhancements

- Machine learning-based anomaly detection
- Dynamic permission adjustment based on agent behavior
- Integration with external security tools
- Container-based isolation for enhanced security
- Performance-based automatic sandbox level adjustment

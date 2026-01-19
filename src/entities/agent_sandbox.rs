//! Agent Sandbox entity implementation

use super::{Entity, GenericEntity};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use uuid::Uuid;
use validator::Validate;

/// Sandbox levels for agents
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SandboxLevel {
    /// Full system access
    Unrestricted,
    /// Normal development access  
    Standard,
    /// Limited access, requires approval for sensitive operations
    Restricted,
    /// Heavily restricted, read-only for most operations
    Isolated,
    /// Safe environment for learning agents
    Training,
}

/// Permission set for an agent
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct PermissionSet {
    /// Commands the agent is allowed to run
    pub allowed_commands: Vec<CommandPermission>,
    /// Paths the agent cannot access
    pub forbidden_paths: Vec<PathRestriction>,
    /// File operations the agent can perform
    pub allowed_file_operations: Vec<FileOperation>,
    /// Network access policy
    pub network_access: NetworkPolicy,
    /// Quality gate permissions
    pub quality_gate_permissions: Vec<QualityGatePermission>,
    /// Workflow permissions
    pub workflow_permissions: WorkflowPermissions,
}

/// Resource limits for an agent
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ResourceLimits {
    /// Maximum memory usage in MB
    #[validate(range(min = 1, max = 16384))]
    pub max_memory_mb: u64,
    /// Maximum CPU percentage
    #[validate(range(min = 1, max = 100))]
    pub max_cpu_percentage: u32,
    /// Maximum disk space usage in MB
    #[validate(range(min = 1, max = 1048576))]
    pub max_disk_space_mb: u64,
    /// Maximum execution time in minutes
    #[validate(range(min = 1, max = 1440))]
    pub max_execution_time_minutes: u32,
    /// Maximum concurrent operations
    #[validate(range(min = 1, max = 100))]
    pub max_concurrent_operations: u32,
    /// Maximum file size in MB
    #[validate(range(min = 1, max = 1024))]
    pub max_file_size_mb: u64,
    /// Maximum network requests per minute
    #[validate(range(min = 1, max = 1000))]
    pub max_network_requests_per_minute: u32,
}

/// Command filtering configuration
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CommandFilter {
    /// Whether to use whitelist mode (only allow listed commands)
    pub whitelist_mode: bool,
    /// Allowed command patterns
    pub allowed_commands: Vec<CommandPattern>,
    /// Forbidden command patterns
    pub forbidden_commands: Vec<CommandPattern>,
    /// Parameter restrictions for commands
    pub parameter_restrictions: HashMap<String, ParameterRestriction>,
    /// Dangerous patterns to watch for
    pub dangerous_patterns: Vec<DangerousPattern>,
}

/// Command patterns for filtering
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum CommandPattern {
    /// Exact command match
    Exact { command: String },
    /// Command prefix match
    Prefix { prefix: String },
    /// Regular expression match
    Regex { pattern: String },
    /// Built-in command type
    Builtin { command_type: BuiltinCommandType },
}

/// Built-in command types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BuiltinCommandType {
    Git,
    Cargo,
    Engram,
    FileSystem,
    Network,
    System,
}

/// Escalation policy for handling restricted operations
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct EscalationPolicy {
    /// Automatically approve operations deemed safe
    pub auto_approve_safe_operations: bool,
    /// Operations that require human approval
    pub require_human_approval: Vec<OperationType>,
    /// Timeout for escalation requests
    #[serde(with = "duration_serde")]
    pub escalation_timeout: Duration,
    /// Action to take when escalation times out
    pub fallback_action: FallbackAction,
    /// Notification channels for escalations
    pub notification_channels: Vec<String>,
}

/// Types of operations that can be escalated
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OperationType {
    FileWrite,
    FileDelete,
    CommandExecution,
    NetworkAccess,
    ConfigChange,
    DatabaseOperation,
    SystemFileAccess,
    PrivilegedOperation,
}

/// Fallback actions for escalation timeouts
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FallbackAction {
    Deny,
    Allow,
    Defer,
}

/// Command permissions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandPermission {
    pub pattern: CommandPattern,
    pub description: String,
    pub risk_level: RiskLevel,
}

/// Path restrictions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathRestriction {
    pub pattern: String,
    pub reason: String,
    pub escalation_allowed: bool,
}

/// File operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FileOperation {
    Read,
    Write,
    Create,
    Delete,
    Execute,
    WriteTemp,
    WriteNonConfig,
    DeleteNonSystem,
}

/// Network access policies
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NetworkPolicy {
    Denied,
    InternalOnly,
    AllowedWithMonitoring,
    Unrestricted,
}

/// Quality gate permissions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityGatePermission {
    pub gate_name: String,
    pub allowed: bool,
    pub requires_approval: bool,
}

/// Workflow permissions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowPermissions {
    pub can_create_workflows: bool,
    pub can_modify_workflows: bool,
    pub can_execute_workflows: bool,
    pub restricted_workflow_types: Vec<String>,
}

/// Parameter restrictions for commands
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterRestriction {
    pub allowed_values: Vec<String>,
    pub forbidden_values: Vec<String>,
    pub max_length: Option<usize>,
    pub pattern_validation: Option<String>,
}

/// Dangerous patterns to detect
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DangerousPattern {
    pub pattern: String,
    pub description: String,
    pub risk_level: RiskLevel,
    pub auto_block: bool,
}

/// Risk levels
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// Agent Sandbox entity
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct AgentSandbox {
    /// Unique identifier
    #[serde(rename = "id")]
    pub id: String,

    /// Agent this sandbox applies to
    #[serde(rename = "agent_id")]
    #[validate(length(min = 1))]
    pub agent_id: String,

    /// Sandbox level
    #[serde(rename = "sandbox_level")]
    pub sandbox_level: SandboxLevel,

    /// Permission set
    #[serde(rename = "permissions")]
    #[validate]
    pub permissions: PermissionSet,

    /// Resource limits
    #[serde(rename = "resource_limits")]
    #[validate]
    pub resource_limits: ResourceLimits,

    /// Command filter
    #[serde(rename = "command_filter")]
    #[validate]
    pub command_filter: CommandFilter,

    /// Escalation policy
    #[serde(rename = "escalation_policy")]
    #[validate]
    pub escalation_policy: EscalationPolicy,

    /// Who created this sandbox
    #[serde(rename = "created_by")]
    #[validate(length(min = 1))]
    pub created_by: String,

    /// Creation timestamp
    #[serde(rename = "created_at")]
    pub created_at: DateTime<Utc>,

    /// Last modification timestamp
    #[serde(rename = "last_modified")]
    pub last_modified: DateTime<Utc>,

    /// Number of violations
    #[serde(rename = "violation_count")]
    pub violation_count: u32,

    /// Associated agent
    #[serde(rename = "agent")]
    pub agent: String,

    /// Additional metadata
    #[serde(
        rename = "metadata",
        skip_serializing_if = "HashMap::is_empty",
        default
    )]
    pub metadata: HashMap<String, serde_json::Value>,
}

impl AgentSandbox {
    /// Create a new agent sandbox
    pub fn new(
        agent_id: String,
        sandbox_level: SandboxLevel,
        created_by: String,
        agent: String,
    ) -> Self {
        let now = Utc::now();

        let (permissions, resource_limits, command_filter, escalation_policy) =
            Self::default_config_for_level(&sandbox_level);

        Self {
            id: Uuid::new_v4().to_string(),
            agent_id,
            sandbox_level,
            permissions,
            resource_limits,
            command_filter,
            escalation_policy,
            created_by,
            created_at: now,
            last_modified: now,
            violation_count: 0,
            agent,
            metadata: HashMap::new(),
        }
    }

    /// Get default configuration for a sandbox level
    pub fn default_config_for_level(
        level: &SandboxLevel,
    ) -> (
        PermissionSet,
        ResourceLimits,
        CommandFilter,
        EscalationPolicy,
    ) {
        match level {
            SandboxLevel::Training => Self::training_config(),
            SandboxLevel::Restricted => Self::restricted_config(),
            SandboxLevel::Standard => Self::standard_config(),
            SandboxLevel::Isolated => Self::isolated_config(),
            SandboxLevel::Unrestricted => Self::unrestricted_config(),
        }
    }

    /// Training sandbox configuration
    fn training_config() -> (
        PermissionSet,
        ResourceLimits,
        CommandFilter,
        EscalationPolicy,
    ) {
        let permissions = PermissionSet {
            allowed_commands: vec![
                CommandPermission {
                    pattern: CommandPattern::Exact {
                        command: "cargo check".to_string(),
                    },
                    description: "Run cargo check".to_string(),
                    risk_level: RiskLevel::Low,
                },
                CommandPermission {
                    pattern: CommandPattern::Exact {
                        command: "cargo test --lib".to_string(),
                    },
                    description: "Run library tests".to_string(),
                    risk_level: RiskLevel::Low,
                },
                CommandPermission {
                    pattern: CommandPattern::Exact {
                        command: "git status".to_string(),
                    },
                    description: "Check git status".to_string(),
                    risk_level: RiskLevel::Low,
                },
            ],
            forbidden_paths: vec![
                PathRestriction {
                    pattern: "/etc/*".to_string(),
                    reason: "System configuration files".to_string(),
                    escalation_allowed: false,
                },
                PathRestriction {
                    pattern: "*/src/security/*".to_string(),
                    reason: "Security-sensitive code".to_string(),
                    escalation_allowed: true,
                },
            ],
            allowed_file_operations: vec![FileOperation::Read, FileOperation::WriteTemp],
            network_access: NetworkPolicy::Denied,
            quality_gate_permissions: vec![],
            workflow_permissions: WorkflowPermissions {
                can_create_workflows: false,
                can_modify_workflows: false,
                can_execute_workflows: false,
                restricted_workflow_types: vec!["security".to_string()],
            },
        };

        let resource_limits = ResourceLimits {
            max_memory_mb: 512,
            max_cpu_percentage: 25,
            max_disk_space_mb: 1024,
            max_execution_time_minutes: 10,
            max_concurrent_operations: 2,
            max_file_size_mb: 10,
            max_network_requests_per_minute: 0,
        };

        let command_filter = CommandFilter {
            whitelist_mode: true,
            allowed_commands: vec![
                CommandPattern::Builtin {
                    command_type: BuiltinCommandType::Cargo,
                },
                CommandPattern::Builtin {
                    command_type: BuiltinCommandType::Git,
                },
                CommandPattern::Builtin {
                    command_type: BuiltinCommandType::Engram,
                },
            ],
            forbidden_commands: vec![],
            parameter_restrictions: HashMap::new(),
            dangerous_patterns: vec![],
        };

        let escalation_policy = EscalationPolicy {
            auto_approve_safe_operations: false,
            require_human_approval: vec![OperationType::FileWrite, OperationType::CommandExecution],
            escalation_timeout: Duration::from_secs(3600), // 1 hour
            fallback_action: FallbackAction::Deny,
            notification_channels: vec!["training-supervisor".to_string()],
        };

        (
            permissions,
            resource_limits,
            command_filter,
            escalation_policy,
        )
    }

    /// Restricted sandbox configuration
    fn restricted_config() -> (
        PermissionSet,
        ResourceLimits,
        CommandFilter,
        EscalationPolicy,
    ) {
        let permissions = PermissionSet {
            allowed_commands: vec![
                CommandPermission {
                    pattern: CommandPattern::Prefix {
                        prefix: "cargo".to_string(),
                    },
                    description: "Cargo commands".to_string(),
                    risk_level: RiskLevel::Low,
                },
                CommandPermission {
                    pattern: CommandPattern::Prefix {
                        prefix: "git".to_string(),
                    },
                    description: "Git commands".to_string(),
                    risk_level: RiskLevel::Low,
                },
                CommandPermission {
                    pattern: CommandPattern::Prefix {
                        prefix: "engram".to_string(),
                    },
                    description: "Engram commands".to_string(),
                    risk_level: RiskLevel::Low,
                },
            ],
            forbidden_paths: vec![
                PathRestriction {
                    pattern: "/etc/*".to_string(),
                    reason: "System configuration".to_string(),
                    escalation_allowed: false,
                },
                PathRestriction {
                    pattern: "*/migrations/*".to_string(),
                    reason: "Database migrations".to_string(),
                    escalation_allowed: true,
                },
            ],
            allowed_file_operations: vec![FileOperation::Read, FileOperation::WriteNonConfig],
            network_access: NetworkPolicy::InternalOnly,
            quality_gate_permissions: vec![],
            workflow_permissions: WorkflowPermissions {
                can_create_workflows: false,
                can_modify_workflows: false,
                can_execute_workflows: true,
                restricted_workflow_types: vec!["deployment".to_string()],
            },
        };

        let resource_limits = ResourceLimits {
            max_memory_mb: 2048,
            max_cpu_percentage: 50,
            max_disk_space_mb: 4096,
            max_execution_time_minutes: 30,
            max_concurrent_operations: 5,
            max_file_size_mb: 50,
            max_network_requests_per_minute: 10,
        };

        let command_filter = CommandFilter {
            whitelist_mode: false,
            allowed_commands: vec![],
            forbidden_commands: vec![
                CommandPattern::Prefix {
                    prefix: "sudo".to_string(),
                },
                CommandPattern::Regex {
                    pattern: r"rm\s+-rf\s+/".to_string(),
                },
            ],
            parameter_restrictions: HashMap::new(),
            dangerous_patterns: vec![],
        };

        let escalation_policy = EscalationPolicy {
            auto_approve_safe_operations: true,
            require_human_approval: vec![
                OperationType::ConfigChange,
                OperationType::DatabaseOperation,
            ],
            escalation_timeout: Duration::from_secs(1800), // 30 minutes
            fallback_action: FallbackAction::Defer,
            notification_channels: vec!["team-lead".to_string()],
        };

        (
            permissions,
            resource_limits,
            command_filter,
            escalation_policy,
        )
    }

    /// Standard sandbox configuration
    fn standard_config() -> (
        PermissionSet,
        ResourceLimits,
        CommandFilter,
        EscalationPolicy,
    ) {
        let permissions = PermissionSet {
            allowed_commands: vec![],
            forbidden_paths: vec![
                PathRestriction {
                    pattern: "/etc/*".to_string(),
                    reason: "System configuration".to_string(),
                    escalation_allowed: true,
                },
                PathRestriction {
                    pattern: "/root/*".to_string(),
                    reason: "Root directory".to_string(),
                    escalation_allowed: false,
                },
            ],
            allowed_file_operations: vec![
                FileOperation::Read,
                FileOperation::Write,
                FileOperation::Create,
                FileOperation::DeleteNonSystem,
            ],
            network_access: NetworkPolicy::AllowedWithMonitoring,
            quality_gate_permissions: vec![],
            workflow_permissions: WorkflowPermissions {
                can_create_workflows: true,
                can_modify_workflows: true,
                can_execute_workflows: true,
                restricted_workflow_types: vec!["production".to_string()],
            },
        };

        let resource_limits = ResourceLimits {
            max_memory_mb: 4096,
            max_cpu_percentage: 75,
            max_disk_space_mb: 8192,
            max_execution_time_minutes: 60,
            max_concurrent_operations: 10,
            max_file_size_mb: 100,
            max_network_requests_per_minute: 50,
        };

        let command_filter = CommandFilter {
            whitelist_mode: false,
            allowed_commands: vec![],
            forbidden_commands: vec![
                CommandPattern::Regex {
                    pattern: r"rm\s+-rf\s+/\*".to_string(),
                },
                CommandPattern::Prefix {
                    prefix: "sudo".to_string(),
                },
                CommandPattern::Prefix {
                    prefix: "dd".to_string(),
                },
            ],
            parameter_restrictions: HashMap::new(),
            dangerous_patterns: vec![],
        };

        let escalation_policy = EscalationPolicy {
            auto_approve_safe_operations: true,
            require_human_approval: vec![
                OperationType::SystemFileAccess,
                OperationType::PrivilegedOperation,
            ],
            escalation_timeout: Duration::from_secs(1800),
            fallback_action: FallbackAction::Defer,
            notification_channels: vec!["dev-team".to_string()],
        };

        (
            permissions,
            resource_limits,
            command_filter,
            escalation_policy,
        )
    }

    /// Isolated sandbox configuration
    fn isolated_config() -> (
        PermissionSet,
        ResourceLimits,
        CommandFilter,
        EscalationPolicy,
    ) {
        let permissions = PermissionSet {
            allowed_commands: vec![CommandPermission {
                pattern: CommandPattern::Exact {
                    command: "git status".to_string(),
                },
                description: "Check git status".to_string(),
                risk_level: RiskLevel::Low,
            }],
            forbidden_paths: vec![PathRestriction {
                pattern: "/*".to_string(),
                reason: "Root file system access restricted".to_string(),
                escalation_allowed: true,
            }],
            allowed_file_operations: vec![FileOperation::Read],
            network_access: NetworkPolicy::Denied,
            quality_gate_permissions: vec![],
            workflow_permissions: WorkflowPermissions {
                can_create_workflows: false,
                can_modify_workflows: false,
                can_execute_workflows: false,
                restricted_workflow_types: vec!["*".to_string()],
            },
        };

        let resource_limits = ResourceLimits {
            max_memory_mb: 256,
            max_cpu_percentage: 10,
            max_disk_space_mb: 512,
            max_execution_time_minutes: 5,
            max_concurrent_operations: 1,
            max_file_size_mb: 1,
            max_network_requests_per_minute: 0,
        };

        let command_filter = CommandFilter {
            whitelist_mode: true,
            allowed_commands: vec![CommandPattern::Exact {
                command: "git status".to_string(),
            }],
            forbidden_commands: vec![],
            parameter_restrictions: HashMap::new(),
            dangerous_patterns: vec![],
        };

        let escalation_policy = EscalationPolicy {
            auto_approve_safe_operations: false,
            require_human_approval: vec![
                OperationType::FileWrite,
                OperationType::CommandExecution,
                OperationType::NetworkAccess,
            ],
            escalation_timeout: Duration::from_secs(7200), // 2 hours
            fallback_action: FallbackAction::Deny,
            notification_channels: vec!["security-team".to_string()],
        };

        (
            permissions,
            resource_limits,
            command_filter,
            escalation_policy,
        )
    }

    /// Unrestricted sandbox configuration
    fn unrestricted_config() -> (
        PermissionSet,
        ResourceLimits,
        CommandFilter,
        EscalationPolicy,
    ) {
        let permissions = PermissionSet {
            allowed_commands: vec![],
            forbidden_paths: vec![],
            allowed_file_operations: vec![
                FileOperation::Read,
                FileOperation::Write,
                FileOperation::Create,
                FileOperation::Delete,
                FileOperation::Execute,
            ],
            network_access: NetworkPolicy::Unrestricted,
            quality_gate_permissions: vec![],
            workflow_permissions: WorkflowPermissions {
                can_create_workflows: true,
                can_modify_workflows: true,
                can_execute_workflows: true,
                restricted_workflow_types: vec![],
            },
        };

        let resource_limits = ResourceLimits {
            max_memory_mb: 16384,
            max_cpu_percentage: 100,
            max_disk_space_mb: 102400,
            max_execution_time_minutes: 1440,
            max_concurrent_operations: 50,
            max_file_size_mb: 1024,
            max_network_requests_per_minute: 1000,
        };

        let command_filter = CommandFilter {
            whitelist_mode: false,
            allowed_commands: vec![],
            forbidden_commands: vec![],
            parameter_restrictions: HashMap::new(),
            dangerous_patterns: vec![],
        };

        let escalation_policy = EscalationPolicy {
            auto_approve_safe_operations: true,
            require_human_approval: vec![],
            escalation_timeout: Duration::from_secs(60),
            fallback_action: FallbackAction::Allow,
            notification_channels: vec![],
        };

        (
            permissions,
            resource_limits,
            command_filter,
            escalation_policy,
        )
    }

    /// Update the sandbox level and apply appropriate configuration
    pub fn update_level(&mut self, new_level: SandboxLevel) {
        self.sandbox_level = new_level.clone();
        let (permissions, resource_limits, command_filter, escalation_policy) =
            Self::default_config_for_level(&new_level);

        self.permissions = permissions;
        self.resource_limits = resource_limits;
        self.command_filter = command_filter;
        self.escalation_policy = escalation_policy;
        self.last_modified = Utc::now();
    }

    /// Increment violation count
    pub fn record_violation(&mut self) {
        self.violation_count += 1;
        self.last_modified = Utc::now();
    }
}

impl Entity for AgentSandbox {
    fn entity_type() -> &'static str {
        "agent_sandbox"
    }

    fn id(&self) -> &str {
        &self.id
    }

    fn agent(&self) -> &str {
        &self.agent
    }

    fn timestamp(&self) -> DateTime<Utc> {
        self.created_at
    }

    fn validate_entity(&self) -> super::Result<()> {
        if let Err(errors) = <AgentSandbox as validator::Validate>::validate(self) {
            let error_messages: Vec<String> = errors
                .field_errors()
                .values()
                .flat_map(|field_errors| field_errors.iter())
                .map(|error| {
                    error
                        .message
                        .clone()
                        .map(|s| s.to_string())
                        .unwrap_or_default()
                })
                .collect();
            return Err(error_messages.join(", "));
        }

        if self.agent_id.is_empty() {
            return Err("Agent ID cannot be empty".to_string());
        }

        if self.created_by.is_empty() {
            return Err("Created by field cannot be empty".to_string());
        }

        Ok(())
    }

    fn to_generic(&self) -> GenericEntity {
        let mut data = serde_json::Map::new();

        data.insert(
            "agent_id".to_string(),
            serde_json::to_value(&self.agent_id).unwrap(),
        );
        data.insert(
            "sandbox_level".to_string(),
            serde_json::to_value(&self.sandbox_level).unwrap(),
        );
        data.insert(
            "permissions".to_string(),
            serde_json::to_value(&self.permissions).unwrap(),
        );
        data.insert(
            "resource_limits".to_string(),
            serde_json::to_value(&self.resource_limits).unwrap(),
        );
        data.insert(
            "command_filter".to_string(),
            serde_json::to_value(&self.command_filter).unwrap(),
        );
        data.insert(
            "escalation_policy".to_string(),
            serde_json::to_value(&self.escalation_policy).unwrap(),
        );
        data.insert(
            "created_by".to_string(),
            serde_json::to_value(&self.created_by).unwrap(),
        );
        data.insert(
            "created_at".to_string(),
            serde_json::to_value(&self.created_at).unwrap(),
        );
        data.insert(
            "last_modified".to_string(),
            serde_json::to_value(&self.last_modified).unwrap(),
        );
        data.insert(
            "violation_count".to_string(),
            serde_json::to_value(&self.violation_count).unwrap(),
        );
        data.insert(
            "metadata".to_string(),
            serde_json::to_value(&self.metadata).unwrap(),
        );

        GenericEntity {
            id: self.id.clone(),
            entity_type: Self::entity_type().to_string(),
            agent: self.agent.clone(),
            timestamp: self.created_at,
            data: serde_json::Value::Object(data),
        }
    }

    fn from_generic(generic: GenericEntity) -> super::Result<Self>
    where
        Self: Sized,
    {
        serde_json::from_value(generic.data)
            .map_err(|e| format!("Failed to deserialize AgentSandbox: {}", e))
    }

    fn as_any(&self) -> &dyn std::any::Any
    where
        Self: Sized,
    {
        self
    }
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_memory_mb: 2048,
            max_cpu_percentage: 50,
            max_disk_space_mb: 4096,
            max_execution_time_minutes: 30,
            max_concurrent_operations: 5,
            max_file_size_mb: 50,
            max_network_requests_per_minute: 20,
        }
    }
}

impl Default for PermissionSet {
    fn default() -> Self {
        Self {
            allowed_commands: vec![],
            forbidden_paths: vec![],
            allowed_file_operations: vec![FileOperation::Read],
            network_access: NetworkPolicy::InternalOnly,
            quality_gate_permissions: vec![],
            workflow_permissions: WorkflowPermissions::default(),
        }
    }
}

impl Default for WorkflowPermissions {
    fn default() -> Self {
        Self {
            can_create_workflows: false,
            can_modify_workflows: false,
            can_execute_workflows: true,
            restricted_workflow_types: vec![],
        }
    }
}

impl Default for CommandFilter {
    fn default() -> Self {
        Self {
            whitelist_mode: false,
            allowed_commands: vec![],
            forbidden_commands: vec![],
            parameter_restrictions: HashMap::new(),
            dangerous_patterns: vec![],
        }
    }
}

impl Default for EscalationPolicy {
    fn default() -> Self {
        Self {
            auto_approve_safe_operations: true,
            require_human_approval: vec![],
            escalation_timeout: Duration::from_secs(1800),
            fallback_action: FallbackAction::Defer,
            notification_channels: vec![],
        }
    }
}

// Helper module for duration serialization
mod duration_serde {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::time::Duration;

    pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        duration.as_secs().serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let secs = u64::deserialize(deserializer)?;
        Ok(Duration::from_secs(secs))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_sandbox_creation() {
        let sandbox = AgentSandbox::new(
            "test-agent".to_string(),
            SandboxLevel::Standard,
            "admin".to_string(),
            "test-agent".to_string(),
        );

        assert_eq!(sandbox.agent_id, "test-agent");
        assert_eq!(sandbox.sandbox_level, SandboxLevel::Standard);
        assert_eq!(sandbox.created_by, "admin");
        assert_eq!(sandbox.violation_count, 0);
    }

    #[test]
    fn test_sandbox_level_update() {
        let mut sandbox = AgentSandbox::new(
            "test-agent".to_string(),
            SandboxLevel::Training,
            "admin".to_string(),
            "test-agent".to_string(),
        );

        assert_eq!(sandbox.sandbox_level, SandboxLevel::Training);

        sandbox.update_level(SandboxLevel::Standard);
        assert_eq!(sandbox.sandbox_level, SandboxLevel::Standard);
    }

    #[test]
    fn test_violation_recording() {
        let mut sandbox = AgentSandbox::new(
            "test-agent".to_string(),
            SandboxLevel::Standard,
            "admin".to_string(),
            "test-agent".to_string(),
        );

        assert_eq!(sandbox.violation_count, 0);

        sandbox.record_violation();
        assert_eq!(sandbox.violation_count, 1);

        sandbox.record_violation();
        assert_eq!(sandbox.violation_count, 2);
    }

    #[test]
    fn test_entity_validation() {
        let sandbox = AgentSandbox::new(
            "test-agent".to_string(),
            SandboxLevel::Standard,
            "admin".to_string(),
            "test-agent".to_string(),
        );

        assert!(sandbox.validate_entity().is_ok());
    }

    #[test]
    fn test_generic_conversion() {
        let sandbox = AgentSandbox::new(
            "test-agent".to_string(),
            SandboxLevel::Standard,
            "admin".to_string(),
            "test-agent".to_string(),
        );

        let generic = sandbox.to_generic();
        let restored = AgentSandbox::from_generic(generic).unwrap();

        assert_eq!(sandbox.id, restored.id);
        assert_eq!(sandbox.agent_id, restored.agent_id);
        assert_eq!(sandbox.sandbox_level, restored.sandbox_level);
        assert_eq!(sandbox.created_by, restored.created_by);
        assert_eq!(sandbox.violation_count, restored.violation_count);
    }
}

//! Escalation Request entity for sandbox permission escalations

use super::{Entity, GenericEntity, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use validator::Validate;

/// Status of an escalation request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum EscalationStatus {
    /// Request is pending human review
    Pending,
    /// Request has been approved
    Approved,
    /// Request has been denied
    Denied,
    /// Request has expired without action
    Expired,
    /// Request was cancelled by the requestor
    Cancelled,
}

/// Priority level for escalation requests
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum EscalationPriority {
    /// Low priority - can wait for review
    Low,
    /// Normal priority - standard review process
    Normal,
    /// High priority - expedited review needed
    High,
    /// Critical priority - immediate attention required
    Critical,
}

/// Type of operation being escalated
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum EscalationOperationType {
    /// File system operations
    FileSystemAccess,
    /// Network operations
    NetworkAccess,
    /// System command execution
    CommandExecution,
    /// Privilege escalation
    PrivilegeEscalation,
    /// Quality gate override
    QualityGateOverride,
    /// Workflow modification
    WorkflowModification,
    /// Resource limit increase
    ResourceLimitIncrease,
    /// Custom operation type
    Custom(String),
}

/// Context information about the blocked operation
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct OperationContext {
    /// The specific operation that was blocked
    #[validate(length(min = 1))]
    pub operation: String,

    /// Parameters of the operation
    pub parameters: HashMap<String, serde_json::Value>,

    /// Resource being accessed
    pub resource: Option<String>,

    /// Reason the operation was blocked
    #[validate(length(min = 1))]
    pub block_reason: String,

    /// Alternative suggestions
    pub alternatives: Vec<String>,

    /// Risk assessment of allowing the operation
    pub risk_assessment: Option<String>,
}

/// Human reviewer information
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ReviewerInfo {
    /// ID of the human reviewer
    #[validate(length(min = 1))]
    pub reviewer_id: String,

    /// Name of the reviewer
    #[validate(length(min = 1))]
    pub reviewer_name: String,

    /// Email of the reviewer
    #[validate(email)]
    pub reviewer_email: Option<String>,

    /// Department or team of the reviewer
    pub department: Option<String>,
}

/// Decision made by the reviewer
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ReviewDecision {
    /// The decision made (approved/denied)
    pub status: EscalationStatus,

    /// Reason for the decision
    #[validate(length(min = 1))]
    pub reason: String,

    /// Conditions attached to approval (if any)
    pub conditions: Vec<String>,

    /// Duration for which approval is valid (in seconds)
    pub approval_duration: Option<u64>,

    /// Whether this decision should be remembered for similar operations
    pub create_policy: bool,

    /// Additional notes from reviewer
    pub notes: Option<String>,
}

/// Escalation Request entity
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct EscalationRequest {
    /// Unique identifier
    #[serde(rename = "id")]
    pub id: String,

    /// Agent that requested the escalation
    #[serde(rename = "agent_id")]
    #[validate(length(min = 1))]
    pub agent_id: String,

    /// Session ID when escalation was requested
    #[serde(rename = "session_id")]
    pub session_id: Option<String>,

    /// Type of operation being escalated
    #[serde(rename = "operation_type")]
    pub operation_type: EscalationOperationType,

    /// Current status of the escalation
    #[serde(rename = "status")]
    pub status: EscalationStatus,

    /// Priority level
    #[serde(rename = "priority")]
    pub priority: EscalationPriority,

    /// Context about the blocked operation
    #[serde(rename = "operation_context")]
    #[validate]
    pub operation_context: OperationContext,

    /// Justification provided by the agent
    #[serde(rename = "justification")]
    #[validate(length(min = 1))]
    pub justification: String,

    /// Expected impact if request is denied
    #[serde(rename = "impact_if_denied")]
    pub impact_if_denied: Option<String>,

    /// Suggested reviewer (if any)
    #[serde(rename = "suggested_reviewer")]
    pub suggested_reviewer: Option<String>,

    /// Assigned reviewer information
    #[serde(rename = "reviewer")]
    pub reviewer: Option<ReviewerInfo>,

    /// Decision made by reviewer
    #[serde(rename = "decision")]
    pub decision: Option<ReviewDecision>,

    /// When the request was created
    #[serde(rename = "created_at")]
    pub created_at: DateTime<Utc>,

    /// When the request was last updated
    #[serde(rename = "updated_at")]
    pub updated_at: DateTime<Utc>,

    /// When the request will expire if not reviewed
    #[serde(rename = "expires_at")]
    pub expires_at: DateTime<Utc>,

    /// When the request was reviewed (if applicable)
    #[serde(rename = "reviewed_at")]
    pub reviewed_at: Option<DateTime<Utc>>,

    /// Number of times this agent has made similar requests
    #[serde(rename = "similar_request_count")]
    pub similar_request_count: u32,

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

impl EscalationRequest {
    /// Create a new escalation request
    pub fn new(
        agent_id: String,
        operation_type: EscalationOperationType,
        operation_context: OperationContext,
        justification: String,
        priority: EscalationPriority,
        agent: String,
    ) -> Self {
        let now = Utc::now();

        // Calculate expiration based on priority
        let expiration_hours = match priority {
            EscalationPriority::Critical => 1,
            EscalationPriority::High => 4,
            EscalationPriority::Normal => 24,
            EscalationPriority::Low => 72,
        };

        Self {
            id: Uuid::new_v4().to_string(),
            agent_id,
            session_id: None,
            operation_type,
            status: EscalationStatus::Pending,
            priority,
            operation_context,
            justification,
            impact_if_denied: None,
            suggested_reviewer: None,
            reviewer: None,
            decision: None,
            created_at: now,
            updated_at: now,
            expires_at: now + chrono::Duration::hours(expiration_hours),
            reviewed_at: None,
            similar_request_count: 0,
            agent,
            metadata: HashMap::new(),
        }
    }

    /// Update the escalation status
    pub fn update_status(&mut self, status: EscalationStatus) {
        self.status = status;
        self.updated_at = Utc::now();
    }

    /// Assign a reviewer
    pub fn assign_reviewer(&mut self, reviewer: ReviewerInfo) {
        self.reviewer = Some(reviewer);
        self.updated_at = Utc::now();
    }

    /// Record a decision
    pub fn record_decision(&mut self, decision: ReviewDecision) {
        self.status = decision.status.clone();
        self.decision = Some(decision);
        self.reviewed_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }

    /// Check if the request has expired
    pub fn is_expired(&self) -> bool {
        self.status == EscalationStatus::Pending && Utc::now() > self.expires_at
    }

    /// Mark as expired
    pub fn mark_expired(&mut self) {
        if self.is_expired() {
            self.status = EscalationStatus::Expired;
            self.updated_at = Utc::now();
        }
    }

    /// Cancel the request
    pub fn cancel(&mut self, reason: Option<String>) {
        self.status = EscalationStatus::Cancelled;
        self.updated_at = Utc::now();

        if let Some(reason) = reason {
            self.metadata.insert(
                "cancellation_reason".to_string(),
                serde_json::Value::String(reason),
            );
        }
    }

    /// Get time remaining before expiration
    pub fn time_to_expiration(&self) -> Option<chrono::Duration> {
        if self.status == EscalationStatus::Pending {
            let now = Utc::now();
            if now < self.expires_at {
                Some(self.expires_at - now)
            } else {
                Some(chrono::Duration::zero())
            }
        } else {
            None
        }
    }

    /// Check if the request is actionable (pending and not expired)
    pub fn is_actionable(&self) -> bool {
        self.status == EscalationStatus::Pending && !self.is_expired()
    }

    /// Create a summary for notifications
    pub fn create_summary(&self) -> String {
        format!(
            "Escalation Request: {} by agent {} for {:?} operation (Priority: {:?})",
            self.id, self.agent_id, self.operation_type, self.priority
        )
    }
}

impl Entity for EscalationRequest {
    fn entity_type() -> &'static str {
        "escalation_request"
    }

    fn id(&self) -> &str {
        &self.id
    }

    fn agent(&self) -> &str {
        &self.agent
    }

    fn timestamp(&self) -> chrono::DateTime<chrono::Utc> {
        self.updated_at
    }

    fn validate_entity(&self) -> Result<()> {
        validator::Validate::validate(self).map_err(|e| format!("Validation failed: {}", e))
    }

    fn to_generic(&self) -> GenericEntity {
        GenericEntity {
            id: self.id.clone(),
            entity_type: Self::entity_type().to_string(),
            agent: self.agent.clone(),
            timestamp: self.updated_at,
            data: serde_json::to_value(self).expect("Failed to serialize EscalationRequest"),
        }
    }

    fn from_generic(entity: GenericEntity) -> Result<Self> {
        if entity.entity_type != Self::entity_type() {
            return Err(format!(
                "Expected entity type '{}', got '{}'",
                Self::entity_type(),
                entity.entity_type
            ));
        }

        serde_json::from_value(entity.data)
            .map_err(|e| format!("Failed to deserialize EscalationRequest: {}", e))
    }

    fn as_any(&self) -> &dyn std::any::Any
    where
        Self: Sized,
    {
        self
    }
}

impl std::fmt::Display for EscalationRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "EscalationRequest({}, agent: {}, operation: {:?}, status: {:?})",
            self.id, self.agent_id, self.operation_type, self.status
        )
    }
}

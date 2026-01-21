//! Compliance entity implementation
//!
//! Compliance items track the organization's adherence to Standards and requirements.
//! They provide visibility into whether the team is meeting established guidelines.
//!
//! ## Compliance Status
//!
//! - **Compliant**: All requirements are being met
//! - **NonCompliant**: Requirements are not being met
//! - **Pending**: Assessment not yet complete
//! - **Exempt**: Requirement does not apply in this context
//!
//! ## Relationship to Rules and Standards
//!
//! Compliance answers the question: **"Are we meeting our standards?"**
//!
//! ```text
//! Standard (What) → Rule (How to enforce) → Compliance (Are we meeting it?)
//! ```
//!
//! Compliance entities:
//! - Reference Standards they track adherence to via `related_standards`
//! - Record violations found during assessment via `violations`
//! - Store evidence of compliance via `evidence`
//! - Track remediation due dates via `due_date`
//!
//! ## Workflow
//!
//! 1. Create a Compliance item referencing relevant Standards
//! 2. Run Rule evaluations to check compliance
//! 3. Record violations and evidence
//! 4. Update status based on findings
//! 5. Track remediation progress until compliant
//!
//! ## Example
//!
//! ```json
//! {
//!   "title": "Q4 Security Compliance",
//!   "category": "security",
//!   "status": "non_compliant",
//!   "severity": "high",
//!   "due_date": "2024-12-31T00:00:00Z",
//!   "related_standards": ["security-standards-v2"],
//!   "violations": [
//!     {"description": "Missing rate limiting on API endpoints"}
//!   ]
//! }
//! ```

use super::{Entity, GenericEntity};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use validator::Validate;

/// Compliance status variants
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ComplianceStatus {
    Compliant,
    NonCompliant,
    Pending,
    Exempt,
}

/// Severity level for compliance violations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SeverityLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// Compliance requirement entity
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Compliance {
    /// Unique identifier
    #[serde(rename = "id")]
    pub id: String,

    /// Requirement title
    #[serde(rename = "title")]
    pub title: String,

    /// Requirement description
    #[serde(rename = "description")]
    pub description: String,

    /// Compliance category
    #[serde(rename = "category")]
    pub category: String,

    /// Current compliance status
    #[serde(rename = "status")]
    pub status: ComplianceStatus,

    /// Associated agent
    #[serde(rename = "agent")]
    pub agent: String,

    /// Creation timestamp
    #[serde(rename = "created_at")]
    pub created_at: DateTime<Utc>,

    /// Last updated timestamp
    #[serde(rename = "updated_at")]
    pub updated_at: DateTime<Utc>,

    /// Due date for compliance
    #[serde(rename = "due_date")]
    pub due_date: Option<DateTime<Utc>>,

    /// Severity level if non-compliant
    #[serde(rename = "severity")]
    pub severity: Option<SeverityLevel>,

    /// Evidence of compliance
    #[serde(rename = "evidence", skip_serializing_if = "Vec::is_empty", default)]
    pub evidence: Vec<ComplianceEvidence>,

    /// Violations found
    #[serde(rename = "violations", skip_serializing_if = "Vec::is_empty", default)]
    pub violations: Vec<ComplianceViolation>,

    /// Related standards
    #[serde(
        rename = "related_standards",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub related_standards: Vec<String>,

    /// Tags for categorization
    #[serde(rename = "tags", skip_serializing_if = "Vec::is_empty", default)]
    pub tags: Vec<String>,

    /// Additional metadata
    #[serde(
        rename = "metadata",
        skip_serializing_if = "HashMap::is_empty",
        default
    )]
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Evidence supporting compliance
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ComplianceEvidence {
    /// Evidence identifier
    #[serde(rename = "id")]
    pub id: String,

    /// Evidence description
    #[serde(rename = "description")]
    pub description: String,

    /// Evidence type (document, test, review, etc.)
    #[serde(rename = "evidence_type")]
    pub evidence_type: String,

    /// Evidence location or URL
    #[serde(rename = "location")]
    pub location: Option<String>,

    /// Timestamp when evidence was collected
    #[serde(rename = "collected_at")]
    pub collected_at: DateTime<Utc>,

    /// Validator or reviewer
    #[serde(rename = "reviewer")]
    pub reviewer: Option<String>,
}

/// Compliance violation
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ComplianceViolation {
    /// Violation identifier
    #[serde(rename = "id")]
    pub id: String,

    /// Violation description
    #[serde(rename = "description")]
    pub description: String,

    /// Severity level
    #[serde(rename = "severity")]
    pub severity: SeverityLevel,

    /// Location of violation
    #[serde(rename = "location")]
    pub location: Option<String>,

    /// When violation was detected
    #[serde(rename = "detected_at")]
    pub detected_at: DateTime<Utc>,

    /// Remediation steps
    #[serde(
        rename = "remediation_steps",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub remediation_steps: Vec<String>,

    /// Status of remediation
    #[serde(rename = "remediation_status")]
    pub remediation_status: String,
}

impl Compliance {
    /// Create a new compliance requirement
    pub fn new(title: String, description: String, category: String, agent: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            description,
            category,
            status: ComplianceStatus::Pending,
            agent,
            created_at: now,
            updated_at: now,
            due_date: None,
            severity: None,
            evidence: Vec::new(),
            violations: Vec::new(),
            related_standards: Vec::new(),
            tags: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Mark as compliant
    pub fn mark_compliant(&mut self) {
        self.status = ComplianceStatus::Compliant;
        self.violations.clear();
        self.updated_at = Utc::now();
    }

    /// Mark as non-compliant with violations
    pub fn mark_non_compliant(&mut self, violations: Vec<ComplianceViolation>) {
        self.status = ComplianceStatus::NonCompliant;
        if !violations.is_empty() {
            self.severity = violations
                .iter()
                .map(|v| match v.severity {
                    SeverityLevel::Critical => 4,
                    SeverityLevel::High => 3,
                    SeverityLevel::Medium => 2,
                    SeverityLevel::Low => 1,
                })
                .max()
                .and_then(|max| match max {
                    4 => Some(SeverityLevel::Critical),
                    3 => Some(SeverityLevel::High),
                    2 => Some(SeverityLevel::Medium),
                    1 => Some(SeverityLevel::Low),
                    _ => None,
                });
        }
        self.violations = violations;
        self.updated_at = Utc::now();
    }

    /// Add evidence
    pub fn add_evidence(&mut self, evidence: ComplianceEvidence) {
        self.evidence.push(evidence);
        self.updated_at = Utc::now();
    }

    /// Add a related standard
    pub fn add_related_standard(&mut self, standard_id: String) {
        if !self.related_standards.contains(&standard_id) {
            self.related_standards.push(standard_id);
        }
    }

    /// Set due date
    pub fn set_due_date(&mut self, due_date: DateTime<Utc>) {
        self.due_date = Some(due_date);
        self.updated_at = Utc::now();
    }
}

impl Entity for Compliance {
    fn entity_type() -> &'static str {
        "compliance"
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

    fn validate_entity(&self) -> crate::Result<()> {
        if let Err(errors) = <Compliance as validator::Validate>::validate(self) {
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
            return Err(crate::EngramError::Validation(error_messages.join(", ")));
        }

        if self.title.is_empty() {
            return Err(crate::EngramError::Validation(
                "Compliance title cannot be empty".to_string(),
            ));
        }

        if self.description.is_empty() {
            return Err(crate::EngramError::Validation(
                "Compliance description cannot be empty".to_string(),
            ));
        }

        if self.category.is_empty() {
            return Err(crate::EngramError::Validation(
                "Compliance category cannot be empty".to_string(),
            ));
        }

        Ok(())
    }

    fn to_generic(&self) -> GenericEntity {
        GenericEntity {
            id: self.id.clone(),
            entity_type: Self::entity_type().to_string(),
            agent: self.agent.clone(),
            timestamp: self.created_at,
            data: serde_json::to_value(self).unwrap_or_default(),
        }
    }

    fn from_generic(entity: GenericEntity) -> crate::Result<Self> {
        serde_json::from_value(entity.data).map_err(|e| {
            crate::EngramError::Deserialization(format!("Failed to deserialize Compliance: {}", e))
        })
    }

    fn as_any(&self) -> &dyn std::any::Any
    where
        Self: Sized,
    {
        self
    }
}

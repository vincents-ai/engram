//! Standard entity implementation

use super::{Entity, GenericEntity, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use validator::Validate;

/// Standard status variants
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum StandardStatus {
    Draft,
    Active,
    Deprecated,
    Superseded,
}

/// Standard category
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum StandardCategory {
    Coding,
    Testing,
    Documentation,
    Security,
    Performance,
    Process,
    Architecture,
}

/// Standard entity for team standards and guidelines
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Standard {
    /// Unique identifier
    #[serde(rename = "id")]
    pub id: String,

    /// Standard title
    #[serde(rename = "title")]
    pub title: String,

    /// Standard description
    #[serde(rename = "description")]
    pub description: String,

    /// Standard category
    #[serde(rename = "category")]
    pub category: StandardCategory,

    /// Current status
    #[serde(rename = "status")]
    pub status: StandardStatus,

    /// Standard version
    #[serde(rename = "version")]
    pub version: String,

    /// Associated agent
    #[serde(rename = "agent")]
    pub agent: String,

    /// Creation timestamp
    #[serde(rename = "created_at")]
    pub created_at: DateTime<Utc>,

    /// Last updated timestamp
    #[serde(rename = "updated_at")]
    pub updated_at: DateTime<Utc>,

    /// Effective date
    #[serde(rename = "effective_date")]
    pub effective_date: DateTime<Utc>,

    /// Superseded by (if applicable)
    #[serde(rename = "superseded_by", skip_serializing_if = "Option::is_none")]
    pub superseded_by: Option<String>,

    /// Supersedes (if applicable)
    #[serde(rename = "supersedes", skip_serializing_if = "Vec::is_empty", default)]
    pub supersedes: Vec<String>,

    /// Related standards
    #[serde(
        rename = "related_standards",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub related_standards: Vec<String>,

    /// Standard requirements/guidelines
    #[serde(rename = "requirements")]
    pub requirements: Vec<StandardRequirement>,

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

/// Standard requirement or guideline
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct StandardRequirement {
    /// Requirement identifier
    #[serde(rename = "id")]
    pub id: String,

    /// Requirement title
    #[serde(rename = "title")]
    pub title: String,

    /// Requirement description
    #[serde(rename = "description")]
    pub description: String,

    /// Whether requirement is mandatory
    #[serde(rename = "mandatory")]
    pub mandatory: bool,

    /// Priority level
    #[serde(rename = "priority")]
    pub priority: super::rule::RulePriority,

    /// Validation criteria
    #[serde(rename = "validation_criteria")]
    pub validation_criteria: Vec<String>,

    /// Evidence required
    #[serde(rename = "evidence_required")]
    pub evidence_required: bool,
}

impl Standard {
    /// Create a new standard
    pub fn new(
        title: String,
        description: String,
        category: StandardCategory,
        version: String,
        agent: String,
        effective_date: DateTime<Utc>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            description,
            category,
            status: StandardStatus::Draft,
            version,
            agent,
            created_at: now,
            updated_at: now,
            effective_date,
            superseded_by: None,
            supersedes: Vec::new(),
            related_standards: Vec::new(),
            requirements: Vec::new(),
            tags: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Activate standard
    pub fn activate(&mut self) {
        self.status = StandardStatus::Active;
        self.updated_at = Utc::now();
    }

    /// Deprecate standard
    pub fn deprecate(&mut self, superseded_by: Option<String>) {
        self.status = StandardStatus::Deprecated;
        self.superseded_by = superseded_by;
        self.updated_at = Utc::now();
    }

    /// Add a requirement
    pub fn add_requirement(&mut self, requirement: StandardRequirement) {
        self.requirements.push(requirement);
        self.updated_at = Utc::now();
    }

    /// Add a related standard
    pub fn add_related_standard(&mut self, standard_id: String) {
        if !self.related_standards.contains(&standard_id) {
            self.related_standards.push(standard_id);
        }
    }

    /// Add superseded standard
    pub fn add_superseded(&mut self, standard_id: String) {
        if !self.supersedes.contains(&standard_id) {
            self.supersedes.push(standard_id);
        }
    }

    /// Check if standard is currently effective
    pub fn is_effective(&self) -> bool {
        self.status == StandardStatus::Active
            && self.effective_date <= Utc::now()
            && self.superseded_by.is_none()
    }
}

impl Entity for Standard {
    fn entity_type() -> &'static str {
        "standard"
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
        if let Err(errors) = <Standard as validator::Validate>::validate(self) {
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

        if self.title.is_empty() {
            return Err("Standard title cannot be empty".to_string());
        }

        if self.description.is_empty() {
            return Err("Standard description cannot be empty".to_string());
        }

        if self.version.is_empty() {
            return Err("Standard version cannot be empty".to_string());
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

    fn from_generic(entity: GenericEntity) -> Result<Self> {
        serde_json::from_value(entity.data)
            .map_err(|e| format!("Failed to deserialize Standard: {}", e))
    }

    fn as_any(&self) -> &dyn std::any::Any
    where
        Self: Sized,
    {
        self
    }
}

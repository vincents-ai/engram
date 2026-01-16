//! Architecture Decision Record (ADR) entity implementation

use super::{Entity, GenericEntity, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use validator::Validate;

/// ADR status variants
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AdrStatus {
    Proposed,
    Accepted,
    Deprecated,
    Superseded,
}

/// Decision outcome
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DecisionOutcome {
    Accepted,
    Rejected,
    Deferred,
}

/// Architecture Decision Record entity
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ADR {
    /// Unique identifier
    #[serde(rename = "id")]
    pub id: String,

    /// ADR title
    #[serde(rename = "title")]
    pub title: String,

    /// Sequential number
    #[serde(rename = "number")]
    pub number: u32,

    /// Current status
    #[serde(rename = "status")]
    pub status: AdrStatus,

    /// Associated agent
    #[serde(rename = "agent")]
    pub agent: String,

    /// Creation timestamp
    #[serde(rename = "created_at")]
    pub created_at: DateTime<Utc>,

    /// Last updated timestamp
    #[serde(rename = "updated_at")]
    pub updated_at: DateTime<Utc>,

    /// Decision date
    #[serde(rename = "decision_date")]
    pub decision_date: Option<DateTime<Utc>>,

    /// Context and problem statement
    #[serde(rename = "context")]
    pub context: String,

    /// Decision description
    #[serde(rename = "decision")]
    pub decision: String,

    /// Consequences of the decision
    #[serde(rename = "consequences")]
    pub consequences: String,

    /// Alternative options considered
    #[serde(
        rename = "alternatives",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub alternatives: Vec<Alternative>,

    /// Implementation notes
    #[serde(rename = "implementation", skip_serializing_if = "Option::is_none")]
    pub implementation: Option<String>,

    /// Related ADRs
    #[serde(
        rename = "related_adrs",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub related_adrs: Vec<String>,

    /// Superseded by (if applicable)
    #[serde(rename = "superseded_by", skip_serializing_if = "Option::is_none")]
    pub superseded_by: Option<String>,

    /// Supersedes (if applicable)
    #[serde(rename = "supersedes", skip_serializing_if = "Vec::is_empty", default)]
    pub supersedes: Vec<String>,

    /// Stakeholders involved
    #[serde(
        rename = "stakeholders",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub stakeholders: Vec<String>,

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

/// Alternative option considered
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Alternative {
    /// Alternative identifier
    #[serde(rename = "id")]
    pub id: String,

    /// Alternative description
    #[serde(rename = "description")]
    pub description: String,

    /// Pros of this alternative
    #[serde(rename = "pros", skip_serializing_if = "Vec::is_empty", default)]
    pub pros: Vec<String>,

    /// Cons of this alternative
    #[serde(rename = "cons", skip_serializing_if = "Vec::is_empty", default)]
    pub cons: Vec<String>,

    /// Why this alternative was not chosen
    #[serde(rename = "rejection_reason")]
    pub rejection_reason: Option<String>,
}

impl ADR {
    /// Create a new ADR
    pub fn new(title: String, number: u32, agent: String, context: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            number,
            status: AdrStatus::Proposed,
            agent,
            created_at: now,
            updated_at: now,
            decision_date: None,
            context,
            decision: String::new(),
            consequences: String::new(),
            alternatives: Vec::new(),
            implementation: None,
            related_adrs: Vec::new(),
            superseded_by: None,
            supersedes: Vec::new(),
            stakeholders: Vec::new(),
            tags: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Accept the decision
    pub fn accept(&mut self, decision: String, consequences: String) {
        self.status = AdrStatus::Accepted;
        self.decision = decision;
        self.consequences = consequences;
        self.decision_date = Some(Utc::now());
        self.updated_at = Utc::now();
    }

    /// Reject the decision
    pub fn reject(&mut self) {
        self.status = AdrStatus::Deprecated;
        self.decision_date = Some(Utc::now());
        self.updated_at = Utc::now();
    }

    /// Deprecate ADR
    pub fn deprecate(&mut self, superseded_by: Option<String>) {
        self.status = AdrStatus::Deprecated;
        self.superseded_by = superseded_by;
        self.updated_at = Utc::now();
    }

    /// Add an alternative
    pub fn add_alternative(&mut self, description: String) -> String {
        let id = Uuid::new_v4().to_string();
        let alternative = Alternative {
            id: id.clone(),
            description,
            pros: Vec::new(),
            cons: Vec::new(),
            rejection_reason: None,
        };
        self.alternatives.push(alternative);
        self.updated_at = Utc::now();
        id
    }

    /// Add pro to alternative
    pub fn add_pro_to_alternative(&mut self, alternative_id: &str, pro: String) {
        if let Some(alternative) = self
            .alternatives
            .iter_mut()
            .find(|alt| alt.id == alternative_id)
        {
            alternative.pros.push(pro);
            self.updated_at = Utc::now();
        }
    }

    /// Add con to alternative
    pub fn add_con_to_alternative(&mut self, alternative_id: &str, con: String) {
        if let Some(alternative) = self
            .alternatives
            .iter_mut()
            .find(|alt| alt.id == alternative_id)
        {
            alternative.cons.push(con);
            self.updated_at = Utc::now();
        }
    }

    /// Set implementation notes
    pub fn set_implementation(&mut self, implementation: String) {
        self.implementation = Some(implementation);
        self.updated_at = Utc::now();
    }

    /// Add related ADR
    pub fn add_related_adr(&mut self, adr_id: String) {
        if !self.related_adrs.contains(&adr_id) {
            self.related_adrs.push(adr_id);
            self.updated_at = Utc::now();
        }
    }

    /// Add stakeholder
    pub fn add_stakeholder(&mut self, stakeholder: String) {
        if !self.stakeholders.contains(&stakeholder) {
            self.stakeholders.push(stakeholder);
            self.updated_at = Utc::now();
        }
    }
}

impl Entity for ADR {
    fn entity_type() -> &'static str {
        "adr"
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
        if let Err(errors) = <ADR as validator::Validate>::validate(self) {
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
            return Err("ADR title cannot be empty".to_string());
        }

        if self.context.is_empty() {
            return Err("ADR context cannot be empty".to_string());
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
        serde_json::from_value(entity.data).map_err(|e| format!("Failed to deserialize ADR: {}", e))
    }

    fn as_any(&self) -> &dyn std::any::Any
    where
        Self: Sized,
    {
        self
    }
}

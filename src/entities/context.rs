//! Context entity implementation

use super::{Entity, GenericEntity};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use validator::Validate;

/// Relevance level for context
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ContextRelevance {
    Low,
    Medium,
    High,
    Critical,
}

/// Context entity representing background information
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Context {
    /// Unique identifier
    #[serde(rename = "id")]
    pub id: String,

    /// Context title
    #[serde(rename = "title")]
    pub title: String,

    /// Context content
    #[serde(rename = "content")]
    pub content: String,

    /// Source of this context
    #[serde(rename = "source")]
    pub source: String,

    /// Source identifier (e.g., URL, file path)
    #[serde(rename = "source_id")]
    pub source_id: Option<String>,

    /// Relevance level
    #[serde(rename = "relevance")]
    pub relevance: ContextRelevance,

    /// Associated agent
    #[serde(rename = "agent")]
    pub agent: String,

    /// Creation timestamp
    #[serde(rename = "created_at")]
    pub created_at: DateTime<Utc>,

    /// Last updated timestamp
    #[serde(rename = "updated_at")]
    pub updated_at: DateTime<Utc>,

    /// Tags for categorization
    #[serde(rename = "tags", skip_serializing_if = "Vec::is_empty", default)]
    pub tags: Vec<String>,

    /// Related entity IDs
    #[serde(
        rename = "related_entities",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub related_entities: Vec<String>,

    /// Additional metadata
    #[serde(
        rename = "metadata",
        skip_serializing_if = "HashMap::is_empty",
        default
    )]
    pub metadata: HashMap<String, serde_json::Value>,
}

impl Context {
    /// Create a new context
    pub fn new(
        title: String,
        content: String,
        source: String,
        relevance: ContextRelevance,
        agent: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            content,
            source,
            source_id: None,
            relevance,
            agent,
            created_at: now,
            updated_at: now,
            tags: Vec::new(),
            related_entities: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Update context content
    pub fn update_content(&mut self, content: String) {
        self.content = content;
        self.updated_at = Utc::now();
    }

    /// Add a related entity
    pub fn add_related_entity(&mut self, entity_id: String) {
        if !self.related_entities.contains(&entity_id) {
            self.related_entities.push(entity_id);
        }
    }

    /// Set source ID
    pub fn set_source_id(&mut self, source_id: String) {
        self.source_id = Some(source_id);
    }
}

impl Entity for Context {
    fn entity_type() -> &'static str {
        "context"
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
        // Use validator crate's validate method via explicit trait qualification
        if let Err(errors) = <Context as validator::Validate>::validate(self) {
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
                "Context title cannot be empty".to_string(),
            ));
        }

        if self.content.is_empty() {
            return Err(crate::EngramError::Validation(
                "Context content cannot be empty".to_string(),
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
            crate::EngramError::Deserialization(format!("Failed to deserialize Context: {}", e))
        })
    }

    fn as_any(&self) -> &dyn std::any::Any
    where
        Self: Sized,
    {
        self
    }
}

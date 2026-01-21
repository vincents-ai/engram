//! Knowledge entity implementation

use super::{Entity, GenericEntity};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use validator::Validate;

/// Knowledge type variants
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum KnowledgeType {
    Fact,
    Pattern,
    Rule,
    Concept,
    Procedure,
    Heuristic,
}

/// Knowledge entity representing stored information
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Knowledge {
    /// Unique identifier
    #[serde(rename = "id")]
    pub id: String,

    /// Knowledge title
    #[serde(rename = "title")]
    pub title: String,

    /// Knowledge content
    #[serde(rename = "content")]
    pub content: String,

    /// Knowledge type
    #[serde(rename = "knowledge_type")]
    pub knowledge_type: KnowledgeType,

    /// Confidence level (0.0 to 1.0)
    #[serde(rename = "confidence")]
    pub confidence: f64,

    /// Associated agent
    #[serde(rename = "agent")]
    pub agent: String,

    /// Creation timestamp
    #[serde(rename = "created_at")]
    pub created_at: DateTime<Utc>,

    /// Last updated timestamp
    #[serde(rename = "updated_at")]
    pub updated_at: DateTime<Utc>,

    /// Source of this knowledge
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,

    /// Related knowledge IDs
    #[serde(
        rename = "related_knowledge",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub related_knowledge: Vec<String>,

    /// Tags for categorization
    #[serde(rename = "tags", skip_serializing_if = "Vec::is_empty", default)]
    pub tags: Vec<String>,

    /// Contexts where this applies
    #[serde(rename = "contexts", skip_serializing_if = "Vec::is_empty", default)]
    pub contexts: Vec<String>,

    /// Usage count (for tracking relevance)
    #[serde(rename = "usage_count", default)]
    pub usage_count: u64,

    /// Last used timestamp
    #[serde(rename = "last_used", skip_serializing_if = "Option::is_none")]
    pub last_used: Option<DateTime<Utc>>,

    /// Additional metadata
    #[serde(
        rename = "metadata",
        skip_serializing_if = "HashMap::is_empty",
        default
    )]
    pub metadata: HashMap<String, serde_json::Value>,
}

impl Knowledge {
    /// Create a new knowledge item
    pub fn new(
        title: String,
        content: String,
        knowledge_type: KnowledgeType,
        confidence: f64,
        agent: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            content,
            knowledge_type,
            confidence: confidence.clamp(0.0, 1.0),
            agent,
            created_at: now,
            updated_at: now,
            source: None,
            related_knowledge: Vec::new(),
            tags: Vec::new(),
            contexts: Vec::new(),
            usage_count: 0,
            last_used: None,
            metadata: HashMap::new(),
        }
    }

    /// Update knowledge content
    pub fn update_content(&mut self, content: String, confidence: f64) {
        self.content = content;
        self.confidence = confidence.clamp(0.0, 1.0);
        self.updated_at = Utc::now();
    }

    /// Add related knowledge
    pub fn add_related_knowledge(&mut self, knowledge_id: String) {
        if !self.related_knowledge.contains(&knowledge_id) {
            self.related_knowledge.push(knowledge_id);
        }
    }

    /// Record usage
    pub fn record_usage(&mut self) {
        self.usage_count += 1;
        self.last_used = Some(Utc::now());
    }

    /// Add a tag
    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
    }

    /// Add a context
    pub fn add_context(&mut self, context: String) {
        if !self.contexts.contains(&context) {
            self.contexts.push(context);
        }
    }

    /// Set source
    pub fn set_source(&mut self, source: String) {
        self.source = Some(source);
    }
}

impl Entity for Knowledge {
    fn entity_type() -> &'static str {
        "knowledge"
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
        if let Err(errors) = <Knowledge as validator::Validate>::validate(self) {
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
                "Knowledge title cannot be empty".to_string(),
            ));
        }

        if self.content.is_empty() {
            return Err(crate::EngramError::Validation(
                "Knowledge content cannot be empty".to_string(),
            ));
        }

        if self.confidence < 0.0 || self.confidence > 1.0 {
            return Err(crate::EngramError::Validation(
                "Confidence must be between 0.0 and 1.0".to_string(),
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
            crate::EngramError::Deserialization(format!("Failed to deserialize Knowledge: {}", e))
        })
    }

    fn as_any(&self) -> &dyn std::any::Any
    where
        Self: Sized,
    {
        self
    }
}

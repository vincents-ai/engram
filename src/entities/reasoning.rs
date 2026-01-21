//! Reasoning chain entity implementation

use super::{Entity, EntityResult, GenericEntity};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use validator::Validate;

/// Step in a reasoning chain
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ReasoningStep {
    /// Step identifier
    #[serde(rename = "id")]
    pub id: String,

    /// Step description
    #[serde(rename = "description")]
    pub description: String,

    /// Step conclusion
    #[serde(rename = "conclusion")]
    pub conclusion: String,

    /// Supporting evidence
    #[serde(rename = "evidence", skip_serializing_if = "Vec::is_empty", default)]
    pub evidence: Vec<String>,

    /// Confidence level (0.0 to 1.0)
    #[serde(rename = "confidence")]
    pub confidence: f64,

    /// Step timestamp
    #[serde(rename = "timestamp")]
    pub timestamp: DateTime<Utc>,
}

/// Reasoning chain entity
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Reasoning {
    /// Unique identifier
    #[serde(rename = "id")]
    pub id: String,

    /// Reasoning title
    #[serde(rename = "title")]
    pub title: String,

    /// Task ID this reasoning belongs to
    #[serde(rename = "task_id")]
    pub task_id: String,

    /// Reasoning steps in order
    #[serde(rename = "steps")]
    pub steps: Vec<ReasoningStep>,

    /// Final conclusion
    #[serde(rename = "conclusion")]
    pub conclusion: String,

    /// Overall confidence
    #[serde(rename = "confidence")]
    pub confidence: f64,

    /// Associated agent
    #[serde(rename = "agent")]
    pub agent: String,

    /// Creation timestamp
    #[serde(rename = "created_at")]
    pub created_at: DateTime<Utc>,

    /// Tags for categorization
    #[serde(rename = "tags", skip_serializing_if = "Vec::is_empty", default)]
    pub tags: Vec<String>,

    /// Supporting context IDs
    #[serde(rename = "context_ids", skip_serializing_if = "Vec::is_empty", default)]
    pub context_ids: Vec<String>,

    /// Supporting knowledge IDs
    #[serde(
        rename = "knowledge_ids",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub knowledge_ids: Vec<String>,

    /// Additional metadata
    #[serde(
        rename = "metadata",
        skip_serializing_if = "HashMap::is_empty",
        default
    )]
    pub metadata: HashMap<String, serde_json::Value>,
}

impl Reasoning {
    /// Create a new reasoning chain
    pub fn new(title: String, task_id: String, agent: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            task_id,
            steps: Vec::new(),
            conclusion: String::new(),
            confidence: 0.0,
            agent,
            created_at: now,
            tags: Vec::new(),
            context_ids: Vec::new(),
            knowledge_ids: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Add a reasoning step
    pub fn add_step(&mut self, description: String, conclusion: String, confidence: f64) {
        let step = ReasoningStep {
            id: Uuid::new_v4().to_string(),
            description,
            conclusion,
            evidence: Vec::new(),
            confidence: confidence.clamp(0.0, 1.0),
            timestamp: Utc::now(),
        };
        self.steps.push(step);
        self.recalculate_confidence();
    }

    /// Add evidence to the last step
    pub fn add_evidence_to_last_step(&mut self, evidence: String) {
        if let Some(last_step) = self.steps.last_mut() {
            last_step.evidence.push(evidence);
        }
    }

    /// Set final conclusion
    pub fn set_conclusion(&mut self, conclusion: String, confidence: f64) {
        self.conclusion = conclusion;
        self.confidence = confidence.clamp(0.0, 1.0);
    }

    /// Recalculate overall confidence based on steps
    fn recalculate_confidence(&mut self) {
        if self.steps.is_empty() {
            self.confidence = 0.0;
            return;
        }

        let total_confidence: f64 = self.steps.iter().map(|s| s.confidence).sum();
        self.confidence = (total_confidence / self.steps.len() as f64).clamp(0.0, 1.0);
    }
}

impl Entity for Reasoning {
    fn entity_type() -> &'static str {
        "reasoning"
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

    fn validate_entity(&self) -> super::EntityResult<()> {
        if let Err(errors) = <Reasoning as validator::Validate>::validate(self) {
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
            return Err("Reasoning title cannot be empty".to_string());
        }

        if self.task_id.is_empty() {
            return Err("Task ID cannot be empty".to_string());
        }

        if self.confidence < 0.0 || self.confidence > 1.0 {
            return Err("Confidence must be between 0.0 and 1.0".to_string());
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

    fn from_generic(entity: GenericEntity) -> EntityResult<Self> {
        serde_json::from_value(entity.data)
            .map_err(|e| format!("Failed to deserialize Reasoning: {}", e))
    }

    fn as_any(&self) -> &dyn std::any::Any
    where
        Self: Sized,
    {
        self
    }
}

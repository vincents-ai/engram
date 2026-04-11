//! Reasoning chain entity implementation

use super::{Entity, GenericEntity};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, schemars::JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum IBISNodeType {
    Question,
    Idea,
    Pro,
    Con,
    Reference,
    Note,
}

impl clap::ValueEnum for IBISNodeType {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Question, Self::Idea, Self::Pro, Self::Con, Self::Reference, Self::Note]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            Self::Question => clap::builder::PossibleValue::new("question"),
            Self::Idea => clap::builder::PossibleValue::new("idea"),
            Self::Pro => clap::builder::PossibleValue::new("pro"),
            Self::Con => clap::builder::PossibleValue::new("con"),
            Self::Reference => clap::builder::PossibleValue::new("reference"),
            Self::Note => clap::builder::PossibleValue::new("note"),
        })
    }
}

/// IBIS polarity (Pro/Con position)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, schemars::JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum IbisPolarity {
    Pro,
    Con,
}

impl clap::ValueEnum for IbisPolarity {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Pro, Self::Con]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            Self::Pro => clap::builder::PossibleValue::new("pro"),
            Self::Con => clap::builder::PossibleValue::new("con"),
        })
    }
}

/// Step in a reasoning chain
#[derive(Debug, Clone, Serialize, Deserialize, Validate, schemars::JsonSchema)]
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

    /// IBIS node type (Question, Idea, Pro, Con, Reference, Note)
    #[serde(rename = "ibis_type", skip_serializing_if = "Option::is_none", default)]
    pub ibis_type: Option<IBISNodeType>,

    /// IBIS polarity (Pro/Con position)
    #[serde(rename = "ibis_polarity", skip_serializing_if = "Option::is_none", default)]
    pub ibis_polarity: Option<IbisPolarity>,

    /// Parent step ID for IBIS hierarchy
    #[serde(rename = "parent_step_id", skip_serializing_if = "Option::is_none", default)]
    pub parent_step_id: Option<String>,
}

/// Reasoning chain entity
#[derive(Debug, Clone, Serialize, Deserialize, Validate, schemars::JsonSchema)]
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

    #[serde(rename = "prov_used", skip_serializing_if = "Vec::is_empty", default)]
    pub prov_used: Vec<String>,

    #[serde(rename = "prov_generated", skip_serializing_if = "Vec::is_empty", default)]
    pub prov_generated: Vec<String>,

    #[serde(rename = "prov_attributed_to", default)]
    pub prov_attributed_to: String,

    #[serde(rename = "ibis_node_type", skip_serializing_if = "Option::is_none", default)]
    pub ibis_node_type: Option<IBISNodeType>,

    #[serde(rename = "ibis_parent_id", skip_serializing_if = "Option::is_none", default)]
    pub ibis_parent_id: Option<String>,

    #[serde(rename = "ibis_position", skip_serializing_if = "Option::is_none", default)]
    pub ibis_position: Option<f64>,
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
            prov_used: Vec::new(),
            prov_generated: Vec::new(),
            prov_attributed_to: String::new(),
            ibis_node_type: None,
            ibis_parent_id: None,
            ibis_position: None,
        }
    }

    /// Add a reasoning step with IBIS metadata
    pub fn add_step(
        &mut self,
        description: String,
        conclusion: String,
        confidence: f64,
        ibis_type: Option<IBISNodeType>,
        ibis_polarity: Option<IbisPolarity>,
        parent_step_id: Option<String>,
    ) {
        let step = ReasoningStep {
            id: Uuid::new_v4().to_string(),
            description,
            conclusion,
            evidence: Vec::new(),
            confidence: confidence.clamp(0.0, 1.0),
            timestamp: Utc::now(),
            ibis_type,
            ibis_polarity,
            parent_step_id,
        };
        self.steps.push(step);
        self.recalculate_confidence();
    }

    /// Add a reasoning step (backwards compatible)
    pub fn add_step_simple(&mut self, description: String, conclusion: String, confidence: f64) {
        self.add_step(description, conclusion, confidence, None, None, None);
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

    fn validate_entity(&self) -> crate::Result<()> {
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
            return Err(crate::EngramError::Validation(error_messages.join(", ")));
        }

        if self.title.is_empty() {
            return Err(crate::EngramError::Validation(
                "Reasoning title cannot be empty".to_string(),
            ));
        }

        if self.task_id.is_empty() {
            return Err(crate::EngramError::Validation(
                "Task ID cannot be empty".to_string(),
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
            crate::EngramError::Deserialization(format!("Failed to deserialize Reasoning: {}", e))
        })
    }

    fn as_any(&self) -> &dyn std::any::Any
    where
        Self: Sized,
    {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reasoning_lifecycle() {
        let mut reasoning = Reasoning::new(
            "Decision".to_string(),
            "task-1".to_string(),
            "agent".to_string(),
        );

        assert_eq!(reasoning.steps.len(), 0);
        assert_eq!(reasoning.confidence, 0.0);

        // Add step
        reasoning.add_step_simple("Step 1".to_string(), "Conclusion 1".to_string(), 0.8);
        assert_eq!(reasoning.steps.len(), 1);
        assert_eq!(reasoning.confidence, 0.8);

        // Add evidence
        reasoning.add_evidence_to_last_step("Evidence A".to_string());
        assert_eq!(reasoning.steps[0].evidence.len(), 1);

        // Add second step
        reasoning.add_step_simple("Step 2".to_string(), "Conclusion 2".to_string(), 0.6);
        // Average confidence: (0.8 + 0.6) / 2 = 0.7
        assert_eq!(reasoning.confidence, 0.7);

        // Final conclusion override
        reasoning.set_conclusion("Final".to_string(), 1.0);
        assert_eq!(reasoning.conclusion, "Final");
        assert_eq!(reasoning.confidence, 1.0);
    }

    #[test]
    fn test_reasoning_validation() {
        let mut reasoning = Reasoning::new(
            "".to_string(), // Invalid empty title
            "task-1".to_string(),
            "agent".to_string(),
        );

        assert!(reasoning.validate_entity().is_err());

        reasoning.title = "Valid".to_string();
        reasoning.task_id = "".to_string(); // Invalid empty task_id
        assert!(reasoning.validate_entity().is_err());

        reasoning.task_id = "task-1".to_string();
        assert!(reasoning.validate_entity().is_ok());
    }
}

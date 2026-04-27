//! Reasoning chain entity implementation

use super::{Entity, GenericEntity};
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use validator::Validate;

/// Step in a reasoning chain
#[derive(Debug, Clone, Serialize, Deserialize, Validate, JsonSchema)]
pub struct ReasoningStep {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "conclusion")]
    pub conclusion: String,

    #[serde(rename = "evidence", skip_serializing_if = "Vec::is_empty", default)]
    pub evidence: Vec<String>,

    #[serde(rename = "confidence")]
    pub confidence: f64,

    #[serde(rename = "timestamp")]
    pub timestamp: DateTime<Utc>,
}

/// IBIS position type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
pub enum IbisPositionType {
    Issue,
    Position,
    Argument,
}

impl std::fmt::Display for IbisPositionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IbisPositionType::Issue => write!(f, "Issue"),
            IbisPositionType::Position => write!(f, "Position"),
            IbisPositionType::Argument => write!(f, "Argument"),
        }
    }
}

/// A single IBIS position
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct IbisPosition {
    #[serde(rename = "position_type")]
    pub position_type: IbisPositionType,

    #[serde(rename = "content")]
    pub content: String,

    #[serde(rename = "responds_to", skip_serializing_if = "Option::is_none")]
    pub responds_to: Option<String>,
}

/// Reasoning chain entity
#[derive(Debug, Clone, Serialize, Deserialize, Validate, JsonSchema)]
pub struct Reasoning {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "title")]
    pub title: String,

    #[serde(rename = "task_id")]
    pub task_id: String,

    #[serde(rename = "steps")]
    pub steps: Vec<ReasoningStep>,

    #[serde(rename = "conclusion")]
    pub conclusion: String,

    #[serde(rename = "confidence")]
    pub confidence: f64,

    #[serde(rename = "agent")]
    pub agent: String,

    #[serde(rename = "created_at")]
    pub created_at: DateTime<Utc>,

    #[serde(rename = "tags", skip_serializing_if = "Vec::is_empty", default)]
    pub tags: Vec<String>,

    #[serde(rename = "context_ids", skip_serializing_if = "Vec::is_empty", default)]
    pub context_ids: Vec<String>,

    #[serde(
        rename = "knowledge_ids",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub knowledge_ids: Vec<String>,

    #[serde(
        rename = "metadata",
        skip_serializing_if = "HashMap::is_empty",
        default
    )]
    pub metadata: HashMap<String, serde_json::Value>,

    #[serde(rename = "ibis_mode", skip_serializing_if = "Option::is_none", default)]
    pub ibis_mode: Option<bool>,

    #[serde(rename = "positions", skip_serializing_if = "Vec::is_empty", default)]
    pub positions: Vec<IbisPosition>,

    #[serde(rename = "prov_used", skip_serializing_if = "Vec::is_empty", default)]
    pub prov_used: Vec<String>,

    #[serde(
        rename = "prov_generated",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub prov_generated: Vec<String>,

    #[serde(
        rename = "prov_attributed_to",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub prov_attributed_to: Option<String>,
}

impl Reasoning {
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
            ibis_mode: None,
            positions: Vec::new(),
            prov_used: Vec::new(),
            prov_generated: Vec::new(),
            prov_attributed_to: None,
        }
    }

    pub fn flatten_positions_to_steps(&mut self) {
        if self.positions.is_empty() {
            return;
        }
        self.steps = self
            .positions
            .iter()
            .map(|pos| ReasoningStep {
                id: Uuid::new_v4().to_string(),
                description: format!("[{}] {}", pos.position_type, pos.content),
                conclusion: pos
                    .responds_to
                    .as_ref()
                    .map(|r| format!("responds to: {}", r))
                    .unwrap_or_default(),
                evidence: Vec::new(),
                confidence: 0.0,
                timestamp: Utc::now(),
            })
            .collect();
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
        reasoning.add_step("Step 1".to_string(), "Conclusion 1".to_string(), 0.8);
        assert_eq!(reasoning.steps.len(), 1);
        assert_eq!(reasoning.confidence, 0.8);

        // Add evidence
        reasoning.add_evidence_to_last_step("Evidence A".to_string());
        assert_eq!(reasoning.steps[0].evidence.len(), 1);

        // Add second step
        reasoning.add_step("Step 2".to_string(), "Conclusion 2".to_string(), 0.6);
        // Average confidence: (0.8 + 0.6) / 2 = 0.7
        assert_eq!(reasoning.confidence, 0.7);

        // Final conclusion override
        reasoning.set_conclusion("Final".to_string(), 1.0);
        assert_eq!(reasoning.conclusion, "Final");
        assert_eq!(reasoning.confidence, 1.0);
    }

    #[test]
    fn test_reasoning_validation() {
        let mut reasoning =
            Reasoning::new("".to_string(), "task-1".to_string(), "agent".to_string());

        assert!(reasoning.validate_entity().is_err());

        reasoning.title = "Valid".to_string();
        reasoning.task_id = "".to_string();
        assert!(reasoning.validate_entity().is_err());

        reasoning.task_id = "task-1".to_string();
        assert!(reasoning.validate_entity().is_ok());
    }

    #[test]
    fn test_ibis_position_serialization() {
        let pos = IbisPosition {
            position_type: IbisPositionType::Issue,
            content: "Should we use gix?".to_string(),
            responds_to: None,
        };
        let json = serde_json::to_string(&pos).unwrap();
        let deserialized: IbisPosition = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.position_type, IbisPositionType::Issue);
        assert_eq!(deserialized.content, "Should we use gix?");
        assert!(deserialized.responds_to.is_none());
    }

    #[test]
    fn test_reasoning_with_ibis_fields() {
        let mut reasoning = Reasoning::new(
            "IBIS Decision".to_string(),
            "task-1".to_string(),
            "agent".to_string(),
        );
        reasoning.ibis_mode = Some(true);
        reasoning.positions.push(IbisPosition {
            position_type: IbisPositionType::Issue,
            content: "Which library?".to_string(),
            responds_to: None,
        });
        reasoning.positions.push(IbisPosition {
            position_type: IbisPositionType::Position,
            content: "Use gix".to_string(),
            responds_to: None,
        });
        reasoning.positions.push(IbisPosition {
            position_type: IbisPositionType::Argument,
            content: "Pure Rust".to_string(),
            responds_to: Some("Use gix".to_string()),
        });

        let json = serde_json::to_string(&reasoning).unwrap();
        let deserialized: Reasoning = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.ibis_mode, Some(true));
        assert_eq!(deserialized.positions.len(), 3);
        assert_eq!(
            deserialized.positions[2].responds_to.as_deref(),
            Some("Use gix")
        );
    }

    #[test]
    fn test_flatten_positions_to_steps() {
        let mut reasoning = Reasoning::new(
            "Flatten test".to_string(),
            "task-1".to_string(),
            "agent".to_string(),
        );
        reasoning.ibis_mode = Some(true);
        reasoning.positions.push(IbisPosition {
            position_type: IbisPositionType::Issue,
            content: "Which library?".to_string(),
            responds_to: None,
        });
        reasoning.positions.push(IbisPosition {
            position_type: IbisPositionType::Position,
            content: "Use gix".to_string(),
            responds_to: Some("Which library?".to_string()),
        });

        reasoning.flatten_positions_to_steps();

        assert_eq!(reasoning.steps.len(), 2);
        assert!(reasoning.steps[0].description.contains("[Issue]"));
        assert!(reasoning.steps[0].description.contains("Which library?"));
        assert!(reasoning.steps[1].description.contains("[Position]"));
        assert!(reasoning.steps[1]
            .conclusion
            .contains("responds to: Which library?"));
    }

    #[test]
    fn test_flatten_positions_empty_is_noop() {
        let mut reasoning = Reasoning::new(
            "Noop".to_string(),
            "task-1".to_string(),
            "agent".to_string(),
        );
        reasoning.add_step("existing".to_string(), "step".to_string(), 0.5);
        reasoning.flatten_positions_to_steps();
        assert_eq!(reasoning.steps.len(), 1);
    }

    #[test]
    fn test_prov_o_fields_serialization() {
        let mut reasoning = Reasoning::new(
            "PROV test".to_string(),
            "task-1".to_string(),
            "agent".to_string(),
        );
        reasoning.prov_used.push("entity-a".to_string());
        reasoning.prov_used.push("entity-b".to_string());
        reasoning.prov_generated.push("entity-c".to_string());
        reasoning.prov_attributed_to = Some("orchestrator".to_string());

        let json = serde_json::to_string(&reasoning).unwrap();
        let deserialized: Reasoning = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.prov_used, vec!["entity-a", "entity-b"]);
        assert_eq!(deserialized.prov_generated, vec!["entity-c"]);
        assert_eq!(
            deserialized.prov_attributed_to.as_deref(),
            Some("orchestrator")
        );
    }

    #[test]
    fn test_reasoning_without_new_fields_backward_compat() {
        let json = r#"{
            "id": "test-id",
            "title": "Old Reasoning",
            "task_id": "task-1",
            "steps": [],
            "conclusion": "done",
            "confidence": 0.9,
            "agent": "old-agent",
            "created_at": "2025-01-01T00:00:00Z"
        }"#;
        let reasoning: Reasoning = serde_json::from_str(json).unwrap();
        assert_eq!(reasoning.title, "Old Reasoning");
        assert!(reasoning.ibis_mode.is_none());
        assert!(reasoning.positions.is_empty());
        assert!(reasoning.prov_used.is_empty());
        assert!(reasoning.prov_generated.is_empty());
        assert!(reasoning.prov_attributed_to.is_none());
    }
}

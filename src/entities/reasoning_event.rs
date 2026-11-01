use super::{Entity, GenericEntity};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReasoningEventType {
    AutoStored,
    Created,
    StatusChanged,
    ConclusionReached,
    EvidenceAdded,
    CounterEvidence,
    AssumptionChallenged,
}

impl clap::ValueEnum for ReasoningEventType {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Self::AutoStored,
            Self::Created,
            Self::StatusChanged,
            Self::ConclusionReached,
            Self::EvidenceAdded,
            Self::CounterEvidence,
            Self::AssumptionChallenged,
        ]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            Self::AutoStored => clap::builder::PossibleValue::new("auto_stored"),
            Self::Created => clap::builder::PossibleValue::new("created"),
            Self::StatusChanged => clap::builder::PossibleValue::new("status_changed"),
            Self::ConclusionReached => clap::builder::PossibleValue::new("conclusion_reached"),
            Self::EvidenceAdded => clap::builder::PossibleValue::new("evidence_added"),
            Self::CounterEvidence => clap::builder::PossibleValue::new("counter_evidence"),
            Self::AssumptionChallenged => {
                clap::builder::PossibleValue::new("assumption_challenged")
            }
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ReasoningEvent {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "reasoning_id")]
    pub reasoning_id: String,

    #[serde(rename = "event_type")]
    pub event_type: ReasoningEventType,

    #[serde(rename = "agent", default)]
    pub agent: String,

    #[serde(rename = "timestamp")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "content")]
    pub content: String,

    #[serde(
        rename = "metadata",
        skip_serializing_if = "HashMap::is_empty",
        default
    )]
    pub metadata: HashMap<String, serde_json::Value>,
}

impl ReasoningEvent {
    pub fn new(reasoning_id: String, event_type: ReasoningEventType, content: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            reasoning_id,
            event_type,
            agent: String::new(),
            timestamp: Utc::now(),
            content,
            metadata: HashMap::new(),
        }
    }
}

impl Entity for ReasoningEvent {
    fn entity_type() -> &'static str {
        "reasoning_event"
    }

    fn id(&self) -> &str {
        &self.id
    }

    fn agent(&self) -> &str {
        &self.agent
    }

    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }

    fn validate_entity(&self) -> crate::Result<()> {
        if self.reasoning_id.is_empty() {
            return Err(crate::EngramError::Validation(
                "reasoning_id cannot be empty".to_string(),
            ));
        }
        if self.content.is_empty() {
            return Err(crate::EngramError::Validation(
                "content cannot be empty".to_string(),
            ));
        }
        Ok(())
    }

    fn to_generic(&self) -> GenericEntity {
        GenericEntity {
            id: self.id.clone(),
            entity_type: Self::entity_type().to_string(),
            agent: self.agent.clone(),
            timestamp: self.timestamp,
            data: serde_json::to_value(self).unwrap_or_default(),
        }
    }

    fn from_generic(entity: GenericEntity) -> crate::Result<Self> {
        serde_json::from_value(entity.data).map_err(|e| {
            crate::EngramError::Deserialization(format!(
                "Failed to deserialize ReasoningEvent: {}",
                e
            ))
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
    fn test_reasoning_event_creation() {
        let event = ReasoningEvent::new(
            "reasoning-123".to_string(),
            ReasoningEventType::Created,
            "Initial reasoning created".to_string(),
        );
        assert_eq!(event.reasoning_id, "reasoning-123");
        assert_eq!(event.event_type, ReasoningEventType::Created);
        assert!(!event.id.is_empty());
    }

    #[test]
    fn test_reasoning_event_roundtrip() {
        let event = ReasoningEvent::new(
            "reasoning-456".to_string(),
            ReasoningEventType::EvidenceAdded,
            "New evidence".to_string(),
        );
        let generic = event.to_generic();
        let restored = ReasoningEvent::from_generic(generic).unwrap();
        assert_eq!(restored.reasoning_id, event.reasoning_id);
        assert_eq!(restored.event_type, event.event_type);
        assert_eq!(restored.content, event.content);
    }

    #[test]
    fn test_reasoning_event_validation() {
        let mut event = ReasoningEvent::new(
            "".to_string(),
            ReasoningEventType::Created,
            "content".to_string(),
        );
        assert!(event.validate_entity().is_err());

        event.reasoning_id = "valid-id".to_string();
        event.content = "".to_string();
        assert!(event.validate_entity().is_err());

        event.content = "valid content".to_string();
        assert!(event.validate_entity().is_ok());
    }

    #[test]
    fn test_reasoning_event_type_serde() {
        let json = r#"{"event_type":"conclusion_reached"}"#;
        let parsed: serde_json::Value = serde_json::from_str(json).unwrap();
        let event_type: ReasoningEventType =
            serde_json::from_value(parsed.get("event_type").unwrap().clone()).unwrap();
        assert_eq!(event_type, ReasoningEventType::ConclusionReached);
    }

    #[test]
    fn test_reasoning_event_type_all_variants() {
        let variants = vec![
            r#""auto_stored""#,
            r#""created""#,
            r#""status_changed""#,
            r#""conclusion_reached""#,
            r#""evidence_added""#,
            r#""counter_evidence""#,
            r#""assumption_challenged""#,
        ];
        for variant in variants {
            let event_type: ReasoningEventType = serde_json::from_str(variant).unwrap();
            assert!(matches!(
                event_type,
                ReasoningEventType::AutoStored
                    | ReasoningEventType::Created
                    | ReasoningEventType::StatusChanged
                    | ReasoningEventType::ConclusionReached
                    | ReasoningEventType::EvidenceAdded
                    | ReasoningEventType::CounterEvidence
                    | ReasoningEventType::AssumptionChallenged
            ));
        }
    }
}

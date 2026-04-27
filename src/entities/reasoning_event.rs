use super::{Entity, GenericEntity};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ReasoningEventType {
    AutoStored,
    TheoryMutated,
    ContradictionFound,
    HypothesisUpdated,
    ConclusionReached,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ReasoningEvent {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "reasoning_id")]
    pub reasoning_id: String,

    #[serde(rename = "event_type")]
    pub event_type: ReasoningEventType,

    #[serde(rename = "content")]
    pub content: String,

    #[serde(rename = "agent")]
    pub agent: String,

    #[serde(rename = "created_at")]
    pub created_at: DateTime<Utc>,

    #[serde(
        rename = "metadata",
        skip_serializing_if = "HashMap::is_empty",
        default
    )]
    pub metadata: HashMap<String, serde_json::Value>,
}

impl ReasoningEvent {
    pub fn new(
        reasoning_id: String,
        event_type: ReasoningEventType,
        content: String,
        agent: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            reasoning_id,
            event_type,
            content,
            agent,
            created_at: Utc::now(),
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
        self.created_at
    }

    fn validate_entity(&self) -> crate::Result<()> {
        if self.reasoning_id.is_empty() {
            return Err(crate::EngramError::Validation(
                "Reasoning ID cannot be empty".to_string(),
            ));
        }
        if self.content.is_empty() {
            return Err(crate::EngramError::Validation(
                "Event content cannot be empty".to_string(),
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
            ReasoningEventType::AutoStored,
            "Reasoning stored".to_string(),
            "agent".to_string(),
        );
        assert_eq!(event.reasoning_id, "reasoning-123");
        assert_eq!(event.event_type, ReasoningEventType::AutoStored);
        assert!(!event.id.is_empty());
    }

    #[test]
    fn test_reasoning_event_serialization() {
        let event = ReasoningEvent::new(
            "reasoning-123".to_string(),
            ReasoningEventType::TheoryMutated,
            "Theory changed".to_string(),
            "agent".to_string(),
        );
        let json = serde_json::to_string(&event).unwrap();
        let deserialized: ReasoningEvent = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.reasoning_id, "reasoning-123");
        assert_eq!(deserialized.event_type, ReasoningEventType::TheoryMutated);
        assert_eq!(deserialized.content, "Theory changed");
    }

    #[test]
    fn test_reasoning_event_custom_type() {
        let event = ReasoningEvent::new(
            "reasoning-123".to_string(),
            ReasoningEventType::Custom("MyCustomEvent".to_string()),
            "Custom".to_string(),
            "agent".to_string(),
        );
        let json = serde_json::to_string(&event).unwrap();
        let deserialized: ReasoningEvent = serde_json::from_str(&json).unwrap();
        assert_eq!(
            deserialized.event_type,
            ReasoningEventType::Custom("MyCustomEvent".to_string())
        );
    }

    #[test]
    fn test_reasoning_event_validation_empty_reasoning_id() {
        let event = ReasoningEvent::new(
            "".to_string(),
            ReasoningEventType::AutoStored,
            "Content".to_string(),
            "agent".to_string(),
        );
        assert!(event.validate_entity().is_err());
    }

    #[test]
    fn test_reasoning_event_validation_empty_content() {
        let event = ReasoningEvent::new(
            "reasoning-123".to_string(),
            ReasoningEventType::AutoStored,
            "".to_string(),
            "agent".to_string(),
        );
        assert!(event.validate_entity().is_err());
    }

    #[test]
    fn test_reasoning_event_entity_type() {
        assert_eq!(ReasoningEvent::entity_type(), "reasoning_event");
    }
}

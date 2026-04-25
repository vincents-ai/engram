//! Re-export relationship types from engram-core and add Entity impl.
//!
//! Data types (EntityRelationship, RelationshipFilter, etc.) live in
//! engram-core. The `Entity` trait implementation lives here because
//! `Entity` is defined in the main crate.

// Re-export all data types from engram-core
pub use engram_core::relationship::{
    EntityRelationType, EntityRelationship, RelationshipConstraints, RelationshipDirection,
    RelationshipFilter, RelationshipStrength,
};

use crate::entities::{Entity, GenericEntity};

impl Entity for EntityRelationship {
    fn entity_type() -> &'static str {
        "relationship"
    }

    fn id(&self) -> &str {
        &self.id
    }

    fn agent(&self) -> &str {
        &self.agent
    }

    fn timestamp(&self) -> chrono::DateTime<chrono::Utc> {
        self.timestamp
    }

    fn validate_entity(&self) -> crate::Result<()> {
        if self.id.trim().is_empty() {
            return Err(crate::EngramError::Validation(
                "Relationship ID cannot be empty".to_string(),
            ));
        }
        if self.source_id.trim().is_empty() {
            return Err(crate::EngramError::Validation(
                "Source ID cannot be empty".to_string(),
            ));
        }
        if self.target_id.trim().is_empty() {
            return Err(crate::EngramError::Validation(
                "Target ID cannot be empty".to_string(),
            ));
        }
        if self.source_id == self.target_id {
            return Err(crate::EngramError::Validation(
                "Self-relationships are not allowed".to_string(),
            ));
        }
        if self.source_type.trim().is_empty() {
            return Err(crate::EngramError::Validation(
                "Source type cannot be empty".to_string(),
            ));
        }
        if self.target_type.trim().is_empty() {
            return Err(crate::EngramError::Validation(
                "Target type cannot be empty".to_string(),
            ));
        }
        if self.agent.trim().is_empty() {
            return Err(crate::EngramError::Validation(
                "Agent cannot be empty".to_string(),
            ));
        }
        self.validate_constraints().map_err(|e| crate::EngramError::Validation(e.to_string()))?;
        Ok(())
    }

    fn to_generic(&self) -> GenericEntity {
        GenericEntity {
            id: self.id.clone(),
            entity_type: Self::entity_type().to_string(),
            agent: self.agent.clone(),
            timestamp: self.timestamp,
            data: serde_json::to_value(self).expect("Failed to serialize relationship"),
        }
    }

    fn from_generic(entity: GenericEntity) -> crate::Result<Self> {
        if entity.entity_type != Self::entity_type() {
            return Err(crate::EngramError::Deserialization(format!(
                "Expected entity type '{}', got '{}'",
                Self::entity_type(),
                entity.entity_type
            )));
        }
        serde_json::from_value(entity.data).map_err(|e| {
            crate::EngramError::Deserialization(format!(
                "Failed to deserialize relationship: {}",
                e
            ))
        })
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

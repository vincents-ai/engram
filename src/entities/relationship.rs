//! Entity relationship management for graph-based operations
//!
//! This module provides comprehensive relationship modeling between entities,
//! supporting graph traversal, relationship queries, and complex data modeling.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{Entity, GenericEntity};

/// Direction of a relationship between entities
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RelationshipDirection {
    /// Bidirectional relationship (A <-> B)
    Bidirectional,
    /// Unidirectional relationship (A -> B)
    Unidirectional,
    /// Inverse relationship (B -> A)
    Inverse,
}

/// Type of relationship between entities
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum EntityRelationType {
    /// Direct dependency (Task A depends on Task B)
    DependsOn,
    /// Parent-child relationship (Task A has child Task B)
    Contains,
    /// Reference relationship (Context A references Knowledge B)
    References,
    /// Fulfills relationship (Task A fulfills Requirement B)
    Fulfills,
    /// Implements relationship (Implementation A implements Standard B)
    Implements,
    /// Supersedes relationship (ADR A supersedes ADR B)
    Supersedes,
    /// Associates with (Session A associated with Task B)
    AssociatedWith,
    /// Influences relationship (Rule A influences Workflow B)
    Influences,
    /// Custom relationship type
    Custom(String),
}

impl std::fmt::Display for EntityRelationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntityRelationType::DependsOn => write!(f, "depends_on"),
            EntityRelationType::Contains => write!(f, "contains"),
            EntityRelationType::References => write!(f, "references"),
            EntityRelationType::Fulfills => write!(f, "fulfills"),
            EntityRelationType::Implements => write!(f, "implements"),
            EntityRelationType::Supersedes => write!(f, "supersedes"),
            EntityRelationType::AssociatedWith => write!(f, "associated_with"),
            EntityRelationType::Influences => write!(f, "influences"),
            EntityRelationType::Custom(name) => write!(f, "{}", name),
        }
    }
}

/// Strength of a relationship (for weighted graph algorithms)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RelationshipStrength {
    Weak,
    Medium,
    Strong,
    Critical,
    Custom(f64), // 0.0 to 1.0
}

impl RelationshipStrength {
    /// Get numeric value for algorithms (0.0 to 1.0)
    pub fn weight(&self) -> f64 {
        match self {
            RelationshipStrength::Weak => 0.25,
            RelationshipStrength::Medium => 0.5,
            RelationshipStrength::Strong => 0.75,
            RelationshipStrength::Critical => 1.0,
            RelationshipStrength::Custom(w) => w.clamp(0.0, 1.0),
        }
    }
}

/// Constraints for relationship validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipConstraints {
    /// Maximum number of outbound relationships of this type
    pub max_outbound: Option<usize>,
    /// Maximum number of inbound relationships of this type
    pub max_inbound: Option<usize>,
    /// Whether this relationship can create cycles
    pub allow_cycles: bool,
    /// Required entity types for source
    pub source_types: Option<Vec<String>>,
    /// Required entity types for target
    pub target_types: Option<Vec<String>>,
}

impl Default for RelationshipConstraints {
    fn default() -> Self {
        Self {
            max_outbound: None,
            max_inbound: None,
            allow_cycles: true,
            source_types: None,
            target_types: None,
        }
    }
}

/// Core entity relationship structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityRelationship {
    /// Unique identifier for this relationship
    pub id: String,
    /// Agent who created this relationship
    pub agent: String,
    /// Creation timestamp
    pub timestamp: DateTime<Utc>,
    /// Source entity ID
    pub source_id: String,
    /// Source entity type
    pub source_type: String,
    /// Target entity ID
    pub target_id: String,
    /// Target entity type
    pub target_type: String,
    /// Type of relationship
    pub relationship_type: EntityRelationType,
    /// Direction of relationship
    pub direction: RelationshipDirection,
    /// Strength of relationship
    pub strength: RelationshipStrength,
    /// Relationship constraints
    pub constraints: RelationshipConstraints,
    /// Custom metadata for the relationship
    pub metadata: HashMap<String, serde_json::Value>,
    /// Optional human-readable description
    pub description: Option<String>,
    /// Whether this relationship is active
    pub active: bool,
}

impl EntityRelationship {
    /// Create a new entity relationship
    pub fn new(
        id: String,
        agent: String,
        source_id: String,
        source_type: String,
        target_id: String,
        target_type: String,
        relationship_type: EntityRelationType,
    ) -> Self {
        Self {
            id,
            agent,
            timestamp: Utc::now(),
            source_id,
            source_type,
            target_id,
            target_type,
            relationship_type,
            direction: RelationshipDirection::Unidirectional,
            strength: RelationshipStrength::Medium,
            constraints: RelationshipConstraints::default(),
            metadata: HashMap::new(),
            description: None,
            active: true,
        }
    }

    /// Set relationship direction
    pub fn with_direction(mut self, direction: RelationshipDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Set relationship strength
    pub fn with_strength(mut self, strength: RelationshipStrength) -> Self {
        self.strength = strength;
        self
    }

    /// Set relationship constraints
    pub fn with_constraints(mut self, constraints: RelationshipConstraints) -> Self {
        self.constraints = constraints;
        self
    }

    /// Add metadata
    pub fn with_metadata(mut self, key: String, value: serde_json::Value) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Set description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Check if this relationship involves the given entity
    pub fn involves_entity(&self, entity_id: &str) -> bool {
        self.source_id == entity_id || self.target_id == entity_id
    }

    /// Get the other entity ID in the relationship
    pub fn get_other_entity(&self, entity_id: &str) -> Option<&str> {
        if self.source_id == entity_id {
            Some(&self.target_id)
        } else if self.target_id == entity_id {
            Some(&self.source_id)
        } else {
            None
        }
    }

    /// Check if this relationship allows traversal from source to target
    pub fn allows_traversal_to(&self, from_id: &str, to_id: &str) -> bool {
        if !self.active {
            return false;
        }

        match self.direction {
            RelationshipDirection::Bidirectional => {
                (self.source_id == from_id && self.target_id == to_id)
                    || (self.source_id == to_id && self.target_id == from_id)
            }
            RelationshipDirection::Unidirectional => {
                self.source_id == from_id && self.target_id == to_id
            }
            RelationshipDirection::Inverse => self.source_id == to_id && self.target_id == from_id,
        }
    }

    /// Validate relationship constraints
    pub fn validate_constraints(&self) -> crate::Result<()> {
        // Check entity type constraints
        if let Some(ref source_types) = self.constraints.source_types {
            if !source_types.contains(&self.source_type) {
                return Err(crate::EngramError::Validation(format!(
                    "Source type '{}' not allowed for this relationship",
                    self.source_type
                )));
            }
        }

        if let Some(ref target_types) = self.constraints.target_types {
            if !target_types.contains(&self.target_type) {
                return Err(crate::EngramError::Validation(format!(
                    "Target type '{}' not allowed for this relationship",
                    self.target_type
                )));
            }
        }

        Ok(())
    }
}

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

    fn timestamp(&self) -> DateTime<Utc> {
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

        self.validate_constraints()?;

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

/// Query filter for relationships
#[derive(Debug, Clone, Default)]
pub struct RelationshipFilter {
    /// Filter by source entity ID
    pub source_id: Option<String>,
    /// Filter by target entity ID
    pub target_id: Option<String>,
    /// Filter by entity ID (either source or target)
    pub entity_id: Option<String>,
    /// Filter by relationship type
    pub relationship_type: Option<EntityRelationType>,
    /// Filter by direction
    pub direction: Option<RelationshipDirection>,
    /// Filter by strength minimum
    pub min_strength: Option<f64>,
    /// Filter by active status
    pub active_only: Option<bool>,
    /// Filter by source entity type
    pub source_type: Option<String>,
    /// Filter by target entity type  
    pub target_type: Option<String>,
    /// Filter by agent
    pub agent: Option<String>,
}

impl RelationshipFilter {
    /// Create a new empty filter
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by source entity
    pub fn source(mut self, source_id: String) -> Self {
        self.source_id = Some(source_id);
        self
    }

    /// Filter by target entity
    pub fn target(mut self, target_id: String) -> Self {
        self.target_id = Some(target_id);
        self
    }

    /// Filter by entity (either source or target)
    pub fn entity(mut self, entity_id: String) -> Self {
        self.entity_id = Some(entity_id);
        self
    }

    /// Filter by relationship type
    pub fn relationship_type(mut self, rel_type: EntityRelationType) -> Self {
        self.relationship_type = Some(rel_type);
        self
    }

    /// Filter by direction
    pub fn direction(mut self, direction: RelationshipDirection) -> Self {
        self.direction = Some(direction);
        self
    }

    /// Filter by minimum strength
    pub fn min_strength(mut self, strength: f64) -> Self {
        self.min_strength = Some(strength);
        self
    }

    /// Filter active relationships only
    pub fn active_only(mut self) -> Self {
        self.active_only = Some(true);
        self
    }

    /// Check if a relationship matches this filter
    pub fn matches(&self, relationship: &EntityRelationship) -> bool {
        if let Some(ref source_id) = self.source_id {
            if &relationship.source_id != source_id {
                return false;
            }
        }

        if let Some(ref target_id) = self.target_id {
            if &relationship.target_id != target_id {
                return false;
            }
        }

        if let Some(ref entity_id) = self.entity_id {
            if !relationship.involves_entity(entity_id) {
                return false;
            }
        }

        if let Some(ref rel_type) = self.relationship_type {
            if &relationship.relationship_type != rel_type {
                return false;
            }
        }

        if let Some(ref direction) = self.direction {
            if &relationship.direction != direction {
                return false;
            }
        }

        if let Some(min_strength) = self.min_strength {
            if relationship.strength.weight() < min_strength {
                return false;
            }
        }

        if let Some(active_only) = self.active_only {
            if active_only && !relationship.active {
                return false;
            }
        }

        if let Some(ref source_type) = self.source_type {
            if &relationship.source_type != source_type {
                return false;
            }
        }

        if let Some(ref target_type) = self.target_type {
            if &relationship.target_type != target_type {
                return false;
            }
        }

        if let Some(ref agent) = self.agent {
            if &relationship.agent != agent {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relationship_creation() {
        let relationship = EntityRelationship::new(
            "rel-001".to_string(),
            "test-agent".to_string(),
            "task-001".to_string(),
            "task".to_string(),
            "task-002".to_string(),
            "task".to_string(),
            EntityRelationType::DependsOn,
        );

        assert_eq!(relationship.id, "rel-001");
        assert_eq!(relationship.agent, "test-agent");
        assert_eq!(relationship.source_id, "task-001");
        assert_eq!(relationship.target_id, "task-002");
        assert_eq!(
            relationship.relationship_type,
            EntityRelationType::DependsOn
        );
        assert!(relationship.active);
    }

    #[test]
    fn test_relationship_direction() {
        let mut relationship = EntityRelationship::new(
            "rel-001".to_string(),
            "test-agent".to_string(),
            "task-001".to_string(),
            "task".to_string(),
            "task-002".to_string(),
            "task".to_string(),
            EntityRelationType::DependsOn,
        );

        // Test unidirectional (default)
        assert!(relationship.allows_traversal_to("task-001", "task-002"));
        assert!(!relationship.allows_traversal_to("task-002", "task-001"));

        // Test bidirectional
        relationship.direction = RelationshipDirection::Bidirectional;
        assert!(relationship.allows_traversal_to("task-001", "task-002"));
        assert!(relationship.allows_traversal_to("task-002", "task-001"));

        // Test inverse
        relationship.direction = RelationshipDirection::Inverse;
        assert!(!relationship.allows_traversal_to("task-001", "task-002"));
        assert!(relationship.allows_traversal_to("task-002", "task-001"));
    }

    #[test]
    fn test_relationship_validation() {
        let relationship = EntityRelationship::new(
            "rel-001".to_string(),
            "test-agent".to_string(),
            "task-001".to_string(),
            "task".to_string(),
            "task-002".to_string(),
            "task".to_string(),
            EntityRelationType::DependsOn,
        );

        assert!(relationship.validate_entity().is_ok());

        // Test self-relationship validation
        let self_rel = EntityRelationship::new(
            "rel-002".to_string(),
            "test-agent".to_string(),
            "task-001".to_string(),
            "task".to_string(),
            "task-001".to_string(),
            "task".to_string(),
            EntityRelationType::DependsOn,
        );

        assert!(self_rel.validate_entity().is_err());
    }

    #[test]
    fn test_relationship_filter() {
        let relationship = EntityRelationship::new(
            "rel-001".to_string(),
            "test-agent".to_string(),
            "task-001".to_string(),
            "task".to_string(),
            "task-002".to_string(),
            "task".to_string(),
            EntityRelationType::DependsOn,
        );

        let filter = RelationshipFilter::new()
            .source("task-001".to_string())
            .relationship_type(EntityRelationType::DependsOn)
            .active_only();

        assert!(filter.matches(&relationship));

        let filter2 = RelationshipFilter::new().source("task-999".to_string());
        assert!(!filter2.matches(&relationship));
    }

    #[test]
    fn test_relationship_strength() {
        assert_eq!(RelationshipStrength::Weak.weight(), 0.25);
        assert_eq!(RelationshipStrength::Medium.weight(), 0.5);
        assert_eq!(RelationshipStrength::Strong.weight(), 0.75);
        assert_eq!(RelationshipStrength::Critical.weight(), 1.0);
        assert_eq!(RelationshipStrength::Custom(0.3).weight(), 0.3);
        assert_eq!(RelationshipStrength::Custom(-0.5).weight(), 0.0);
        assert_eq!(RelationshipStrength::Custom(1.5).weight(), 1.0);
    }
}

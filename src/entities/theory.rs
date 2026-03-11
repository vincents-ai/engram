//! Theory entity implementation for agent cognitive modeling (Naur, 1985)
//!
//! Based on Peter Naur's "Programming as Theory Building" (1985), this entity
//! represents an agent's internal theory of the system - the mental model
//! that informs how the agent understands and operates on the domain.

use super::{Entity, GenericEntity};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use validator::Validate;

/// Represents an agent's internal theory of the system
///
/// A theory goes beyond context; it explicitly maps the domain model to the
/// system implementation and records the "why" (design rationale), which Naur
/// identified as the most critical and easily lost knowledge.
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Theory {
    /// Unique identifier
    #[serde(rename = "id")]
    pub id: String,

    /// High-level name of the theory or domain module
    #[serde(rename = "domain_name")]
    pub domain_name: String,

    /// Agent's understanding of domain concepts and their rules
    #[serde(rename = "conceptual_model")]
    pub conceptual_model: HashMap<String, String>,

    /// How the conceptual model maps to actual code/state (e.g., "User" -> "users table")
    #[serde(rename = "system_mapping")]
    pub system_mapping: HashMap<String, String>,

    /// The "Why": Justifications for why the system is mapped this way
    #[serde(rename = "design_rationale")]
    pub design_rationale: HashMap<String, String>,

    /// Known invariant truths about this system state
    #[serde(rename = "invariants", skip_serializing_if = "Vec::is_empty", default)]
    pub invariants: Vec<String>,

    /// Associated agent
    #[serde(rename = "agent")]
    pub agent: String,

    /// Creation timestamp
    #[serde(rename = "created_at")]
    pub created_at: DateTime<Utc>,

    /// Last update timestamp
    #[serde(rename = "last_updated")]
    pub last_updated: DateTime<Utc>,

    /// Number of times this theory has been refined
    #[serde(rename = "iteration_count")]
    pub iteration_count: u32,

    /// IDs of StateReflections that informed this theory version
    #[serde(
        rename = "reflection_ids",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub reflection_ids: Vec<String>,

    /// Task this theory was created for
    #[serde(rename = "task_id", skip_serializing_if = "Option::is_none", default)]
    pub task_id: Option<String>,

    /// Additional metadata
    #[serde(
        rename = "metadata",
        skip_serializing_if = "HashMap::is_empty",
        default
    )]
    pub metadata: HashMap<String, serde_json::Value>,
}

impl Theory {
    /// Create a new theory for a domain
    pub fn new(domain_name: String, agent: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            domain_name,
            conceptual_model: HashMap::new(),
            system_mapping: HashMap::new(),
            design_rationale: HashMap::new(),
            invariants: Vec::new(),
            agent,
            created_at: now,
            last_updated: now,
            iteration_count: 1,
            reflection_ids: Vec::new(),
            task_id: None,
            metadata: HashMap::new(),
        }
    }

    /// Create a new theory for a specific task
    pub fn for_task(domain_name: String, agent: String, task_id: String) -> Self {
        let mut theory = Self::new(domain_name, agent);
        theory.task_id = Some(task_id);
        theory
    }

    /// Add a concept to the conceptual model
    pub fn add_concept(&mut self, concept: String, definition: String) {
        self.conceptual_model.insert(concept, definition);
        self.touch();
    }

    /// Add a system mapping
    pub fn add_mapping(&mut self, concept: String, implementation: String) {
        self.system_mapping.insert(concept, implementation);
        self.touch();
    }

    /// Add design rationale for a decision
    pub fn add_rationale(&mut self, decision: String, reason: String) {
        self.design_rationale.insert(decision, reason);
        self.touch();
    }

    /// Add an invariant that must hold true
    pub fn add_invariant(&mut self, invariant: String) {
        if !self.invariants.contains(&invariant) {
            self.invariants.push(invariant);
            self.touch();
        }
    }

    /// Apply updates from a completed state reflection
    pub fn apply_reflection_updates(
        &mut self,
        updates: HashMap<String, String>,
        reflection_id: String,
    ) {
        for (key, new_value) in updates {
            self.conceptual_model.insert(key, new_value);
        }
        if !self.reflection_ids.contains(&reflection_id) {
            self.reflection_ids.push(reflection_id);
        }
        self.touch();
    }

    /// Update timestamp and increment iteration
    fn touch(&mut self) {
        self.last_updated = Utc::now();
        self.iteration_count += 1;
    }

    /// Check if a concept exists in the theory
    pub fn has_concept(&self, concept: &str) -> bool {
        self.conceptual_model.contains_key(concept)
    }

    /// Get the system mapping for a concept
    pub fn get_mapping(&self, concept: &str) -> Option<&String> {
        self.system_mapping.get(concept)
    }
}

impl Entity for Theory {
    fn entity_type() -> &'static str {
        "theory"
    }

    fn id(&self) -> &str {
        &self.id
    }

    fn agent(&self) -> &str {
        &self.agent
    }

    fn timestamp(&self) -> DateTime<Utc> {
        self.last_updated
    }

    fn validate_entity(&self) -> crate::Result<()> {
        if let Err(errors) = <Theory as validator::Validate>::validate(self) {
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

        if self.domain_name.is_empty() {
            return Err(crate::EngramError::Validation(
                "Theory domain_name cannot be empty".to_string(),
            ));
        }

        Ok(())
    }

    fn to_generic(&self) -> GenericEntity {
        GenericEntity {
            id: self.id.clone(),
            entity_type: Self::entity_type().to_string(),
            agent: self.agent.clone(),
            timestamp: self.last_updated,
            data: serde_json::to_value(self).unwrap_or_default(),
        }
    }

    fn from_generic(entity: GenericEntity) -> crate::Result<Self> {
        serde_json::from_value(entity.data).map_err(|e| {
            crate::EngramError::Deserialization(format!("Failed to deserialize Theory: {}", e))
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
    fn test_theory_creation() {
        let theory = Theory::new("User Management".to_string(), "the-architect".to_string());

        assert_eq!(theory.domain_name, "User Management");
        assert_eq!(theory.agent, "the-architect");
        assert_eq!(theory.iteration_count, 1);
        assert!(theory.conceptual_model.is_empty());
    }

    #[test]
    fn test_theory_for_task() {
        let theory = Theory::for_task(
            "Authentication".to_string(),
            "the-architect".to_string(),
            "task-123".to_string(),
        );

        assert_eq!(theory.task_id, Some("task-123".to_string()));
    }

    #[test]
    fn test_theory_evolution() {
        let mut theory = Theory::new("API".to_string(), "agent".to_string());

        theory.add_concept(
            "User".to_string(),
            "A person who uses the system".to_string(),
        );
        assert!(theory.has_concept("User"));
        assert_eq!(theory.iteration_count, 2);

        theory.add_mapping("User".to_string(), "users table in PostgreSQL".to_string());
        assert_eq!(
            theory.get_mapping("User"),
            Some(&"users table in PostgreSQL".to_string())
        );
        assert_eq!(theory.iteration_count, 3);

        theory.add_rationale(
            "Use PostgreSQL".to_string(),
            "ACID compliance required for financial data".to_string(),
        );
        assert!(theory.design_rationale.contains_key("Use PostgreSQL"));

        theory.add_invariant("User email must be unique".to_string());
        assert!(theory
            .invariants
            .contains(&"User email must be unique".to_string()));
    }

    #[test]
    fn test_apply_reflection_updates() {
        let mut theory = Theory::new("Domain".to_string(), "agent".to_string());
        let initial_iteration = theory.iteration_count;

        let mut updates = HashMap::new();
        updates.insert("Concept".to_string(), "Refined understanding".to_string());

        theory.apply_reflection_updates(updates, "reflection-1".to_string());

        assert_eq!(
            theory.conceptual_model.get("Concept"),
            Some(&"Refined understanding".to_string())
        );
        assert!(theory.reflection_ids.contains(&"reflection-1".to_string()));
        assert!(theory.iteration_count > initial_iteration);
    }

    #[test]
    fn test_theory_validation() {
        let mut theory = Theory::new("".to_string(), "agent".to_string());
        assert!(theory.validate_entity().is_err());

        theory.domain_name = "Valid Domain".to_string();
        assert!(theory.validate_entity().is_ok());
    }

    #[test]
    fn test_theory_serialization() {
        let mut theory = Theory::new("Test".to_string(), "agent".to_string());
        theory.add_concept("A".to_string(), "B".to_string());

        let generic = theory.to_generic();
        assert_eq!(generic.entity_type, "theory");

        let restored = Theory::from_generic(generic).unwrap();
        assert_eq!(restored.domain_name, "Test");
        assert_eq!(restored.conceptual_model.get("A"), Some(&"B".to_string()));
    }
}

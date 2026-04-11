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

/// Cost level for reversing a design decision
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReversalCost {
    Low,
    Medium,
    High,
}

impl clap::ValueEnum for ReversalCost {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Low, Self::Medium, Self::High]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            Self::Low => clap::builder::PossibleValue::new("low"),
            Self::Medium => clap::builder::PossibleValue::new("medium"),
            Self::High => clap::builder::PossibleValue::new("high"),
        })
    }
}

/// A structured design decision with rationale
#[derive(Debug, Clone, Serialize, Deserialize, Validate, schemars::JsonSchema)]
pub struct DesignDecision {
    /// The decision that was made
    #[serde(rename = "decision")]
    pub decision: String,

    /// Why this decision was made
    #[serde(rename = "rationale")]
    pub rationale: String,

    /// Alternative approaches that were considered and rejected
    #[serde(
        rename = "alternatives_discarded",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub alternatives_discarded: Vec<String>,

    /// How difficult it would be to reverse this decision
    #[serde(rename = "reversal_cost")]
    pub reversal_cost: ReversalCost,

    /// Who made or approved this decision
    #[serde(
        rename = "stakeholder",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub stakeholder: Option<String>,
}

impl DesignDecision {
    /// Create a new design decision
    pub fn new(decision: String, rationale: String, reversal_cost: ReversalCost) -> Self {
        Self {
            decision,
            rationale,
            alternatives_discarded: Vec::new(),
            reversal_cost,
            stakeholder: None,
        }
    }

    /// Add an alternative that was discarded
    pub fn add_alternative(&mut self, alternative: String) {
        self.alternatives_discarded.push(alternative);
    }
}

/// Type of invariant
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum InvariantType {
    Assertion,
    Constraint,
    Precondition,
    Postcondition,
    Invariant,
}

impl clap::ValueEnum for InvariantType {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Self::Assertion,
            Self::Constraint,
            Self::Precondition,
            Self::Postcondition,
            Self::Invariant,
        ]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            Self::Assertion => clap::builder::PossibleValue::new("assertion"),
            Self::Constraint => clap::builder::PossibleValue::new("constraint"),
            Self::Precondition => clap::builder::PossibleValue::new("precondition"),
            Self::Postcondition => clap::builder::PossibleValue::new("postcondition"),
            Self::Invariant => clap::builder::PossibleValue::new("invariant"),
        })
    }
}

/// A structured invariant with type and optional Gherkin scenario linkage
#[derive(Debug, Clone, Serialize, Deserialize, Validate, schemars::JsonSchema)]
pub struct Invariant {
    /// Unique identifier for this invariant
    #[serde(rename = "id")]
    pub id: String,

    /// Description of the invariant
    #[serde(rename = "description")]
    pub description: String,

    /// Type of invariant
    #[serde(rename = "invariant_type")]
    pub invariant_type: InvariantType,

    /// Optional Gherkin scenario that tests this invariant
    #[serde(
        rename = "gherkin_scenario",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub gherkin_scenario: Option<String>,
}

impl std::fmt::Display for Invariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description)
    }
}

impl Invariant {
    /// Create a new invariant
    pub fn new(description: String, invariant_type: InvariantType) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            description,
            invariant_type,
            gherkin_scenario: None,
        }
    }

    /// Create from a simple string (backwards compatible)
    pub fn from_string(s: String) -> Self {
        Self::new(s, InvariantType::Assertion)
    }
}

/// C4 model abstraction level
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum C4Level {
    Context,
    Container,
    Component,
    Code,
}

impl clap::ValueEnum for C4Level {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Context, Self::Container, Self::Component, Self::Code]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            Self::Context => clap::builder::PossibleValue::new("context"),
            Self::Container => clap::builder::PossibleValue::new("container"),
            Self::Component => clap::builder::PossibleValue::new("component"),
            Self::Code => clap::builder::PossibleValue::new("code"),
        })
    }
}

/// Represents an agent's internal theory of the system
///
/// A theory goes beyond context; it explicitly maps the domain model to the
/// system implementation and records the "why" (design rationale), which Naur
/// identified as the most critical and easily lost knowledge.
#[derive(Debug, Clone, Serialize, Deserialize, Validate, schemars::JsonSchema)]
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

    /// The "Why": Structured design decisions with rationale
    #[serde(
        rename = "design_rationale",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub design_rationale: Vec<DesignDecision>,

    /// Known invariant truths about this system state
    #[serde(rename = "invariants", skip_serializing_if = "Vec::is_empty", default)]
    pub invariants: Vec<Invariant>,

    /// C4 model abstraction level (Context, Container, Component, Code)
    #[serde(rename = "c4_level", skip_serializing_if = "Option::is_none", default)]
    pub c4_level: Option<C4Level>,

    /// Parent theory ID for hierarchical theories
    #[serde(
        rename = "parent_theory_id",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub parent_theory_id: Option<String>,

    /// Bounded context scope for this theory (None = global scope)
    #[serde(
        rename = "bounded_context",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub bounded_context: Option<String>,

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

    /// Last accessed timestamp (set when entity is read/used)
    #[serde(
        rename = "last_accessed_at",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub last_accessed_at: Option<DateTime<Utc>>,

    /// Number of times referenced by relationships
    #[serde(rename = "citation_count", default)]
    pub citation_count: u32,

    /// Decay weight for relevance scoring (1.0 = fresh, decays toward 0.0)
    #[serde(rename = "decay_weight", default)]
    pub decay_weight: f64,
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
            design_rationale: Vec::new(),
            invariants: Vec::new(),
            c4_level: None,
            parent_theory_id: None,
            bounded_context: None,
            agent,
            created_at: now,
            last_updated: now,
            iteration_count: 1,
            reflection_ids: Vec::new(),
            task_id: None,
            metadata: HashMap::new(),
            last_accessed_at: None,
            citation_count: 0,
            decay_weight: 1.0,
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

    /// Add a design decision with full rationale
    pub fn add_rationale(
        &mut self,
        decision: String,
        rationale: String,
        reversal_cost: ReversalCost,
        alternatives: Vec<String>,
        stakeholder: Option<String>,
    ) {
        let mut design_decision = DesignDecision::new(decision, rationale, reversal_cost);
        design_decision.alternatives_discarded = alternatives;
        design_decision.stakeholder = stakeholder;
        self.design_rationale.push(design_decision);
        self.touch();
    }

    /// Add a simple rationale (backwards compatible)
    pub fn add_rationale_simple(&mut self, decision: String, reason: String) {
        let design_decision = DesignDecision::new(decision, reason, ReversalCost::Medium);
        self.design_rationale.push(design_decision);
        self.touch();
    }

    /// Add an invariant that must hold true
    pub fn add_invariant(&mut self, description: String) {
        let invariant = Invariant::new(description, InvariantType::Assertion);
        if !self
            .invariants
            .iter()
            .any(|i| i.description == invariant.description)
        {
            self.invariants.push(invariant);
            self.touch();
        }
    }

    /// Add a structured invariant
    pub fn add_invariant_struct(&mut self, invariant: Invariant) {
        if !self.invariants.iter().any(|i| i.id == invariant.id) {
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

    /// Record access (updates last_accessed_at)
    pub fn record_access(&mut self) {
        self.last_accessed_at = Some(Utc::now());
    }

    /// Record citation (increments citation_count)
    pub fn record_citation(&mut self) {
        self.citation_count += 1;
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

        theory.add_rationale_simple(
            "Use PostgreSQL".to_string(),
            "ACID compliance required for financial data".to_string(),
        );
        assert!(theory
            .design_rationale
            .iter()
            .any(|d| d.decision == "Use PostgreSQL"));

        theory.add_invariant("User email must be unique".to_string());
        assert!(theory
            .invariants
            .iter()
            .any(|i| i.description == "User email must be unique"));
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

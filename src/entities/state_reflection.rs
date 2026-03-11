//! State Reflection entity for evaluating system state against theory
//!
//! Reflection is the mechanism of taking the current observed state and
//! explicitly hunting for contradictions against the Theory (Naur, 1985).
//! When observations conflict with the theory, cognitive dissonance is recorded
//! and theory updates are proposed.

use super::{Entity, GenericEntity};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use validator::Validate;

/// Represents a reflection on system state against an agent's theory
///
/// This entity captures the moment when an agent observes something that
/// conflicts with its internal model, enabling theory evolution.
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct StateReflection {
    /// Unique identifier
    #[serde(rename = "id")]
    pub id: String,

    /// The ID of the Theory being reflected upon
    #[serde(rename = "theory_id")]
    pub theory_id: String,

    /// The ID of the Task or Reasoning chain that triggered this reflection
    #[serde(rename = "trigger_context_id")]
    pub trigger_context_id: String,

    /// The raw state observed (e.g., error logs, unexpected DB state, test outputs)
    #[serde(rename = "observed_state")]
    pub observed_state: String,

    /// Where the observed state conflicts with the current Theory
    /// (Naur's "death of a theory" moment)
    #[serde(
        rename = "cognitive_dissonance",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub cognitive_dissonance: Vec<String>,

    /// Proposed changes to the agent's internal model to resolve the dissonance
    #[serde(
        rename = "proposed_theory_updates",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub proposed_theory_updates: Vec<String>,

    /// A measure of how much the underlying theory is currently failing
    /// 0.0 = perfect match, 1.0 = total paradigm shift needed
    #[serde(rename = "dissonance_score")]
    pub dissonance_score: f64,

    /// Associated agent
    #[serde(rename = "agent")]
    pub agent: String,

    /// Creation timestamp
    #[serde(rename = "timestamp")]
    pub timestamp: DateTime<Utc>,

    /// Type of observation that triggered this reflection
    #[serde(
        rename = "trigger_type",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub trigger_type: Option<TriggerType>,

    /// Whether this reflection has been resolved (theory updated)
    #[serde(rename = "resolved", skip_serializing_if = "Option::is_none", default)]
    pub resolved: Option<bool>,

    /// ID of the theory version after applying updates (if resolved)
    #[serde(
        rename = "resolved_theory_id",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub resolved_theory_id: Option<String>,

    /// Additional metadata
    #[serde(
        rename = "metadata",
        skip_serializing_if = "HashMap::is_empty",
        default
    )]
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Types of observations that can trigger state reflection
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TriggerType {
    /// Test failure
    TestFailure,
    /// Runtime error
    RuntimeError,
    /// Unexpected output
    UnexpectedOutput,
    /// Type mismatch
    TypeMismatch,
    /// Behavioral deviation
    BehavioralDeviation,
    /// Manual observation
    ManualObservation,
    /// Performance anomaly
    PerformanceAnomaly,
    /// Security concern
    SecurityConcern,
}

impl StateReflection {
    /// Create a new state reflection
    pub fn new(
        theory_id: String,
        trigger_context_id: String,
        observed_state: String,
        agent: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            theory_id,
            trigger_context_id,
            observed_state,
            cognitive_dissonance: Vec::new(),
            proposed_theory_updates: Vec::new(),
            dissonance_score: 0.0,
            agent,
            timestamp: Utc::now(),
            trigger_type: None,
            resolved: None,
            resolved_theory_id: None,
            metadata: HashMap::new(),
        }
    }

    /// Create with a specific trigger type
    pub fn with_trigger_type(
        theory_id: String,
        trigger_context_id: String,
        observed_state: String,
        agent: String,
        trigger_type: TriggerType,
    ) -> Self {
        let mut reflection = Self::new(theory_id, trigger_context_id, observed_state, agent);
        reflection.trigger_type = Some(trigger_type);
        reflection
    }

    /// Record a cognitive dissonance (gap between theory and observation)
    pub fn record_dissonance(&mut self, gap_description: String, impact_score: f64) {
        self.cognitive_dissonance.push(gap_description);
        self.dissonance_score = self.dissonance_score.max(impact_score).clamp(0.0, 1.0);
    }

    /// Propose a theory update to resolve dissonance
    pub fn propose_update(&mut self, update: String) {
        self.proposed_theory_updates.push(update);
    }

    /// Mark this reflection as resolved
    pub fn resolve(&mut self, new_theory_id: String) {
        self.resolved = Some(true);
        self.resolved_theory_id = Some(new_theory_id);
    }

    /// Check if dissonance requires theory mutation before continuing
    pub fn requires_theory_mutation(&self, threshold: f64) -> bool {
        self.dissonance_score >= threshold && !self.proposed_theory_updates.is_empty()
    }

    /// Get the severity level based on dissonance score
    pub fn severity(&self) -> Severity {
        match self.dissonance_score {
            s if s >= 0.8 => Severity::Critical,
            s if s >= 0.5 => Severity::High,
            s if s >= 0.3 => Severity::Medium,
            s if s > 0.0 => Severity::Low,
            _ => Severity::None,
        }
    }
}

/// Severity levels for state reflection
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    /// No dissonance
    None,
    /// Minor discrepancy
    Low,
    /// Moderate conflict
    Medium,
    /// Significant theory failure
    High,
    /// Theory is fundamentally broken
    Critical,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::None => write!(f, "none"),
            Severity::Low => write!(f, "low"),
            Severity::Medium => write!(f, "medium"),
            Severity::High => write!(f, "high"),
            Severity::Critical => write!(f, "critical"),
        }
    }
}

impl Entity for StateReflection {
    fn entity_type() -> &'static str {
        "state_reflection"
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
        if let Err(errors) = <StateReflection as validator::Validate>::validate(self) {
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

        if self.theory_id.is_empty() {
            return Err(crate::EngramError::Validation(
                "theory_id cannot be empty".to_string(),
            ));
        }

        if self.dissonance_score < 0.0 || self.dissonance_score > 1.0 {
            return Err(crate::EngramError::Validation(
                "dissonance_score must be between 0.0 and 1.0".to_string(),
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
                "Failed to deserialize StateReflection: {}",
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
    fn test_reflection_creation() {
        let reflection = StateReflection::new(
            "theory-1".to_string(),
            "task-1".to_string(),
            "Test failed: expected 5, got 3".to_string(),
            "the-architect".to_string(),
        );

        assert_eq!(reflection.theory_id, "theory-1");
        assert_eq!(reflection.trigger_context_id, "task-1");
        assert!(reflection.cognitive_dissonance.is_empty());
        assert_eq!(reflection.dissonance_score, 0.0);
    }

    #[test]
    fn test_reflection_with_trigger_type() {
        let reflection = StateReflection::with_trigger_type(
            "theory-1".to_string(),
            "task-1".to_string(),
            "Error".to_string(),
            "agent".to_string(),
            TriggerType::TestFailure,
        );

        assert_eq!(reflection.trigger_type, Some(TriggerType::TestFailure));
    }

    #[test]
    fn test_dissonance_recording() {
        let mut reflection = StateReflection::new(
            "theory-1".to_string(),
            "task-1".to_string(),
            "observed".to_string(),
            "agent".to_string(),
        );

        reflection.record_dissonance("Concept A maps to wrong table".to_string(), 0.5);
        assert_eq!(reflection.cognitive_dissonance.len(), 1);
        assert_eq!(reflection.dissonance_score, 0.5);

        reflection.record_dissonance("Another issue".to_string(), 0.3);
        assert_eq!(reflection.dissonance_score, 0.5); // Max, not sum

        reflection.record_dissonance("Critical issue".to_string(), 0.9);
        assert_eq!(reflection.dissonance_score, 0.9);
    }

    #[test]
    fn test_severity_levels() {
        let mut reflection = StateReflection::new(
            "t".to_string(),
            "c".to_string(),
            "o".to_string(),
            "a".to_string(),
        );

        assert_eq!(reflection.severity(), Severity::None);

        reflection.dissonance_score = 0.1;
        assert_eq!(reflection.severity(), Severity::Low);

        reflection.dissonance_score = 0.4;
        assert_eq!(reflection.severity(), Severity::Medium);

        reflection.dissonance_score = 0.6;
        assert_eq!(reflection.severity(), Severity::High);

        reflection.dissonance_score = 0.9;
        assert_eq!(reflection.severity(), Severity::Critical);
    }

    #[test]
    fn test_requires_theory_mutation() {
        let mut reflection = StateReflection::new(
            "t".to_string(),
            "c".to_string(),
            "o".to_string(),
            "a".to_string(),
        );

        reflection.dissonance_score = 0.6;
        assert!(!reflection.requires_theory_mutation(0.5)); // No proposed updates

        reflection.propose_update("Update concept A".to_string());
        assert!(reflection.requires_theory_mutation(0.5));
        assert!(!reflection.requires_theory_mutation(0.7)); // Below threshold
    }

    #[test]
    fn test_reflection_resolution() {
        let mut reflection = StateReflection::new(
            "theory-1".to_string(),
            "task-1".to_string(),
            "observed".to_string(),
            "agent".to_string(),
        );

        reflection.resolve("theory-2".to_string());

        assert_eq!(reflection.resolved, Some(true));
        assert_eq!(reflection.resolved_theory_id, Some("theory-2".to_string()));
    }

    #[test]
    fn test_reflection_validation() {
        let mut reflection = StateReflection::new(
            "".to_string(), // Invalid empty theory_id
            "task-1".to_string(),
            "observed".to_string(),
            "agent".to_string(),
        );

        assert!(reflection.validate_entity().is_err());

        reflection.theory_id = "theory-1".to_string();
        assert!(reflection.validate_entity().is_ok());

        reflection.dissonance_score = 1.5; // Invalid
        assert!(reflection.validate_entity().is_err());
    }

    #[test]
    fn test_reflection_serialization() {
        let mut reflection = StateReflection::new(
            "theory-1".to_string(),
            "task-1".to_string(),
            "observed".to_string(),
            "agent".to_string(),
        );
        reflection.record_dissonance("Issue".to_string(), 0.7);

        let generic = reflection.to_generic();
        assert_eq!(generic.entity_type, "state_reflection");

        let restored = StateReflection::from_generic(generic).unwrap();
        assert_eq!(restored.theory_id, "theory-1");
        assert_eq!(restored.dissonance_score, 0.7);
        assert_eq!(restored.cognitive_dissonance.len(), 1);
    }
}

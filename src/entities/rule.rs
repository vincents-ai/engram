//! Rule entity implementation
//!
//! Rules define automated policies and validations that the system enforces.
//! Rules are evaluated against entities to ensure compliance with team policies.
//!
//! ## Rule Types
//!
//! - **Validation**: Check entities meet criteria before creation/update
//! - **Transformation**: Modify entity data during processing
//! - **Enforcement**: Block operations that violate policies
//! - **Notification**: Trigger alerts when conditions are met
//!
//! ## Execution Flow
//!
//! Rules are evaluated by the [`RuleEngine`](crate::engines::rule_engine) when
//! entities are created or modified. The engine checks the `condition` expression
//! and executes the `action` if the condition evaluates to true.
//!
//! ## Relationship to Standards and Compliance
//!
//! Rules are the **implementation mechanism** for Standards. A Standard defines
//! "what" should be done, while Rules define "how" it's enforced automatically.
//! Compliance items track adherence to Standards, using Rules as the validation layer.
//!
//! Example: Standard "All code must have tests" → Rule "Reject commits without test files"
//!
//! ## Example
//!
//! ```json
//! {
//!   "title": "Require task reasoning",
//!   "rule_type": "validation",
//!   "condition": {"equals": [{"entity_type": "task"}, "reasoning_count", 0]},
//!   "action": {"error": "Task requires at least one reasoning entity"},
//!   "entity_types": ["task"]
//! }
//! ```

use super::{Entity, GenericEntity};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use validator::Validate;

/// Rule status variants
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum RuleStatus {
    Active,
    Inactive,
    Deprecated,
}

/// Rule priority levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum RulePriority {
    Low,
    Medium,
    High,
    Critical,
}

/// Rule type variants
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum RuleType {
    Validation,
    Transformation,
    Enforcement,
    Notification,
}

/// Rule entity for system rules and policies
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Rule {
    /// Unique identifier
    #[serde(rename = "id")]
    pub id: String,

    /// Rule title
    #[serde(rename = "title")]
    pub title: String,

    /// Rule description
    #[serde(rename = "description")]
    pub description: String,

    /// Rule type
    #[serde(rename = "rule_type")]
    pub rule_type: RuleType,

    /// Current status
    #[serde(rename = "status")]
    pub status: RuleStatus,

    /// Priority level
    #[serde(rename = "priority")]
    pub priority: RulePriority,

    /// Associated agent
    #[serde(rename = "agent")]
    pub agent: String,

    /// Rule condition (JSON logic)
    #[serde(rename = "condition")]
    pub condition: serde_json::Value,

    /// Rule action to execute
    #[serde(rename = "action")]
    pub action: serde_json::Value,

    /// Creation timestamp
    #[serde(rename = "created_at")]
    pub created_at: DateTime<Utc>,

    /// Last updated timestamp
    #[serde(rename = "updated_at")]
    pub updated_at: DateTime<Utc>,

    /// Entity types this rule applies to
    #[serde(rename = "entity_types")]
    pub entity_types: Vec<String>,

    /// Execution history
    #[serde(
        rename = "execution_history",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub execution_history: Vec<RuleExecution>,

    /// Tags for categorization
    #[serde(rename = "tags", skip_serializing_if = "Vec::is_empty", default)]
    pub tags: Vec<String>,

    /// Related rules
    #[serde(
        rename = "related_rules",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub related_rules: Vec<String>,

    /// Additional metadata
    #[serde(
        rename = "metadata",
        skip_serializing_if = "HashMap::is_empty",
        default
    )]
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Rule execution record
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct RuleExecution {
    /// Execution identifier
    #[serde(rename = "id")]
    pub id: String,

    /// When rule was executed
    #[serde(rename = "executed_at")]
    pub executed_at: DateTime<Utc>,

    /// Entity that triggered the rule
    #[serde(rename = "trigger_entity")]
    pub trigger_entity: String,

    /// Execution result
    #[serde(rename = "result")]
    pub result: RuleExecutionResult,

    /// Execution duration in milliseconds
    #[serde(rename = "duration_ms")]
    pub duration_ms: u64,

    /// Any error message
    #[serde(rename = "error_message", skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

/// Rule execution result
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum RuleExecutionResult {
    Success,
    Failed,
    Skipped,
}

impl Rule {
    /// Create a new rule
    pub fn new(
        title: String,
        description: String,
        rule_type: RuleType,
        priority: RulePriority,
        agent: String,
        condition: serde_json::Value,
        action: serde_json::Value,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            description,
            rule_type,
            status: RuleStatus::Active,
            priority,
            agent,
            condition,
            action,
            created_at: now,
            updated_at: now,
            entity_types: Vec::new(),
            execution_history: Vec::new(),
            tags: Vec::new(),
            related_rules: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Execute rule against an entity
    pub fn execute(&mut self, entity: &super::GenericEntity) -> RuleExecutionResult {
        let start_time = Utc::now();

        let result = self.evaluate_condition(entity);

        let execution = RuleExecution {
            id: Uuid::new_v4().to_string(),
            executed_at: start_time,
            trigger_entity: entity.id.clone(),
            result: result.clone(),
            duration_ms: (Utc::now()
                .signed_duration_since(start_time)
                .num_milliseconds()
                .max(0)) as u64,
            error_message: if matches!(result, RuleExecutionResult::Failed) {
                Some("Condition evaluation failed".to_string())
            } else {
                None
            },
        };

        self.execution_history.push(execution);
        self.updated_at = Utc::now();

        result
    }

    fn evaluate_condition(&self, entity: &super::GenericEntity) -> RuleExecutionResult {
        if !self.entity_types.is_empty() && !self.entity_types.contains(&entity.entity_type) {
            return RuleExecutionResult::Skipped;
        }

        // For now, assume success if rule is active
        if self.status == RuleStatus::Active {
            RuleExecutionResult::Success
        } else {
            RuleExecutionResult::Skipped
        }
    }

    /// Deactivate rule
    pub fn deactivate(&mut self) {
        self.status = RuleStatus::Inactive;
        self.updated_at = Utc::now();
    }

    /// Activate rule
    pub fn activate(&mut self) {
        self.status = RuleStatus::Active;
        self.updated_at = Utc::now();
    }

    /// Deprecate rule
    pub fn deprecate(&mut self) {
        self.status = RuleStatus::Deprecated;
        self.updated_at = Utc::now();
    }

    /// Add entity type this rule applies to
    pub fn add_entity_type(&mut self, entity_type: String) {
        if !self.entity_types.contains(&entity_type) {
            self.entity_types.push(entity_type);
        }
    }

    /// Add a related rule
    pub fn add_related_rule(&mut self, rule_id: String) {
        if !self.related_rules.contains(&rule_id) {
            self.related_rules.push(rule_id);
        }
    }
}

impl Entity for Rule {
    fn entity_type() -> &'static str {
        "rule"
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
        if let Err(errors) = <Rule as validator::Validate>::validate(self) {
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
                "Rule title cannot be empty".to_string(),
            ));
        }

        if self.description.is_empty() {
            return Err(crate::EngramError::Validation(
                "Rule description cannot be empty".to_string(),
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
            crate::EngramError::Deserialization(format!("Failed to deserialize Rule: {}", e))
        })
    }

    fn as_any(&self) -> &dyn std::any::Any
    where
        Self: Sized,
    {
        self
    }
}

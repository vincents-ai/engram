//! Session entity implementation

use super::{Entity, EntityResult, GenericEntity};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use validator::Validate;

/// Session status variants
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SessionStatus {
    Active,
    Paused,
    Completed,
    Cancelled,
}

/// Session entity for tracking agent sessions
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Session {
    /// Unique identifier
    #[serde(rename = "id")]
    pub id: String,

    /// Session title/name
    #[serde(rename = "title")]
    pub title: String,

    /// Associated agent
    #[serde(rename = "agent")]
    pub agent: String,

    /// Current status
    #[serde(rename = "status")]
    pub status: SessionStatus,

    /// Start timestamp
    #[serde(rename = "start_time")]
    pub start_time: DateTime<Utc>,

    /// End timestamp
    #[serde(rename = "end_time")]
    pub end_time: Option<DateTime<Utc>>,

    /// Duration in seconds (calculated)
    #[serde(rename = "duration_seconds")]
    pub duration_seconds: Option<u64>,

    /// Tasks worked on during session
    #[serde(rename = "task_ids", skip_serializing_if = "Vec::is_empty", default)]
    pub task_ids: Vec<String>,

    /// Context items used
    #[serde(rename = "context_ids", skip_serializing_if = "Vec::is_empty", default)]
    pub context_ids: Vec<String>,

    /// Knowledge items referenced
    #[serde(
        rename = "knowledge_ids",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub knowledge_ids: Vec<String>,

    /// Session goals
    #[serde(rename = "goals", skip_serializing_if = "Vec::is_empty", default)]
    pub goals: Vec<String>,

    /// Session outcomes
    #[serde(rename = "outcomes", skip_serializing_if = "Vec::is_empty", default)]
    pub outcomes: Vec<String>,

    /// SPACE framework metrics
    #[serde(rename = "space_metrics")]
    pub space_metrics: Option<SpaceMetrics>,

    /// DORA metrics
    #[serde(rename = "dora_metrics")]
    pub dora_metrics: Option<DoraMetrics>,

    /// Tags for categorization
    #[serde(rename = "tags", skip_serializing_if = "Vec::is_empty", default)]
    pub tags: Vec<String>,

    /// Additional metadata
    #[serde(
        rename = "metadata",
        skip_serializing_if = "HashMap::is_empty",
        default
    )]
    pub metadata: HashMap<String, serde_json::Value>,
}

/// SPACE framework metrics
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct SpaceMetrics {
    /// Satisfaction score (0-100)
    #[serde(rename = "satisfaction_score")]
    pub satisfaction_score: f64,

    /// Performance score (0-100)
    #[serde(rename = "performance_score")]
    pub performance_score: f64,

    /// Activity score (0-100)
    #[serde(rename = "activity_score")]
    pub activity_score: f64,

    /// Communication score (0-100)
    #[serde(rename = "communication_score")]
    pub communication_score: f64,

    /// Efficiency score (0-100)
    #[serde(rename = "efficiency_score")]
    pub efficiency_score: f64,

    /// Overall score (0-100)
    #[serde(rename = "overall_score")]
    pub overall_score: f64,
}

/// DORA metrics
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct DoraMetrics {
    /// Deployment frequency (per week)
    #[serde(rename = "deployment_frequency")]
    pub deployment_frequency: f64,

    /// Lead time for changes (in days)
    #[serde(rename = "lead_time")]
    pub lead_time: f64,

    /// Change failure rate (percentage)
    #[serde(rename = "change_failure_rate")]
    pub change_failure_rate: f64,

    /// Mean time to recovery (in hours)
    #[serde(rename = "mean_time_to_recover")]
    pub mean_time_to_recover: f64,
}

impl Session {
    /// Create a new session
    pub fn new(title: String, agent: String, goals: Vec<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            agent,
            status: SessionStatus::Active,
            start_time: now,
            end_time: None,
            duration_seconds: None,
            task_ids: Vec::new(),
            context_ids: Vec::new(),
            knowledge_ids: Vec::new(),
            goals,
            outcomes: Vec::new(),
            space_metrics: None,
            dora_metrics: None,
            tags: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Complete the session
    pub fn complete(&mut self, outcomes: Vec<String>) {
        self.status = SessionStatus::Completed;
        self.end_time = Some(Utc::now());
        self.outcomes = outcomes;
        self.calculate_duration();
    }

    /// Pause the session
    pub fn pause(&mut self) {
        self.status = SessionStatus::Paused;
    }

    /// Resume the session
    pub fn resume(&mut self) {
        self.status = SessionStatus::Active;
    }

    /// Cancel the session
    pub fn cancel(&mut self) {
        self.status = SessionStatus::Cancelled;
        self.end_time = Some(Utc::now());
        self.calculate_duration();
    }

    /// Calculate duration in seconds
    fn calculate_duration(&mut self) {
        if let Some(end_time) = self.end_time {
            let duration = end_time.signed_duration_since(self.start_time);
            self.duration_seconds = Some(duration.num_seconds().max(0) as u64);
        }
    }

    /// Add a task to the session
    pub fn add_task(&mut self, task_id: String) {
        if !self.task_ids.contains(&task_id) {
            self.task_ids.push(task_id);
        }
    }

    /// Add a context to the session
    pub fn add_context(&mut self, context_id: String) {
        if !self.context_ids.contains(&context_id) {
            self.context_ids.push(context_id);
        }
    }

    /// Add knowledge to the session
    pub fn add_knowledge(&mut self, knowledge_id: String) {
        if !self.knowledge_ids.contains(&knowledge_id) {
            self.knowledge_ids.push(knowledge_id);
        }
    }

    /// Set SPACE metrics
    pub fn set_space_metrics(&mut self, metrics: SpaceMetrics) {
        self.space_metrics = Some(metrics);
    }

    /// Set DORA metrics
    pub fn set_dora_metrics(&mut self, metrics: DoraMetrics) {
        self.dora_metrics = Some(metrics);
    }
}

impl Entity for Session {
    fn entity_type() -> &'static str {
        "session"
    }

    fn id(&self) -> &str {
        &self.id
    }

    fn agent(&self) -> &str {
        &self.agent
    }

    fn timestamp(&self) -> DateTime<Utc> {
        self.start_time
    }

    fn validate_entity(&self) -> super::EntityResult<()> {
        if let Err(errors) = <Session as validator::Validate>::validate(self) {
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
            return Err(error_messages.join(", "));
        }

        if self.title.is_empty() {
            return Err("Session title cannot be empty".to_string());
        }

        if self.agent.is_empty() {
            return Err("Session agent cannot be empty".to_string());
        }

        Ok(())
    }

    fn to_generic(&self) -> GenericEntity {
        GenericEntity {
            id: self.id.clone(),
            entity_type: Self::entity_type().to_string(),
            agent: self.agent.clone(),
            timestamp: self.start_time,
            data: serde_json::to_value(self).unwrap_or_default(),
        }
    }

    fn from_generic(entity: GenericEntity) -> EntityResult<Self> {
        serde_json::from_value(entity.data)
            .map_err(|e| format!("Failed to deserialize Session: {}", e))
    }

    fn as_any(&self) -> &dyn std::any::Any
    where
        Self: Sized,
    {
        self
    }
}

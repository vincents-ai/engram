//! Task entity implementation

use super::{Entity, GenericEntity};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use validator::Validate;

/// Task status variants
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
    Blocked,
    Cancelled,
}

/// Task priority variants
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TaskPriority {
    Low,
    Medium,
    High,
    Critical,
}

/// Task entity representing a work item with status tracking
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Task {
    /// Unique identifier
    #[serde(rename = "id")]
    pub id: String,

    /// Task title
    #[serde(rename = "title")]
    pub title: String,

    /// Detailed description
    #[serde(rename = "description")]
    pub description: String,

    /// Current status
    #[serde(rename = "status")]
    pub status: TaskStatus,

    /// Priority level
    #[serde(rename = "priority")]
    pub priority: TaskPriority,

    /// Assigned agent
    #[serde(rename = "agent")]
    pub agent: String,

    /// Start time
    #[serde(rename = "start_time")]
    pub start_time: DateTime<Utc>,

    /// End time
    #[serde(rename = "end_time")]
    pub end_time: Option<DateTime<Utc>>,

    /// Parent task ID
    #[serde(rename = "parent", skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,

    /// Child task IDs
    #[serde(rename = "children", skip_serializing_if = "Vec::is_empty", default)]
    pub children: Vec<String>,

    /// Tags for categorization
    #[serde(rename = "tags", skip_serializing_if = "Vec::is_empty", default)]
    pub tags: Vec<String>,

    /// Associated context IDs
    #[serde(rename = "context_ids", skip_serializing_if = "Vec::is_empty", default)]
    pub context_ids: Vec<String>,

    /// Knowledge items
    #[serde(rename = "knowledge", skip_serializing_if = "Vec::is_empty", default)]
    pub knowledge: Vec<String>,

    /// Related files
    #[serde(rename = "files", skip_serializing_if = "Vec::is_empty", default)]
    pub files: Vec<String>,

    /// Task outcome
    #[serde(rename = "outcome", skip_serializing_if = "Option::is_none")]
    pub outcome: Option<String>,

    /// Reason for blocking the task
    #[serde(rename = "block_reason", skip_serializing_if = "Option::is_none")]
    pub block_reason: Option<String>,

    #[serde(rename = "workflow_id", skip_serializing_if = "Option::is_none")]
    pub workflow_id: Option<String>,

    #[serde(rename = "workflow_state", skip_serializing_if = "Option::is_none")]
    pub workflow_state: Option<String>,

    /// Additional metadata
    #[serde(
        rename = "metadata",
        skip_serializing_if = "HashMap::is_empty",
        default
    )]
    pub metadata: HashMap<String, serde_json::Value>,
}

impl Task {
    /// Create a new task
    pub fn new(
        title: String,
        description: String,
        agent: String,
        priority: TaskPriority,
        workflow_id: Option<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            description,
            status: TaskStatus::Todo,
            priority,
            agent,
            start_time: now,
            end_time: None,
            parent: None,
            children: Vec::new(),
            tags: Vec::new(),
            context_ids: Vec::new(),
            knowledge: Vec::new(),
            files: Vec::new(),
            outcome: None,
            workflow_id,
            workflow_state: None,
            block_reason: None,
            metadata: HashMap::new(),
        }
    }

    /// Mark task as in progress
    pub fn start(&mut self) {
        self.status = TaskStatus::InProgress;
        self.block_reason = None;
    }

    /// Block the task
    pub fn block(&mut self, reason: String) {
        self.status = TaskStatus::Blocked;
        self.block_reason = Some(reason);
    }

    /// Update workflow state
    pub fn update_workflow_state(&mut self, state: String) {
        self.workflow_state = Some(state);
    }

    /// Complete the task
    pub fn complete(&mut self, outcome: String) {
        self.status = TaskStatus::Done;
        self.end_time = Some(Utc::now());
        self.outcome = Some(outcome);
    }

    /// Add a child task
    pub fn add_child(&mut self, child_id: String) {
        if !self.children.contains(&child_id) {
            self.children.push(child_id);
        }
    }

    /// Add a tag
    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
    }
}

impl Entity for Task {
    fn entity_type() -> &'static str {
        "task"
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

    fn validate_entity(&self) -> crate::Result<()> {
        if self.title.is_empty() {
            return Err(crate::EngramError::Validation(
                "Task title cannot be empty".to_string(),
            ));
        }

        if self.agent.is_empty() {
            return Err(crate::EngramError::Validation(
                "Task agent cannot be empty".to_string(),
            ));
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

    fn from_generic(entity: GenericEntity) -> crate::Result<Self> {
        serde_json::from_value(entity.data).map_err(|e| {
            crate::EngramError::Deserialization(format!("Failed to deserialize Task: {}", e))
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
    fn test_task_creation() {
        let task = Task::new(
            "Test Task".to_string(),
            "Description".to_string(),
            "agent-1".to_string(),
            TaskPriority::High,
            None,
        );

        assert_eq!(task.title, "Test Task");
        assert_eq!(task.status, TaskStatus::Todo);
        assert_eq!(task.priority, TaskPriority::High);
    }

    #[test]
    fn test_task_lifecycle() {
        let mut task = Task::new(
            "Test".to_string(),
            "Desc".to_string(),
            "agent".to_string(),
            TaskPriority::Medium,
            None,
        );

        // Start
        task.start();
        assert_eq!(task.status, TaskStatus::InProgress);

        // Block
        task.block("Waiting".to_string());
        assert_eq!(task.status, TaskStatus::Blocked);
        assert_eq!(task.block_reason, Some("Waiting".to_string()));

        // Complete
        task.complete("Done".to_string());
        assert_eq!(task.status, TaskStatus::Done);
        assert!(task.end_time.is_some());
        assert_eq!(task.outcome, Some("Done".to_string()));
    }

    #[test]
    fn test_task_validation() {
        let mut task = Task::new(
            "".to_string(), // Invalid empty title
            "Desc".to_string(),
            "agent".to_string(),
            TaskPriority::Low,
            None,
        );

        assert!(task.validate_entity().is_err());

        task.title = "Valid Title".to_string();
        assert!(task.validate_entity().is_ok());

        task.agent = "".to_string(); // Invalid empty agent
        assert!(task.validate_entity().is_err());
    }
}

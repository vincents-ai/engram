//! Workflow Instance entity implementation

use super::{Entity, EntityResult, GenericEntity};
use crate::engines::workflow_engine::{
    WorkflowExecutionContext, WorkflowExecutionEvent, WorkflowStatus,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Workflow instance execution state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowInstance {
    /// Unique identifier
    #[serde(rename = "id")]
    pub id: String,

    /// Associated workflow ID
    #[serde(rename = "workflow_id")]
    pub workflow_id: String,

    /// Current state name
    #[serde(rename = "current_state")]
    pub current_state: String,

    /// Execution context
    #[serde(rename = "context")]
    pub context: WorkflowExecutionContext,

    /// Instance status
    #[serde(rename = "status")]
    pub status: WorkflowStatus,

    /// Start timestamp
    #[serde(rename = "started_at")]
    pub started_at: DateTime<Utc>,

    /// Last update timestamp
    #[serde(rename = "updated_at")]
    pub updated_at: DateTime<Utc>,

    /// Completion timestamp
    #[serde(rename = "completed_at", skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<Utc>>,

    /// Execution history
    #[serde(rename = "execution_history")]
    pub execution_history: Vec<WorkflowExecutionEvent>,
}

impl Entity for WorkflowInstance {
    fn entity_type() -> &'static str {
        "workflow_instance"
    }

    fn id(&self) -> &str {
        &self.id
    }

    fn agent(&self) -> &str {
        &self.context.executing_agent
    }

    fn timestamp(&self) -> DateTime<Utc> {
        self.updated_at
    }

    fn validate_entity(&self) -> EntityResult<()> {
        if self.id.is_empty() {
            return Err("Workflow instance ID cannot be empty".to_string());
        }
        if self.workflow_id.is_empty() {
            return Err("Workflow ID cannot be empty".to_string());
        }
        if self.current_state.is_empty() {
            return Err("Current state cannot be empty".to_string());
        }
        if self.context.executing_agent.is_empty() {
            return Err("Executing agent cannot be empty".to_string());
        }
        Ok(())
    }

    fn to_generic(&self) -> GenericEntity {
        GenericEntity {
            id: self.id.clone(),
            entity_type: Self::entity_type().to_string(),
            agent: self.context.executing_agent.clone(),
            timestamp: self.updated_at,
            data: serde_json::to_value(self).unwrap(),
        }
    }

    fn from_generic(entity: GenericEntity) -> EntityResult<Self> {
        serde_json::from_value(entity.data)
            .map_err(|e| format!("Failed to deserialize workflow instance: {}", e))
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

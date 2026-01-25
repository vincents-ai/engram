//! Workflow Instance entity implementation

use super::{Entity, GenericEntity};
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

    fn validate_entity(&self) -> crate::Result<()> {
        if self.id.is_empty() {
            return Err(crate::EngramError::Validation(
                "Workflow instance ID cannot be empty".to_string(),
            ));
        }
        if self.workflow_id.is_empty() {
            return Err(crate::EngramError::Validation(
                "Workflow ID cannot be empty".to_string(),
            ));
        }
        if self.current_state.is_empty() {
            return Err(crate::EngramError::Validation(
                "Current state cannot be empty".to_string(),
            ));
        }
        if self.context.executing_agent.is_empty() {
            return Err(crate::EngramError::Validation(
                "Executing agent cannot be empty".to_string(),
            ));
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

    fn from_generic(entity: GenericEntity) -> crate::Result<Self> {
        serde_json::from_value(entity.data).map_err(|e| {
            crate::EngramError::Deserialization(format!(
                "Failed to deserialize workflow instance: {}",
                e
            ))
        })
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_workflow_instance_creation() {
        let id = "instance-1";
        let workflow_id = "workflow-1";
        let current_state = "start";
        let executing_agent = "agent-1";
        let now = Utc::now();

        let context = WorkflowExecutionContext {
            entity_id: Some(id.to_string()),
            entity_type: Some("workflow_instance".to_string()),
            executing_agent: executing_agent.to_string(),
            variables: HashMap::new(),
            metadata: HashMap::new(),
            permissions: Vec::new(),
        };

        let instance = WorkflowInstance {
            id: id.to_string(),
            workflow_id: workflow_id.to_string(),
            current_state: current_state.to_string(),
            context,
            status: WorkflowStatus::Running,
            started_at: now,
            updated_at: now,
            completed_at: None,
            execution_history: vec![],
        };

        assert_eq!(instance.id, id);
        assert_eq!(instance.workflow_id, workflow_id);
        assert_eq!(instance.current_state, current_state);
        assert_eq!(instance.agent(), executing_agent);
    }

    #[test]
    fn test_workflow_instance_validation() {
        let id = "instance-1";
        let workflow_id = "workflow-1";
        let current_state = "start";
        let executing_agent = "agent-1";
        let now = Utc::now();

        let create_instance = |id: &str, wf_id: &str, state: &str, agent: &str| WorkflowInstance {
            id: id.to_string(),
            workflow_id: wf_id.to_string(),
            current_state: state.to_string(),
            context: WorkflowExecutionContext {
                entity_id: Some(id.to_string()),
                entity_type: Some("workflow_instance".to_string()),
                executing_agent: agent.to_string(),
                variables: HashMap::new(),
                metadata: HashMap::new(),
                permissions: Vec::new(),
            },
            status: WorkflowStatus::Running,
            started_at: now,
            updated_at: now,
            completed_at: None,
            execution_history: vec![],
        };

        // Valid instance
        let instance = create_instance(id, workflow_id, current_state, executing_agent);
        assert!(instance.validate_entity().is_ok());

        // Empty ID
        let instance = create_instance("", workflow_id, current_state, executing_agent);
        assert!(instance.validate_entity().is_err());

        // Empty workflow ID
        let instance = create_instance(id, "", current_state, executing_agent);
        assert!(instance.validate_entity().is_err());

        // Empty state
        let instance = create_instance(id, workflow_id, "", executing_agent);
        assert!(instance.validate_entity().is_err());

        // Empty agent
        let instance = create_instance(id, workflow_id, current_state, "");
        assert!(instance.validate_entity().is_err());
    }
}

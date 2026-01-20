//! Escalation Handler for Sandbox Permission Requests
//!
//! Handles the creation, approval, denial, and timeout of escalation requests
//! when agents need elevated permissions beyond their sandbox level.

use crate::entities::{
    Entity, EscalationOperationType, EscalationPriority, EscalationRequest, EscalationStatus,
    OperationContext, ReviewDecision, ReviewerInfo,
};
use crate::sandbox::{SandboxError, SandboxRequest, SandboxResult};
use crate::storage::Storage;
use chrono::Utc;
use std::collections::HashMap;

/// Handles escalation requests for sandbox operations
pub struct EscalationHandler {
    storage: Box<dyn Storage>,
    /// Cache of recent escalations for performance
    escalation_cache: HashMap<String, EscalationRequest>,
}

impl EscalationHandler {
    /// Create a new escalation handler
    pub fn new(storage: Box<dyn Storage>) -> Self {
        Self {
            storage,
            escalation_cache: HashMap::new(),
        }
    }

    /// Create a new escalation request from a sandbox request
    pub async fn create_escalation(
        &mut self,
        request: &SandboxRequest,
        block_reason: String,
        operation_type: EscalationOperationType,
        priority: EscalationPriority,
    ) -> SandboxResult<String> {
        // Create operation context from the sandbox request
        let operation_context = OperationContext {
            operation: request.operation.clone(),
            parameters: match request.parameters.as_object() {
                Some(obj) => obj.iter().map(|(k, v)| (k.clone(), v.clone())).collect(),
                None => HashMap::new(),
            },
            resource: Some(request.resource_type.clone()),
            block_reason: block_reason.clone(),
            alternatives: self.suggest_alternatives(&request.operation),
            risk_assessment: Some(self.assess_risk(&request.operation)),
        };

        // Count similar past requests for this agent
        let similar_count = self
            .count_similar_escalations(&request.agent_id, &request.operation)
            .await?;

        // Create the escalation request
        let mut escalation = EscalationRequest::new(
            request.agent_id.clone(),
            operation_type,
            operation_context,
            format!(
                "Agent {} requests permission for operation: {}",
                request.agent_id, request.operation
            ),
            priority,
            "default".to_string(), // Agent who created this entity
        );

        escalation.session_id = request.session_id.clone();
        escalation.similar_request_count = similar_count;
        escalation.impact_if_denied = Some(format!(
            "Operation '{}' cannot proceed without approval",
            request.operation
        ));

        // Store the escalation request
        let generic_entity = escalation.to_generic();
        self.storage.store(&generic_entity).map_err(|e| {
            SandboxError::StorageError(format!("Failed to store escalation: {}", e))
        })?;

        // Cache for quick lookup
        let escalation_id = escalation.id.clone();
        self.escalation_cache
            .insert(escalation_id.clone(), escalation);

        Ok(escalation_id)
    }

    /// Approve an escalation request
    pub async fn approve_escalation(
        &mut self,
        escalation_id: &str,
        reviewer: ReviewerInfo,
        reason: String,
        conditions: Vec<String>,
        approval_duration_seconds: Option<u64>,
        create_policy: bool,
    ) -> SandboxResult<()> {
        let mut escalation = self.get_escalation(escalation_id).await?;

        // Ensure request is still pending
        if escalation.status != EscalationStatus::Pending {
            return Err(SandboxError::InvalidConfig(format!(
                "Escalation request {} is not in pending status",
                escalation_id
            )));
        }

        // Check if request has expired
        if Utc::now() > escalation.expires_at {
            escalation.status = EscalationStatus::Expired;
            self.update_escalation(&escalation).await?;
            return Err(SandboxError::EscalationRequired(format!(
                "Escalation request {} has expired",
                escalation_id
            )));
        }

        // Create approval decision
        let decision = ReviewDecision {
            status: EscalationStatus::Approved,
            reason,
            conditions,
            approval_duration: approval_duration_seconds,
            create_policy,
            notes: None,
        };

        // Update escalation
        escalation.status = EscalationStatus::Approved;
        escalation.reviewer = Some(reviewer);
        escalation.decision = Some(decision);
        escalation.reviewed_at = Some(Utc::now());
        escalation.updated_at = Utc::now();

        self.update_escalation(&escalation).await?;

        Ok(())
    }

    /// Deny an escalation request
    pub async fn deny_escalation(
        &mut self,
        escalation_id: &str,
        reviewer: ReviewerInfo,
        reason: String,
        notes: Option<String>,
    ) -> SandboxResult<()> {
        let mut escalation = self.get_escalation(escalation_id).await?;

        // Ensure request is still pending
        if escalation.status != EscalationStatus::Pending {
            return Err(SandboxError::InvalidConfig(format!(
                "Escalation request {} is not in pending status",
                escalation_id
            )));
        }

        // Create denial decision
        let decision = ReviewDecision {
            status: EscalationStatus::Denied,
            reason,
            conditions: Vec::new(),
            approval_duration: None,
            create_policy: false,
            notes,
        };

        // Update escalation
        escalation.status = EscalationStatus::Denied;
        escalation.reviewer = Some(reviewer);
        escalation.decision = Some(decision);
        escalation.reviewed_at = Some(Utc::now());
        escalation.updated_at = Utc::now();

        self.update_escalation(&escalation).await?;

        Ok(())
    }

    /// Cancel an escalation request (by the requesting agent or system)
    pub async fn cancel_escalation(
        &mut self,
        escalation_id: &str,
        reason: Option<String>,
    ) -> SandboxResult<()> {
        let mut escalation = self.get_escalation(escalation_id).await?;

        // Only allow cancellation of pending requests
        if escalation.status != EscalationStatus::Pending {
            return Err(SandboxError::InvalidConfig(format!(
                "Cannot cancel escalation {} with status {:?}",
                escalation_id, escalation.status
            )));
        }

        escalation.status = EscalationStatus::Cancelled;
        escalation.updated_at = Utc::now();

        if let Some(reason_text) = reason {
            escalation
                .metadata
                .insert("cancellation_reason".to_string(), reason_text.into());
        }

        self.update_escalation(&escalation).await?;

        Ok(())
    }

    /// Get an escalation request by ID
    pub async fn get_escalation(
        &mut self,
        escalation_id: &str,
    ) -> SandboxResult<EscalationRequest> {
        // Check cache first
        if let Some(escalation) = self.escalation_cache.get(escalation_id) {
            return Ok(escalation.clone());
        }

        // Load from storage
        let generic_entity = self
            .storage
            .get(escalation_id, "escalation_request")
            .map_err(|e| SandboxError::StorageError(format!("Failed to get escalation: {}", e)))?
            .ok_or_else(|| {
                SandboxError::InvalidConfig(format!(
                    "Escalation request {} not found",
                    escalation_id
                ))
            })?;

        let escalation = EscalationRequest::from_generic(generic_entity).map_err(|e| {
            SandboxError::StorageError(format!("Failed to parse escalation: {}", e))
        })?;

        // Cache for future lookups
        self.escalation_cache
            .insert(escalation_id.to_string(), escalation.clone());

        Ok(escalation)
    }

    /// List all pending escalation requests
    pub async fn list_pending_escalations(&mut self) -> SandboxResult<Vec<EscalationRequest>> {
        self.list_escalations_by_status(EscalationStatus::Pending)
            .await
    }

    /// List escalations by status
    pub async fn list_escalations_by_status(
        &mut self,
        status: EscalationStatus,
    ) -> SandboxResult<Vec<EscalationRequest>> {
        let all_ids = self.storage.list_ids("escalation_request").map_err(|e| {
            SandboxError::StorageError(format!("Failed to list escalations: {}", e))
        })?;

        let mut escalations = Vec::new();

        for id in all_ids {
            if let Ok(escalation) = self.get_escalation(&id).await {
                if escalation.status == status {
                    escalations.push(escalation);
                }
            }
        }

        // Sort by priority (Critical first) and then by creation time
        escalations.sort_by(|a, b| {
            let priority_order = |p: &EscalationPriority| match p {
                EscalationPriority::Critical => 0,
                EscalationPriority::High => 1,
                EscalationPriority::Normal => 2,
                EscalationPriority::Low => 3,
            };

            let a_priority = priority_order(&a.priority);
            let b_priority = priority_order(&b.priority);

            a_priority
                .cmp(&b_priority)
                .then_with(|| a.created_at.cmp(&b.created_at))
        });

        Ok(escalations)
    }

    /// List escalations for a specific agent
    pub async fn list_agent_escalations(
        &mut self,
        agent_id: &str,
    ) -> SandboxResult<Vec<EscalationRequest>> {
        let all_ids = self.storage.list_ids("escalation_request").map_err(|e| {
            SandboxError::StorageError(format!("Failed to list escalations: {}", e))
        })?;

        let mut escalations = Vec::new();

        for id in all_ids {
            if let Ok(escalation) = self.get_escalation(&id).await {
                if escalation.agent_id == agent_id {
                    escalations.push(escalation);
                }
            }
        }

        // Sort by creation time (newest first)
        escalations.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        Ok(escalations)
    }

    /// Check for expired escalation requests and mark them as expired
    pub async fn process_expired_escalations(&mut self) -> SandboxResult<usize> {
        let pending = self.list_pending_escalations().await?;
        let now = Utc::now();
        let mut expired_count = 0;

        for mut escalation in pending {
            if now > escalation.expires_at {
                escalation.status = EscalationStatus::Expired;
                escalation.updated_at = now;
                self.update_escalation(&escalation).await?;
                expired_count += 1;
            }
        }

        Ok(expired_count)
    }

    /// Get statistics about escalation requests
    pub async fn get_statistics(&mut self) -> SandboxResult<EscalationStatistics> {
        let all_ids = self.storage.list_ids("escalation_request").map_err(|e| {
            SandboxError::StorageError(format!("Failed to list escalations: {}", e))
        })?;

        let mut stats = EscalationStatistics::default();
        stats.total_requests = all_ids.len();

        for id in all_ids {
            if let Ok(escalation) = self.get_escalation(&id).await {
                match escalation.status {
                    EscalationStatus::Pending => stats.pending_count += 1,
                    EscalationStatus::Approved => {
                        stats.approved_count += 1;
                        if let Some(decision) = &escalation.decision {
                            if let Some(duration) = decision.approval_duration {
                                stats.total_approval_duration_seconds += duration;
                            }
                        }
                    }
                    EscalationStatus::Denied => stats.denied_count += 1,
                    EscalationStatus::Expired => stats.expired_count += 1,
                    EscalationStatus::Cancelled => stats.cancelled_count += 1,
                }

                // Calculate average response time for reviewed requests
                if let Some(reviewed_at) = escalation.reviewed_at {
                    let response_time = (reviewed_at - escalation.created_at).num_seconds();
                    stats.total_response_time_seconds += response_time as u64;
                    stats.reviewed_count += 1;
                }
            }
        }

        // Calculate averages
        if stats.reviewed_count > 0 {
            stats.average_response_time_seconds =
                stats.total_response_time_seconds / stats.reviewed_count as u64;
        }

        if stats.approved_count > 0 {
            stats.average_approval_duration_seconds =
                stats.total_approval_duration_seconds / stats.approved_count as u64;
        }

        Ok(stats)
    }

    /// Check if an agent has an active approval for a specific operation
    pub async fn has_active_approval(
        &mut self,
        agent_id: &str,
        operation: &str,
    ) -> SandboxResult<Option<EscalationRequest>> {
        let agent_escalations = self.list_agent_escalations(agent_id).await?;
        let now = Utc::now();

        for escalation in agent_escalations {
            if escalation.status == EscalationStatus::Approved
                && escalation.operation_context.operation == operation
            {
                // Check if approval is still valid
                if let Some(decision) = &escalation.decision {
                    if let Some(duration_seconds) = decision.approval_duration {
                        if let Some(reviewed_at) = escalation.reviewed_at {
                            let expires_at =
                                reviewed_at + chrono::Duration::seconds(duration_seconds as i64);
                            if now < expires_at {
                                return Ok(Some(escalation));
                            }
                        }
                    } else {
                        // Approval has no expiration
                        return Ok(Some(escalation));
                    }
                }
            }
        }

        Ok(None)
    }

    /// Helper: Update an escalation in storage and cache
    async fn update_escalation(&mut self, escalation: &EscalationRequest) -> SandboxResult<()> {
        let generic_entity = escalation.to_generic();
        self.storage.store(&generic_entity).map_err(|e| {
            SandboxError::StorageError(format!("Failed to update escalation: {}", e))
        })?;

        // Update cache
        self.escalation_cache
            .insert(escalation.id.clone(), escalation.clone());

        Ok(())
    }

    /// Helper: Count similar escalations for an agent
    async fn count_similar_escalations(
        &mut self,
        agent_id: &str,
        operation: &str,
    ) -> SandboxResult<u32> {
        let agent_escalations = self.list_agent_escalations(agent_id).await?;

        Ok(agent_escalations
            .iter()
            .filter(|e| e.operation_context.operation == operation)
            .count() as u32)
    }

    /// Helper: Suggest alternative operations
    fn suggest_alternatives(&self, operation: &str) -> Vec<String> {
        match operation {
            "file_delete" => vec![
                "Use file_move to archive instead".to_string(),
                "Request permission for specific file patterns".to_string(),
            ],
            "network_request" => vec![
                "Use internal API endpoints if available".to_string(),
                "Request proxy configuration".to_string(),
            ],
            "execute_command" => vec![
                "Use engram built-in operations".to_string(),
                "Request specific command whitelist".to_string(),
            ],
            _ => vec!["Contact administrator for guidance".to_string()],
        }
    }

    /// Helper: Assess risk of an operation
    fn assess_risk(&self, operation: &str) -> String {
        match operation {
            op if op.contains("delete") || op.contains("remove") => {
                "High risk: Irreversible data modification".to_string()
            }
            op if op.contains("network") => {
                "Medium risk: External communication and data exfiltration potential".to_string()
            }
            op if op.contains("execute") || op.contains("command") => {
                "High risk: Arbitrary code execution".to_string()
            }
            op if op.contains("write") || op.contains("modify") => {
                "Medium risk: Data modification".to_string()
            }
            _ => "Low risk: Read-only or safe operation".to_string(),
        }
    }

    /// Clear the escalation cache
    pub fn clear_cache(&mut self) {
        self.escalation_cache.clear();
    }
}

/// Statistics about escalation requests
#[derive(Debug, Clone, Default)]
pub struct EscalationStatistics {
    pub total_requests: usize,
    pub pending_count: usize,
    pub approved_count: usize,
    pub denied_count: usize,
    pub expired_count: usize,
    pub cancelled_count: usize,
    pub reviewed_count: usize,
    pub total_response_time_seconds: u64,
    pub average_response_time_seconds: u64,
    pub total_approval_duration_seconds: u64,
    pub average_approval_duration_seconds: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::GitStorage;
    use tempfile::TempDir;

    fn create_test_storage() -> (Box<dyn Storage>, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let storage =
            Box::new(GitStorage::new(temp_dir.path().to_str().unwrap(), "test-agent").unwrap());
        (storage, temp_dir)
    }

    #[tokio::test]
    async fn test_create_escalation() {
        let (storage, _temp_dir) = create_test_storage();
        let mut handler = EscalationHandler::new(storage);

        let request = SandboxRequest {
            agent_id: "test-agent".to_string(),
            operation: "file_delete".to_string(),
            resource_type: "/important/file.txt".to_string(),
            parameters: serde_json::json!({}),
            timestamp: Utc::now(),
            session_id: Some("session-123".to_string()),
        };

        let escalation_id = handler
            .create_escalation(
                &request,
                "File deletion not permitted".to_string(),
                EscalationOperationType::FileSystemAccess,
                EscalationPriority::Normal,
            )
            .await
            .unwrap();

        assert!(!escalation_id.is_empty());

        // Verify escalation was created
        let escalation = handler.get_escalation(&escalation_id).await.unwrap();
        assert_eq!(escalation.agent_id, "test-agent");
        assert_eq!(escalation.status, EscalationStatus::Pending);
        assert_eq!(escalation.operation_context.operation, "file_delete");
    }

    #[tokio::test]
    async fn test_approve_escalation() {
        let (storage, _temp_dir) = create_test_storage();
        let mut handler = EscalationHandler::new(storage);

        let request = SandboxRequest {
            agent_id: "test-agent".to_string(),
            operation: "network_request".to_string(),
            resource_type: "https://api.example.com".to_string(),
            parameters: serde_json::json!({}),
            timestamp: Utc::now(),
            session_id: None,
        };

        let escalation_id = handler
            .create_escalation(
                &request,
                "External network not allowed".to_string(),
                EscalationOperationType::NetworkAccess,
                EscalationPriority::High,
            )
            .await
            .unwrap();

        let reviewer = ReviewerInfo {
            reviewer_id: "reviewer-1".to_string(),
            reviewer_name: "John Reviewer".to_string(),
            reviewer_email: Some("john@example.com".to_string()),
            department: Some("Security".to_string()),
        };

        handler
            .approve_escalation(
                &escalation_id,
                reviewer,
                "Approved for testing purposes".to_string(),
                vec!["Monitor network traffic".to_string()],
                Some(3600), // 1 hour
                false,
            )
            .await
            .unwrap();

        let escalation = handler.get_escalation(&escalation_id).await.unwrap();
        assert_eq!(escalation.status, EscalationStatus::Approved);
        assert!(escalation.reviewer.is_some());
        assert!(escalation.decision.is_some());
    }

    #[tokio::test]
    async fn test_deny_escalation() {
        let (storage, _temp_dir) = create_test_storage();
        let mut handler = EscalationHandler::new(storage);

        let request = SandboxRequest {
            agent_id: "test-agent".to_string(),
            operation: "execute_command".to_string(),
            resource_type: "rm -rf /".to_string(),
            parameters: serde_json::json!({}),
            timestamp: Utc::now(),
            session_id: None,
        };

        let escalation_id = handler
            .create_escalation(
                &request,
                "Dangerous command blocked".to_string(),
                EscalationOperationType::CommandExecution,
                EscalationPriority::Critical,
            )
            .await
            .unwrap();

        let reviewer = ReviewerInfo {
            reviewer_id: "reviewer-1".to_string(),
            reviewer_name: "John Reviewer".to_string(),
            reviewer_email: Some("john@example.com".to_string()),
            department: Some("Security".to_string()),
        };

        handler
            .deny_escalation(
                &escalation_id,
                reviewer,
                "Command is too dangerous to approve".to_string(),
                Some("This operation would destroy the system".to_string()),
            )
            .await
            .unwrap();

        let escalation = handler.get_escalation(&escalation_id).await.unwrap();
        assert_eq!(escalation.status, EscalationStatus::Denied);
        assert!(escalation.decision.is_some());
    }
}

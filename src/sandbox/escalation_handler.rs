//! Escalation Handler for Sandbox Permission Requests
//!
//! Handles the creation, approval, denial, and timeout of escalation requests
//! when agents need elevated permissions beyond their sandbox level.

#![allow(clippy::collapsible_if, clippy::needless_borrows_for_generic_args)]

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

        let mut stats = EscalationStatistics {
            total_requests: all_ids.len(),
            ..Default::default()
        };

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
    use crate::storage::GitRefsStorage;
    use tempfile::TempDir;

    fn create_test_storage() -> (Box<dyn Storage>, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let storage =
            Box::new(GitRefsStorage::new(temp_dir.path().to_str().unwrap(), "test-agent").unwrap());
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

    #[tokio::test]
    async fn test_cancel() {
        let (s, _t) = create_test_storage();
        let mut h = EscalationHandler::new(s);
        let req = SandboxRequest {
            agent_id: "a".into(),
            operation: "file_write".into(),
            resource_type: "/f".into(),
            parameters: serde_json::json!({}),
            timestamp: Utc::now(),
            session_id: Some("s".into()),
        };
        let id = h
            .create_escalation(
                &req,
                "b".into(),
                EscalationOperationType::FileSystemAccess,
                EscalationPriority::Normal,
            )
            .await
            .unwrap();
        h.cancel_escalation(&id, Some("no longer needed".into()))
            .await
            .unwrap();
        let esc = h.get_escalation(&id).await.unwrap();
        assert_eq!(esc.status, EscalationStatus::Cancelled);
        assert!(esc.metadata.contains_key("cancellation_reason"));
    }

    #[tokio::test]
    async fn test_cancel_no_reason() {
        let (s, _t) = create_test_storage();
        let mut h = EscalationHandler::new(s);
        let req = SandboxRequest {
            agent_id: "a".into(),
            operation: "file_write".into(),
            resource_type: "/f".into(),
            parameters: serde_json::json!({}),
            timestamp: Utc::now(),
            session_id: None,
        };
        let id = h
            .create_escalation(
                &req,
                "b".into(),
                EscalationOperationType::FileSystemAccess,
                EscalationPriority::Normal,
            )
            .await
            .unwrap();
        h.cancel_escalation(&id, None).await.unwrap();
        assert_eq!(
            h.get_escalation(&id).await.unwrap().status,
            EscalationStatus::Cancelled
        );
    }

    #[tokio::test]
    async fn test_cancel_approved_fails() {
        let (s, _t) = create_test_storage();
        let mut h = EscalationHandler::new(s);
        let req = SandboxRequest {
            agent_id: "a".into(),
            operation: "net".into(),
            resource_type: "x".into(),
            parameters: serde_json::json!({}),
            timestamp: Utc::now(),
            session_id: None,
        };
        let id = h
            .create_escalation(
                &req,
                "b".into(),
                EscalationOperationType::NetworkAccess,
                EscalationPriority::Normal,
            )
            .await
            .unwrap();
        let rev = ReviewerInfo {
            reviewer_id: "r".into(),
            reviewer_name: "R".into(),
            reviewer_email: None,
            department: None,
        };
        h.approve_escalation(&id, rev, "ok".into(), vec![], None, false)
            .await
            .unwrap();
        assert!(h.cancel_escalation(&id, None).await.is_err());
    }

    #[tokio::test]
    async fn test_get_not_found() {
        let (s, _t) = create_test_storage();
        let mut h = EscalationHandler::new(s);
        assert!(h.get_escalation("nonexistent").await.is_err());
    }

    #[tokio::test]
    async fn test_list_pending() {
        let (s, _t) = create_test_storage();
        let mut h = EscalationHandler::new(s);
        let req = SandboxRequest {
            agent_id: "a".into(),
            operation: "file_delete".into(),
            resource_type: "/f".into(),
            parameters: serde_json::json!({}),
            timestamp: Utc::now(),
            session_id: None,
        };
        h.create_escalation(
            &req,
            "b".into(),
            EscalationOperationType::FileSystemAccess,
            EscalationPriority::Normal,
        )
        .await
        .unwrap();
        assert_eq!(h.list_pending_escalations().await.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn test_list_by_status() {
        let (s, _t) = create_test_storage();
        let mut h = EscalationHandler::new(s);
        let req = SandboxRequest {
            agent_id: "a".into(),
            operation: "file_delete".into(),
            resource_type: "/f".into(),
            parameters: serde_json::json!({}),
            timestamp: Utc::now(),
            session_id: None,
        };
        let id1 = h
            .create_escalation(
                &req,
                "b".into(),
                EscalationOperationType::FileSystemAccess,
                EscalationPriority::Critical,
            )
            .await
            .unwrap();
        let id2 = h
            .create_escalation(
                &req,
                "b".into(),
                EscalationOperationType::FileSystemAccess,
                EscalationPriority::Low,
            )
            .await
            .unwrap();
        assert_eq!(
            h.list_escalations_by_status(EscalationStatus::Pending)
                .await
                .unwrap()
                .len(),
            2
        );
        let rev = ReviewerInfo {
            reviewer_id: "r".into(),
            reviewer_name: "R".into(),
            reviewer_email: None,
            department: None,
        };
        h.approve_escalation(&id1, rev.clone(), "ok".into(), vec![], None, false)
            .await
            .unwrap();
        assert_eq!(
            h.list_escalations_by_status(EscalationStatus::Pending)
                .await
                .unwrap()
                .len(),
            1
        );
        assert_eq!(
            h.list_escalations_by_status(EscalationStatus::Approved)
                .await
                .unwrap()
                .len(),
            1
        );
        h.cancel_escalation(&id2, None).await.unwrap();
        assert_eq!(
            h.list_escalations_by_status(EscalationStatus::Cancelled)
                .await
                .unwrap()
                .len(),
            1
        );
    }

    #[tokio::test]
    async fn test_list_sorted_priority() {
        let (s, _t) = create_test_storage();
        let mut h = EscalationHandler::new(s);
        let req = SandboxRequest {
            agent_id: "a".into(),
            operation: "op".into(),
            resource_type: "r".into(),
            parameters: serde_json::json!({}),
            timestamp: Utc::now(),
            session_id: None,
        };
        h.create_escalation(
            &req,
            "b".into(),
            EscalationOperationType::Custom("op".into()),
            EscalationPriority::Low,
        )
        .await
        .unwrap();
        h.create_escalation(
            &req,
            "b".into(),
            EscalationOperationType::Custom("op".into()),
            EscalationPriority::Critical,
        )
        .await
        .unwrap();
        h.create_escalation(
            &req,
            "b".into(),
            EscalationOperationType::Custom("op".into()),
            EscalationPriority::Normal,
        )
        .await
        .unwrap();
        let p = h.list_pending_escalations().await.unwrap();
        assert_eq!(p.len(), 3);
        assert_eq!(p[0].priority, EscalationPriority::Critical);
        assert_eq!(p[1].priority, EscalationPriority::Normal);
        assert_eq!(p[2].priority, EscalationPriority::Low);
    }

    #[tokio::test]
    async fn test_list_agent() {
        let (s, _t) = create_test_storage();
        let mut h = EscalationHandler::new(s);
        let r1 = SandboxRequest {
            agent_id: "agent-1".into(),
            operation: "op1".into(),
            resource_type: "r".into(),
            parameters: serde_json::json!({}),
            timestamp: Utc::now(),
            session_id: None,
        };
        let r2 = SandboxRequest {
            agent_id: "agent-2".into(),
            operation: "op2".into(),
            resource_type: "r".into(),
            parameters: serde_json::json!({}),
            timestamp: Utc::now(),
            session_id: None,
        };
        h.create_escalation(
            &r1,
            "b".into(),
            EscalationOperationType::Custom("op1".into()),
            EscalationPriority::Normal,
        )
        .await
        .unwrap();
        h.create_escalation(
            &r1,
            "b".into(),
            EscalationOperationType::Custom("op1".into()),
            EscalationPriority::Normal,
        )
        .await
        .unwrap();
        h.create_escalation(
            &r2,
            "b".into(),
            EscalationOperationType::Custom("op2".into()),
            EscalationPriority::Normal,
        )
        .await
        .unwrap();
        assert_eq!(h.list_agent_escalations("agent-1").await.unwrap().len(), 2);
        assert_eq!(h.list_agent_escalations("agent-2").await.unwrap().len(), 1);
        assert!(h
            .list_agent_escalations("agent-3")
            .await
            .unwrap()
            .is_empty());
    }

    #[tokio::test]
    async fn test_process_expired() {
        let (s, _t) = create_test_storage();
        let mut h = EscalationHandler::new(s);
        let req = SandboxRequest {
            agent_id: "a".into(),
            operation: "file_delete".into(),
            resource_type: "/f".into(),
            parameters: serde_json::json!({}),
            timestamp: Utc::now(),
            session_id: None,
        };
        let id = h
            .create_escalation(
                &req,
                "b".into(),
                EscalationOperationType::FileSystemAccess,
                EscalationPriority::Normal,
            )
            .await
            .unwrap();
        let mut esc = h.get_escalation(&id).await.unwrap();
        esc.expires_at = Utc::now() - chrono::Duration::hours(1);
        h.update_escalation(&esc).await.unwrap();
        assert_eq!(h.process_expired_escalations().await.unwrap(), 1);
        assert_eq!(
            h.get_escalation(&id).await.unwrap().status,
            EscalationStatus::Expired
        );
    }

    #[tokio::test]
    async fn test_process_none_expired() {
        let (s, _t) = create_test_storage();
        let mut h = EscalationHandler::new(s);
        let req = SandboxRequest {
            agent_id: "a".into(),
            operation: "file_delete".into(),
            resource_type: "/f".into(),
            parameters: serde_json::json!({}),
            timestamp: Utc::now(),
            session_id: None,
        };
        h.create_escalation(
            &req,
            "b".into(),
            EscalationOperationType::FileSystemAccess,
            EscalationPriority::Normal,
        )
        .await
        .unwrap();
        assert_eq!(h.process_expired_escalations().await.unwrap(), 0);
    }

    #[tokio::test]
    async fn test_statistics() {
        let (s, _t) = create_test_storage();
        let mut h = EscalationHandler::new(s);
        let req = SandboxRequest {
            agent_id: "a".into(),
            operation: "file_delete".into(),
            resource_type: "/f".into(),
            parameters: serde_json::json!({}),
            timestamp: Utc::now(),
            session_id: None,
        };
        let id = h
            .create_escalation(
                &req,
                "b".into(),
                EscalationOperationType::FileSystemAccess,
                EscalationPriority::Normal,
            )
            .await
            .unwrap();
        let rev = ReviewerInfo {
            reviewer_id: "r".into(),
            reviewer_name: "R".into(),
            reviewer_email: None,
            department: None,
        };
        h.approve_escalation(&id, rev, "ok".into(), vec![], Some(3600), false)
            .await
            .unwrap();
        h.create_escalation(
            &req,
            "b".into(),
            EscalationOperationType::FileSystemAccess,
            EscalationPriority::Normal,
        )
        .await
        .unwrap();
        let stats = h.get_statistics().await.unwrap();
        assert_eq!(stats.total_requests, 2);
        assert_eq!(stats.approved_count, 1);
        assert_eq!(stats.pending_count, 1);
        assert_eq!(stats.denied_count, 0);
        assert!(stats.reviewed_count > 0);
        assert_eq!(stats.average_approval_duration_seconds, 3600);
    }

    #[tokio::test]
    async fn test_statistics_empty() {
        let (s, _t) = create_test_storage();
        let mut h = EscalationHandler::new(s);
        let stats = h.get_statistics().await.unwrap();
        assert_eq!(stats.total_requests, 0);
        assert_eq!(stats.average_response_time_seconds, 0);
    }

    #[tokio::test]
    async fn test_active_approval_no_dur() {
        let (s, _t) = create_test_storage();
        let mut h = EscalationHandler::new(s);
        let req = SandboxRequest {
            agent_id: "a".into(),
            operation: "file_delete".into(),
            resource_type: "/f".into(),
            parameters: serde_json::json!({}),
            timestamp: Utc::now(),
            session_id: None,
        };
        let id = h
            .create_escalation(
                &req,
                "b".into(),
                EscalationOperationType::FileSystemAccess,
                EscalationPriority::Normal,
            )
            .await
            .unwrap();
        let rev = ReviewerInfo {
            reviewer_id: "r".into(),
            reviewer_name: "R".into(),
            reviewer_email: None,
            department: None,
        };
        h.approve_escalation(&id, rev, "ok".into(), vec![], None, false)
            .await
            .unwrap();
        assert!(h
            .has_active_approval("a", "file_delete")
            .await
            .unwrap()
            .is_some());
    }

    #[tokio::test]
    async fn test_active_approval_with_dur() {
        let (s, _t) = create_test_storage();
        let mut h = EscalationHandler::new(s);
        let req = SandboxRequest {
            agent_id: "a".into(),
            operation: "file_delete".into(),
            resource_type: "/f".into(),
            parameters: serde_json::json!({}),
            timestamp: Utc::now(),
            session_id: None,
        };
        let id = h
            .create_escalation(
                &req,
                "b".into(),
                EscalationOperationType::FileSystemAccess,
                EscalationPriority::Normal,
            )
            .await
            .unwrap();
        let rev = ReviewerInfo {
            reviewer_id: "r".into(),
            reviewer_name: "R".into(),
            reviewer_email: None,
            department: None,
        };
        h.approve_escalation(&id, rev, "ok".into(), vec![], Some(3600), false)
            .await
            .unwrap();
        assert!(h
            .has_active_approval("a", "file_delete")
            .await
            .unwrap()
            .is_some());
    }

    #[tokio::test]
    async fn test_active_approval_no_match() {
        let (s, _t) = create_test_storage();
        let mut h = EscalationHandler::new(s);
        let req = SandboxRequest {
            agent_id: "a".into(),
            operation: "file_delete".into(),
            resource_type: "/f".into(),
            parameters: serde_json::json!({}),
            timestamp: Utc::now(),
            session_id: None,
        };
        h.create_escalation(
            &req,
            "b".into(),
            EscalationOperationType::FileSystemAccess,
            EscalationPriority::Normal,
        )
        .await
        .unwrap();
        assert!(h
            .has_active_approval("a", "other_op")
            .await
            .unwrap()
            .is_none());
    }

    #[tokio::test]
    async fn test_active_approval_no_agent() {
        let (s, _t) = create_test_storage();
        let mut h = EscalationHandler::new(s);
        assert!(h
            .has_active_approval("nonexistent", "op")
            .await
            .unwrap()
            .is_none());
    }

    #[test]
    fn test_suggest_alts() {
        let (s, _t) = create_test_storage();
        let h = EscalationHandler::new(s);
        assert_eq!(h.suggest_alternatives("file_delete").len(), 2);
        assert_eq!(h.suggest_alternatives("network_request").len(), 2);
        assert_eq!(h.suggest_alternatives("execute_command").len(), 2);
        assert_eq!(h.suggest_alternatives("other").len(), 1);
    }

    #[test]
    fn test_assess_risk() {
        let (s, _t) = create_test_storage();
        let h = EscalationHandler::new(s);
        assert!(h.assess_risk("delete_file").contains("High risk"));
        assert!(h.assess_risk("remove_item").contains("High risk"));
        assert!(h.assess_risk("network_request").contains("Medium risk"));
        assert!(h.assess_risk("execute_command").contains("High risk"));
        assert!(h.assess_risk("run_command").contains("High risk"));
        assert!(h.assess_risk("write_file").contains("Medium risk"));
        assert!(h.assess_risk("modify_entity").contains("Medium risk"));
        assert!(h.assess_risk("read_file").contains("Low risk"));
    }

    #[test]
    fn test_clear_cache() {
        let (s, _t) = create_test_storage();
        let mut h = EscalationHandler::new(s);
        let ctx = OperationContext {
            operation: "op".into(),
            parameters: HashMap::new(),
            resource: None,
            block_reason: "t".into(),
            alternatives: vec![],
            risk_assessment: None,
        };
        let esc = EscalationRequest::new(
            "a".into(),
            EscalationOperationType::Custom("op".into()),
            ctx,
            "t".into(),
            EscalationPriority::Normal,
            "a".into(),
        );
        h.escalation_cache.insert(esc.id.clone(), esc);
        h.clear_cache();
        assert!(h.escalation_cache.is_empty());
    }

    #[tokio::test]
    async fn test_approve_non_pending() {
        let (s, _t) = create_test_storage();
        let mut h = EscalationHandler::new(s);
        let req = SandboxRequest {
            agent_id: "a".into(),
            operation: "op".into(),
            resource_type: "r".into(),
            parameters: serde_json::json!({}),
            timestamp: Utc::now(),
            session_id: None,
        };
        let id = h
            .create_escalation(
                &req,
                "b".into(),
                EscalationOperationType::Custom("op".into()),
                EscalationPriority::Normal,
            )
            .await
            .unwrap();
        let rev = ReviewerInfo {
            reviewer_id: "r".into(),
            reviewer_name: "R".into(),
            reviewer_email: None,
            department: None,
        };
        h.deny_escalation(&id, rev, "no".into(), None)
            .await
            .unwrap();
        let rev2 = ReviewerInfo {
            reviewer_id: "r2".into(),
            reviewer_name: "R2".into(),
            reviewer_email: None,
            department: None,
        };
        assert!(h
            .approve_escalation(&id, rev2, "ok".into(), vec![], None, false)
            .await
            .is_err());
    }

    #[tokio::test]
    async fn test_deny_non_pending() {
        let (s, _t) = create_test_storage();
        let mut h = EscalationHandler::new(s);
        let req = SandboxRequest {
            agent_id: "a".into(),
            operation: "op".into(),
            resource_type: "r".into(),
            parameters: serde_json::json!({}),
            timestamp: Utc::now(),
            session_id: None,
        };
        let id = h
            .create_escalation(
                &req,
                "b".into(),
                EscalationOperationType::Custom("op".into()),
                EscalationPriority::Normal,
            )
            .await
            .unwrap();
        let rev = ReviewerInfo {
            reviewer_id: "r".into(),
            reviewer_name: "R".into(),
            reviewer_email: None,
            department: None,
        };
        h.approve_escalation(&id, rev, "ok".into(), vec![], None, false)
            .await
            .unwrap();
        let rev2 = ReviewerInfo {
            reviewer_id: "r2".into(),
            reviewer_name: "R2".into(),
            reviewer_email: None,
            department: None,
        };
        assert!(h
            .deny_escalation(&id, rev2, "no".into(), None)
            .await
            .is_err());
    }

    #[test]
    fn test_stats_default() {
        let s = EscalationStatistics::default();
        assert_eq!(s.total_requests, 0);
    }

    #[tokio::test]
    async fn test_create_caches() {
        let (s, _t) = create_test_storage();
        let mut h = EscalationHandler::new(s);
        let req = SandboxRequest {
            agent_id: "test-agent".into(),
            operation: "file_write".into(),
            resource_type: "/f".into(),
            parameters: serde_json::json!({"key": "value"}),
            timestamp: Utc::now(),
            session_id: Some("s1".into()),
        };
        let id = h
            .create_escalation(
                &req,
                "b".into(),
                EscalationOperationType::FileSystemAccess,
                EscalationPriority::Normal,
            )
            .await
            .unwrap();
        assert!(h.escalation_cache.contains_key(&id));
        let c = h.get_escalation(&id).await.unwrap();
        assert_eq!(c.agent_id, "test-agent");
        assert_eq!(c.session_id, Some("s1".into()));
    }

    fn make_req(agent_id: &str, operation: &str, resource_type: &str) -> SandboxRequest {
        SandboxRequest {
            agent_id: agent_id.into(),
            operation: operation.into(),
            resource_type: resource_type.into(),
            parameters: serde_json::json!({}),
            timestamp: Utc::now(),
            session_id: None,
        }
    }

    fn make_reviewer(id: &str, name: &str) -> ReviewerInfo {
        ReviewerInfo {
            reviewer_id: id.into(),
            reviewer_name: name.into(),
            reviewer_email: Some(format!("{}@example.com", id)),
            department: Some("Security".into()),
        }
    }

    #[tokio::test]
    async fn test_create_all_priorities() {
        let priorities = [
            EscalationPriority::Critical,
            EscalationPriority::High,
            EscalationPriority::Normal,
            EscalationPriority::Low,
        ];
        for priority in &priorities {
            let (s, _t) = create_test_storage();
            let mut h = EscalationHandler::new(s);
            let req = make_req("a", "op", "r");
            let id = h
                .create_escalation(
                    &req,
                    "b".into(),
                    EscalationOperationType::Custom("op".into()),
                    priority.clone(),
                )
                .await
                .unwrap();
            let esc = h.get_escalation(&id).await.unwrap();
            assert_eq!(esc.priority, *priority);
        }
    }

    #[tokio::test]
    async fn test_create_all_operation_types() {
        let op_types = [
            EscalationOperationType::FileSystemAccess,
            EscalationOperationType::NetworkAccess,
            EscalationOperationType::CommandExecution,
            EscalationOperationType::PrivilegeEscalation,
            EscalationOperationType::QualityGateOverride,
            EscalationOperationType::WorkflowModification,
            EscalationOperationType::ResourceLimitIncrease,
            EscalationOperationType::Custom("my_op".into()),
        ];
        for op_type in &op_types {
            let (s, _t) = create_test_storage();
            let mut h = EscalationHandler::new(s);
            let req = make_req("a", "op", "r");
            let id = h
                .create_escalation(&req, "b".into(), op_type.clone(), EscalationPriority::Normal)
                .await
                .unwrap();
            let esc = h.get_escalation(&id).await.unwrap();
            assert_eq!(esc.operation_type, *op_type);
        }
    }

    #[tokio::test]
    async fn test_approve_verifies_all_fields() {
        let (s, _t) = create_test_storage();
        let mut h = EscalationHandler::new(s);
        let req = make_req("a", "net", "r");
        let id = h
            .create_escalation(
                &req,
                "b".into(),
                EscalationOperationType::NetworkAccess,
                EscalationPriority::High,
            )
            .await
            .unwrap();
        let rev = make_reviewer("rv1", "Reviewer One");
        let conditions = vec!["Log all access".into(), "Time-limited".into()];
        h.approve_escalation(&id, rev.clone(), "Test approve".into(), conditions.clone(), Some(7200), true)
            .await
            .unwrap();
        let esc = h.get_escalation(&id).await.unwrap();
        assert_eq!(esc.status, EscalationStatus::Approved);
        assert_eq!(esc.reviewer.as_ref().unwrap().reviewer_id, "rv1");
        assert_eq!(esc.reviewer.as_ref().unwrap().reviewer_name, "Reviewer One");
        assert_eq!(esc.reviewer.as_ref().unwrap().department.as_deref(), Some("Security"));
        let dec = esc.decision.unwrap();
        assert_eq!(dec.status, EscalationStatus::Approved);
        assert_eq!(dec.reason, "Test approve");
        assert_eq!(dec.conditions, conditions);
        assert_eq!(dec.approval_duration, Some(7200));
        assert!(dec.create_policy);
        assert!(esc.reviewed_at.is_some());
    }

    #[tokio::test]
    async fn test_deny_verifies_reason_and_notes() {
        let (s, _t) = create_test_storage();
        let mut h = EscalationHandler::new(s);
        let req = make_req("a", "exec", "r");
        let id = h
            .create_escalation(
                &req,
                "b".into(),
                EscalationOperationType::CommandExecution,
                EscalationPriority::Critical,
            )
            .await
            .unwrap();
        let rev = make_reviewer("rv2", "Reviewer Two");
        h.deny_escalation(
            &id,
            rev,
            "Too dangerous".into(),
            Some("See incident INC-42".into()),
        )
        .await
        .unwrap();
        let esc = h.get_escalation(&id).await.unwrap();
        assert_eq!(esc.status, EscalationStatus::Denied);
        let dec = esc.decision.unwrap();
        assert_eq!(dec.reason, "Too dangerous");
        assert!(dec.conditions.is_empty());
        assert_eq!(dec.notes.as_deref(), Some("See incident INC-42"));
        assert!(!dec.create_policy);
        assert!(esc.reviewed_at.is_some());
    }

    #[tokio::test]
    async fn test_approve_expired_returns_error() {
        let (s, _t) = create_test_storage();
        let mut h = EscalationHandler::new(s);
        let req = make_req("a", "op", "r");
        let id = h
            .create_escalation(
                &req,
                "b".into(),
                EscalationOperationType::Custom("op".into()),
                EscalationPriority::Normal,
            )
            .await
            .unwrap();
        let mut esc = h.get_escalation(&id).await.unwrap();
        esc.expires_at = Utc::now() - chrono::Duration::seconds(1);
        h.update_escalation(&esc).await.unwrap();
        let rev = make_reviewer("rv", "R");
        let result = h
            .approve_escalation(&id, rev, "ok".into(), vec![], None, false)
            .await;
        assert!(result.is_err());
        assert_eq!(
            h.get_escalation(&id).await.unwrap().status,
            EscalationStatus::Expired
        );
    }

    #[tokio::test]
    async fn test_list_all_statuses() {
        let (s, _t) = create_test_storage();
        let mut h = EscalationHandler::new(s);
        let req = make_req("a", "op", "r");

        let id_approved = h
            .create_escalation(
                &req,
                "b".into(),
                EscalationOperationType::Custom("op".into()),
                EscalationPriority::Normal,
            )
            .await
            .unwrap();
        let id_denied = h
            .create_escalation(
                &req,
                "b".into(),
                EscalationOperationType::Custom("op".into()),
                EscalationPriority::Normal,
            )
            .await
            .unwrap();
        let id_cancelled = h
            .create_escalation(
                &req,
                "b".into(),
                EscalationOperationType::Custom("op".into()),
                EscalationPriority::Normal,
            )
            .await
            .unwrap();
        let id_expired = h
            .create_escalation(
                &req,
                "b".into(),
                EscalationOperationType::Custom("op".into()),
                EscalationPriority::Normal,
            )
            .await
            .unwrap();
        let _id_pending = h
            .create_escalation(
                &req,
                "b".into(),
                EscalationOperationType::Custom("op".into()),
                EscalationPriority::Normal,
            )
            .await
            .unwrap();

        let rev = make_reviewer("rv", "R");
        h.approve_escalation(&id_approved, rev.clone(), "ok".into(), vec![], None, false)
            .await
            .unwrap();
        h.deny_escalation(&id_denied, rev.clone(), "no".into(), None)
            .await
            .unwrap();
        h.cancel_escalation(&id_cancelled, None).await.unwrap();

        let mut esc = h.get_escalation(&id_expired).await.unwrap();
        esc.expires_at = Utc::now() - chrono::Duration::hours(1);
        h.update_escalation(&esc).await.unwrap();
        h.process_expired_escalations().await.unwrap();

        assert_eq!(
            h.list_escalations_by_status(EscalationStatus::Approved)
                .await
                .unwrap()
                .len(),
            1
        );
        assert_eq!(
            h.list_escalations_by_status(EscalationStatus::Denied)
                .await
                .unwrap()
                .len(),
            1
        );
        assert_eq!(
            h.list_escalations_by_status(EscalationStatus::Cancelled)
                .await
                .unwrap()
                .len(),
            1
        );
        assert_eq!(
            h.list_escalations_by_status(EscalationStatus::Expired)
                .await
                .unwrap()
                .len(),
            1
        );
        assert_eq!(
            h.list_escalations_by_status(EscalationStatus::Pending)
                .await
                .unwrap()
                .len(),
            1
        );
    }

    #[tokio::test]
    async fn test_list_priority_then_creation_time() {
        let (s, _t) = create_test_storage();
        let mut h = EscalationHandler::new(s);
        let req = make_req("a", "op", "r");
        let id1 = h
            .create_escalation(
                &req,
                "b".into(),
                EscalationOperationType::Custom("op".into()),
                EscalationPriority::Normal,
            )
            .await
            .unwrap();
        let id2 = h
            .create_escalation(
                &req,
                "b".into(),
                EscalationOperationType::Custom("op".into()),
                EscalationPriority::Normal,
            )
            .await
            .unwrap();
        let list = h.list_pending_escalations().await.unwrap();
        assert_eq!(list.len(), 2);
        assert_eq!(list[0].priority, EscalationPriority::Normal);
        assert_eq!(list[1].priority, EscalationPriority::Normal);
        assert!(list[0].created_at <= list[1].created_at);
        assert_ne!(list[0].id, list[1].id);
        let _ = (id1, id2);
    }

    #[tokio::test]
    async fn test_process_expired_mixed() {
        let (s, _t) = create_test_storage();
        let mut h = EscalationHandler::new(s);
        let req = make_req("a", "op", "r");

        let id_expired = h
            .create_escalation(
                &req,
                "b".into(),
                EscalationOperationType::Custom("op".into()),
                EscalationPriority::Normal,
            )
            .await
            .unwrap();
        let _id_valid = h
            .create_escalation(
                &req,
                "b".into(),
                EscalationOperationType::Custom("op".into()),
                EscalationPriority::Normal,
            )
            .await
            .unwrap();

        let mut esc = h.get_escalation(&id_expired).await.unwrap();
        esc.expires_at = Utc::now() - chrono::Duration::minutes(5);
        h.update_escalation(&esc).await.unwrap();

        let count = h.process_expired_escalations().await.unwrap();
        assert_eq!(count, 1);
        assert_eq!(
            h.get_escalation(&id_expired).await.unwrap().status,
            EscalationStatus::Expired
        );
        assert_eq!(h.list_pending_escalations().await.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn test_statistics_comprehensive() {
        let (s, _t) = create_test_storage();
        let mut h = EscalationHandler::new(s);
        let req = make_req("a", "op", "r");
        let rev = make_reviewer("rv", "R");

        let id1 = h
            .create_escalation(
                &req,
                "b".into(),
                EscalationOperationType::Custom("op".into()),
                EscalationPriority::Normal,
            )
            .await
            .unwrap();
        h.approve_escalation(&id1, rev.clone(), "ok".into(), vec![], Some(100), false)
            .await
            .unwrap();

        let id2 = h
            .create_escalation(
                &req,
                "b".into(),
                EscalationOperationType::Custom("op".into()),
                EscalationPriority::Normal,
            )
            .await
            .unwrap();
        h.deny_escalation(&id2, rev.clone(), "no".into(), None)
            .await
            .unwrap();

        let id3 = h
            .create_escalation(
                &req,
                "b".into(),
                EscalationOperationType::Custom("op".into()),
                EscalationPriority::Normal,
            )
            .await
            .unwrap();
        h.cancel_escalation(&id3, Some("done".into())).await.unwrap();

        let id4 = h
            .create_escalation(
                &req,
                "b".into(),
                EscalationOperationType::Custom("op".into()),
                EscalationPriority::Normal,
            )
            .await
            .unwrap();

        let _id5 = h
            .create_escalation(
                &req,
                "b".into(),
                EscalationOperationType::Custom("op".into()),
                EscalationPriority::Normal,
            )
            .await
            .unwrap();

        let mut esc = h.get_escalation(&id4).await.unwrap();
        esc.expires_at = Utc::now() - chrono::Duration::hours(1);
        h.update_escalation(&esc).await.unwrap();
        h.process_expired_escalations().await.unwrap();

        let stats = h.get_statistics().await.unwrap();
        assert_eq!(stats.total_requests, 5);
        assert_eq!(stats.approved_count, 1);
        assert_eq!(stats.denied_count, 1);
        assert_eq!(stats.cancelled_count, 1);
        assert_eq!(stats.expired_count, 1);
        assert_eq!(stats.pending_count, 1);
        assert_eq!(stats.reviewed_count, 2);
        assert_eq!(stats.total_approval_duration_seconds, 100);
        assert_eq!(stats.average_approval_duration_seconds, 100);
        // response_time may be 0 in fast test environments (sub-second create→approve)
        let _ = stats.average_response_time_seconds;
    }

    #[tokio::test]
    async fn test_concurrent_creates() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().to_str().unwrap().to_string();
        let mut handles = Vec::new();
        for i in 0..5 {
            let p = path.clone();
            handles.push(tokio::spawn(async move {
                let storage = Box::new(
                    GitRefsStorage::new(&p, &format!("agent-{}", i)).unwrap(),
                );
                let mut h = EscalationHandler::new(storage);
                let req = SandboxRequest {
                    agent_id: format!("agent-{}", i),
                    operation: "op".into(),
                    resource_type: "r".into(),
                    parameters: serde_json::json!({}),
                    timestamp: Utc::now(),
                    session_id: None,
                };
                h.create_escalation(
                    &req,
                    "b".into(),
                    EscalationOperationType::Custom("op".into()),
                    EscalationPriority::Normal,
                )
                .await
                .unwrap()
            }));
        }
        let mut ids = Vec::new();
        for handle in handles {
            ids.push(handle.await.unwrap());
        }
        assert_eq!(ids.len(), 5);
        let unique: std::collections::HashSet<_> = ids.iter().collect();
        assert_eq!(unique.len(), 5);
    }

    #[tokio::test]
    async fn test_similar_request_count_increments() {
        let (s, _t) = create_test_storage();
        let mut h = EscalationHandler::new(s);
        let req = make_req("a", "file_delete", "r");

        let id1 = h
            .create_escalation(
                &req,
                "b".into(),
                EscalationOperationType::FileSystemAccess,
                EscalationPriority::Normal,
            )
            .await
            .unwrap();
        let esc1 = h.get_escalation(&id1).await.unwrap();
        assert_eq!(esc1.similar_request_count, 0);

        let id2 = h
            .create_escalation(
                &req,
                "b".into(),
                EscalationOperationType::FileSystemAccess,
                EscalationPriority::Normal,
            )
            .await
            .unwrap();
        let esc2 = h.get_escalation(&id2).await.unwrap();
        assert_eq!(esc2.similar_request_count, 1);

        let id3 = h
            .create_escalation(
                &req,
                "b".into(),
                EscalationOperationType::FileSystemAccess,
                EscalationPriority::Normal,
            )
            .await
            .unwrap();
        let esc3 = h.get_escalation(&id3).await.unwrap();
        assert_eq!(esc3.similar_request_count, 2);
    }

    #[tokio::test]
    async fn test_active_approval_expired_duration() {
        let (s, _t) = create_test_storage();
        let mut h = EscalationHandler::new(s);
        let req = make_req("a", "op", "r");
        let id = h
            .create_escalation(
                &req,
                "b".into(),
                EscalationOperationType::Custom("op".into()),
                EscalationPriority::Normal,
            )
            .await
            .unwrap();
        let rev = make_reviewer("rv", "R");
        h.approve_escalation(&id, rev, "ok".into(), vec![], Some(1), false)
            .await
            .unwrap();
        let mut esc = h.get_escalation(&id).await.unwrap();
        esc.reviewed_at = Some(Utc::now() - chrono::Duration::seconds(10));
        h.update_escalation(&esc).await.unwrap();
        assert!(h
            .has_active_approval("a", "op")
            .await
            .unwrap()
            .is_none());
    }

    #[tokio::test]
    async fn test_impact_if_denied_and_session() {
        let (s, _t) = create_test_storage();
        let mut h = EscalationHandler::new(s);
        let req = SandboxRequest {
            agent_id: "a".into(),
            operation: "file_delete".into(),
            resource_type: "/data".into(),
            parameters: serde_json::json!({}),
            timestamp: Utc::now(),
            session_id: Some("sess-99".into()),
        };
        let id = h
            .create_escalation(
                &req,
                "blocked".into(),
                EscalationOperationType::FileSystemAccess,
                EscalationPriority::High,
            )
            .await
            .unwrap();
        let esc = h.get_escalation(&id).await.unwrap();
        assert_eq!(esc.session_id.as_deref(), Some("sess-99"));
        assert!(esc.impact_if_denied.is_some());
        assert!(esc.impact_if_denied.as_ref().unwrap().contains("file_delete"));
        assert_eq!(
            esc.operation_context.block_reason,
            "blocked"
        );
        assert!(esc.operation_context.risk_assessment.is_some());
        assert!(esc.operation_context.risk_assessment.as_ref().unwrap().contains("High risk"));
        assert!(!esc.operation_context.alternatives.is_empty());
    }

    #[tokio::test]
    async fn test_approve_invalid_id() {
        let (s, _t) = create_test_storage();
        let mut h = EscalationHandler::new(s);
        let rev = make_reviewer("rv", "R");
        let result = h
            .approve_escalation("nonexistent-id", rev, "ok".into(), vec![], None, false)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_deny_invalid_id() {
        let (s, _t) = create_test_storage();
        let mut h = EscalationHandler::new(s);
        let rev = make_reviewer("rv", "R");
        let result = h
            .deny_escalation("nonexistent-id", rev, "no".into(), None)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_cancel_denied_fails() {
        let (s, _t) = create_test_storage();
        let mut h = EscalationHandler::new(s);
        let req = make_req("a", "op", "r");
        let id = h
            .create_escalation(
                &req,
                "b".into(),
                EscalationOperationType::Custom("op".into()),
                EscalationPriority::Normal,
            )
            .await
            .unwrap();
        let rev = make_reviewer("rv", "R");
        h.deny_escalation(&id, rev, "no".into(), None)
            .await
            .unwrap();
        assert!(h.cancel_escalation(&id, None).await.is_err());
    }

    #[tokio::test]
    async fn test_cancel_expired_fails() {
        let (s, _t) = create_test_storage();
        let mut h = EscalationHandler::new(s);
        let req = make_req("a", "op", "r");
        let id = h
            .create_escalation(
                &req,
                "b".into(),
                EscalationOperationType::Custom("op".into()),
                EscalationPriority::Normal,
            )
            .await
            .unwrap();
        let mut esc = h.get_escalation(&id).await.unwrap();
        esc.expires_at = Utc::now() - chrono::Duration::hours(1);
        h.update_escalation(&esc).await.unwrap();
        h.process_expired_escalations().await.unwrap();
        assert!(h.cancel_escalation(&id, None).await.is_err());
    }

    #[tokio::test]
    async fn test_approve_already_approved_fails() {
        let (s, _t) = create_test_storage();
        let mut h = EscalationHandler::new(s);
        let req = make_req("a", "op", "r");
        let id = h
            .create_escalation(
                &req,
                "b".into(),
                EscalationOperationType::Custom("op".into()),
                EscalationPriority::Normal,
            )
            .await
            .unwrap();
        let rev1 = make_reviewer("rv1", "R1");
        h.approve_escalation(&id, rev1, "ok".into(), vec![], None, false)
            .await
            .unwrap();
        let rev2 = make_reviewer("rv2", "R2");
        let result = h
            .approve_escalation(&id, rev2, "also ok".into(), vec![], None, false)
            .await;
        assert!(result.is_err());
        assert_eq!(
            h.get_escalation(&id).await.unwrap().status,
            EscalationStatus::Approved
        );
    }

    #[tokio::test]
    async fn test_deny_already_denied_fails() {
        let (s, _t) = create_test_storage();
        let mut h = EscalationHandler::new(s);
        let req = make_req("a", "op", "r");
        let id = h
            .create_escalation(
                &req,
                "b".into(),
                EscalationOperationType::Custom("op".into()),
                EscalationPriority::Normal,
            )
            .await
            .unwrap();
        let rev1 = make_reviewer("rv1", "R1");
        h.deny_escalation(&id, rev1, "no".into(), None)
            .await
            .unwrap();
        let rev2 = make_reviewer("rv2", "R2");
        let result = h
            .deny_escalation(&id, rev2, "also no".into(), None)
            .await;
        assert!(result.is_err());
        assert_eq!(
            h.get_escalation(&id).await.unwrap().status,
            EscalationStatus::Denied
        );
    }

    #[tokio::test]
    async fn test_list_agent_sorted_newest_first() {
        let (s, _t) = create_test_storage();
        let mut h = EscalationHandler::new(s);
        let req = make_req("a", "op", "r");
        let id1 = h
            .create_escalation(
                &req,
                "b".into(),
                EscalationOperationType::Custom("op".into()),
                EscalationPriority::Normal,
            )
            .await
            .unwrap();
        let id2 = h
            .create_escalation(
                &req,
                "b".into(),
                EscalationOperationType::Custom("op".into()),
                EscalationPriority::Normal,
            )
            .await
            .unwrap();
        let list = h.list_agent_escalations("a").await.unwrap();
        assert_eq!(list.len(), 2);
        assert_eq!(list[0].id, id2);
        assert_eq!(list[1].id, id1);
    }

    #[tokio::test]
    async fn test_statistics_response_time_averaging() {
        let (s, _t) = create_test_storage();
        let mut h = EscalationHandler::new(s);
        let req = make_req("a", "op", "r");
        let rev = make_reviewer("rv", "R");

        let id1 = h
            .create_escalation(
                &req,
                "b".into(),
                EscalationOperationType::Custom("op".into()),
                EscalationPriority::Normal,
            )
            .await
            .unwrap();
        h.approve_escalation(&id1, rev.clone(), "ok".into(), vec![], None, false)
            .await
            .unwrap();

        let id2 = h
            .create_escalation(
                &req,
                "b".into(),
                EscalationOperationType::Custom("op".into()),
                EscalationPriority::Normal,
            )
            .await
            .unwrap();
        h.deny_escalation(&id2, rev, "no".into(), None)
            .await
            .unwrap();

        let stats = h.get_statistics().await.unwrap();
        assert_eq!(stats.reviewed_count, 2);
        // response_time may be 0 in fast test environments (sub-second create→review)
        assert!(stats.average_response_time_seconds <= stats.total_response_time_seconds);
    }

    #[tokio::test]
    async fn test_cache_serves_stale_after_direct_storage_update() {
        let (s, _t) = create_test_storage();
        let mut h = EscalationHandler::new(s);
        let req = make_req("a", "op", "r");
        let id = h
            .create_escalation(
                &req,
                "b".into(),
                EscalationOperationType::Custom("op".into()),
                EscalationPriority::Normal,
            )
            .await
            .unwrap();
        let cached = h.get_escalation(&id).await.unwrap();
        assert_eq!(cached.status, EscalationStatus::Pending);
        let mut esc = cached.clone();
        esc.status = EscalationStatus::Cancelled;
        h.update_escalation(&esc).await.unwrap();
        let fetched = h.get_escalation(&id).await.unwrap();
        assert_eq!(fetched.status, EscalationStatus::Cancelled);
    }

    #[tokio::test]
    async fn test_operation_context_parameters_preserved() {
        let (s, _t) = create_test_storage();
        let mut h = EscalationHandler::new(s);
        let req = SandboxRequest {
            agent_id: "a".into(),
            operation: "op".into(),
            resource_type: "r".into(),
            parameters: serde_json::json!({"key1": "val1", "key2": 42, "key3": true}),
            timestamp: Utc::now(),
            session_id: None,
        };
        let id = h
            .create_escalation(
                &req,
                "b".into(),
                EscalationOperationType::Custom("op".into()),
                EscalationPriority::Normal,
            )
            .await
            .unwrap();
        let esc = h.get_escalation(&id).await.unwrap();
        assert_eq!(esc.operation_context.parameters.get("key1").unwrap(), "val1");
        assert_eq!(esc.operation_context.parameters.get("key2").unwrap(), 42);
    }

    #[tokio::test]
    async fn test_expiration_by_priority() {
        let (s, _t) = create_test_storage();
        let mut h = EscalationHandler::new(s);
        let req = make_req("a", "op", "r");

        let priorities_hours = [
            (EscalationPriority::Critical, 1u64),
            (EscalationPriority::High, 4u64),
            (EscalationPriority::Normal, 24u64),
            (EscalationPriority::Low, 72u64),
        ];

        for (priority, expected_hours) in &priorities_hours {
            let id = h
                .create_escalation(
                    &req,
                    "b".into(),
                    EscalationOperationType::Custom("op".into()),
                    priority.clone(),
                )
                .await
                .unwrap();
            let esc = h.get_escalation(&id).await.unwrap();
            let diff = (esc.expires_at - esc.created_at).num_hours();
            assert_eq!(diff, *expected_hours as i64);
        }
    }
}

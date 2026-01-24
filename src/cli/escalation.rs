//! Escalation command implementations

use crate::entities::{
    Entity, EscalationOperationType, EscalationPriority, EscalationRequest, EscalationStatus,
    OperationContext, ReviewDecision, ReviewerInfo,
};
use crate::error::EngramError;
use crate::storage::Storage;
use clap::Subcommand;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Read, Write};

/// Escalation input structure for JSON
#[derive(Debug, Deserialize)]
pub struct EscalationInput {
    pub agent_id: String,
    pub operation_type: String,
    pub operation: String,
    pub block_reason: String,
    pub justification: String,
    pub priority: Option<String>,
    pub impact_if_denied: Option<String>,
    pub suggested_reviewer: Option<String>,
    pub parameters: Option<HashMap<String, serde_json::Value>>,
}

/// Review decision input structure
#[derive(Debug, Deserialize)]
pub struct ReviewInput {
    pub status: String,
    pub reason: String,
    pub conditions: Option<Vec<String>>,
    pub approval_duration: Option<u64>,
    pub create_policy: Option<bool>,
    pub notes: Option<String>,
    pub reviewer_id: String,
    pub reviewer_name: String,
    pub reviewer_email: Option<String>,
}

/// Escalation commands
#[derive(Subcommand)]
pub enum EscalationCommands {
    Create {
        #[arg(long, short)]
        agent: Option<String>,

        #[arg(long, short)]
        operation_type: Option<String>,

        #[arg(long)]
        operation: Option<String>,

        #[arg(long)]
        block_reason: Option<String>,

        #[arg(long, short)]
        justification: Option<String>,

        #[arg(long, short, default_value = "normal")]
        priority: String,

        #[arg(long)]
        impact: Option<String>,

        #[arg(long)]
        reviewer: Option<String>,

        #[arg(long, conflicts_with_all = ["agent"])]
        stdin: bool,

        #[arg(long, conflicts_with_all = ["agent", "stdin"])]
        file: Option<String>,

        #[arg(long)]
        json: bool,
    },
    /// List escalation requests
    List {
        /// Filter by agent ID
        #[arg(long)]
        agent_id: Option<String>,

        /// Filter by status (pending, approved, denied, expired, cancelled)
        #[arg(long)]
        status: Option<String>,

        /// Filter by priority (low, normal, high, critical)
        #[arg(long)]
        priority: Option<String>,

        /// Filter by operation type
        #[arg(long)]
        operation_type: Option<String>,

        /// Show only expired requests
        #[arg(long)]
        expired_only: bool,

        /// Show only actionable (pending and not expired) requests
        #[arg(long)]
        actionable_only: bool,

        /// Agent to filter by
        #[arg(long, short)]
        agent: Option<String>,

        /// Output in JSON format
        #[arg(long)]
        json: bool,
    },
    /// Get escalation request details
    Get {
        /// Escalation request ID
        #[arg()]
        id: String,

        /// Output in JSON format
        #[arg(long)]
        json: bool,
    },
    /// Review an escalation request
    Review {
        /// Escalation request ID
        #[arg()]
        id: String,

        /// Decision (approved, denied)
        #[arg(long, short)]
        status: Option<String>,

        /// Reason for the decision
        #[arg(long, short)]
        reason: Option<String>,

        /// Reviewer ID
        #[arg(long)]
        reviewer_id: Option<String>,

        /// Reviewer name
        #[arg(long)]
        reviewer_name: Option<String>,

        /// Approval duration in seconds
        #[arg(long)]
        duration: Option<u64>,

        /// Create policy from this decision
        #[arg(long)]
        create_policy: bool,

        /// Additional notes
        #[arg(long)]
        notes: Option<String>,

        /// Read review data from stdin as JSON
        #[arg(long)]
        stdin: bool,

        /// Read review data from file as JSON
        #[arg(long, conflicts_with = "stdin")]
        file: Option<String>,

        /// Output in JSON format
        #[arg(long)]
        json: bool,
    },
    /// Cancel an escalation request
    Cancel {
        /// Escalation request ID
        #[arg()]
        id: String,

        /// Reason for cancellation
        #[arg(long, short)]
        reason: Option<String>,

        /// Force cancellation without confirmation
        #[arg(long)]
        force: bool,

        /// Output in JSON format
        #[arg(long)]
        json: bool,
    },
    /// Mark expired escalation requests
    Cleanup {
        /// Actually mark requests as expired (dry run by default)
        #[arg(long)]
        apply: bool,

        /// Output in JSON format
        #[arg(long)]
        json: bool,
    },
    /// Show escalation statistics
    Stats {
        /// Agent ID to show stats for
        #[arg(long, short)]
        agent_id: Option<String>,

        /// Time period in days to analyze
        #[arg(long, default_value = "30")]
        days: u64,

        /// Output in JSON format
        #[arg(long)]
        json: bool,
    },
}

/// Create a new escalation request
pub fn create_escalation<S: Storage>(
    storage: &mut S,
    agent: Option<String>,
    operation_type: Option<String>,
    operation: Option<String>,
    block_reason: Option<String>,
    justification: Option<String>,
    priority: String,
    impact: Option<String>,
    reviewer: Option<String>,
    stdin: bool,
    file: Option<String>,
    json: bool,
) -> Result<(), EngramError> {
    let escalation_input = if stdin {
        read_escalation_input_from_stdin()?
    } else if let Some(file_path) = file {
        read_escalation_input_from_file(&file_path)?
    } else {
        let agent_id = agent
            .clone()
            .ok_or_else(|| EngramError::Validation("Agent is required".to_string()))?;
        let operation_type = operation_type
            .ok_or_else(|| EngramError::Validation("Operation type is required".to_string()))?;
        let operation = operation
            .ok_or_else(|| EngramError::Validation("Operation is required".to_string()))?;
        let block_reason = block_reason
            .ok_or_else(|| EngramError::Validation("Block reason is required".to_string()))?;
        let justification = justification
            .ok_or_else(|| EngramError::Validation("Justification is required".to_string()))?;

        EscalationInput {
            agent_id,
            operation_type,
            operation,
            block_reason,
            justification,
            priority: Some(priority),
            impact_if_denied: impact,
            suggested_reviewer: reviewer,
            parameters: None,
        }
    };

    let operation_type = parse_operation_type(&escalation_input.operation_type)?;
    let priority = parse_priority(
        &escalation_input
            .priority
            .unwrap_or_else(|| "normal".to_string()),
    )?;
    let agent = agent.unwrap_or_else(|| "default".to_string());

    let operation_context = OperationContext {
        operation: escalation_input.operation,
        parameters: escalation_input.parameters.unwrap_or_default(),
        resource: None,
        block_reason: escalation_input.block_reason,
        alternatives: Vec::new(),
        risk_assessment: None,
    };

    let mut escalation = EscalationRequest::new(
        escalation_input.agent_id,
        operation_type,
        operation_context,
        escalation_input.justification,
        priority,
        agent,
    );

    if let Some(impact) = escalation_input.impact_if_denied {
        escalation.impact_if_denied = Some(impact);
    }

    if let Some(suggested_reviewer) = escalation_input.suggested_reviewer {
        escalation.suggested_reviewer = Some(suggested_reviewer);
    }

    storage.store(&escalation.to_generic())?;

    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&escalation.to_generic())?
        );
    } else {
        println!("✅ Escalation request created successfully:");
        println!("  ID: {}", escalation.id);
        println!("  Agent: {}", escalation.agent_id);
        println!("  Operation: {:?}", escalation.operation_type);
        println!("  Priority: {:?}", escalation.priority);
        println!("  Status: {:?}", escalation.status);

        if let Some(time_remaining) = escalation.time_to_expiration() {
            let hours = time_remaining.num_hours();
            let minutes = time_remaining.num_minutes() % 60;
            println!("  Expires in: {}h {}m", hours, minutes);
        }
    }

    Ok(())
}

/// List escalation requests
pub fn list_escalations<S: Storage>(
    storage: &S,
    agent_id: Option<String>,
    status: Option<String>,
    priority: Option<String>,
    operation_type: Option<String>,
    expired_only: bool,
    actionable_only: bool,
    agent: Option<String>,
    json: bool,
) -> Result<(), EngramError> {
    let ids = storage.list_ids("escalation_request")?;
    let mut escalations = Vec::new();

    for id in ids {
        if let Ok(Some(entity)) = storage.get(&id, "escalation_request") {
            match EscalationRequest::from_generic(entity) {
                Ok(mut escalation) => {
                    // Check for expiration and update if needed
                    if escalation.is_expired() && escalation.status == EscalationStatus::Pending {
                        escalation.mark_expired();
                    }

                    // Apply filters
                    if let Some(filter_agent_id) = &agent_id {
                        if escalation.agent_id != *filter_agent_id {
                            continue;
                        }
                    }

                    if let Some(filter_agent) = &agent {
                        if escalation.agent != *filter_agent {
                            continue;
                        }
                    }

                    if let Some(filter_status) = &status {
                        if format!("{:?}", escalation.status).to_lowercase()
                            != filter_status.to_lowercase()
                        {
                            continue;
                        }
                    }

                    if let Some(filter_priority) = &priority {
                        if format!("{:?}", escalation.priority).to_lowercase()
                            != filter_priority.to_lowercase()
                        {
                            continue;
                        }
                    }

                    if let Some(filter_op_type) = &operation_type {
                        let op_type_str = match escalation.operation_type {
                            EscalationOperationType::Custom(ref s) => s.clone(),
                            _ => format!("{:?}", escalation.operation_type).to_lowercase(),
                        };
                        if op_type_str != filter_op_type.to_lowercase() {
                            continue;
                        }
                    }

                    if expired_only && escalation.status != EscalationStatus::Expired {
                        continue;
                    }

                    if actionable_only && !escalation.is_actionable() {
                        continue;
                    }

                    escalations.push(escalation);
                }
                Err(_) => continue,
            }
        }
    }

    if json {
        let generic_escalations: Vec<_> = escalations.iter().map(|e| e.to_generic()).collect();
        println!("{}", serde_json::to_string_pretty(&generic_escalations)?);
    } else {
        if escalations.is_empty() {
            println!("No escalation requests found.");
        } else {
            println!("🚨 Escalation Requests ({} found):", escalations.len());
            for escalation in escalations {
                let status_icon = match escalation.status {
                    EscalationStatus::Pending => "⏳",
                    EscalationStatus::Approved => "✅",
                    EscalationStatus::Denied => "❌",
                    EscalationStatus::Expired => "⏰",
                    EscalationStatus::Cancelled => "🚫",
                };

                println!(
                    "  {} {} [{}] - {} ({:?}, {:?})",
                    status_icon,
                    escalation.id,
                    escalation.agent_id,
                    escalation.operation_context.operation,
                    escalation.operation_type,
                    escalation.priority
                );

                if escalation.status == EscalationStatus::Pending {
                    if let Some(time_remaining) = escalation.time_to_expiration() {
                        let hours = time_remaining.num_hours();
                        let minutes = time_remaining.num_minutes() % 60;
                        if time_remaining.num_seconds() > 0 {
                            println!("    ⏱️  Expires in: {}h {}m", hours, minutes);
                        } else {
                            println!("    ⚠️  Expired");
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

/// Get escalation request details
pub fn get_escalation<S: Storage>(storage: &S, id: String, json: bool) -> Result<(), EngramError> {
    match storage.get(&id, "escalation_request")? {
        Some(entity) => {
            let mut escalation = EscalationRequest::from_generic(entity)?;

            // Check for expiration and update if needed
            if escalation.is_expired() && escalation.status == EscalationStatus::Pending {
                escalation.mark_expired();
            }

            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&escalation.to_generic())?
                );
            } else {
                println!("🚨 Escalation Request Details:");
                println!("  ID: {}", escalation.id);
                println!("  Agent: {}", escalation.agent_id);
                println!("  Operation Type: {:?}", escalation.operation_type);
                println!("  Status: {:?}", escalation.status);
                println!("  Priority: {:?}", escalation.priority);
                println!(
                    "  Created: {}",
                    escalation.created_at.format("%Y-%m-%d %H:%M:%S UTC")
                );

                if escalation.status == EscalationStatus::Pending {
                    if let Some(time_remaining) = escalation.time_to_expiration() {
                        let hours = time_remaining.num_hours();
                        let minutes = time_remaining.num_minutes() % 60;
                        if time_remaining.num_seconds() > 0 {
                            println!("  Expires in: {}h {}m", hours, minutes);
                        } else {
                            println!("  Status: ⚠️ EXPIRED");
                        }
                    }
                }

                println!("\n📋 Operation Context:");
                println!("  Operation: {}", escalation.operation_context.operation);
                println!(
                    "  Block Reason: {}",
                    escalation.operation_context.block_reason
                );

                if !escalation.operation_context.parameters.is_empty() {
                    println!(
                        "  Parameters: {}",
                        serde_json::to_string_pretty(&escalation.operation_context.parameters)?
                    );
                }

                println!("\n💬 Justification:");
                println!("  {}", escalation.justification);

                if let Some(impact) = &escalation.impact_if_denied {
                    println!("\n⚠️ Impact if Denied:");
                    println!("  {}", impact);
                }

                if let Some(reviewer) = &escalation.reviewer {
                    println!("\n👤 Assigned Reviewer:");
                    println!("  Name: {}", reviewer.reviewer_name);
                    println!("  ID: {}", reviewer.reviewer_id);
                    if let Some(email) = &reviewer.reviewer_email {
                        println!("  Email: {}", email);
                    }
                }

                if let Some(decision) = &escalation.decision {
                    println!("\n📝 Decision:");
                    println!("  Status: {:?}", decision.status);
                    println!("  Reason: {}", decision.reason);

                    if !decision.conditions.is_empty() {
                        println!("  Conditions: {}", decision.conditions.join(", "));
                    }

                    if let Some(duration) = decision.approval_duration {
                        println!("  Valid for: {} seconds", duration);
                    }

                    if let Some(notes) = &decision.notes {
                        println!("  Notes: {}", notes);
                    }
                }
            }
        }
        None => {
            return Err(EngramError::NotFound(format!(
                "Escalation request with ID {} not found",
                id
            )));
        }
    }

    Ok(())
}

/// Review an escalation request
pub fn review_escalation<S: Storage>(
    storage: &mut S,
    id: String,
    status: Option<String>,
    reason: Option<String>,
    reviewer_id: Option<String>,
    reviewer_name: Option<String>,
    duration: Option<u64>,
    create_policy: bool,
    notes: Option<String>,
    stdin: bool,
    file: Option<String>,
    json: bool,
) -> Result<(), EngramError> {
    let mut escalation = match storage.get(&id, "escalation_request")? {
        Some(entity) => EscalationRequest::from_generic(entity)
            .map_err(|e| EngramError::Validation(e.to_string()))?,
        None => {
            return Err(EngramError::NotFound(format!(
                "Escalation request with ID {} not found",
                id
            )));
        }
    };

    // Check if request is still actionable
    if !escalation.is_actionable() {
        return Err(EngramError::InvalidOperation(format!(
            "Escalation request {} is not actionable (status: {:?})",
            id, escalation.status
        )));
    }

    let review_input = if stdin {
        read_review_input_from_stdin()?
    } else if let Some(file_path) = file {
        read_review_input_from_file(&file_path)?
    } else {
        let status = status
            .ok_or_else(|| EngramError::Validation("Decision status is required".to_string()))?;
        let reason = reason
            .ok_or_else(|| EngramError::Validation("Decision reason is required".to_string()))?;
        let reviewer_id = reviewer_id
            .ok_or_else(|| EngramError::Validation("Reviewer ID is required".to_string()))?;
        let reviewer_name = reviewer_name
            .ok_or_else(|| EngramError::Validation("Reviewer name is required".to_string()))?;

        ReviewInput {
            status,
            reason,
            conditions: None,
            approval_duration: duration,
            create_policy: Some(create_policy),
            notes,
            reviewer_id,
            reviewer_name,
            reviewer_email: None,
        }
    };

    let decision_status = parse_escalation_status(&review_input.status)?;

    // Create reviewer info
    let reviewer_info = ReviewerInfo {
        reviewer_id: review_input.reviewer_id,
        reviewer_name: review_input.reviewer_name,
        reviewer_email: review_input.reviewer_email,
        department: None,
    };

    // Create decision
    let decision = ReviewDecision {
        status: decision_status.clone(),
        reason: review_input.reason,
        conditions: review_input.conditions.unwrap_or_default(),
        approval_duration: review_input.approval_duration,
        create_policy: review_input.create_policy.unwrap_or(false),
        notes: review_input.notes,
    };

    // Assign reviewer and record decision
    escalation.assign_reviewer(reviewer_info);
    escalation.record_decision(decision);

    storage.store(&escalation.to_generic())?;

    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&escalation.to_generic())?
        );
    } else {
        println!("✅ Escalation request reviewed successfully:");
        println!("  ID: {}", escalation.id);
        println!("  Decision: {:?}", decision_status);
        println!(
            "  Reviewer: {}",
            escalation.reviewer.as_ref().unwrap().reviewer_name
        );

        if let Some(duration) = escalation.decision.as_ref().unwrap().approval_duration {
            println!("  Valid for: {} seconds", duration);
        }
    }

    Ok(())
}

/// Cancel an escalation request
pub fn cancel_escalation<S: Storage>(
    storage: &mut S,
    id: String,
    reason: Option<String>,
    force: bool,
    json: bool,
) -> Result<(), EngramError> {
    if !force {
        print!(
            "Are you sure you want to cancel escalation request {}? (y/N): ",
            id
        );
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if !input.trim().to_lowercase().starts_with('y') {
            println!("Operation cancelled.");
            return Ok(());
        }
    }

    let mut escalation = match storage.get(&id, "escalation_request")? {
        Some(entity) => EscalationRequest::from_generic(entity)
            .map_err(|e| EngramError::Validation(e.to_string()))?,
        None => {
            return Err(EngramError::NotFound(format!(
                "Escalation request with ID {} not found",
                id
            )));
        }
    };

    escalation.cancel(reason);
    storage.store(&escalation.to_generic())?;

    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&escalation.to_generic())?
        );
    } else {
        println!("✅ Escalation request {} cancelled successfully.", id);
    }

    Ok(())
}

/// Cleanup expired escalation requests
pub fn cleanup_escalations<S: Storage>(
    storage: &mut S,
    apply: bool,
    json: bool,
) -> Result<(), EngramError> {
    let ids = storage.list_ids("escalation_request")?;
    let mut expired_requests = Vec::new();
    let mut updated_count = 0;

    for id in ids {
        if let Ok(Some(entity)) = storage.get(&id, "escalation_request") {
            if let Ok(mut escalation) = EscalationRequest::from_generic(entity) {
                if escalation.is_expired() && escalation.status == EscalationStatus::Pending {
                    expired_requests.push(escalation.clone());

                    if apply {
                        escalation.mark_expired();
                        storage.store(&escalation.to_generic())?;
                        updated_count += 1;
                    }
                }
            }
        }
    }

    if json {
        let result = serde_json::json!({
            "expired_requests": expired_requests.len(),
            "updated": if apply { updated_count } else { 0 },
            "dry_run": !apply,
            "requests": expired_requests.iter().map(|e| serde_json::json!({
                "id": e.id,
                "agent_id": e.agent_id,
                "operation": e.operation_context.operation,
                "created_at": e.created_at,
                "expired_hours_ago": (chrono::Utc::now() - e.expires_at).num_hours()
            })).collect::<Vec<_>>()
        });
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        if expired_requests.is_empty() {
            println!("No expired escalation requests found.");
        } else {
            if apply {
                println!("✅ Marked {} expired escalation requests.", updated_count);
            } else {
                println!(
                    "🔍 Found {} expired escalation requests (dry run):",
                    expired_requests.len()
                );
                println!("Run with --apply to mark them as expired.");
            }

            for request in &expired_requests {
                let hours_expired = (chrono::Utc::now() - request.expires_at).num_hours();
                println!(
                    "  • {} [{}] - {} (expired {}h ago)",
                    request.id,
                    request.agent_id,
                    request.operation_context.operation,
                    hours_expired
                );
            }
        }
    }

    Ok(())
}

/// Show escalation statistics
pub fn show_escalation_stats<S: Storage>(
    storage: &S,
    agent_id: Option<String>,
    days: u64,
    json: bool,
) -> Result<(), EngramError> {
    let ids = storage.list_ids("escalation_request")?;
    let cutoff_date = chrono::Utc::now() - chrono::Duration::days(days as i64);

    let mut total_requests = 0;
    let mut status_counts = HashMap::new();
    let mut priority_counts = HashMap::new();
    let mut operation_type_counts = HashMap::new();
    let mut agent_requests = Vec::new();

    for id in ids {
        if let Ok(Some(entity)) = storage.get(&id, "escalation_request") {
            if let Ok(escalation) = EscalationRequest::from_generic(entity) {
                // Skip requests outside the time window
                if escalation.created_at < cutoff_date {
                    continue;
                }

                // Apply agent filter
                if let Some(filter_agent_id) = &agent_id {
                    if escalation.agent_id != *filter_agent_id {
                        continue;
                    }
                    agent_requests.push(escalation.clone());
                } else {
                    total_requests += 1;
                    *status_counts
                        .entry(format!("{:?}", escalation.status))
                        .or_insert(0) += 1;
                    *priority_counts
                        .entry(format!("{:?}", escalation.priority))
                        .or_insert(0) += 1;
                    *operation_type_counts
                        .entry(format!("{:?}", escalation.operation_type))
                        .or_insert(0) += 1;
                    agent_requests.push(escalation);
                }
            }
        }
    }

    if json {
        let stats = serde_json::json!({
            "time_period_days": days,
            "total_requests": if agent_id.is_some() { agent_requests.len() } else { total_requests },
            "status_distribution": status_counts,
            "priority_distribution": priority_counts,
            "operation_type_distribution": operation_type_counts,
            "agent_filter": agent_id,
            "requests": agent_requests.iter().map(|e| serde_json::json!({
                "id": e.id,
                "agent_id": e.agent_id,
                "status": format!("{:?}", e.status),
                "priority": format!("{:?}", e.priority),
                "operation_type": format!("{:?}", e.operation_type),
                "created_at": e.created_at
            })).collect::<Vec<_>>()
        });
        println!("{}", serde_json::to_string_pretty(&stats)?);
    } else {
        if let Some(filter_agent_id) = agent_id {
            println!(
                "🚨 Escalation Stats for Agent: {} (last {} days)",
                filter_agent_id, days
            );
            if agent_requests.is_empty() {
                println!("  No escalation requests found for this agent.");
            } else {
                println!("  Total requests: {}", agent_requests.len());
                for request in agent_requests {
                    println!(
                        "  • {} - {:?} ({:?}, {:?})",
                        request.operation_context.operation,
                        request.status,
                        request.priority,
                        request.operation_type
                    );
                }
            }
        } else {
            println!("🚨 Escalation Statistics (last {} days):", days);
            println!("  Total requests: {}", total_requests);

            println!("  Status distribution:");
            for (status, count) in status_counts {
                println!("    {}: {}", status, count);
            }

            println!("  Priority distribution:");
            for (priority, count) in priority_counts {
                println!("    {}: {}", priority, count);
            }

            println!("  Operation type distribution:");
            for (op_type, count) in operation_type_counts {
                println!("    {}: {}", op_type, count);
            }
        }
    }

    Ok(())
}

// Helper functions

fn read_escalation_input_from_stdin() -> Result<EscalationInput, EngramError> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    Ok(serde_json::from_str(&input)?)
}

fn read_escalation_input_from_file(file_path: &str) -> Result<EscalationInput, EngramError> {
    let content = fs::read_to_string(file_path)?;
    Ok(serde_json::from_str(&content)?)
}

fn read_review_input_from_stdin() -> Result<ReviewInput, EngramError> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    Ok(serde_json::from_str(&input)?)
}

fn read_review_input_from_file(file_path: &str) -> Result<ReviewInput, EngramError> {
    let content = fs::read_to_string(file_path)?;
    Ok(serde_json::from_str(&content)?)
}

fn parse_operation_type(op_type: &str) -> Result<EscalationOperationType, EngramError> {
    match op_type {
        "filesystem" | "file_system" => Ok(EscalationOperationType::FileSystemAccess),
        "network" => Ok(EscalationOperationType::NetworkAccess),
        "command" => Ok(EscalationOperationType::CommandExecution),
        "privilege" => Ok(EscalationOperationType::PrivilegeEscalation),
        "quality_gate" => Ok(EscalationOperationType::QualityGateOverride),
        "workflow" => Ok(EscalationOperationType::WorkflowModification),
        "resource_limit" => Ok(EscalationOperationType::ResourceLimitIncrease),
        custom => Ok(EscalationOperationType::Custom(custom.to_string())),
    }
}

fn parse_priority(priority: &str) -> Result<EscalationPriority, EngramError> {
    match priority.to_lowercase().as_str() {
        "low" => Ok(EscalationPriority::Low),
        "normal" => Ok(EscalationPriority::Normal),
        "high" => Ok(EscalationPriority::High),
        "critical" => Ok(EscalationPriority::Critical),
        _ => Err(EngramError::Validation(format!(
            "Invalid priority: {}. Must be one of: low, normal, high, critical",
            priority
        ))),
    }
}

fn parse_escalation_status(status: &str) -> Result<EscalationStatus, EngramError> {
    match status.to_lowercase().as_str() {
        "approved" => Ok(EscalationStatus::Approved),
        "denied" => Ok(EscalationStatus::Denied),
        _ => Err(EngramError::Validation(format!(
            "Invalid decision status: {}. Must be one of: approved, denied",
            status
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::{EscalationOperationType, EscalationPriority, EscalationStatus};
    use crate::storage::MemoryStorage;
    use crate::storage::Storage;

    #[test]
    fn test_create_escalation() {
        let mut storage = MemoryStorage::new("test-agent");

        let result = create_escalation(
            &mut storage,
            Some("agent-1".to_string()),
            Some("network".to_string()),
            Some("curl google.com".to_string()),
            Some("Network access restricted".to_string()),
            Some("Need to fetch data".to_string()),
            "high".to_string(),
            Some("Cannot complete task".to_string()),
            Some("admin".to_string()),
            false,
            None,
            false,
        );

        assert!(result.is_ok());

        let query_result = storage
            .query_by_type("escalation_request", None, None, None)
            .unwrap();
        assert_eq!(query_result.total_count, 1);

        let entity = &query_result.entities[0];
        assert_eq!(entity.data.get("agent_id").unwrap(), "agent-1");
    }

    #[test]
    fn test_get_escalation() {
        let mut storage = MemoryStorage::new("test-agent");

        create_escalation(
            &mut storage,
            Some("agent-1".to_string()),
            Some("filesystem".to_string()),
            Some("rm -rf /".to_string()),
            Some("Dangerous".to_string()),
            Some("Testing".to_string()),
            "critical".to_string(),
            None,
            None,
            false,
            None,
            false,
        )
        .unwrap();

        let query_result = storage
            .query_by_type("escalation_request", None, None, None)
            .unwrap();
        let id = &query_result.entities[0].id;

        let result = get_escalation(&storage, id.clone(), false);
        assert!(result.is_ok());

        let result = get_escalation(&storage, "non-existent".to_string(), false);
        assert!(result.is_err());
    }

    #[test]
    fn test_review_escalation() {
        let mut storage = MemoryStorage::new("test-agent");

        create_escalation(
            &mut storage,
            Some("agent-1".to_string()),
            Some("command".to_string()),
            Some("ls".to_string()),
            Some("Blocked".to_string()),
            Some("Need listing".to_string()),
            "normal".to_string(),
            None,
            None,
            false,
            None,
            false,
        )
        .unwrap();

        let query_result = storage
            .query_by_type("escalation_request", None, None, None)
            .unwrap();
        let id = query_result.entities[0].id.clone();

        let result = review_escalation(
            &mut storage,
            id.clone(),
            Some("approved".to_string()),
            Some("Safe command".to_string()),
            Some("reviewer-1".to_string()),
            Some("Reviewer One".to_string()),
            Some(3600),
            false,
            Some("Proceed with caution".to_string()),
            false,
            None,
            false,
        );
        assert!(result.is_ok());

        let generic = storage.get(&id, "escalation_request").unwrap().unwrap();
        let escalation = EscalationRequest::from_generic(generic).unwrap();

        assert!(matches!(escalation.status, EscalationStatus::Approved));
    }

    #[test]
    fn test_cancel_escalation() {
        let mut storage = MemoryStorage::new("test-agent");

        create_escalation(
            &mut storage,
            Some("agent-1".to_string()),
            Some("command".to_string()),
            Some("ls".to_string()),
            Some("Blocked".to_string()),
            Some("Justification".to_string()),
            "normal".to_string(),
            None,
            None,
            false,
            None,
            false,
        )
        .unwrap();

        let query_result = storage
            .query_by_type("escalation_request", None, None, None)
            .unwrap();
        let id = query_result.entities[0].id.clone();

        let result = cancel_escalation(
            &mut storage,
            id.clone(),
            Some("Not needed anymore".to_string()),
            true, // force
            false,
        );
        assert!(result.is_ok());

        let generic = storage.get(&id, "escalation_request").unwrap().unwrap();
        let escalation = EscalationRequest::from_generic(generic).unwrap();

        assert!(matches!(escalation.status, EscalationStatus::Cancelled));
    }

    #[test]
    fn test_list_escalations() {
        let mut storage = MemoryStorage::new("test-agent");

        create_escalation(
            &mut storage,
            Some("agent-1".to_string()),
            Some("network".to_string()),
            Some("curl".to_string()),
            Some("Blocked".to_string()),
            Some("Justification".to_string()),
            "normal".to_string(),
            None,
            None,
            false,
            None,
            false,
        )
        .unwrap();

        let result = list_escalations(
            &storage,
            Some("agent-1".to_string()),
            None,
            None,
            None,
            false,
            false,
            None,
            false,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_priority() {
        assert!(matches!(
            parse_priority("high").unwrap(),
            EscalationPriority::High
        ));
        assert!(matches!(
            parse_priority("NORMAL").unwrap(),
            EscalationPriority::Normal
        ));
        assert!(parse_priority("invalid").is_err());
    }

    #[test]
    fn test_parse_operation_type() {
        assert!(matches!(
            parse_operation_type("network").unwrap(),
            EscalationOperationType::NetworkAccess
        ));
        assert!(matches!(
            parse_operation_type("unknown").unwrap(),
            EscalationOperationType::Custom(_)
        ));
    }
}

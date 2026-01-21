use crate::entities::{Compliance, Entity};
use crate::error::EngramError;
use crate::storage::Storage;
use clap::Subcommand;

/// Compliance commands
#[derive(Debug, Subcommand)]
pub enum ComplianceCommands {
    /// Create a new compliance requirement
    Create {
        /// Compliance requirement title
        #[arg(long, short)]
        title: String,

        /// Compliance category
        #[arg(long)]
        category: String,

        /// Severity level
        #[arg(long, default_value = "medium")]
        severity: String,

        /// Description
        #[arg(long)]
        description: String,

        /// Agent to assign
        #[arg(long, short)]
        agent: Option<String>,
    },
    /// List compliance requirements
    List {
        /// Agent filter
        #[arg(long, short)]
        agent: Option<String>,

        /// Category filter
        #[arg(long)]
        category: Option<String>,

        /// Limit results
        #[arg(long, short)]
        limit: Option<usize>,
    },
    /// Show compliance requirement details
    Show {
        /// Requirement ID
        #[arg(help = "Compliance requirement ID to show")]
        id: String,
    },
    /// Update compliance requirement
    Update {
        /// Requirement ID
        #[arg(long, short)]
        id: String,

        /// Field to update (status, severity, description)
        #[arg(long, short)]
        field: String,

        /// New value
        #[arg(long, short)]
        value: String,
    },
    /// Delete compliance requirement
    Delete {
        /// Requirement ID
        #[arg(long, short)]
        id: String,
    },
}

/// Create compliance requirement
pub fn create_compliance<S: Storage>(
    storage: &mut S,
    title: String,
    description: String,
    category: String,
    agent: Option<String>,
) -> Result<(), EngramError> {
    let compliance = Compliance::new(
        title,
        description,
        category,
        agent.unwrap_or_else(|| "default".to_string()),
    );

    let generic = compliance.to_generic();
    storage.store(&generic)?;

    println!("✅ Compliance requirement created:");
    display_compliance(&compliance);

    Ok(())
}

/// List compliance requirements
pub fn list_compliance<S: Storage>(
    storage: &S,
    agent: Option<&str>,
    category: Option<&str>,
    limit: Option<usize>,
) -> Result<(), EngramError> {
    let mut compliance_items =
        storage.query_by_agent(agent.unwrap_or("default"), Some("compliance"))?;

    // Filter by category if specified
    if let Some(category_filter) = category {
        compliance_items.retain(|generic_item| {
            if let Ok(compliance_obj) = Compliance::from_generic(generic_item.clone()) {
                compliance_obj.category.to_lowercase() == category_filter.to_lowercase()
            } else {
                false
            }
        });
    }

    // Apply limit
    if let Some(limit_val) = limit {
        compliance_items.truncate(limit_val);
    }

    if compliance_items.is_empty() {
        println!("No compliance requirements found");
        return Ok(());
    }

    println!(
        "🔍 Compliance Requirements ({} found):",
        compliance_items.len()
    );
    for generic_item in &compliance_items {
        if let Ok(compliance_obj) = Compliance::from_generic(generic_item.clone()) {
            display_compliance_summary(&compliance_obj);
        }
    }

    Ok(())
}

/// Show compliance requirement details
pub fn show_compliance<S: Storage>(storage: &S, id: &str) -> Result<(), EngramError> {
    let generic = storage.get(id, "compliance")?;

    if let Some(generic_item) = generic {
        let compliance = Compliance::from_generic(generic_item)?;
        display_compliance(&compliance);
    } else {
        println!("❌ Compliance requirement '{}' not found", id);
    }

    Ok(())
}

/// Update compliance requirement
pub fn update_compliance<S: Storage>(
    storage: &mut S,
    id: &str,
    field: &str,
    value: &str,
) -> Result<(), EngramError> {
    let generic = storage.get(id, "compliance")?;

    if let Some(generic_item) = generic {
        let mut compliance = Compliance::from_generic(generic_item)?;

        match field.to_lowercase().as_str() {
            "status" => match value.to_lowercase().as_str() {
                "compliant" => compliance.mark_compliant(),
                "noncompliant" | "non-compliant" => {
                    compliance.mark_non_compliant(Vec::new());
                }
                _ => {
                    return Err(EngramError::Validation(format!(
                        "Invalid status: {}",
                        value
                    )))
                }
            },
            "description" => {
                compliance.description = value.to_string();
                compliance.updated_at = chrono::Utc::now();
            }
            _ => {
                return Err(EngramError::Validation(format!(
                    "Cannot update field: {}",
                    field
                )))
            }
        }

        let updated_generic = compliance.to_generic();
        storage.store(&updated_generic)?;

        println!("✅ Compliance requirement updated:");
        display_compliance(&compliance);
    } else {
        println!("❌ Compliance requirement '{}' not found", id);
    }

    Ok(())
}

/// Delete compliance requirement
pub fn delete_compliance<S: Storage>(storage: &mut S, id: &str) -> Result<(), EngramError> {
    storage.delete(id, "compliance")?;
    println!("✅ Compliance requirement '{}' deleted", id);
    Ok(())
}

/// Display compliance requirement in detail
fn display_compliance(compliance: &Compliance) {
    println!("ID: {}", compliance.id);
    println!("Title: {}", compliance.title);
    println!("Category: {}", compliance.category);
    println!("Status: {:?}", compliance.status);
    println!("Agent: {}", compliance.agent);
    println!("Description: {}", compliance.description);

    if let Some(severity) = &compliance.severity {
        println!("Severity: {:?}", severity);
    }

    if let Some(due_date) = &compliance.due_date {
        println!("Due Date: {}", due_date.format("%Y-%m-%d %H:%M"));
    }

    println!(
        "Created: {}",
        compliance.created_at.format("%Y-%m-%d %H:%M")
    );
    println!(
        "Updated: {}",
        compliance.updated_at.format("%Y-%m-%d %H:%M")
    );

    if !compliance.tags.is_empty() {
        println!("Tags: {}", compliance.tags.join(", "));
    }

    if !compliance.violations.is_empty() {
        println!("Violations: {} found", compliance.violations.len());
    }

    if !compliance.evidence.is_empty() {
        println!("Evidence: {} items", compliance.evidence.len());
    }
}

/// Display compliance requirement summary
fn display_compliance_summary(compliance: &Compliance) {
    let status_icon = match compliance.status {
        crate::entities::ComplianceStatus::Compliant => "✅",
        crate::entities::ComplianceStatus::NonCompliant => "❌",
        crate::entities::ComplianceStatus::Pending => "⏳",
        crate::entities::ComplianceStatus::Exempt => "🔒",
    };

    println!(
        "  {} [{}] {} - {} ({})",
        status_icon,
        compliance.id[..8].to_string(),
        compliance.title,
        compliance.category,
        compliance.agent
    );
}

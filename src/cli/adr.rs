use crate::entities::{AdrStatus, Entity, ADR};
use crate::error::EngramError;
use crate::storage::Storage;
use clap::Subcommand;

/// ADR commands
#[derive(Debug, Subcommand)]
pub enum AdrCommands {
    /// Create a new ADR
    Create {
        /// ADR title
        #[arg(long, short)]
        title: String,

        /// ADR number
        #[arg(long)]
        number: u32,

        /// Context and problem statement
        #[arg(long)]
        context: String,

        /// Agent to assign
        #[arg(long, short)]
        agent: Option<String>,
    },
    /// Get ADR details
    Get {
        /// ADR ID
        #[arg(help = "ADR ID to retrieve")]
        id: String,
    },
    /// Update ADR
    Update {
        /// ADR ID
        #[arg(help = "ADR ID to update")]
        id: String,

        /// ADR title
        #[arg(long)]
        title: Option<String>,

        /// ADR status (proposed, accepted, deprecated, superseded)
        #[arg(long)]
        status: Option<String>,

        /// Context and problem statement
        #[arg(long)]
        context: Option<String>,

        /// Decision description
        #[arg(long)]
        decision: Option<String>,

        /// Consequences of the decision
        #[arg(long)]
        consequences: Option<String>,

        /// Implementation notes
        #[arg(long)]
        implementation: Option<String>,

        /// Superseded by ADR ID
        #[arg(long)]
        superseded_by: Option<String>,
    },
    /// Delete ADR
    Delete {
        /// ADR ID
        #[arg(help = "ADR ID to delete")]
        id: String,
    },
    /// List ADRs
    List {
        /// Status filter
        #[arg(long)]
        status: Option<String>,

        /// Text search
        #[arg(long)]
        search: Option<String>,

        /// Limit results
        #[arg(long, default_value = "20")]
        limit: usize,

        /// Offset for pagination
        #[arg(long, default_value = "0")]
        offset: usize,
    },
    /// Accept an ADR
    Accept {
        /// ADR ID
        #[arg(help = "ADR ID to accept")]
        id: String,

        /// Decision description
        #[arg(long)]
        decision: String,

        /// Consequences of the decision
        #[arg(long)]
        consequences: String,
    },
    /// Add alternative to ADR
    AddAlternative {
        /// ADR ID
        #[arg(help = "ADR ID to add alternative to")]
        id: String,

        /// Alternative description
        #[arg(long)]
        description: String,
    },
    /// Add stakeholder to ADR
    AddStakeholder {
        /// ADR ID
        #[arg(help = "ADR ID to add stakeholder to")]
        id: String,

        /// Stakeholder name
        #[arg(long)]
        stakeholder: String,
    },
}

/// Create a new ADR
pub fn create_adr<S: Storage>(
    storage: &mut S,
    title: String,
    number: u32,
    context: String,
    agent: Option<String>,
) -> Result<(), EngramError> {
    let adr = ADR::new(
        title,
        number,
        agent.unwrap_or_else(|| "cli".to_string()),
        context,
    );

    let generic = adr.to_generic();
    storage.store(&generic)?;

    println!("✅ ADR created: {}", adr.id());
    display_adr(&adr);

    Ok(())
}

/// Get ADR details
pub fn get_adr<S: Storage>(storage: &S, id: &str) -> Result<(), EngramError> {
    if let Some(generic) = storage.get(id, "adr")? {
        let adr = ADR::from_generic(generic)?;
        display_adr(&adr);
    } else {
        println!("❌ ADR not found: {}", id);
    }
    Ok(())
}

/// Update ADR
pub fn update_adr<S: Storage>(
    storage: &mut S,
    id: &str,
    title: Option<String>,
    status: Option<String>,
    context: Option<String>,
    decision: Option<String>,
    consequences: Option<String>,
    implementation: Option<String>,
    superseded_by: Option<String>,
) -> Result<(), EngramError> {
    if let Some(generic) = storage.get(id, "adr")? {
        let mut adr =
            ADR::from_generic(generic).map_err(|e| EngramError::Validation(e.to_string()))?;

        let mut updated = false;

        if let Some(title) = title {
            adr.title = title;
            updated = true;
        }

        if let Some(status_str) = status {
            let new_status = match status_str.to_lowercase().as_str() {
                "proposed" => AdrStatus::Proposed,
                "accepted" => AdrStatus::Accepted,
                "deprecated" => AdrStatus::Deprecated,
                "superseded" => AdrStatus::Superseded,
                _ => {
                    println!("❌ Invalid status. Use: proposed, accepted, deprecated, superseded");
                    return Ok(());
                }
            };
            adr.status = new_status;
            updated = true;
        }

        if let Some(context) = context {
            adr.context = context;
            updated = true;
        }

        if let Some(decision) = decision {
            adr.decision = decision;
            updated = true;
        }

        if let Some(consequences) = consequences {
            adr.consequences = consequences;
            updated = true;
        }

        if let Some(implementation) = implementation {
            adr.set_implementation(implementation);
            updated = true;
        }

        if let Some(superseded_by_id) = superseded_by {
            if superseded_by_id.is_empty() {
                adr.superseded_by = None;
            } else {
                adr.superseded_by = Some(superseded_by_id);
                adr.status = AdrStatus::Superseded;
            }
            updated = true;
        }

        if !updated {
            println!("No updates specified");
            return Ok(());
        }

        adr.updated_at = chrono::Utc::now();
        let updated_generic = adr.to_generic();
        storage.store(&updated_generic)?;

        println!("✅ ADR updated: {}", id);
    } else {
        println!("❌ ADR not found: {}", id);
    }
    Ok(())
}

/// Delete ADR
pub fn delete_adr<S: Storage>(storage: &mut S, id: &str) -> Result<(), EngramError> {
    if let Some(generic) = storage.get(id, "adr")? {
        let mut adr =
            ADR::from_generic(generic).map_err(|e| EngramError::Validation(e.to_string()))?;
        adr.deprecate(None);
        let updated_generic = adr.to_generic();
        storage.store(&updated_generic)?;
        println!("✅ ADR deleted (deprecated): {}", id);
    } else {
        println!("❌ ADR not found: {}", id);
    }
    Ok(())
}

/// List ADRs
pub fn list_adrs<S: Storage>(
    storage: &S,
    status: Option<String>,
    search: Option<String>,
    limit: usize,
    offset: usize,
) -> Result<(), EngramError> {
    use crate::storage::QueryFilter;
    use serde_json::Value;
    use std::collections::HashMap;

    let mut filter = QueryFilter {
        entity_type: Some("adr".to_string()),
        text_search: search,
        limit: Some(limit),
        offset: Some(offset),
        ..Default::default()
    };

    let mut field_filters = HashMap::new();

    if let Some(status_filter) = status {
        field_filters.insert("status".to_string(), Value::String(status_filter));
    }

    if !field_filters.is_empty() {
        filter.field_filters = field_filters;
    }

    let result = storage.query(&filter)?;

    println!("📋 ADRs List");
    println!("============");

    if result.entities.is_empty() {
        println!("No ADRs found matching the criteria.");
        return Ok(());
    }

    println!(
        "Found {} ADRs (showing {} to {} of {})",
        result.total_count,
        offset + 1,
        offset + result.entities.len(),
        result.total_count
    );
    println!();

    for (i, entity) in result.entities.iter().enumerate() {
        let adr_data = &entity.data;
        let index = offset + i + 1;

        let title = adr_data
            .get("title")
            .and_then(|v| v.as_str())
            .unwrap_or("Untitled ADR");

        let number = adr_data.get("number").and_then(|v| v.as_u64()).unwrap_or(0);

        let status = adr_data
            .get("status")
            .and_then(|v| v.as_str())
            .unwrap_or("proposed");

        let status_symbol = match status {
            "accepted" => "✅",
            "rejected" => "❌",
            "deprecated" => "🗑️",
            "superseded" => "⬆️",
            "proposed" => "📝",
            _ => "❓",
        };

        println!("{}. {} ADR-{:03} {}", index, status_symbol, number, title);

        println!("   ID: {}", entity.id);

        if let Some(context) = adr_data.get("context").and_then(|v| v.as_str()) {
            let truncated = if context.len() > 80 {
                format!("{}...", &context[..77])
            } else {
                context.to_string()
            };
            println!("   📝 Context: {}", truncated);
        }

        if status == "accepted" {
            if let Some(decision) = adr_data.get("decision").and_then(|v| v.as_str()) {
                let truncated = if decision.len() > 60 {
                    format!("{}...", &decision[..57])
                } else {
                    decision.to_string()
                };
                println!("   ✅ Decision: {}", truncated);
            }
        }

        if let Some(alternatives) = adr_data.get("alternatives").and_then(|v| v.as_array()) {
            if !alternatives.is_empty() {
                println!("   🔀 {} alternative(s) considered", alternatives.len());
            }
        }

        println!(
            "   📊 Status: {} | 👤 Agent: {} | 📅 {}",
            status,
            entity.agent,
            entity.timestamp.format("%Y-%m-%d %H:%M")
        );

        println!();
    }

    if result.has_more {
        println!("💡 Use --offset {} to see more ADRs", offset + limit);
    }

    println!("💡 Use 'engram adr get <id>' to view full ADR details");
    println!("💡 Use 'engram adr accept <id>' to accept a proposed ADR");

    Ok(())
}

/// Accept an ADR
pub fn accept_adr<S: Storage>(
    storage: &mut S,
    id: &str,
    decision: String,
    consequences: String,
) -> Result<(), EngramError> {
    if let Some(generic) = storage.get(id, "adr")? {
        let mut adr =
            ADR::from_generic(generic).map_err(|e| EngramError::Validation(e.to_string()))?;
        adr.accept(decision, consequences);
        let updated_generic = adr.to_generic();
        storage.store(&updated_generic)?;
        println!("✅ ADR accepted: {}", id);
        display_adr(&adr);
    } else {
        println!("❌ ADR not found: {}", id);
    }
    Ok(())
}

/// Add alternative to ADR
pub fn add_alternative<S: Storage>(
    storage: &mut S,
    id: &str,
    description: String,
) -> Result<(), EngramError> {
    if let Some(generic) = storage.get(id, "adr")? {
        let mut adr =
            ADR::from_generic(generic).map_err(|e| EngramError::Validation(e.to_string()))?;
        let alt_id = adr.add_alternative(description);
        let updated_generic = adr.to_generic();
        storage.store(&updated_generic)?;
        println!("✅ Alternative added to ADR {}: {}", id, alt_id);
    } else {
        println!("❌ ADR not found: {}", id);
    }
    Ok(())
}

/// Add stakeholder to ADR
pub fn add_stakeholder<S: Storage>(
    storage: &mut S,
    id: &str,
    stakeholder: String,
) -> Result<(), EngramError> {
    if let Some(generic) = storage.get(id, "adr")? {
        let mut adr =
            ADR::from_generic(generic).map_err(|e| EngramError::Validation(e.to_string()))?;
        adr.add_stakeholder(stakeholder.clone());
        let updated_generic = adr.to_generic();
        storage.store(&updated_generic)?;
        println!("✅ Stakeholder added to ADR {}: {}", id, stakeholder);
    } else {
        println!("❌ ADR not found: {}", id);
    }
    Ok(())
}

/// Display ADR information
fn display_adr(adr: &ADR) {
    println!("📋 ADR: {}", adr.id());
    println!("🔢 Number: ADR-{:03}", adr.number);
    println!("📝 Title: {}", adr.title);
    println!("📊 Status: {:?}", adr.status);
    println!("🤖 Agent: {}", adr.agent);
    println!("🕐 Created: {}", adr.created_at.format("%Y-%m-%d %H:%M"));
    println!("🔄 Updated: {}", adr.updated_at.format("%Y-%m-%d %H:%M"));

    if let Some(decision_date) = adr.decision_date {
        println!(
            "📅 Decision Date: {}",
            decision_date.format("%Y-%m-%d %H:%M")
        );
    }

    println!("📝 Context:");
    println!("{}", adr.context);

    if !adr.decision.is_empty() {
        println!("✅ Decision:");
        println!("{}", adr.decision);
    }

    if !adr.consequences.is_empty() {
        println!("🎯 Consequences:");
        println!("{}", adr.consequences);
    }

    if let Some(ref implementation) = adr.implementation {
        println!("🛠️ Implementation:");
        println!("{}", implementation);
    }

    if !adr.alternatives.is_empty() {
        println!("🔄 Alternatives: {}", adr.alternatives.len());
        for (i, alt) in adr.alternatives.iter().enumerate() {
            println!("  {}. {}", i + 1, alt.description);
            if !alt.pros.is_empty() {
                println!("     ✅ Pros: {:?}", alt.pros);
            }
            if !alt.cons.is_empty() {
                println!("     ❌ Cons: {:?}", alt.cons);
            }
            if let Some(ref reason) = alt.rejection_reason {
                println!("     🚫 Rejected: {}", reason);
            }
        }
    }

    if !adr.stakeholders.is_empty() {
        println!("👥 Stakeholders: {:?}", adr.stakeholders);
    }

    if !adr.related_adrs.is_empty() {
        println!("🔗 Related ADRs: {:?}", adr.related_adrs);
    }

    if let Some(ref superseded_by) = adr.superseded_by {
        println!("🔗 Superseded By: {}", superseded_by);
    }

    if !adr.supersedes.is_empty() {
        println!("📋 Supersedes: {:?}", adr.supersedes);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::{AdrStatus, Entity, ADR};
    use crate::storage::MemoryStorage;
    use crate::storage::Storage;

    #[test]
    fn test_create_adr() {
        let mut storage = MemoryStorage::new("test-agent");
        let title = "Use Rust for CLI".to_string();
        let number = 1;
        let context = "We need a performant CLI tool.".to_string();
        let agent = Some("test-agent".to_string());

        let result = create_adr(&mut storage, title, number, context, agent);
        assert!(result.is_ok());

        let query_result = storage.query_by_type("adr", None, None, None).unwrap();
        assert_eq!(query_result.total_count, 1);

        let entity = &query_result.entities[0];
        assert_eq!(entity.data.get("title").unwrap(), "Use Rust for CLI");
        assert_eq!(entity.data.get("number").unwrap(), 1);
    }

    #[test]
    fn test_get_adr() {
        let mut storage = MemoryStorage::new("test-agent");

        create_adr(
            &mut storage,
            "Test ADR".to_string(),
            1,
            "Context".to_string(),
            None,
        )
        .unwrap();

        let query_result = storage.query_by_type("adr", None, None, None).unwrap();
        let id = &query_result.entities[0].id;

        let result = get_adr(&storage, id);
        assert!(result.is_ok());

        let result = get_adr(&storage, "non-existent");
        assert!(result.is_ok()); // Should assume not found message printed
    }

    #[test]
    fn test_update_adr() {
        let mut storage = MemoryStorage::new("test-agent");

        create_adr(
            &mut storage,
            "Old Title".to_string(),
            1,
            "Context".to_string(),
            None,
        )
        .unwrap();

        let query_result = storage.query_by_type("adr", None, None, None).unwrap();
        let id = &query_result.entities[0].id;

        let result = update_adr(
            &mut storage,
            id,
            Some("New Title".to_string()),
            Some("accepted".to_string()),
            Some("New Context".to_string()),
            Some("Decision".to_string()),
            Some("Consequences".to_string()),
            Some("Implementation".to_string()),
            None,
        );
        assert!(result.is_ok());

        let generic = storage.get(id, "adr").unwrap().unwrap();
        let adr = ADR::from_generic(generic).unwrap();

        assert_eq!(adr.title, "New Title");
        assert!(matches!(adr.status, AdrStatus::Accepted));
        assert_eq!(adr.context, "New Context");
        assert_eq!(adr.decision, "Decision");
        assert_eq!(adr.consequences, "Consequences");
        assert_eq!(adr.implementation.unwrap(), "Implementation");
    }

    #[test]
    fn test_delete_adr() {
        let mut storage = MemoryStorage::new("test-agent");

        create_adr(
            &mut storage,
            "To Delete".to_string(),
            1,
            "Context".to_string(),
            None,
        )
        .unwrap();

        let query_result = storage.query_by_type("adr", None, None, None).unwrap();
        let id = &query_result.entities[0].id;

        let result = delete_adr(&mut storage, id);
        assert!(result.is_ok());

        let generic = storage.get(id, "adr").unwrap().unwrap();
        let adr = ADR::from_generic(generic).unwrap();
        assert!(matches!(adr.status, AdrStatus::Deprecated));
    }

    #[test]
    fn test_accept_adr() {
        let mut storage = MemoryStorage::new("test-agent");

        create_adr(
            &mut storage,
            "To Accept".to_string(),
            1,
            "Context".to_string(),
            None,
        )
        .unwrap();

        let query_result = storage.query_by_type("adr", None, None, None).unwrap();
        let id = &query_result.entities[0].id;

        let result = accept_adr(
            &mut storage,
            id,
            "We decide to do X".to_string(),
            "X will happen".to_string(),
        );
        assert!(result.is_ok());

        let generic = storage.get(id, "adr").unwrap().unwrap();
        let adr = ADR::from_generic(generic).unwrap();

        assert!(matches!(adr.status, AdrStatus::Accepted));
        assert_eq!(adr.decision, "We decide to do X");
        assert_eq!(adr.consequences, "X will happen");
        assert!(adr.decision_date.is_some());
    }

    #[test]
    fn test_add_alternative() {
        let mut storage = MemoryStorage::new("test-agent");

        create_adr(
            &mut storage,
            "With Alternatives".to_string(),
            1,
            "Context".to_string(),
            None,
        )
        .unwrap();

        let query_result = storage.query_by_type("adr", None, None, None).unwrap();
        let id = &query_result.entities[0].id;

        let result = add_alternative(&mut storage, id, "Option A".to_string());
        assert!(result.is_ok());

        let generic = storage.get(id, "adr").unwrap().unwrap();
        let adr = ADR::from_generic(generic).unwrap();

        assert_eq!(adr.alternatives.len(), 1);
        assert_eq!(adr.alternatives[0].description, "Option A");
    }

    #[test]
    fn test_add_stakeholder() {
        let mut storage = MemoryStorage::new("test-agent");

        create_adr(
            &mut storage,
            "With Stakeholders".to_string(),
            1,
            "Context".to_string(),
            None,
        )
        .unwrap();

        let query_result = storage.query_by_type("adr", None, None, None).unwrap();
        let id = &query_result.entities[0].id;

        let result = add_stakeholder(&mut storage, id, "Dev Team".to_string());
        assert!(result.is_ok());

        let generic = storage.get(id, "adr").unwrap().unwrap();
        let adr = ADR::from_generic(generic).unwrap();

        assert_eq!(adr.stakeholders.len(), 1);
        assert_eq!(adr.stakeholders[0], "Dev Team");
    }

    #[test]
    fn test_list_adrs() {
        let mut storage = MemoryStorage::new("test-agent");

        create_adr(
            &mut storage,
            "ADR 1".to_string(),
            1,
            "Context".to_string(),
            None,
        )
        .unwrap();

        create_adr(
            &mut storage,
            "ADR 2".to_string(),
            2,
            "Context".to_string(),
            None,
        )
        .unwrap();

        let result = list_adrs(&storage, None, None, 10, 0);
        assert!(result.is_ok());

        let result = list_adrs(&storage, Some("proposed".to_string()), None, 10, 0);
        assert!(result.is_ok());
    }
}

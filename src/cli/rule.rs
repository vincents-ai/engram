use crate::entities::{Entity, Rule, RulePriority, RuleStatus, RuleType};
use crate::error::EngramError;
use crate::storage::Storage;
use clap::Subcommand;

/// Rule commands
#[derive(Debug, Subcommand)]
pub enum RuleCommands {
    /// Create a new rule
    Create {
        /// Rule title
        #[arg(long, short)]
        title: String,

        /// Rule description
        #[arg(long)]
        description: Option<String>,

        /// Rule type (validation, transformation, enforcement, notification)
        #[arg(long, default_value = "validation")]
        rule_type: String,

        /// Priority level (low, medium, high, critical)
        #[arg(long, default_value = "medium")]
        priority: String,

        /// Entity types (comma-separated)
        #[arg(long)]
        entity_types: Option<String>,

        /// Rule condition (JSON)
        #[arg(long, default_value = "{}")]
        condition: String,

        /// Rule action (JSON)
        #[arg(long, default_value = "{}")]
        action: String,

        /// Agent to assign
        #[arg(long, short)]
        agent: Option<String>,
    },
    /// Get rule details
    Get {
        /// Rule ID
        #[arg(help = "Rule ID to retrieve")]
        id: String,
    },
    /// Update rule
    Update {
        /// Rule ID
        #[arg(help = "Rule ID to update")]
        id: String,

        /// Rule title
        #[arg(long)]
        title: Option<String>,

        /// Rule description
        #[arg(long)]
        description: Option<String>,

        /// Rule type
        #[arg(long)]
        rule_type: Option<String>,

        /// Priority level
        #[arg(long)]
        priority: Option<String>,

        /// Entity types (comma-separated)
        #[arg(long)]
        entity_types: Option<String>,

        /// Rule condition (JSON)
        #[arg(long)]
        condition: Option<String>,

        /// Rule action (JSON)
        #[arg(long)]
        action: Option<String>,

        /// Rule status (active, inactive, deprecated)
        #[arg(long)]
        status: Option<String>,
    },
    /// Delete rule
    Delete {
        /// Rule ID
        #[arg(help = "Rule ID to delete")]
        id: String,
    },
    /// List rules
    List {
        /// Rule type filter
        #[arg(long)]
        rule_type: Option<String>,

        /// Priority filter
        #[arg(long)]
        priority: Option<String>,

        /// Entity type filter
        #[arg(long)]
        entity_type: Option<String>,

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
    /// Execute rule
    Execute {
        /// Rule ID
        #[arg(help = "Rule ID to execute")]
        id: String,

        /// Target entity ID
        #[arg(long)]
        entity_id: String,

        /// Target entity type
        #[arg(long)]
        entity_type: String,
    },
}

/// Create a new rule
pub fn create_rule<S: Storage>(
    storage: &mut S,
    title: String,
    description: Option<String>,
    rule_type: String,
    priority: String,
    entity_types: Option<String>,
    condition: String,
    action: String,
    agent: Option<String>,
) -> Result<(), EngramError> {
    let rule_type = match rule_type.to_lowercase().as_str() {
        "validation" => RuleType::Validation,
        "transformation" => RuleType::Transformation,
        "enforcement" => RuleType::Enforcement,
        "notification" => RuleType::Notification,
        _ => {
            println!(
                "❌ Invalid rule type. Use: validation, transformation, enforcement, notification"
            );
            return Ok(());
        }
    };

    let priority = match priority.to_lowercase().as_str() {
        "low" => RulePriority::Low,
        "medium" => RulePriority::Medium,
        "high" => RulePriority::High,
        "critical" => RulePriority::Critical,
        _ => {
            println!("❌ Invalid priority. Use: low, medium, high, critical");
            return Ok(());
        }
    };

    let condition_json: serde_json::Value = serde_json::from_str(&condition)
        .map_err(|e| EngramError::Validation(format!("Invalid JSON in condition: {}", e)))?;

    let action_json: serde_json::Value = serde_json::from_str(&action)
        .map_err(|e| EngramError::Validation(format!("Invalid JSON in action: {}", e)))?;

    let mut rule = Rule::new(
        title,
        description.unwrap_or_default(),
        rule_type,
        priority,
        agent.unwrap_or_else(|| "cli".to_string()),
        condition_json,
        action_json,
    );

    if let Some(entity_types_str) = entity_types {
        let types: Vec<String> = entity_types_str
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        rule.entity_types = types;
    }

    let generic = rule.to_generic();
    storage.store(&generic)?;

    println!("✅ Rule created: {}", rule.id());
    display_rule(&rule);

    Ok(())
}

/// Get rule details
pub fn get_rule<S: Storage>(storage: &S, id: &str) -> Result<(), EngramError> {
    if let Some(generic) = storage.get(id, "rule")? {
        let rule =
            Rule::from_generic(generic).map_err(|e| EngramError::Validation(e.to_string()))?;
        display_rule(&rule);
    } else {
        println!("❌ Rule not found: {}", id);
    }
    Ok(())
}

/// Update rule
pub fn update_rule<S: Storage>(
    storage: &mut S,
    id: &str,
    title: Option<String>,
    description: Option<String>,
    rule_type: Option<String>,
    priority: Option<String>,
    entity_types: Option<String>,
    condition: Option<String>,
    action: Option<String>,
    status: Option<String>,
) -> Result<(), EngramError> {
    if let Some(generic) = storage.get(id, "rule")? {
        let mut rule =
            Rule::from_generic(generic).map_err(|e| EngramError::Validation(e.to_string()))?;

        let mut updated = false;

        if let Some(title) = title {
            rule.title = title;
            updated = true;
        }

        if let Some(description) = description {
            rule.description = description;
            updated = true;
        }

        if let Some(rule_type_str) = rule_type {
            let new_rule_type = match rule_type_str.to_lowercase().as_str() {
                "validation" => RuleType::Validation,
                "transformation" => RuleType::Transformation,
                "enforcement" => RuleType::Enforcement,
                "notification" => RuleType::Notification,
                _ => {
                    println!("❌ Invalid rule type. Use: validation, transformation, enforcement, notification");
                    return Ok(());
                }
            };
            rule.rule_type = new_rule_type;
            updated = true;
        }

        if let Some(priority_str) = priority {
            let new_priority = match priority_str.to_lowercase().as_str() {
                "low" => RulePriority::Low,
                "medium" => RulePriority::Medium,
                "high" => RulePriority::High,
                "critical" => RulePriority::Critical,
                _ => {
                    println!("❌ Invalid priority. Use: low, medium, high, critical");
                    return Ok(());
                }
            };
            rule.priority = new_priority;
            updated = true;
        }

        if let Some(entity_types_str) = entity_types {
            let types: Vec<String> = entity_types_str
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            rule.entity_types = types;
            updated = true;
        }

        if let Some(condition_str) = condition {
            let condition_json: serde_json::Value =
                serde_json::from_str(&condition_str).map_err(|e| {
                    EngramError::Validation(format!("Invalid JSON in condition: {}", e))
                })?;
            rule.condition = condition_json;
            updated = true;
        }

        if let Some(action_str) = action {
            let action_json: serde_json::Value = serde_json::from_str(&action_str)
                .map_err(|e| EngramError::Validation(format!("Invalid JSON in action: {}", e)))?;
            rule.action = action_json;
            updated = true;
        }

        if let Some(status_str) = status {
            let new_status = match status_str.to_lowercase().as_str() {
                "active" => RuleStatus::Active,
                "inactive" => RuleStatus::Inactive,
                "deprecated" => RuleStatus::Deprecated,
                _ => {
                    println!("❌ Invalid status. Use: active, inactive, deprecated");
                    return Ok(());
                }
            };
            rule.status = new_status;
            updated = true;
        }

        if !updated {
            println!("No updates specified");
            return Ok(());
        }

        rule.updated_at = chrono::Utc::now();
        let updated_generic = rule.to_generic();
        storage.store(&updated_generic)?;

        println!("✅ Rule updated: {}", id);
    } else {
        println!("❌ Rule not found: {}", id);
    }
    Ok(())
}

/// Delete rule
pub fn delete_rule<S: Storage>(storage: &mut S, id: &str) -> Result<(), EngramError> {
    if let Some(generic) = storage.get(id, "rule")? {
        let mut rule =
            Rule::from_generic(generic).map_err(|e| EngramError::Validation(e.to_string()))?;
        rule.deactivate();
        let updated_generic = rule.to_generic();
        storage.store(&updated_generic)?;
        println!("✅ Rule deleted (deactivated): {}", id);
    } else {
        println!("❌ Rule not found: {}", id);
    }
    Ok(())
}

/// List rules
pub fn list_rules<S: Storage>(
    storage: &S,
    rule_type: Option<String>,
    priority: Option<String>,
    entity_type: Option<String>,
    status: Option<String>,
    search: Option<String>,
    limit: usize,
    offset: usize,
) -> Result<(), EngramError> {
    use crate::storage::QueryFilter;
    use serde_json::Value;
    use std::collections::HashMap;

    let mut filter = QueryFilter {
        entity_type: Some("rule".to_string()),
        text_search: search,
        limit: Some(limit),
        offset: Some(offset),
        ..Default::default()
    };

    let mut field_filters = HashMap::new();

    if let Some(rule_type_filter) = rule_type {
        field_filters.insert("rule_type".to_string(), Value::String(rule_type_filter));
    }

    if let Some(priority_filter) = priority {
        field_filters.insert("priority".to_string(), Value::String(priority_filter));
    }

    if let Some(entity_type_filter) = entity_type {
        field_filters.insert("applies_to".to_string(), Value::String(entity_type_filter));
    }

    if let Some(status_filter) = status {
        field_filters.insert(
            "active".to_string(),
            Value::Bool(status_filter.to_lowercase() == "active"),
        );
    }

    if !field_filters.is_empty() {
        filter.field_filters = field_filters;
    }

    let result = storage.query(&filter)?;

    println!("📋 Rules List");
    println!("==============");

    if result.entities.is_empty() {
        println!("No rules found matching the criteria.");
        return Ok(());
    }

    println!(
        "Found {} rules (showing {} to {} of {})",
        result.total_count,
        offset + 1,
        offset + result.entities.len(),
        result.total_count
    );
    println!();

    for (i, entity) in result.entities.iter().enumerate() {
        let rule_data = &entity.data;
        let index = offset + i + 1;

        let name = rule_data
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("Unnamed Rule");

        let rule_type = rule_data
            .get("rule_type")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");

        let priority = rule_data
            .get("priority")
            .and_then(|v| v.as_str())
            .unwrap_or("medium");

        let active = rule_data
            .get("active")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);

        let status_symbol = if active { "🟢" } else { "🔴" };
        let priority_symbol = match priority {
            "high" => "🔥",
            "medium" => "⚡",
            "low" => "📍",
            _ => "❓",
        };

        println!(
            "{}. {} {} [{}] {}",
            index,
            status_symbol,
            priority_symbol,
            rule_type.to_uppercase(),
            name
        );

        println!("   ID: {}", entity.id);

        if let Some(description) = rule_data.get("description").and_then(|v| v.as_str()) {
            let truncated = if description.len() > 80 {
                format!("{}...", &description[..77])
            } else {
                description.to_string()
            };
            println!("   📝 {}", truncated);
        }

        if let Some(applies_to) = rule_data.get("applies_to").and_then(|v| v.as_str()) {
            println!("   🎯 Applies to: {}", applies_to);
        }

        println!(
            "   👤 Agent: {} | 📅 {}",
            entity.agent,
            entity.timestamp.format("%Y-%m-%d %H:%M")
        );

        println!();
    }

    if result.has_more {
        println!("💡 Use --offset {} to see more rules", offset + limit);
    }

    println!("💡 Use 'engram rule get <id>' to view full rule details");
    println!("💡 Use 'engram rule execute <id> <entity-id>' to execute a rule");

    Ok(())
}

/// Execute rule
pub fn execute_rule<S: Storage>(
    storage: &mut S,
    id: &str,
    entity_id: String,
    entity_type: String,
) -> Result<(), EngramError> {
    if let Some(generic) = storage.get(id, "rule")? {
        let mut rule =
            Rule::from_generic(generic).map_err(|e| EngramError::Validation(e.to_string()))?;

        if let Some(target_entity) = storage.get(&entity_id, &entity_type)? {
            let result = rule.execute(&target_entity);

            println!("✅ Rule executed: {}", id);
            println!("📋 Target entity: {} ({})", entity_id, entity_type);
            println!("📊 Result: {:?}", result);

            let updated_generic = rule.to_generic();
            storage.store(&updated_generic)?;
        } else {
            println!(
                "❌ Target entity not found: {} ({})",
                entity_id, entity_type
            );
        }
    } else {
        println!("❌ Rule not found: {}", id);
    }
    Ok(())
}

/// Display rule information
fn display_rule(rule: &Rule) {
    println!("📋 Rule: {}", rule.id());
    println!("📝 Title: {}", rule.title);
    println!("📄 Description: {}", rule.description);
    println!("🔧 Type: {:?}", rule.rule_type);
    println!("📊 Status: {:?}", rule.status);
    println!("⚡ Priority: {:?}", rule.priority);
    println!("🤖 Agent: {}", rule.agent);
    if !rule.entity_types.is_empty() {
        println!("🎯 Entity Types: {:?}", rule.entity_types);
    }
    println!("📝 Condition: {}", rule.condition);
    println!("⚡ Action: {}", rule.action);
    println!("🕐 Created: {}", rule.created_at.format("%Y-%m-%d %H:%M"));
    println!("🔄 Updated: {}", rule.updated_at.format("%Y-%m-%d %H:%M"));
    if !rule.execution_history.is_empty() {
        println!("📊 Executions: {}", rule.execution_history.len());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::{MemoryStorage, Storage};
    use crate::entities::{Rule, RuleStatus, RuleType, RulePriority};

    fn create_test_storage() -> MemoryStorage {
        MemoryStorage::new("default")
    }

    #[test]
    fn test_create_rule() {
        let mut storage = create_test_storage();
        create_rule(
            &mut storage,
            "Test Rule".to_string(),
            Some("Description".to_string()),
            "validation".to_string(),
            "medium".to_string(),
            Some("task".to_string()),
            r#"{"field": "status", "operator": "eq", "value": "done"}"#.to_string(),
            r#"{"type": "notify", "message": "Task done"}"#.to_string(),
            Some("agent1".to_string()),
        ).unwrap();

        let rules = storage.query_by_agent("agent1", Some("rule")).unwrap();
        assert_eq!(rules.len(), 1);
        let rule = Rule::from_generic(rules[0].clone()).unwrap();
        assert_eq!(rule.title, "Test Rule");
        assert_eq!(rule.rule_type, RuleType::Validation);
    }

    #[test]
    fn test_update_rule() {
        let mut storage = create_test_storage();
        create_rule(
            &mut storage,
            "Old Rule".to_string(),
            None,
            "validation".to_string(),
            "low".to_string(),
            None,
            "{}".to_string(),
            "{}".to_string(),
            Some("agent1".to_string()),
        ).unwrap();

        let rules = storage.query_by_agent("agent1", Some("rule")).unwrap();
        let id = &rules[0].id;

        update_rule(
            &mut storage,
            id,
            Some("New Title".to_string()),
            Some("New Desc".to_string()),
            Some("enforcement".to_string()),
            Some("high".to_string()),
            None,
            None,
            None,
            Some("deprecated".to_string()),
        ).unwrap();

        let generic = storage.get(id, "rule").unwrap().unwrap();
        let rule = Rule::from_generic(generic).unwrap();
        
        assert_eq!(rule.title, "New Title");
        assert_eq!(rule.rule_type, RuleType::Enforcement);
        assert_eq!(rule.status, RuleStatus::Deprecated);
    }

    #[test]
    fn test_delete_rule() {
        let mut storage = create_test_storage();
        create_rule(
            &mut storage,
            "Delete Me".to_string(),
            None,
            "validation".to_string(),
            "low".to_string(),
            None,
            "{}".to_string(),
            "{}".to_string(),
            Some("agent1".to_string()),
        ).unwrap();

        let rules = storage.query_by_agent("agent1", Some("rule")).unwrap();
        let id = &rules[0].id;

        delete_rule(&mut storage, id).unwrap();
        
        // Deletion marks as inactive/deactivated rather than removing
        let generic = storage.get(id, "rule").unwrap().unwrap();
        let rule = Rule::from_generic(generic).unwrap();
        assert_eq!(rule.status, RuleStatus::Inactive);
    }

    #[test]
    fn test_list_rules() {
        let mut storage = create_test_storage();
        create_rule(
            &mut storage,
            "R1".to_string(), None, "validation".to_string(), "high".to_string(), None, "{}".to_string(), "{}".to_string(), None
        ).unwrap();
        
        list_rules(&storage, None, None, None, None, None, 10, 0).unwrap();
        list_rules(&storage, Some("validation".to_string()), None, None, None, None, 10, 0).unwrap();
    }
}

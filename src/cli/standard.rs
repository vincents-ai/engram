use crate::entities::{Entity, Standard, StandardCategory, StandardRequirement, StandardStatus};
use crate::error::EngramError;
use crate::storage::Storage;
use chrono::Utc;
use clap::Subcommand;

/// Standard commands
#[derive(Debug, Subcommand)]
pub enum StandardCommands {
    /// Create a new standard
    Create {
        /// Standard title
        #[arg(long, short)]
        title: String,

        /// Standard description
        #[arg(long)]
        description: Option<String>,

        /// Standard category (coding, testing, documentation, security, performance, process, architecture)
        #[arg(long, default_value = "process")]
        category: String,

        /// Version
        #[arg(long, default_value = "1.0")]
        version: String,

        /// Effective date (ISO 8601 format, defaults to now)
        #[arg(long)]
        effective_date: Option<String>,

        /// Agent to assign
        #[arg(long, short)]
        agent: Option<String>,
    },
    /// Get standard details
    Get {
        /// Standard ID
        #[arg(help = "Standard ID to retrieve")]
        id: String,
    },
    /// Update standard
    Update {
        /// Standard ID
        #[arg(help = "Standard ID to update")]
        id: String,

        /// Standard title
        #[arg(long)]
        title: Option<String>,

        /// Standard description
        #[arg(long)]
        description: Option<String>,

        /// Standard category
        #[arg(long)]
        category: Option<String>,

        /// Version
        #[arg(long)]
        version: Option<String>,

        /// Standard status (draft, active, deprecated, superseded)
        #[arg(long)]
        status: Option<String>,

        /// Effective date (ISO 8601 format)
        #[arg(long)]
        effective_date: Option<String>,

        /// Superseded by standard ID
        #[arg(long)]
        superseded_by: Option<String>,
    },
    /// Delete standard
    Delete {
        /// Standard ID
        #[arg(help = "Standard ID to delete")]
        id: String,
    },
    /// List standards
    List {
        /// Category filter
        #[arg(long)]
        category: Option<String>,

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
    /// Add requirement to standard
    AddRequirement {
        /// Standard ID
        #[arg(help = "Standard ID to add requirement to")]
        id: String,

        /// Requirement title
        #[arg(long)]
        title: String,

        /// Requirement description
        #[arg(long)]
        description: String,

        /// Whether requirement is mandatory
        #[arg(long, action)]
        mandatory: bool,

        /// Priority level (low, medium, high, critical)
        #[arg(long, default_value = "medium")]
        priority: String,

        /// Evidence required
        #[arg(long, action)]
        evidence_required: bool,
    },
}

/// Create a new standard
pub fn create_standard<S: Storage>(
    storage: &mut S,
    title: String,
    description: Option<String>,
    category: String,
    version: String,
    effective_date: Option<String>,
    agent: Option<String>,
) -> Result<(), EngramError> {
    let category = match category.to_lowercase().as_str() {
        "coding" => StandardCategory::Coding,
        "testing" => StandardCategory::Testing,
        "documentation" => StandardCategory::Documentation,
        "security" => StandardCategory::Security,
        "performance" => StandardCategory::Performance,
        "process" => StandardCategory::Process,
        "architecture" => StandardCategory::Architecture,
        _ => {
            println!("❌ Invalid category. Use: coding, testing, documentation, security, performance, process, architecture");
            return Ok(());
        }
    };

    let effective_date = match effective_date {
        Some(date_str) => chrono::DateTime::parse_from_rfc3339(&date_str)
            .map_err(|e| EngramError::Validation(format!("Invalid date format: {}", e)))?
            .with_timezone(&Utc),
        None => Utc::now(),
    };

    let standard = Standard::new(
        title,
        description.unwrap_or_default(),
        category,
        version,
        agent.unwrap_or_else(|| "cli".to_string()),
        effective_date,
    );

    let generic = standard.to_generic();
    storage.store(&generic)?;

    println!("✅ Standard created: {}", standard.id());
    display_standard(&standard);

    Ok(())
}

/// Get standard details
pub fn get_standard<S: Storage>(storage: &S, id: &str) -> Result<(), EngramError> {
    if let Some(generic) = storage.get(id, "standard")? {
        let standard =
            Standard::from_generic(generic).map_err(|e| EngramError::Validation(e.to_string()))?;
        display_standard(&standard);
    } else {
        println!("❌ Standard not found: {}", id);
    }
    Ok(())
}

/// Update standard
pub fn update_standard<S: Storage>(
    storage: &mut S,
    id: &str,
    title: Option<String>,
    description: Option<String>,
    category: Option<String>,
    version: Option<String>,
    status: Option<String>,
    effective_date: Option<String>,
    superseded_by: Option<String>,
) -> Result<(), EngramError> {
    if let Some(generic) = storage.get(id, "standard")? {
        let mut standard =
            Standard::from_generic(generic).map_err(|e| EngramError::Validation(e.to_string()))?;

        let mut updated = false;

        if let Some(title) = title {
            standard.title = title;
            updated = true;
        }

        if let Some(description) = description {
            standard.description = description;
            updated = true;
        }

        if let Some(category_str) = category {
            let new_category = match category_str.to_lowercase().as_str() {
                "coding" => StandardCategory::Coding,
                "testing" => StandardCategory::Testing,
                "documentation" => StandardCategory::Documentation,
                "security" => StandardCategory::Security,
                "performance" => StandardCategory::Performance,
                "process" => StandardCategory::Process,
                "architecture" => StandardCategory::Architecture,
                _ => {
                    println!("❌ Invalid category. Use: coding, testing, documentation, security, performance, process, architecture");
                    return Ok(());
                }
            };
            standard.category = new_category;
            updated = true;
        }

        if let Some(version) = version {
            standard.version = version;
            updated = true;
        }

        if let Some(status_str) = status {
            let new_status = match status_str.to_lowercase().as_str() {
                "draft" => StandardStatus::Draft,
                "active" => StandardStatus::Active,
                "deprecated" => StandardStatus::Deprecated,
                "superseded" => StandardStatus::Superseded,
                _ => {
                    println!("❌ Invalid status. Use: draft, active, deprecated, superseded");
                    return Ok(());
                }
            };
            standard.status = new_status;
            updated = true;
        }

        if let Some(date_str) = effective_date {
            let new_effective_date = chrono::DateTime::parse_from_rfc3339(&date_str)
                .map_err(|e| EngramError::Validation(format!("Invalid date format: {}", e)))?
                .with_timezone(&Utc);
            standard.effective_date = new_effective_date;
            updated = true;
        }

        if let Some(superseded_by_id) = superseded_by {
            if superseded_by_id.is_empty() {
                standard.superseded_by = None;
            } else {
                standard.superseded_by = Some(superseded_by_id);
                standard.status = StandardStatus::Superseded;
            }
            updated = true;
        }

        if !updated {
            println!("No updates specified");
            return Ok(());
        }

        standard.updated_at = chrono::Utc::now();
        let updated_generic = standard.to_generic();
        storage.store(&updated_generic)?;

        println!("✅ Standard updated: {}", id);
    } else {
        println!("❌ Standard not found: {}", id);
    }
    Ok(())
}

/// Delete standard
pub fn delete_standard<S: Storage>(storage: &mut S, id: &str) -> Result<(), EngramError> {
    if let Some(generic) = storage.get(id, "standard")? {
        let mut standard =
            Standard::from_generic(generic).map_err(|e| EngramError::Validation(e.to_string()))?;
        standard.deprecate(None);
        let updated_generic = standard.to_generic();
        storage.store(&updated_generic)?;
        println!("✅ Standard deleted (deprecated): {}", id);
    } else {
        println!("❌ Standard not found: {}", id);
    }
    Ok(())
}

/// List standards
pub fn list_standards<S: Storage>(
    storage: &S,
    category: Option<String>,
    status: Option<String>,
    search: Option<String>,
    limit: usize,
    offset: usize,
) -> Result<(), EngramError> {
    use crate::storage::QueryFilter;
    use serde_json::Value;
    use std::collections::HashMap;

    let mut filter = QueryFilter {
        entity_type: Some("standard".to_string()),
        text_search: search,
        limit: Some(limit),
        offset: Some(offset),
        ..Default::default()
    };

    let mut field_filters = HashMap::new();

    if let Some(category_filter) = category {
        field_filters.insert("category".to_string(), Value::String(category_filter));
    }

    if let Some(status_filter) = status {
        field_filters.insert("status".to_string(), Value::String(status_filter));
    }

    if !field_filters.is_empty() {
        filter.field_filters = field_filters;
    }

    let result = storage.query(&filter)?;

    println!("📋 Standards List");
    println!("=================");

    if result.entities.is_empty() {
        println!("No standards found matching the criteria.");
        return Ok(());
    }

    println!(
        "Found {} standards (showing {} to {} of {})",
        result.total_count,
        offset + 1,
        offset + result.entities.len(),
        result.total_count
    );
    println!();

    for (i, entity) in result.entities.iter().enumerate() {
        let standard_data = &entity.data;
        let index = offset + i + 1;

        let title = standard_data
            .get("title")
            .and_then(|v| v.as_str())
            .unwrap_or("Untitled Standard");

        let category = standard_data
            .get("category")
            .and_then(|v| v.as_str())
            .unwrap_or("general");

        let status = standard_data
            .get("status")
            .and_then(|v| v.as_str())
            .unwrap_or("draft");

        let version = standard_data
            .get("version")
            .and_then(|v| v.as_str())
            .unwrap_or("1.0");

        let status_symbol = match status {
            "active" => "🟢",
            "draft" => "🟡",
            "deprecated" => "🔴",
            "review" => "🔵",
            _ => "⚪",
        };

        let category_symbol = match category {
            "coding" => "💻",
            "security" => "🔒",
            "quality" => "⚡",
            "process" => "🔄",
            "documentation" => "📝",
            _ => "📋",
        };

        println!(
            "{}. {} {} {} v{}",
            index, status_symbol, category_symbol, title, version
        );

        println!("   ID: {}", entity.id);

        if let Some(description) = standard_data.get("description").and_then(|v| v.as_str()) {
            let truncated = if description.len() > 80 {
                format!("{}...", &description[..77])
            } else {
                description.to_string()
            };
            println!("   📄 {}", truncated);
        }

        println!("   🏷️  Category: {} | 📊 Status: {}", category, status);

        if let Some(requirements) = standard_data.get("requirements").and_then(|v| v.as_array()) {
            println!("   📋 {} requirements", requirements.len());
        }

        println!(
            "   👤 Agent: {} | 📅 {}",
            entity.agent,
            entity.timestamp.format("%Y-%m-%d %H:%M")
        );

        println!();
    }

    if result.has_more {
        println!("💡 Use --offset {} to see more standards", offset + limit);
    }

    println!("💡 Use 'engram standard get <id>' to view full standard details");
    println!("💡 Use 'engram standard add-requirement <id>' to add requirements");

    Ok(())
}

/// Add requirement to standard
pub fn add_requirement<S: Storage>(
    storage: &mut S,
    id: &str,
    title: String,
    description: String,
    mandatory: bool,
    priority: String,
    evidence_required: bool,
) -> Result<(), EngramError> {
    if let Some(generic) = storage.get(id, "standard")? {
        let mut standard =
            Standard::from_generic(generic).map_err(|e| EngramError::Validation(e.to_string()))?;

        let priority = match priority.to_lowercase().as_str() {
            "low" => crate::entities::rule::RulePriority::Low,
            "medium" => crate::entities::rule::RulePriority::Medium,
            "high" => crate::entities::rule::RulePriority::High,
            "critical" => crate::entities::rule::RulePriority::Critical,
            _ => {
                println!("❌ Invalid priority. Use: low, medium, high, critical");
                return Ok(());
            }
        };

        let requirement = StandardRequirement {
            id: uuid::Uuid::new_v4().to_string(),
            title,
            description,
            mandatory,
            priority,
            validation_criteria: Vec::new(),
            evidence_required,
        };

        standard.add_requirement(requirement);

        let updated_generic = standard.to_generic();
        storage.store(&updated_generic)?;

        println!("✅ Requirement added to standard: {}", id);
    } else {
        println!("❌ Standard not found: {}", id);
    }
    Ok(())
}

/// Display standard information
fn display_standard(standard: &Standard) {
    println!("📋 Standard: {}", standard.id());
    println!("📝 Title: {}", standard.title);
    println!("📄 Description: {}", standard.description);
    println!("🏷️ Category: {:?}", standard.category);
    println!("📊 Status: {:?}", standard.status);
    println!("🔢 Version: {}", standard.version);
    println!("🤖 Agent: {}", standard.agent);
    println!(
        "📅 Effective Date: {}",
        standard.effective_date.format("%Y-%m-%d %H:%M")
    );
    println!(
        "🕐 Created: {}",
        standard.created_at.format("%Y-%m-%d %H:%M")
    );
    println!(
        "🔄 Updated: {}",
        standard.updated_at.format("%Y-%m-%d %H:%M")
    );

    if let Some(ref superseded_by) = standard.superseded_by {
        println!("🔗 Superseded By: {}", superseded_by);
    }

    if !standard.supersedes.is_empty() {
        println!("📋 Supersedes: {:?}", standard.supersedes);
    }

    if !standard.related_standards.is_empty() {
        println!("🔗 Related Standards: {:?}", standard.related_standards);
    }

    if !standard.requirements.is_empty() {
        println!("📌 Requirements: {}", standard.requirements.len());
        for (i, req) in standard.requirements.iter().enumerate() {
            println!(
                "  {}. {} - {} ({})",
                i + 1,
                req.title,
                if req.mandatory {
                    "Mandatory"
                } else {
                    "Optional"
                },
                format!("{:?}", req.priority)
            );
        }
    }

    println!("✅ Is Effective: {}", standard.is_effective());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::{rule::RulePriority, Entity, Standard, StandardCategory, StandardStatus};
    use crate::storage::MemoryStorage;
    use crate::storage::Storage;

    #[test]
    fn test_create_standard() {
        let mut storage = MemoryStorage::new("test-agent");
        let title = "Code Style".to_string();
        let description = Some("Standard code style".to_string());
        let category = "coding".to_string();
        let version = "1.0".to_string();
        let agent = Some("test-agent".to_string());

        let result = create_standard(
            &mut storage,
            title,
            description,
            category,
            version,
            None,
            agent,
        );
        assert!(result.is_ok());

        // Use query_by_type which is available in the trait
        let query_result = storage.query_by_type("standard", None, None, None).unwrap();
        assert_eq!(query_result.total_count, 1);

        let entity = &query_result.entities[0];
        assert_eq!(entity.data.get("title").unwrap(), "Code Style");
        assert_eq!(entity.data.get("category").unwrap(), "coding");
    }

    #[test]
    fn test_get_standard() {
        let mut storage = MemoryStorage::new("test-agent");
        let title = "Security Standard".to_string();

        create_standard(
            &mut storage,
            title,
            None,
            "security".to_string(),
            "1.0".to_string(),
            None,
            None,
        )
        .unwrap();

        let query_result = storage.query_by_type("standard", None, None, None).unwrap();
        let id = &query_result.entities[0].id;

        let result = get_standard(&storage, id);
        assert!(result.is_ok());

        let result = get_standard(&storage, "non-existent");
        assert!(result.is_ok()); // Should just print error message
    }

    #[test]
    fn test_update_standard() {
        let mut storage = MemoryStorage::new("test-agent");

        create_standard(
            &mut storage,
            "Old Title".to_string(),
            None,
            "process".to_string(),
            "1.0".to_string(),
            None,
            None,
        )
        .unwrap();

        let query_result = storage.query_by_type("standard", None, None, None).unwrap();
        let id = &query_result.entities[0].id;

        let result = update_standard(
            &mut storage,
            id,
            Some("New Title".to_string()),
            Some("New Description".to_string()),
            Some("coding".to_string()),
            Some("2.0".to_string()),
            Some("active".to_string()),
            None,
            None,
        );
        assert!(result.is_ok());

        let generic = storage.get(id, "standard").unwrap().unwrap();
        let standard = Standard::from_generic(generic).unwrap();

        assert_eq!(standard.title, "New Title");
        assert_eq!(standard.description, "New Description");
        assert!(matches!(standard.category, StandardCategory::Coding));
        assert_eq!(standard.version, "2.0");
        assert!(matches!(standard.status, StandardStatus::Active));
    }

    #[test]
    fn test_delete_standard() {
        let mut storage = MemoryStorage::new("test-agent");

        create_standard(
            &mut storage,
            "To Delete".to_string(),
            None,
            "process".to_string(),
            "1.0".to_string(),
            None,
            None,
        )
        .unwrap();

        let query_result = storage.query_by_type("standard", None, None, None).unwrap();
        let id = &query_result.entities[0].id;

        let result = delete_standard(&mut storage, id);
        assert!(result.is_ok());

        let generic = storage.get(id, "standard").unwrap().unwrap();
        let standard = Standard::from_generic(generic).unwrap();
        assert!(matches!(standard.status, StandardStatus::Deprecated));
    }

    #[test]
    fn test_list_standards() {
        let mut storage = MemoryStorage::new("test-agent");

        create_standard(
            &mut storage,
            "Std 1".to_string(),
            None,
            "coding".to_string(),
            "1.0".to_string(),
            None,
            None,
        )
        .unwrap();

        create_standard(
            &mut storage,
            "Std 2".to_string(),
            None,
            "security".to_string(),
            "1.0".to_string(),
            None,
            None,
        )
        .unwrap();

        // List all
        let result = list_standards(&storage, None, None, None, 10, 0);
        assert!(result.is_ok());

        // Filter by category
        let result = list_standards(&storage, Some("coding".to_string()), None, None, 10, 0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_add_requirement() {
        let mut storage = MemoryStorage::new("test-agent");

        create_standard(
            &mut storage,
            "With Req".to_string(),
            None,
            "process".to_string(),
            "1.0".to_string(),
            None,
            None,
        )
        .unwrap();

        let query_result = storage.query_by_type("standard", None, None, None).unwrap();
        let id = &query_result.entities[0].id;

        let result = add_requirement(
            &mut storage,
            id,
            "Req 1".to_string(),
            "Description".to_string(),
            true,
            "high".to_string(),
            false,
        );
        assert!(result.is_ok());

        let generic = storage.get(id, "standard").unwrap().unwrap();
        let standard = Standard::from_generic(generic).unwrap();

        assert_eq!(standard.requirements.len(), 1);
        let req = &standard.requirements[0];
        assert_eq!(req.title, "Req 1");
        assert_eq!(req.mandatory, true);
        assert!(matches!(req.priority, RulePriority::High));
    }

    #[test]
    fn test_create_standard_invalid_category() {
        let mut storage = MemoryStorage::new("test-agent");
        let result = create_standard(
            &mut storage,
            "Invalid Cat".to_string(),
            None,
            "invalid_category".to_string(),
            "1.0".to_string(),
            None,
            None,
        );
        // It returns Ok but prints an error message, and doesn't create the standard
        assert!(result.is_ok());

        let query_result = storage.query_by_type("standard", None, None, None).unwrap();
        assert_eq!(query_result.total_count, 0);
    }

    #[test]
    fn test_update_standard_invalid_status() {
        let mut storage = MemoryStorage::new("test-agent");
        create_standard(
            &mut storage,
            "Test".to_string(),
            None,
            "coding".to_string(),
            "1.0".to_string(),
            None,
            None,
        )
        .unwrap();

        let query_result = storage.query_by_type("standard", None, None, None).unwrap();
        let id = &query_result.entities[0].id;

        let result = update_standard(
            &mut storage,
            id,
            None,
            None,
            None,
            None,
            Some("invalid_status".to_string()),
            None,
            None,
        );
        assert!(result.is_ok()); // Returns Ok but prints error

        // Verify status didn't change (default is Draft)
        let generic = storage.get(id, "standard").unwrap().unwrap();
        let standard = Standard::from_generic(generic).unwrap();
        assert!(matches!(standard.status, StandardStatus::Draft));
    }
}

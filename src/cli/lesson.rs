//! Lesson command implementations

use crate::entities::{Entity, Lesson, LessonCategory, LessonSeverity};
use crate::error::EngramError;
use crate::storage::Storage;
use clap::Subcommand;

/// Lesson commands
#[derive(Debug, Subcommand)]
pub enum LessonCommands {
    /// Record a new lesson
    Create {
        /// Short title summarising the lesson
        #[arg(long, short)]
        title: String,

        /// Description of the mistake that was made
        #[arg(long, short)]
        mistake: String,

        /// Explanation of the correct approach
        #[arg(long, short = 'c')]
        correction: String,

        /// Rule or heuristic to prevent recurrence
        #[arg(long, short = 'p')]
        prevention_rule: String,

        /// Domain (e.g. "rust", "postgres")
        #[arg(long, short = 'd', default_value = "")]
        domain: String,

        /// Category: code | domain | process | design
        #[arg(long, short = 'k', default_value = "code")]
        category: String,

        /// Severity: low | medium | high
        #[arg(long, short = 's', default_value = "low")]
        severity: String,

        /// Assigned agent
        #[arg(long, short)]
        agent: Option<String>,

        /// Tags (comma-separated)
        #[arg(long)]
        tags: Option<String>,
    },
    /// List lessons
    List {
        /// Agent filter
        #[arg(long, short)]
        agent: Option<String>,

        /// Category filter: code | domain | process | design
        #[arg(long, short = 'k')]
        category: Option<String>,

        /// Domain filter
        #[arg(long, short = 'd')]
        domain: Option<String>,

        /// Severity filter: low | medium | high
        #[arg(long, short = 's')]
        severity: Option<String>,

        /// Limit results
        #[arg(long, short)]
        limit: Option<usize>,

        /// Show all results (no limit)
        #[arg(long, conflicts_with = "limit")]
        all: bool,

        /// Offset for pagination
        #[arg(long, short)]
        offset: Option<usize>,
    },
    /// Show lesson details
    Show {
        /// Lesson UUID
        #[arg(long, short)]
        id: String,
    },
    /// Update a lesson
    Update {
        /// Lesson UUID
        #[arg(long, short)]
        id: String,

        /// Updated mistake description
        #[arg(long)]
        mistake: Option<String>,

        /// Updated correction
        #[arg(long)]
        correction: Option<String>,

        /// Updated prevention rule
        #[arg(long)]
        prevention_rule: Option<String>,

        /// Add a tag
        #[arg(long)]
        add_tag: Option<String>,
    },
    /// Delete a lesson
    Delete {
        /// Lesson UUID
        #[arg(long, short)]
        id: String,
    },
}

// ── helpers ─────────────────────────────────────────────────────────────────

fn parse_category(s: &str) -> Result<LessonCategory, EngramError> {
    match s.to_lowercase().as_str() {
        "code" => Ok(LessonCategory::Code),
        "domain" => Ok(LessonCategory::Domain),
        "process" => Ok(LessonCategory::Process),
        "design" => Ok(LessonCategory::Design),
        _ => Err(EngramError::Validation(format!(
            "Invalid category '{}'. Must be one of: code, domain, process, design",
            s
        ))),
    }
}

fn parse_severity(s: &str) -> Result<LessonSeverity, EngramError> {
    match s.to_lowercase().as_str() {
        "low" => Ok(LessonSeverity::Low),
        "medium" => Ok(LessonSeverity::Medium),
        "high" => Ok(LessonSeverity::High),
        _ => Err(EngramError::Validation(format!(
            "Invalid severity '{}'. Must be one of: low, medium, high",
            s
        ))),
    }
}

// ── CRUD functions ───────────────────────────────────────────────────────────

/// Create a new lesson
pub fn create_lesson<S: Storage>(
    storage: &mut S,
    title: String,
    mistake: String,
    correction: String,
    prevention_rule: String,
    domain: String,
    category: String,
    severity: String,
    agent: Option<String>,
    tags: Option<String>,
) -> Result<(), EngramError> {
    let cat = parse_category(&category)?;
    let sev = parse_severity(&severity)?;
    let agent_name = agent.unwrap_or_else(|| "default".to_string());

    let mut lesson = Lesson::new(
        title,
        mistake,
        correction,
        prevention_rule,
        domain,
        cat,
        sev,
        agent_name,
    );

    if let Some(tags_str) = tags {
        for tag in tags_str.split(',') {
            lesson.add_tag(tag.trim().to_string());
        }
    }

    lesson.validate_entity()?;

    let generic = lesson.to_generic();
    storage.store(&generic)?;

    println!("Lesson created successfully with ID: {}", lesson.id);
    Ok(())
}

use crate::cli::utils::{create_table, truncate};
use prettytable::row;

/// List lessons
pub fn list_lessons<S: Storage>(
    storage: &S,
    agent: Option<String>,
    category: Option<String>,
    domain: Option<String>,
    severity: Option<String>,
    limit: Option<usize>,
    all: bool,
    offset: Option<usize>,
) -> Result<(), EngramError> {
    let ids = storage.list_ids(Lesson::entity_type())?;

    let mut items: Vec<Lesson> = Vec::new();

    for id in ids {
        if let Some(entity) = storage.get(&id, Lesson::entity_type())? {
            if let Ok(lesson) = Lesson::from_generic(entity) {
                if let Some(ref f) = agent {
                    if lesson.agent != *f {
                        continue;
                    }
                }
                if let Some(ref f) = category {
                    if lesson.category.to_string() != f.to_lowercase() {
                        continue;
                    }
                }
                if let Some(ref f) = domain {
                    if lesson.domain != *f {
                        continue;
                    }
                }
                if let Some(ref f) = severity {
                    if lesson.severity.to_string() != f.to_lowercase() {
                        continue;
                    }
                }
                items.push(lesson);
            }
        }
    }

    let total_count = items.len();

    if let Some(off) = offset {
        items = items.into_iter().skip(off).collect();
    }

    if !all {
        if let Some(lim) = limit {
            items.truncate(lim);
        }
    }

    if items.is_empty() {
        println!("No lessons found matching the criteria.");
        return Ok(());
    }

    let mut table = create_table();
    table.set_titles(row![
        "ID",
        "Domain",
        "Category",
        "Severity",
        "Prevention Rule",
        "Created"
    ]);

    for lesson in &items {
        table.add_row(row![
            &lesson.id[..8],
            truncate(&lesson.domain, 15),
            lesson.category.to_string(),
            lesson.severity.to_string(),
            truncate(&lesson.prevention_rule, 80),
            lesson.created_at.format("%Y-%m-%d")
        ]);
    }

    table.printstd();

    if total_count > items.len() {
        println!(
            "(Showing {} of {} — use --all, --offset N, or --limit N)",
            items.len(),
            total_count
        );
    }

    Ok(())
}

/// Show lesson details
pub fn show_lesson<S: Storage>(storage: &S, id: &str) -> Result<(), EngramError> {
    let entity = storage
        .get(id, Lesson::entity_type())?
        .ok_or_else(|| EngramError::NotFound(format!("Lesson not found: {}", id)))?;

    let lesson =
        Lesson::from_generic(entity).map_err(|e| EngramError::Validation(e.to_string()))?;

    println!("Lesson Details:");
    println!("===============");
    println!("ID: {}", lesson.id);
    println!("Title: {}", lesson.title);
    println!("Domain: {}", lesson.domain);
    println!("Category: {}", lesson.category);
    println!("Severity: {}", lesson.severity);
    println!("Agent: {}", lesson.agent);
    println!("Created: {}", lesson.created_at);
    println!("Updated: {}", lesson.updated_at);
    println!();
    println!("Mistake:");
    println!("  {}", lesson.mistake);
    println!();
    println!("Correction:");
    println!("  {}", lesson.correction);
    println!();
    println!("Prevention Rule:");
    println!("  {}", lesson.prevention_rule);

    if !lesson.tags.is_empty() {
        println!();
        println!("Tags: {}", lesson.tags.join(", "));
    }

    Ok(())
}

/// Update a lesson
pub fn update_lesson<S: Storage>(
    storage: &mut S,
    id: &str,
    mistake: Option<String>,
    correction: Option<String>,
    prevention_rule: Option<String>,
    add_tag: Option<String>,
) -> Result<(), EngramError> {
    let entity = storage
        .get(id, Lesson::entity_type())?
        .ok_or_else(|| EngramError::NotFound(format!("Lesson not found: {}", id)))?;

    let mut lesson =
        Lesson::from_generic(entity).map_err(|e| EngramError::Validation(e.to_string()))?;

    lesson.update(mistake, correction, prevention_rule);

    if let Some(tag) = add_tag {
        lesson.add_tag(tag);
    }

    lesson.validate_entity()?;

    let generic = lesson.to_generic();
    storage.store(&generic)?;

    println!("Lesson updated successfully: {}", id);
    Ok(())
}

/// Delete a lesson
pub fn delete_lesson<S: Storage>(storage: &mut S, id: &str) -> Result<(), EngramError> {
    storage.delete(id, Lesson::entity_type())?;
    println!("Lesson deleted successfully: {}", id);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::MemoryStorage;

    fn create_test_storage() -> MemoryStorage {
        MemoryStorage::new("default")
    }

    fn create_sample_lesson(storage: &mut MemoryStorage) {
        create_lesson(
            storage,
            "Validate slugs".to_string(),
            "Used spaces in slugs".to_string(),
            "Use [a-z0-9-]+ only".to_string(),
            "Validate slug with regex".to_string(),
            "rust".to_string(),
            "code".to_string(),
            "medium".to_string(),
            None,
            None,
        )
        .unwrap();
    }

    #[test]
    fn test_create_lesson_basic() {
        let mut storage = create_test_storage();
        create_sample_lesson(&mut storage);

        let ids = storage.list_ids("lesson").unwrap();
        assert_eq!(ids.len(), 1);

        let entity = storage.get(&ids[0], "lesson").unwrap().unwrap();
        let lesson = Lesson::from_generic(entity).unwrap();
        assert_eq!(lesson.title, "Validate slugs");
        assert_eq!(lesson.category, LessonCategory::Code);
        assert_eq!(lesson.severity, LessonSeverity::Medium);
    }

    #[test]
    fn test_create_lesson_invalid_category() {
        let mut storage = create_test_storage();
        let result = create_lesson(
            &mut storage,
            "Title".to_string(),
            "mistake".to_string(),
            "correction".to_string(),
            "rule".to_string(),
            "".to_string(),
            "invalid_cat".to_string(),
            "low".to_string(),
            None,
            None,
        );
        assert!(matches!(result, Err(EngramError::Validation(_))));
    }

    #[test]
    fn test_create_lesson_invalid_severity() {
        let mut storage = create_test_storage();
        let result = create_lesson(
            &mut storage,
            "Title".to_string(),
            "mistake".to_string(),
            "correction".to_string(),
            "rule".to_string(),
            "".to_string(),
            "code".to_string(),
            "extreme".to_string(),
            None,
            None,
        );
        assert!(matches!(result, Err(EngramError::Validation(_))));
    }

    #[test]
    fn test_create_lesson_empty_mistake_fails_validation() {
        let mut storage = create_test_storage();
        let result = create_lesson(
            &mut storage,
            "Title".to_string(),
            "".to_string(), // empty mistake
            "correction".to_string(),
            "rule".to_string(),
            "".to_string(),
            "code".to_string(),
            "low".to_string(),
            None,
            None,
        );
        assert!(matches!(result, Err(EngramError::Validation(_))));
    }

    #[test]
    fn test_create_lesson_with_tags() {
        let mut storage = create_test_storage();
        create_lesson(
            &mut storage,
            "Tag test".to_string(),
            "mistake".to_string(),
            "correction".to_string(),
            "rule".to_string(),
            "rust".to_string(),
            "code".to_string(),
            "low".to_string(),
            None,
            Some("rust,validation,slug".to_string()),
        )
        .unwrap();

        let ids = storage.list_ids("lesson").unwrap();
        let entity = storage.get(&ids[0], "lesson").unwrap().unwrap();
        let lesson = Lesson::from_generic(entity).unwrap();
        assert_eq!(lesson.tags.len(), 3);
    }

    #[test]
    fn test_show_lesson() {
        let mut storage = create_test_storage();
        create_sample_lesson(&mut storage);
        let ids = storage.list_ids("lesson").unwrap();
        assert!(show_lesson(&storage, &ids[0]).is_ok());
    }

    #[test]
    fn test_show_lesson_not_found() {
        let storage = create_test_storage();
        let result = show_lesson(&storage, "missing-id");
        assert!(matches!(result, Err(EngramError::NotFound(_))));
    }

    #[test]
    fn test_update_lesson() {
        let mut storage = create_test_storage();
        create_sample_lesson(&mut storage);
        let ids = storage.list_ids("lesson").unwrap();
        let id = &ids[0];

        update_lesson(
            &mut storage,
            id,
            Some("Updated mistake".to_string()),
            None,
            None,
            None,
        )
        .unwrap();

        let entity = storage.get(id, "lesson").unwrap().unwrap();
        let lesson = Lesson::from_generic(entity).unwrap();
        assert_eq!(lesson.mistake, "Updated mistake");
        // correction unchanged
        assert_eq!(lesson.correction, "Use [a-z0-9-]+ only");
    }

    #[test]
    fn test_update_lesson_not_found() {
        let mut storage = create_test_storage();
        let result = update_lesson(&mut storage, "missing-id", None, None, None, None);
        assert!(matches!(result, Err(EngramError::NotFound(_))));
    }

    #[test]
    fn test_update_lesson_add_tag() {
        let mut storage = create_test_storage();
        create_sample_lesson(&mut storage);
        let ids = storage.list_ids("lesson").unwrap();
        let id = &ids[0];

        update_lesson(
            &mut storage,
            id,
            None,
            None,
            None,
            Some("newtag".to_string()),
        )
        .unwrap();

        let entity = storage.get(id, "lesson").unwrap().unwrap();
        let lesson = Lesson::from_generic(entity).unwrap();
        assert!(lesson.tags.contains(&"newtag".to_string()));
    }

    #[test]
    fn test_delete_lesson() {
        let mut storage = create_test_storage();
        create_sample_lesson(&mut storage);
        let ids = storage.list_ids("lesson").unwrap();
        let id = ids[0].clone();

        delete_lesson(&mut storage, &id).unwrap();

        let result = storage.get(&id, "lesson").unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_delete_lesson_not_found() {
        let mut storage = create_test_storage();
        let result = delete_lesson(&mut storage, "missing-id");
        assert!(matches!(result, Err(EngramError::NotFound(_))));
    }

    #[test]
    fn test_list_lessons_no_filter() {
        let mut storage = create_test_storage();
        create_sample_lesson(&mut storage);
        create_lesson(
            &mut storage,
            "Process lesson".to_string(),
            "mistake2".to_string(),
            "correction2".to_string(),
            "rule2".to_string(),
            "agile".to_string(),
            "process".to_string(),
            "high".to_string(),
            None,
            None,
        )
        .unwrap();

        assert!(list_lessons(&storage, None, None, None, None, None, false, None).is_ok());
    }

    #[test]
    fn test_list_lessons_category_filter() {
        let mut storage = create_test_storage();
        create_sample_lesson(&mut storage); // code category
        create_lesson(
            &mut storage,
            "Design lesson".to_string(),
            "mistake".to_string(),
            "correction".to_string(),
            "rule".to_string(),
            "arch".to_string(),
            "design".to_string(),
            "low".to_string(),
            None,
            None,
        )
        .unwrap();

        // Filter for code — should run without error
        assert!(list_lessons(
            &storage,
            None,
            Some("code".to_string()),
            None,
            None,
            None,
            false,
            None
        )
        .is_ok());
    }

    #[test]
    fn test_list_lessons_empty() {
        let storage = create_test_storage();
        assert!(list_lessons(&storage, None, None, None, None, None, false, None).is_ok());
    }

    #[test]
    fn test_parse_category_all_variants() {
        assert!(matches!(parse_category("code"), Ok(LessonCategory::Code)));
        assert!(matches!(
            parse_category("domain"),
            Ok(LessonCategory::Domain)
        ));
        assert!(matches!(
            parse_category("process"),
            Ok(LessonCategory::Process)
        ));
        assert!(matches!(
            parse_category("design"),
            Ok(LessonCategory::Design)
        ));
        assert!(matches!(parse_category("CODE"), Ok(LessonCategory::Code))); // case-insensitive
        assert!(parse_category("other").is_err());
    }

    #[test]
    fn test_parse_severity_all_variants() {
        assert!(matches!(parse_severity("low"), Ok(LessonSeverity::Low)));
        assert!(matches!(
            parse_severity("medium"),
            Ok(LessonSeverity::Medium)
        ));
        assert!(matches!(parse_severity("high"), Ok(LessonSeverity::High)));
        assert!(matches!(parse_severity("HIGH"), Ok(LessonSeverity::High))); // case-insensitive
        assert!(parse_severity("critical").is_err());
    }
}

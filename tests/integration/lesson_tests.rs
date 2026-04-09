use engram::entities::{Entity, Lesson, LessonCategory, LessonSeverity};
use engram::storage::GitRefsStorage;
use engram::Storage;
use std::collections::HashMap;
use tempfile::TempDir;

#[cfg(test)]
mod lesson_integration_tests {
    use super::*;

    fn setup_test_storage() -> (TempDir, GitRefsStorage) {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let storage = GitRefsStorage::new(temp_dir.path().to_str().unwrap(), "test-agent")
            .expect("Failed to create GitRefsStorage");
        (temp_dir, storage)
    }

    fn make_lesson(title: &str, category: LessonCategory, severity: LessonSeverity) -> Lesson {
        Lesson::new(
            title.to_string(),
            format!("Mistake: {}", title),
            format!("Correction: {}", title),
            format!("Prevention: {}", title),
            "rust".to_string(),
            category,
            severity,
            "test-agent".to_string(),
        )
    }

    #[test]
    fn test_create_store_and_retrieve_lesson() {
        let (_temp_dir, mut storage) = setup_test_storage();

        let lesson = make_lesson(
            "Validate slug format",
            LessonCategory::Code,
            LessonSeverity::Medium,
        );
        let generic = lesson.to_generic();

        storage.store(&generic).expect("Failed to store lesson");

        let retrieved = storage
            .get(&lesson.id(), Lesson::entity_type())
            .expect("Failed to retrieve lesson")
            .expect("Lesson not found");

        assert_eq!(retrieved.id, lesson.id());
        assert_eq!(retrieved.entity_type, "lesson");
        assert_eq!(retrieved.agent, "test-agent");

        let restored = Lesson::from_generic(retrieved).expect("Failed to deserialize lesson");
        assert_eq!(restored.title, "Validate slug format");
        assert_eq!(restored.category, LessonCategory::Code);
        assert_eq!(restored.severity, LessonSeverity::Medium);
        assert_eq!(restored.domain, "rust");
    }

    #[test]
    fn test_full_round_trip_lesson() {
        let (_temp_dir, mut storage) = setup_test_storage();

        let mut lesson = make_lesson(
            "Full round trip",
            LessonCategory::Process,
            LessonSeverity::High,
        );
        lesson.add_tag("process".to_string());
        lesson.add_tag("critical".to_string());

        let generic = lesson.to_generic();
        storage.store(&generic).expect("Failed to store lesson");

        let retrieved = storage
            .get(&lesson.id(), Lesson::entity_type())
            .expect("Failed to retrieve lesson")
            .expect("Lesson not found");

        let restored = Lesson::from_generic(retrieved).expect("Failed to restore lesson");

        assert_eq!(restored.title, "Full round trip");
        assert_eq!(restored.category, LessonCategory::Process);
        assert_eq!(restored.severity, LessonSeverity::High);
        assert_eq!(restored.tags.len(), 2);
        assert!(restored.tags.contains(&"process".to_string()));
        assert!(restored.tags.contains(&"critical".to_string()));
        assert_eq!(restored.mistake, "Mistake: Full round trip");
        assert_eq!(restored.correction, "Correction: Full round trip");
        assert_eq!(restored.prevention_rule, "Prevention: Full round trip");
    }

    #[test]
    fn test_store_multiple_lessons_and_list() {
        let (_temp_dir, mut storage) = setup_test_storage();

        let l1 = make_lesson("Lesson 1", LessonCategory::Code, LessonSeverity::Low);
        let l2 = make_lesson("Lesson 2", LessonCategory::Design, LessonSeverity::Medium);
        let l3 = make_lesson("Lesson 3", LessonCategory::Domain, LessonSeverity::High);

        storage.store(&l1.to_generic()).expect("Failed to store l1");
        storage.store(&l2.to_generic()).expect("Failed to store l2");
        storage.store(&l3.to_generic()).expect("Failed to store l3");

        let all = storage
            .get_all(Lesson::entity_type())
            .expect("Failed to list lessons");
        assert_eq!(all.len(), 3);
    }

    #[test]
    fn test_filter_lessons_by_category() {
        let (_temp_dir, mut storage) = setup_test_storage();

        let l1 = make_lesson("Code lesson", LessonCategory::Code, LessonSeverity::Low);
        let l2 = make_lesson(
            "Design lesson",
            LessonCategory::Design,
            LessonSeverity::Medium,
        );
        let l3 = make_lesson(
            "Another code lesson",
            LessonCategory::Code,
            LessonSeverity::High,
        );

        storage.store(&l1.to_generic()).expect("Failed to store l1");
        storage.store(&l2.to_generic()).expect("Failed to store l2");
        storage.store(&l3.to_generic()).expect("Failed to store l3");

        let mut field_filters = HashMap::new();
        field_filters.insert(
            "category".to_string(),
            serde_json::Value::String("code".to_string()),
        );

        let result = storage
            .query_by_type(Lesson::entity_type(), Some(&field_filters), None, None)
            .expect("Failed to query lessons by category");

        assert_eq!(result.entities.len(), 2, "Should find 2 code lessons");

        for entity in result.entities {
            let restored = Lesson::from_generic(entity).unwrap();
            assert_eq!(restored.category, LessonCategory::Code);
        }
    }

    #[test]
    fn test_filter_lessons_by_severity() {
        let (_temp_dir, mut storage) = setup_test_storage();

        let l1 = make_lesson("High severity", LessonCategory::Code, LessonSeverity::High);
        let l2 = make_lesson("Low severity", LessonCategory::Code, LessonSeverity::Low);
        let l3 = make_lesson("Another high", LessonCategory::Domain, LessonSeverity::High);

        storage.store(&l1.to_generic()).expect("Failed to store l1");
        storage.store(&l2.to_generic()).expect("Failed to store l2");
        storage.store(&l3.to_generic()).expect("Failed to store l3");

        let mut field_filters = HashMap::new();
        field_filters.insert(
            "severity".to_string(),
            serde_json::Value::String("high".to_string()),
        );

        let result = storage
            .query_by_type(Lesson::entity_type(), Some(&field_filters), None, None)
            .expect("Failed to query lessons by severity");

        assert_eq!(
            result.entities.len(),
            2,
            "Should find 2 high-severity lessons"
        );

        for entity in result.entities {
            let restored = Lesson::from_generic(entity).unwrap();
            assert_eq!(restored.severity, LessonSeverity::High);
        }
    }

    #[test]
    fn test_filter_lessons_by_domain() {
        let (_temp_dir, mut storage) = setup_test_storage();

        let mut l1 = make_lesson("Rust lesson", LessonCategory::Code, LessonSeverity::Medium);
        l1.domain = "rust".to_string();

        let mut l2 = make_lesson(
            "Postgres lesson",
            LessonCategory::Code,
            LessonSeverity::Medium,
        );
        l2.domain = "postgres".to_string();

        let mut l3 = make_lesson(
            "Another rust lesson",
            LessonCategory::Design,
            LessonSeverity::Low,
        );
        l3.domain = "rust".to_string();

        storage.store(&l1.to_generic()).expect("Failed to store l1");
        storage.store(&l2.to_generic()).expect("Failed to store l2");
        storage.store(&l3.to_generic()).expect("Failed to store l3");

        let mut field_filters = HashMap::new();
        field_filters.insert(
            "domain".to_string(),
            serde_json::Value::String("rust".to_string()),
        );

        let result = storage
            .query_by_type(Lesson::entity_type(), Some(&field_filters), None, None)
            .expect("Failed to query lessons by domain");

        assert_eq!(
            result.entities.len(),
            2,
            "Should find 2 rust-domain lessons"
        );
    }

    #[test]
    fn test_query_lessons_by_agent() {
        let (_temp_dir, mut storage) = setup_test_storage();

        let mut l1 = make_lesson(
            "Agent one lesson",
            LessonCategory::Code,
            LessonSeverity::Low,
        );
        l1.agent = "agent-one".to_string();

        let mut l2 = make_lesson(
            "Agent two lesson",
            LessonCategory::Design,
            LessonSeverity::High,
        );
        l2.agent = "agent-two".to_string();

        storage.store(&l1.to_generic()).expect("Failed to store l1");
        storage.store(&l2.to_generic()).expect("Failed to store l2");

        let result = storage
            .query_by_agent("agent-one", Some(Lesson::entity_type()))
            .expect("Failed to query by agent");

        assert_eq!(result.len(), 1);
        let restored = Lesson::from_generic(result.into_iter().next().unwrap()).unwrap();
        assert_eq!(restored.title, "Agent one lesson");
    }

    #[test]
    fn test_delete_lesson() {
        let (_temp_dir, mut storage) = setup_test_storage();

        let lesson = make_lesson("To delete", LessonCategory::Code, LessonSeverity::Low);
        let id = lesson.id().to_string();

        storage
            .store(&lesson.to_generic())
            .expect("Failed to store lesson");

        assert!(storage.get(&id, Lesson::entity_type()).unwrap().is_some());

        storage
            .delete(&id, Lesson::entity_type())
            .expect("Failed to delete lesson");

        assert!(storage.get(&id, Lesson::entity_type()).unwrap().is_none());
    }

    #[test]
    fn test_delete_non_existent_lesson_does_not_error() {
        let (_temp_dir, mut storage) = setup_test_storage();

        let result = storage.delete("nonexistent-id", Lesson::entity_type());
        assert!(
            result.is_ok(),
            "Deleting non-existent lesson should not error"
        );
    }

    #[test]
    fn test_get_non_existent_lesson_returns_none() {
        let (_temp_dir, storage) = setup_test_storage();

        let result = storage.get("nonexistent-id", Lesson::entity_type());
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_update_lesson_overwrites_existing() {
        let (_temp_dir, mut storage) = setup_test_storage();

        let mut lesson = make_lesson("Updatable", LessonCategory::Code, LessonSeverity::Low);
        let id = lesson.id().to_string();

        storage
            .store(&lesson.to_generic())
            .expect("Failed to store initial");

        lesson.severity = LessonSeverity::High;
        lesson.update(
            Some("Updated mistake".to_string()),
            Some("Updated correction".to_string()),
            None,
        );

        storage
            .store(&lesson.to_generic())
            .expect("Failed to store updated");

        let retrieved = storage
            .get(&id, Lesson::entity_type())
            .unwrap()
            .expect("Lesson not found after update");

        let restored = Lesson::from_generic(retrieved).unwrap();
        assert_eq!(restored.mistake, "Updated mistake");
        assert_eq!(restored.correction, "Updated correction");
        assert_eq!(restored.severity, LessonSeverity::High);
    }

    #[test]
    fn test_list_ids_for_lessons() {
        let (_temp_dir, mut storage) = setup_test_storage();

        let l1 = make_lesson("List test 1", LessonCategory::Code, LessonSeverity::Low);
        let l2 = make_lesson(
            "List test 2",
            LessonCategory::Domain,
            LessonSeverity::Medium,
        );

        storage.store(&l1.to_generic()).expect("Failed to store l1");
        storage.store(&l2.to_generic()).expect("Failed to store l2");

        let ids = storage
            .list_ids(Lesson::entity_type())
            .expect("Failed to list lesson IDs");

        assert_eq!(ids.len(), 2);
        assert!(ids.contains(&l1.id().to_string()));
        assert!(ids.contains(&l2.id().to_string()));
    }

    #[test]
    fn test_text_search_finds_lesson() {
        let (_temp_dir, mut storage) = setup_test_storage();

        let lesson = make_lesson(
            "Unique searchable lesson content",
            LessonCategory::Process,
            LessonSeverity::High,
        );
        storage
            .store(&lesson.to_generic())
            .expect("Failed to store lesson");

        let results = storage
            .text_search(
                "Unique searchable lesson content",
                Some(&["lesson".to_string()]),
                Some(10),
            )
            .expect("Failed to text search");

        assert!(!results.is_empty(), "Should find lesson by text search");
        assert_eq!(results[0].entity_type, "lesson");
    }

    #[test]
    fn test_empty_storage_returns_no_lessons() {
        let (_temp_dir, storage) = setup_test_storage();

        let all = storage
            .get_all(Lesson::entity_type())
            .expect("Failed to list from empty storage");
        assert!(all.is_empty());

        let ids = storage
            .list_ids(Lesson::entity_type())
            .expect("Failed to list IDs from empty storage");
        assert!(ids.is_empty());
    }

    #[test]
    fn test_all_categories_round_trip() {
        let (_temp_dir, mut storage) = setup_test_storage();

        for (cat, cat_str) in [
            (LessonCategory::Code, "code"),
            (LessonCategory::Domain, "domain"),
            (LessonCategory::Process, "process"),
            (LessonCategory::Design, "design"),
        ] {
            let cat_clone = cat.clone();
            let lesson = make_lesson(&format!("{} lesson", cat_str), cat, LessonSeverity::Medium);
            let id = lesson.id().to_string();

            storage
                .store(&lesson.to_generic())
                .expect(&format!("Failed to store {} lesson", cat_str));

            let retrieved = storage
                .get(&id, Lesson::entity_type())
                .unwrap()
                .expect(&format!("{} lesson not found", cat_str));

            let restored = Lesson::from_generic(retrieved).unwrap();
            assert_eq!(
                restored.category, cat_clone,
                "Category mismatch for {}",
                cat_str
            );
        }
    }

    #[test]
    fn test_all_severities_round_trip() {
        let (_temp_dir, mut storage) = setup_test_storage();

        for (sev, sev_str) in [
            (LessonSeverity::Low, "low"),
            (LessonSeverity::Medium, "medium"),
            (LessonSeverity::High, "high"),
        ] {
            let sev_clone = sev.clone();
            let lesson = make_lesson(
                &format!("{} severity lesson", sev_str),
                LessonCategory::Code,
                sev,
            );
            let id = lesson.id().to_string();

            storage
                .store(&lesson.to_generic())
                .expect(&format!("Failed to store {} severity lesson", sev_str));

            let retrieved = storage
                .get(&id, Lesson::entity_type())
                .unwrap()
                .expect(&format!("{} severity lesson not found", sev_str));

            let restored = Lesson::from_generic(retrieved).unwrap();
            assert_eq!(
                restored.severity, sev_clone,
                "Severity mismatch for {}",
                sev_str
            );
        }
    }
}

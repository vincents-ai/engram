use engram::entities::{Entity, Persona};
use engram::storage::GitRefsStorage;
use engram::Storage;
use std::collections::HashMap;
use tempfile::TempDir;

#[cfg(test)]
mod persona_integration_tests {
    use super::*;

    fn setup_test_storage() -> (TempDir, GitRefsStorage) {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let storage = GitRefsStorage::new(temp_dir.path().to_str().unwrap(), "test-agent")
            .expect("Failed to create GitRefsStorage");
        (temp_dir, storage)
    }

    fn make_persona(slug: &str, title: &str) -> Persona {
        Persona::new(
            slug.to_string(),
            title.to_string(),
            format!("Description for {}", title),
            format!("Instructions for {}", title),
            "rust".to_string(),
            "test-agent".to_string(),
        )
    }

    fn make_persona_with_domain(slug: &str, title: &str, domain: &str) -> Persona {
        Persona::new(
            slug.to_string(),
            title.to_string(),
            format!("Description for {}", title),
            format!("Instructions for {}", title),
            domain.to_string(),
            "test-agent".to_string(),
        )
    }

    #[test]
    fn test_create_store_and_retrieve_persona() {
        let (_temp_dir, mut storage) = setup_test_storage();

        let persona = make_persona("rust-expert", "Rust Expert");
        let generic = persona.to_generic();

        storage.store(&generic).expect("Failed to store persona");

        let retrieved = storage
            .get(&persona.id(), Persona::entity_type())
            .expect("Failed to retrieve persona")
            .expect("Persona not found");

        assert_eq!(retrieved.id, persona.id());
        assert_eq!(retrieved.entity_type, "persona");
        assert_eq!(retrieved.agent, "test-agent");

        let restored = Persona::from_generic(retrieved).expect("Failed to deserialize persona");
        assert_eq!(restored.slug, "rust-expert");
        assert_eq!(restored.title, "Rust Expert");
        assert_eq!(restored.version, "1.0.0");
    }

    #[test]
    fn test_full_round_trip_persona() {
        let (_temp_dir, mut storage) = setup_test_storage();

        let mut persona = make_persona("security-reviewer", "Security Reviewer");
        persona.add_tag("security".to_string());
        persona.add_tag("audit".to_string());
        persona.set_base_persona("01-the-one".to_string());
        persona.add_cov_question("Q1".to_string());
        persona.add_cov_question("Q2".to_string());
        persona.add_cov_question("Q3".to_string());
        persona.set_fap("WHO".to_string(), "Security engineers".to_string());
        persona.set_fap("WHAT".to_string(), "Prevent vulnerabilities".to_string());
        persona.set_fap("WHY".to_string(), "Security is critical".to_string());
        persona.add_ov_requirement("Check inputs".to_string());

        let generic = persona.to_generic();
        storage.store(&generic).expect("Failed to store persona");

        let retrieved = storage
            .get(&persona.id(), Persona::entity_type())
            .expect("Failed to retrieve persona")
            .expect("Persona not found");

        let restored = Persona::from_generic(retrieved).expect("Failed to restore persona");

        assert_eq!(restored.slug, "security-reviewer");
        assert_eq!(restored.tags.len(), 2);
        assert!(restored.tags.contains(&"security".to_string()));
        assert!(restored.tags.contains(&"audit".to_string()));
        assert_eq!(restored.base_persona, Some("01-the-one".to_string()));
        assert_eq!(restored.cov_questions.len(), 3);
        assert_eq!(restored.fap_table.len(), 3);
        assert_eq!(restored.ov_requirements.len(), 1);
        assert_eq!(restored.domain, "rust");
    }

    #[test]
    fn test_store_multiple_personas_and_list() {
        let (_temp_dir, mut storage) = setup_test_storage();

        let p1 = make_persona("rust-expert", "Rust Expert");
        let p2 = make_persona("go-expert", "Go Expert");
        let p3 = make_persona("python-expert", "Python Expert");

        let g1 = p1.to_generic();
        let g2 = p2.to_generic();
        let g3 = p3.to_generic();

        storage.store(&g1).expect("Failed to store p1");
        storage.store(&g2).expect("Failed to store p2");
        storage.store(&g3).expect("Failed to store p3");

        let all = storage
            .get_all(Persona::entity_type())
            .expect("Failed to list personas");
        assert_eq!(all.len(), 3);
    }

    #[test]
    fn test_query_personas_by_field_filter() {
        let (_temp_dir, mut storage) = setup_test_storage();

        let p1 = make_persona("rust-expert", "Rust Expert");
        let p2 = make_persona_with_domain("go-expert", "Go Expert", "go");

        storage.store(&p1.to_generic()).expect("Failed to store p1");
        storage.store(&p2.to_generic()).expect("Failed to store p2");

        let mut field_filters = HashMap::new();
        field_filters.insert(
            "domain".to_string(),
            serde_json::Value::String("rust".to_string()),
        );

        let result = storage
            .query_by_type(Persona::entity_type(), Some(&field_filters), None, None)
            .expect("Failed to query personas by domain");

        assert_eq!(result.entities.len(), 1);
        let restored = Persona::from_generic(result.entities.into_iter().next().unwrap()).unwrap();
        assert_eq!(restored.slug, "rust-expert");
    }

    #[test]
    fn test_query_personas_by_agent() {
        let (_temp_dir, mut storage) = setup_test_storage();

        let mut p1 = make_persona("agent-one-persona", "Agent One");
        p1.agent = "agent-one".to_string();

        let mut p2 = make_persona("agent-two-persona", "Agent Two");
        p2.agent = "agent-two".to_string();

        storage.store(&p1.to_generic()).expect("Failed to store p1");
        storage.store(&p2.to_generic()).expect("Failed to store p2");

        let result = storage
            .query_by_agent("agent-one", Some(Persona::entity_type()))
            .expect("Failed to query by agent");

        assert_eq!(result.len(), 1);
        let restored = Persona::from_generic(result.into_iter().next().unwrap()).unwrap();
        assert_eq!(restored.slug, "agent-one-persona");
    }

    #[test]
    fn test_duplicate_slug_stores_as_separate_entities() {
        let (_temp_dir, mut storage) = setup_test_storage();

        let p1 = make_persona("expert", "Expert One");
        let p2 = make_persona("expert", "Expert Two");

        let id1 = p1.id().to_string();
        let id2 = p2.id().to_string();

        assert_ne!(id1, id2, "Two personas should have different IDs");

        storage.store(&p1.to_generic()).expect("Failed to store p1");
        storage.store(&p2.to_generic()).expect("Failed to store p2");

        let all = storage
            .get_all(Persona::entity_type())
            .expect("Failed to list personas");
        assert_eq!(
            all.len(),
            2,
            "Both personas with same slug should be stored"
        );

        let r1 = storage
            .get(&id1, Persona::entity_type())
            .expect("Failed to get p1")
            .expect("p1 not found");
        let r2 = storage
            .get(&id2, Persona::entity_type())
            .expect("Failed to get p2")
            .expect("p2 not found");

        let restored1 = Persona::from_generic(r1).unwrap();
        let restored2 = Persona::from_generic(r2).unwrap();

        assert_eq!(restored1.title, "Expert One");
        assert_eq!(restored2.title, "Expert Two");
    }

    #[test]
    fn test_delete_persona() {
        let (_temp_dir, mut storage) = setup_test_storage();

        let persona = make_persona("to-delete", "To Delete");
        let id = persona.id().to_string();

        storage
            .store(&persona.to_generic())
            .expect("Failed to store persona");

        assert!(storage.get(&id, Persona::entity_type()).unwrap().is_some());

        storage
            .delete(&id, Persona::entity_type())
            .expect("Failed to delete persona");

        assert!(storage.get(&id, Persona::entity_type()).unwrap().is_none());
    }

    #[test]
    fn test_delete_non_existent_persona_does_not_error() {
        let (_temp_dir, mut storage) = setup_test_storage();

        let result = storage.delete("nonexistent-id", Persona::entity_type());
        assert!(
            result.is_ok(),
            "Deleting non-existent persona should not error"
        );
    }

    #[test]
    fn test_get_non_existent_persona_returns_none() {
        let (_temp_dir, storage) = setup_test_storage();

        let result = storage.get("nonexistent-id", Persona::entity_type());
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_update_persona_overwrites_existing() {
        let (_temp_dir, mut storage) = setup_test_storage();

        let mut persona = make_persona("updatable", "Updatable Persona");
        let id = persona.id().to_string();

        storage
            .store(&persona.to_generic())
            .expect("Failed to store initial");

        persona.title = "Updated Title".to_string();
        persona.description = "Updated description".to_string();
        persona.updated_at = chrono::Utc::now();

        storage
            .store(&persona.to_generic())
            .expect("Failed to store updated");

        let retrieved = storage
            .get(&id, Persona::entity_type())
            .unwrap()
            .expect("Persona not found after update");

        let restored = Persona::from_generic(retrieved).unwrap();
        assert_eq!(restored.title, "Updated Title");
        assert_eq!(restored.description, "Updated description");
    }

    #[test]
    fn test_list_ids_for_personas() {
        let (_temp_dir, mut storage) = setup_test_storage();

        let p1 = make_persona("list-test-1", "List Test 1");
        let p2 = make_persona("list-test-2", "List Test 2");

        storage.store(&p1.to_generic()).expect("Failed to store p1");
        storage.store(&p2.to_generic()).expect("Failed to store p2");

        let ids = storage
            .list_ids(Persona::entity_type())
            .expect("Failed to list persona IDs");

        assert_eq!(ids.len(), 2);
        assert!(ids.contains(&p1.id().to_string()));
        assert!(ids.contains(&p2.id().to_string()));
    }

    #[test]
    fn test_text_search_finds_persona() {
        let (_temp_dir, mut storage) = setup_test_storage();

        let persona = make_persona("searchable", "Searchable Persona With Unique Keywords");
        storage
            .store(&persona.to_generic())
            .expect("Failed to store persona");

        let results = storage
            .text_search("Unique Keywords", Some(&["persona".to_string()]), Some(10))
            .expect("Failed to text search");

        assert!(!results.is_empty(), "Should find persona by text search");
        assert_eq!(results[0].entity_type, "persona");
    }

    #[test]
    fn test_empty_storage_returns_no_personas() {
        let (_temp_dir, storage) = setup_test_storage();

        let all = storage
            .get_all(Persona::entity_type())
            .expect("Failed to list from empty storage");
        assert!(all.is_empty());

        let ids = storage
            .list_ids(Persona::entity_type())
            .expect("Failed to list IDs from empty storage");
        assert!(ids.is_empty());
    }
}

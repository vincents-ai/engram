use engram::entities::*;
use engram::nlq::{IntentClassifier, QueryIntent};
use proptest::prelude::*;

// ---------------------------------------------------------------------------
// (a) Entity ID handling — roundtrip through to_generic / from_generic
// ---------------------------------------------------------------------------

proptest! {
    #[test]
    fn task_roundtrip_preserves_fields(
        title in "[a-zA-Z]{1,100}",
        description in ".{0,500}",
        agent in "[a-zA-Z0-9_-]{1,50}"
    ) {
        let task = Task::new(
            title,
            description,
            agent,
            TaskPriority::High,
            None,
        );

        let generic = task.to_generic();
        let restored = Task::from_generic(generic).unwrap();

        assert_eq!(restored.id, task.id);
        assert_eq!(restored.title, task.title);
        assert_eq!(restored.description, task.description);
        assert_eq!(restored.agent, task.agent);
        assert_eq!(restored.priority, task.priority);
        assert_eq!(restored.status, task.status);
    }

    #[test]
    fn context_roundtrip_preserves_fields(
        title in "[a-zA-Z\\s]{1,100}",
        content in ".{1,1000}",
        source in "[a-zA-Z0-9_-]{1,50}"
    ) {
        let ctx = Context::new(
            title,
            content,
            source,
            ContextRelevance::High,
            "agent-1".into(),
        );

        let generic = ctx.to_generic();
        let restored = Context::from_generic(generic).unwrap();

        assert_eq!(restored.id, ctx.id);
        assert_eq!(restored.title, ctx.title);
        assert_eq!(restored.content, ctx.content);
        assert_eq!(restored.source, ctx.source);
        assert_eq!(restored.relevance, ctx.relevance);
    }

    #[test]
    fn task_from_generic_never_panics_on_any_data(data in proptest::collection::vec(proptest::arbitrary::any::<u8>(), 0..2000)) {
        let json_str = String::from_utf8_lossy(&data);
        let generic = GenericEntity {
            id: "prop-test-id".into(),
            entity_type: "task".into(),
            agent: "agent".into(),
            timestamp: chrono::Utc::now(),
            data: serde_json::Value::String(json_str.into_owned()),
        };
        let _ = Task::from_generic(generic);
    }

    #[test]
    fn context_from_generic_never_panics_on_any_data(data in proptest::collection::vec(proptest::arbitrary::any::<u8>(), 0..2000)) {
        let json_str = String::from_utf8_lossy(&data);
        let generic = GenericEntity {
            id: "prop-test-id".into(),
            entity_type: "context".into(),
            agent: "agent".into(),
            timestamp: chrono::Utc::now(),
            data: serde_json::Value::String(json_str.into_owned()),
        };
        let _ = Context::from_generic(generic);
    }
}

// ---------------------------------------------------------------------------
// (b) Task entity validation
// ---------------------------------------------------------------------------

proptest! {
    #[test]
    fn task_with_empty_title_fails_validation(
        description in ".{0,500}",
        agent in "[a-zA-Z0-9_-]{1,50}"
    ) {
        let mut task = Task::new(
            String::new(),
            description,
            agent,
            TaskPriority::Medium,
            None,
        );
        assert!(task.validate_entity().is_err());

        task.title = "non-empty".into();
        assert!(task.validate_entity().is_ok());
    }

    #[test]
    fn task_with_empty_agent_fails_validation(
        title in "[a-zA-Z]{1,100}",
        description in ".{0,500}"
    ) {
        let mut task = Task::new(
            title,
            description,
            String::new(),
            TaskPriority::Low,
            None,
        );
        assert!(task.validate_entity().is_err());

        task.agent = "some-agent".into();
        assert!(task.validate_entity().is_ok());
    }

    #[test]
    fn task_new_produces_valid_entity_for_non_empty_inputs(
        title in "[a-zA-Z\\s]{1,200}",
        description in ".{1,1000}",
        agent in "[a-zA-Z0-9_-]{1,100}"
    ) {
        let priorities = [TaskPriority::Low, TaskPriority::Medium, TaskPriority::High, TaskPriority::Critical];
        for priority in &priorities {
            let task = Task::new(
                title.clone(),
                description.clone(),
                agent.clone(),
                priority.clone(),
                None,
            );
            assert!(!task.id.is_empty());
            assert!(task.validate_entity().is_ok());
        }
    }
}

// ---------------------------------------------------------------------------
// (c) Context entity validation — roundtrip with unicode, empty, long strings
// ---------------------------------------------------------------------------

proptest! {
    #[test]
    fn context_unicode_content_survives_serialization(
        title in "[a-zA-Z]{1,50}",
        content in proptest::string::string_regex("(?:[\\x20-\\x7E]|[\\xC0-\\xF4][\\x80-\\xBF]{0,3}){0,500}").unwrap()
    ) {
        let ctx = Context::new(
            title,
            content.clone(),
            "src".into(),
            ContextRelevance::Medium,
            "agent".into(),
        );

        let generic = ctx.to_generic();
        let restored = Context::from_generic(generic).unwrap();

        assert_eq!(restored.content, content);
    }

    #[test]
    fn context_roundtrip_with_long_content(
        title in "[a-zA-Z]{1,50}",
        base_content in ".{1,200}",
        repeat in 0usize..20
    ) {
        let long_content = base_content.repeat(repeat + 1);
        let ctx = Context::new(
            title,
            long_content.clone(),
            "src".into(),
            ContextRelevance::Low,
            "agent".into(),
        );

        let generic = ctx.to_generic();
        let restored = Context::from_generic(generic).unwrap();
        assert_eq!(restored.content.len(), long_content.len());
        assert_eq!(restored.content, long_content);
    }

    #[test]
    fn context_with_empty_title_or_content_fails_validation(
        content in ".{1,100}",
        title in ".{1,100}"
    ) {
        let empty_title = Context::new(
            String::new(),
            content,
            "src".into(),
            ContextRelevance::Low,
            "agent".into(),
        );
        assert!(empty_title.validate_entity().is_err());

        let empty_content = Context::new(
            title,
            String::new(),
            "src".into(),
            ContextRelevance::Low,
            "agent".into(),
        );
        assert!(empty_content.validate_entity().is_err());
    }
}

// ---------------------------------------------------------------------------
// (d) Workflow state machine invariants
// ---------------------------------------------------------------------------

proptest! {
    #[test]
    fn workflow_valid_transitions_reference_existing_states(
        num_states in 2usize..8,
        num_transitions in 1usize..10,
        seed in 0u64..10000
    ) {
        let mut rng = fastrand::Rng::with_seed(seed);
        let state_ids: Vec<String> = (0..num_states).map(|i| format!("state-{}", i)).collect();

        let states: Vec<WorkflowState> = state_ids.iter().enumerate().map(|(i, id)| {
            WorkflowState {
                id: id.clone(),
                name: format!("State {}", i),
                state_type: if i == 0 { StateType::Start } else { StateType::Done },
                description: format!("Description for state {}", i),
                is_final: i == num_states - 1,
                prompts: None,
                guards: vec![],
                post_functions: vec![],
                commit_policy: None,
            }
        }).collect();

        let mut wf = Workflow::new("Prop Test Workflow".into(), "desc".into(), "agent".into());
        for s in &states {
            wf.add_state(s.clone());
        }
        wf.set_initial_state(state_ids[0].clone());
        wf.add_final_state(state_ids[state_ids.len() - 1].clone());

        for t in 0..num_transitions {
            let from = state_ids[rng.usize(..state_ids.len())].clone();
            let to = state_ids[rng.usize(..state_ids.len())].clone();
            let transition = WorkflowTransition {
                id: format!("t-{}", t),
                name: format!("Transition {}", t),
                from_state: from,
                to_state: to,
                transition_type: TransitionType::Manual,
                description: format!("Trans {}", t),
                conditions: vec![],
                actions: vec![],
                trigger: None,
            };
            wf.add_transition(transition);
        }

        let state_id_set: std::collections::HashSet<&str> = state_ids.iter().map(|s| s.as_str()).collect();

        for transition in &wf.transitions {
            prop_assert!(state_id_set.contains(transition.from_state.as_str()),
                "from_state {} not in states", transition.from_state);
            prop_assert!(state_id_set.contains(transition.to_state.as_str()),
                "to_state {} not in states", transition.to_state);
        }

        prop_assert!(state_id_set.contains(wf.initial_state.as_str()),
            "initial_state {} not in states", wf.initial_state);

        for fs in &wf.final_states {
            prop_assert!(state_id_set.contains(fs.as_str()),
                "final_state {} not in states", fs);
        }
    }
}

// ---------------------------------------------------------------------------
// (e) NLQ query handling — classify never panics
// ---------------------------------------------------------------------------

proptest! {
    #[test]
    fn classify_never_panics_on_arbitrary_input(data in proptest::collection::vec(proptest::arbitrary::any::<u8>(), 0..2000)) {
        let query = String::from_utf8_lossy(&data);
        let classifier = IntentClassifier::new();
        let _ = classifier.classify(&query);
    }

    #[test]
    fn empty_string_returns_unknown(query in proptest::string::string_regex("").unwrap()) {
        let classifier = IntentClassifier::new();
        let intent = classifier.classify(&query).unwrap();
        prop_assert_eq!(intent, QueryIntent::Unknown);
    }

    #[test]
    fn whitespace_only_returns_unknown(ws in proptest::string::string_regex(" *").unwrap()) {
        let classifier = IntentClassifier::new();
        let intent = classifier.classify(&ws).unwrap();
        prop_assert_eq!(intent, QueryIntent::Unknown);
    }

    #[test]
    fn non_empty_string_always_returns_some_intent(non_empty in ".{1,500}") {
        let classifier = IntentClassifier::new();
        let result = classifier.classify(&non_empty);
        prop_assert!(result.is_ok(), "classify panicked or returned Err on input: {:?}", non_empty);
        let intent = result.unwrap();
        prop_assert_ne!(intent, QueryIntent::Unknown,
            "non-empty input should not be Unknown: {:?}", non_empty);
    }

    #[test]
    fn unicode_queries_never_panic(
        prefix in "[a-zA-Z]{0,10}",
        middle in proptest::string::string_regex("[\\x20-\\x7E]{1,50}").unwrap(),
        suffix in "[a-zA-Z]{0,10}"
    ) {
        let query = format!("{}{}{}", prefix, middle, suffix);
        let classifier = IntentClassifier::new();
        let _ = classifier.classify(&query);
    }
}

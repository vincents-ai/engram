//! TUI view unit tests and insta snapshot tests.
//!
//! Uses a deterministic `FixedTestBackend` that returns hard-coded entities
//! so renders are reproducible regardless of the real engram workspace.
//!
//! View draw functions are private, so we go through `LocusTuiApp::draw()`
//! which is exposed under `#[cfg(test)]` in mod.rs.

#[cfg(test)]
mod tests {
    use crate::entities::{
        AgentSandbox, Compliance, Context, ContextRelevance, EntityRelationType,
        EntityRelationship, EscalationRequest, ExecutionResult, Knowledge, ProgressiveGateConfig,
        Reasoning, Rule, Session, Standard, StateReflection, Task, TaskPriority, Theory, Workflow,
        WorkflowInstance, ADR,
    };
    use crate::error::EngramError;
    use crate::locus_tui::backend::LocusTuiBackend;
    use crate::locus_tui::LocusTuiApp;
    use crate::storage::memory_only_storage::MemoryStorage;
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;

    // ── FixedTestBackend ──────────────────────────────────────────────────────

    /// A deterministic in-memory backend that returns fixed test data.
    ///
    /// The returned data is completely independent of any real engram workspace,
    /// making tests reproducible in any environment.
    pub struct FixedTestBackend;

    impl LocusTuiBackend for FixedTestBackend {
        fn list_tasks(&self) -> Result<Vec<Task>, EngramError> {
            let mut t1 = Task::new(
                "Implement OAuth".to_string(),
                "OAuth 2 integration for the API".to_string(),
                "agent-alpha".to_string(),
                TaskPriority::High,
                None,
            );
            t1.start(); // in_progress

            let t2 = Task::new(
                "Write documentation".to_string(),
                "OpenAPI spec and README".to_string(),
                "agent-alpha".to_string(),
                TaskPriority::Medium,
                None,
            );

            let mut t3 = Task::new(
                "Fix rate-limiter bug".to_string(),
                "Rate limiter fires too early on reset".to_string(),
                "agent-beta".to_string(),
                TaskPriority::High,
                None,
            );
            t3.complete("Fixed token bucket algorithm".to_string());

            Ok(vec![t1, t2, t3])
        }

        fn list_contexts(&self) -> Result<Vec<Context>, EngramError> {
            let c1 = Context::new(
                "OAuth spec RFC 6749".to_string(),
                "The OAuth 2.0 Authorization Framework defines a protocol for delegated authorization."
                    .to_string(),
                "https://rfc-editor.org/rfc/rfc6749".to_string(),
                ContextRelevance::High,
                "agent-alpha".to_string(),
            );
            let c2 = Context::new(
                "Rate limiter design notes".to_string(),
                "Token bucket approach: 100 req/s per client IP. Reset on 60s boundary."
                    .to_string(),
                "internal".to_string(),
                ContextRelevance::Medium,
                "agent-beta".to_string(),
            );
            Ok(vec![c1, c2])
        }

        fn list_reasoning(&self) -> Result<Vec<Reasoning>, EngramError> {
            let mut r1 = Reasoning::new(
                "Choose OAuth over API keys".to_string(),
                "task-oauth-001".to_string(),
                "agent-alpha".to_string(),
            );
            r1.set_conclusion(
                "OAuth 2.0 provides delegated auth without sharing credentials".to_string(),
                0.95,
            );

            let mut r2 = Reasoning::new(
                "Token bucket vs leaky bucket".to_string(),
                "task-ratelimit-001".to_string(),
                "agent-beta".to_string(),
            );
            r2.add_step(
                "Compare burst tolerance".to_string(),
                "Token bucket allows short bursts".to_string(),
                0.88,
            );

            Ok(vec![r1, r2])
        }

        fn list_relationships(&self) -> Result<Vec<EntityRelationship>, EngramError> {
            let rel = EntityRelationship::new(
                "agent-alpha".to_string(),
                "agent-alpha".to_string(),
                "src-task-001".to_string(),
                "task".to_string(),
                "tgt-ctx-001".to_string(),
                "context".to_string(),
                EntityRelationType::References,
            );
            Ok(vec![rel])
        }

        fn list_adrs(&self) -> Result<Vec<ADR>, EngramError> {
            let mut adr1 = ADR::new(
                "Use Rust for backend services".to_string(),
                1,
                "agent-alpha".to_string(),
                "Need a memory-safe systems language for high-throughput services".to_string(),
            );
            adr1.accept(
                "Rust for all new backend services".to_string(),
                "Strong type system, zero-cost abstractions, fearless concurrency".to_string(),
            );

            let adr2 = ADR::new(
                "PostgreSQL as primary datastore".to_string(),
                2,
                "agent-beta".to_string(),
                "Need ACID-compliant relational storage".to_string(),
            );

            Ok(vec![adr1, adr2])
        }

        fn list_theories(&self) -> Result<Vec<Theory>, EngramError> {
            let mut t1 = Theory::new("Storage Layer".to_string(), "agent-alpha".to_string());
            t1.add_concept(
                "Entity".to_string(),
                "Typed data blob stored in git objects".to_string(),
            );
            t1.add_rationale(
                "Use git for storage".to_string(),
                "Git provides free versioning and auditability".to_string(),
            );
            t1.add_invariant("Entity IDs are UUID v4".to_string());

            let t2 = Theory::new("API Layer".to_string(), "agent-beta".to_string());

            Ok(vec![t1, t2])
        }

        fn update_adr_status(
            &mut self,
            _id: &str,
            _status: crate::entities::AdrStatus,
        ) -> Result<(), Box<dyn std::error::Error>> {
            Ok(())
        }

        fn update_task_status(
            &mut self,
            _id: &str,
            _status: crate::entities::TaskStatus,
        ) -> Result<(), Box<dyn std::error::Error>> {
            Ok(())
        }

        fn list_workflows(&self) -> Result<Vec<Workflow>, EngramError> {
            Ok(vec![])
        }
        fn list_workflow_instances(&self) -> Result<Vec<WorkflowInstance>, EngramError> {
            Ok(vec![])
        }
        fn list_knowledge(&self) -> Result<Vec<Knowledge>, EngramError> {
            Ok(vec![])
        }
        fn list_sessions(&self) -> Result<Vec<Session>, EngramError> {
            Ok(vec![])
        }
        fn list_compliance(&self) -> Result<Vec<Compliance>, EngramError> {
            Ok(vec![])
        }
        fn list_rules(&self) -> Result<Vec<Rule>, EngramError> {
            Ok(vec![])
        }
        fn list_standards(&self) -> Result<Vec<Standard>, EngramError> {
            Ok(vec![])
        }
        fn list_state_reflections(&self) -> Result<Vec<StateReflection>, EngramError> {
            Ok(vec![])
        }
        fn list_escalations(&self) -> Result<Vec<EscalationRequest>, EngramError> {
            Ok(vec![])
        }
        fn list_sandboxes(&self) -> Result<Vec<AgentSandbox>, EngramError> {
            Ok(vec![])
        }
        fn list_execution_results(&self) -> Result<Vec<ExecutionResult>, EngramError> {
            Ok(vec![])
        }
        fn list_progressive_configs(&self) -> Result<Vec<ProgressiveGateConfig>, EngramError> {
            Ok(vec![])
        }
        fn list_remote_names(&self) -> Vec<String> {
            vec![]
        }
        fn get_sync_status_data(
            &self,
            _remote_name: &str,
        ) -> Result<Vec<crate::locus_tui::app::SyncStatusRow>, EngramError> {
            Ok(vec![])
        }
    }

    // ── Helpers ───────────────────────────────────────────────────────────────

    /// Convert a ratatui buffer to a plain string (rows joined by newlines).
    fn buffer_to_string(buf: &ratatui::buffer::Buffer) -> String {
        let width = buf.area.width as usize;
        buf.content
            .chunks(width)
            .map(|row| {
                row.iter()
                    .map(|cell| cell.symbol())
                    .collect::<Vec<_>>()
                    .join("")
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Build a fully loaded LocusTuiApp backed by FixedTestBackend.
    fn make_loaded_app() -> LocusTuiApp<MemoryStorage> {
        let storage = MemoryStorage::new("test-agent");
        let backend: Box<dyn LocusTuiBackend> = Box::new(FixedTestBackend);
        let mut app = LocusTuiApp::new_with_backend(storage, backend);
        app.load_all_data();
        app
    }

    /// Render the app to a fresh 120×40 terminal and return the buffer content.
    fn render_to_string(app: &mut LocusTuiApp<MemoryStorage>) -> String {
        let backend = TestBackend::new(120, 40);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal.draw(|f| app.draw(f)).unwrap();
        buffer_to_string(terminal.backend().buffer())
    }

    // ── FixedTestBackend unit tests ───────────────────────────────────────────

    #[test]
    fn test_fixed_backend_implements_trait() {
        let b: Box<dyn LocusTuiBackend> = Box::new(FixedTestBackend);
        // Must compile — trait is object-safe.
        let tasks = b.list_tasks().unwrap();
        assert!(!tasks.is_empty());
    }

    #[test]
    fn test_fixed_backend_list_tasks_returns_three() {
        let b = FixedTestBackend;
        let tasks = b.list_tasks().unwrap();
        assert_eq!(tasks.len(), 3);
        assert_eq!(tasks[0].title, "Implement OAuth");
        assert_eq!(tasks[1].title, "Write documentation");
        assert_eq!(tasks[2].title, "Fix rate-limiter bug");
    }

    #[test]
    fn test_fixed_backend_task_statuses() {
        use crate::entities::TaskStatus;
        let b = FixedTestBackend;
        let tasks = b.list_tasks().unwrap();
        assert_eq!(tasks[0].status, TaskStatus::InProgress);
        assert_eq!(tasks[1].status, TaskStatus::Todo);
        assert_eq!(tasks[2].status, TaskStatus::Done);
    }

    #[test]
    fn test_fixed_backend_list_contexts_returns_two() {
        let b = FixedTestBackend;
        let ctxs = b.list_contexts().unwrap();
        assert_eq!(ctxs.len(), 2);
        assert_eq!(ctxs[0].title, "OAuth spec RFC 6749");
        assert_eq!(ctxs[1].title, "Rate limiter design notes");
    }

    #[test]
    fn test_fixed_backend_list_reasoning_returns_two() {
        let b = FixedTestBackend;
        let rsns = b.list_reasoning().unwrap();
        assert_eq!(rsns.len(), 2);
        assert_eq!(rsns[0].title, "Choose OAuth over API keys");
        assert_eq!(rsns[1].title, "Token bucket vs leaky bucket");
    }

    #[test]
    fn test_fixed_backend_list_adrs_returns_two() {
        let b = FixedTestBackend;
        let adrs = b.list_adrs().unwrap();
        assert_eq!(adrs.len(), 2);
        assert_eq!(adrs[0].title, "Use Rust for backend services");
        assert_eq!(adrs[0].number, 1);
        assert_eq!(adrs[1].title, "PostgreSQL as primary datastore");
        assert_eq!(adrs[1].number, 2);
    }

    #[test]
    fn test_fixed_backend_list_theories_returns_two() {
        let b = FixedTestBackend;
        let theories = b.list_theories().unwrap();
        assert_eq!(theories.len(), 2);
        assert_eq!(theories[0].domain_name, "Storage Layer");
        assert_eq!(theories[1].domain_name, "API Layer");
    }

    #[test]
    fn test_fixed_backend_list_relationships_returns_one() {
        let b = FixedTestBackend;
        let rels = b.list_relationships().unwrap();
        assert_eq!(rels.len(), 1);
    }

    // ── load_all_data integration tests ──────────────────────────────────────

    #[test]
    fn test_load_all_data_populates_tasks() {
        let app = make_loaded_app();
        assert_eq!(app.app_state.recent_tasks.len(), 3);
        assert_eq!(app.app_state.all_tasks.len(), 3);
        assert_eq!(app.app_state.task_summary.total, 3);
        assert_eq!(app.app_state.task_summary.in_progress, 1);
        assert_eq!(app.app_state.task_summary.todo, 1);
        assert_eq!(app.app_state.task_summary.done, 1);
    }

    #[test]
    fn test_load_all_data_populates_contexts() {
        let app = make_loaded_app();
        assert_eq!(app.app_state.contexts.len(), 2);
        assert_eq!(app.app_state.contexts[0].title, "OAuth spec RFC 6749");
    }

    #[test]
    fn test_load_all_data_populates_reasoning_nodes() {
        let app = make_loaded_app();
        assert_eq!(app.app_state.reasoning_nodes.len(), 2);
        assert_eq!(
            app.app_state.reasoning_nodes[0].title,
            "Choose OAuth over API keys"
        );
    }

    #[test]
    fn test_load_all_data_populates_adrs() {
        let app = make_loaded_app();
        assert_eq!(app.app_state.all_adrs.len(), 2);
        assert_eq!(
            app.app_state.all_adrs[0].title,
            "Use Rust for backend services"
        );
    }

    #[test]
    fn test_load_all_data_populates_theories() {
        let app = make_loaded_app();
        assert_eq!(app.app_state.all_theories.len(), 2);
        assert_eq!(app.app_state.all_theories[0].domain_name, "Storage Layer");
    }

    // ── Dashboard view render tests ───────────────────────────────────────────

    #[test]
    fn test_dashboard_view_renders_title_bar() {
        let mut app = make_loaded_app();
        let content = render_to_string(&mut app);
        assert!(
            content.contains("Engram Locus"),
            "expected 'Engram Locus' in: {content}"
        );
        assert!(
            content.contains("[Dashboard]"),
            "expected '[Dashboard]' in: {content}"
        );
    }

    #[test]
    fn test_dashboard_view_shows_task_counts() {
        let mut app = make_loaded_app();
        let content = render_to_string(&mut app);
        // The title bar shows counts from the LocusIntegration (MemoryStorage),
        // which is separate from the FixedTestBackend. The app_state summary
        // stats (from load_all_data) show 3 total tasks in the stat boxes.
        assert!(content.contains("Total"), "missing Total stat box");
        // The stat boxes show the correct counts from app_state
        assert!(content.contains("  3"), "expected count 3 in stat box");
    }

    #[test]
    fn test_dashboard_view_shows_summary_stats() {
        let mut app = make_loaded_app();
        let content = render_to_string(&mut app);
        // Summary stat boxes: Total, Todo, In Progress, Done
        assert!(content.contains("Total"), "missing Total panel");
        assert!(content.contains("Todo"), "missing Todo panel");
        assert!(content.contains("Done"), "missing Done panel");
    }

    #[test]
    fn test_dashboard_view_shows_recent_tasks_table() {
        let mut app = make_loaded_app();
        let content = render_to_string(&mut app);
        assert!(
            content.contains("Recent Tasks"),
            "missing Recent Tasks header"
        );
        assert!(
            content.contains("Implement OAuth"),
            "missing task title in: {content}"
        );
    }

    // ── Tasks view render tests ───────────────────────────────────────────────

    #[test]
    fn test_tasks_view_shows_filter_bar() {
        let mut app = make_loaded_app();
        app.app_state.next_view(); // Dashboard -> Tasks
        let content = render_to_string(&mut app);
        assert!(
            content.contains("Filters"),
            "missing Filters bar in: {content}"
        );
        assert!(content.contains("[All]"), "missing [All] filter chip");
    }

    #[test]
    fn test_tasks_view_shows_task_rows() {
        let mut app = make_loaded_app();
        app.app_state.next_view(); // Tasks
        let content = render_to_string(&mut app);
        assert!(
            content.contains("Implement OAuth"),
            "missing task in tasks view"
        );
        assert!(content.contains("Write documentation"), "missing task 2");
        assert!(content.contains("Fix rate-limiter bug"), "missing task 3");
    }

    #[test]
    fn test_tasks_view_title_shows_count() {
        let mut app = make_loaded_app();
        app.app_state.next_view();
        let content = render_to_string(&mut app);
        // "Tasks (3 / 3)" or similar
        assert!(
            content.contains("Tasks (3"),
            "expected task count in title: {content}"
        );
    }

    // ── Reasoning view render tests ───────────────────────────────────────────

    #[test]
    fn test_reasoning_view_shows_node_count() {
        let mut app = make_loaded_app();
        // Navigate to Reasoning
        app.app_state.next_view(); // Tasks
        app.app_state.next_view(); // Reasoning
        let content = render_to_string(&mut app);
        assert!(
            content.contains("2 nodes") || content.contains("[2 nodes]"),
            "expected node count in reasoning view: {content}"
        );
    }

    #[test]
    fn test_reasoning_view_shows_node_titles() {
        let mut app = make_loaded_app();
        app.app_state.next_view();
        app.app_state.next_view();
        let content = render_to_string(&mut app);
        assert!(
            content.contains("Choose OAuth over API keys"),
            "missing reasoning node: {content}"
        );
        assert!(
            content.contains("Token bucket vs leaky bucket"),
            "missing reasoning node 2: {content}"
        );
    }

    #[test]
    fn test_reasoning_view_shows_detail_pane() {
        let mut app = make_loaded_app();
        app.app_state.next_view();
        app.app_state.next_view();
        let content = render_to_string(&mut app);
        assert!(content.contains("Detail"), "missing Detail pane: {content}");
    }

    // ── Relationships view render tests ──────────────────────────────────────

    #[test]
    fn test_relationships_view_shows_nodes_pane() {
        let mut app = make_loaded_app();
        // Navigate to Relationships (index 3)
        for _ in 0..3 {
            app.app_state.next_view();
        }
        let content = render_to_string(&mut app);
        assert!(
            content.contains("Nodes"),
            "missing Nodes pane in relationships view: {content}"
        );
    }

    #[test]
    fn test_relationships_view_shows_edges_pane() {
        let mut app = make_loaded_app();
        for _ in 0..3 {
            app.app_state.next_view();
        }
        let content = render_to_string(&mut app);
        assert!(
            content.contains("Edges from:"),
            "missing Edges pane: {content}"
        );
    }

    // ── Contexts view render tests ────────────────────────────────────────────

    #[test]
    fn test_contexts_view_shows_context_table() {
        let mut app = make_loaded_app();
        // Navigate to Contexts (index 4)
        for _ in 0..4 {
            app.app_state.next_view();
        }
        let content = render_to_string(&mut app);
        assert!(
            content.contains("Contexts (2)"),
            "expected Contexts (2) in title: {content}"
        );
        assert!(
            content.contains("OAuth spec RFC 6749"),
            "missing context title: {content}"
        );
    }

    #[test]
    fn test_contexts_view_shows_content_pane() {
        let mut app = make_loaded_app();
        for _ in 0..4 {
            app.app_state.next_view();
        }
        let content = render_to_string(&mut app);
        assert!(
            content.contains("Content"),
            "missing Content pane: {content}"
        );
    }

    #[test]
    fn test_contexts_view_shows_first_context_content() {
        let mut app = make_loaded_app();
        for _ in 0..4 {
            app.app_state.next_view();
        }
        let content = render_to_string(&mut app);
        // The detail pane shows the selected context's content
        assert!(
            content.contains("OAuth 2.0 Authorization Framework"),
            "missing context content: {content}"
        );
    }

    // ── ADRs view render tests ────────────────────────────────────────────────

    #[test]
    fn test_adrs_view_shows_adr_table() {
        let mut app = make_loaded_app();
        // Navigate to ADRs (index 5)
        for _ in 0..5 {
            app.app_state.next_view();
        }
        let content = render_to_string(&mut app);
        assert!(
            content.contains("ADRs (2)"),
            "expected ADRs (2) in title: {content}"
        );
        assert!(
            content.contains("Use Rust for backend services"),
            "missing ADR title: {content}"
        );
    }

    #[test]
    fn test_adrs_view_shows_second_adr() {
        let mut app = make_loaded_app();
        for _ in 0..5 {
            app.app_state.next_view();
        }
        let content = render_to_string(&mut app);
        assert!(
            content.contains("PostgreSQL as primary datastore"),
            "missing ADR 2 title: {content}"
        );
    }

    #[test]
    fn test_adrs_view_shows_detail_pane() {
        let mut app = make_loaded_app();
        for _ in 0..5 {
            app.app_state.next_view();
        }
        let content = render_to_string(&mut app);
        assert!(content.contains("Detail"), "missing Detail pane: {content}");
        // First ADR has context
        assert!(
            content.contains("memory-safe systems language"),
            "missing ADR context in detail pane: {content}"
        );
    }

    #[test]
    fn test_adrs_view_shows_adr_status() {
        let mut app = make_loaded_app();
        for _ in 0..5 {
            app.app_state.next_view();
        }
        let content = render_to_string(&mut app);
        // First ADR is accepted, second is proposed
        assert!(
            content.contains("accepted"),
            "missing accepted status: {content}"
        );
        assert!(
            content.contains("proposed"),
            "missing proposed status: {content}"
        );
    }

    // ── Theories view render tests ────────────────────────────────────────────

    #[test]
    fn test_theories_view_shows_theory_list() {
        let mut app = make_loaded_app();
        // Navigate to Theories (index 6)
        for _ in 0..6 {
            app.app_state.next_view();
        }
        let content = render_to_string(&mut app);
        assert!(
            content.contains("Theories (2)"),
            "expected Theories (2): {content}"
        );
        assert!(
            content.contains("Storage Layer"),
            "missing theory domain: {content}"
        );
    }

    #[test]
    fn test_theories_view_shows_second_theory() {
        let mut app = make_loaded_app();
        for _ in 0..6 {
            app.app_state.next_view();
        }
        let content = render_to_string(&mut app);
        assert!(
            content.contains("API Layer"),
            "missing API Layer theory: {content}"
        );
    }

    #[test]
    fn test_theories_view_shows_detail_pane() {
        let mut app = make_loaded_app();
        for _ in 0..6 {
            app.app_state.next_view();
        }
        let content = render_to_string(&mut app);
        assert!(content.contains("Detail"), "missing Detail pane: {content}");
        assert!(
            content.contains("Domain:"),
            "missing Domain label: {content}"
        );
        assert!(
            content.contains("Storage Layer"),
            "missing domain name in detail: {content}"
        );
    }

    #[test]
    fn test_theories_view_shows_conceptual_model() {
        let mut app = make_loaded_app();
        for _ in 0..6 {
            app.app_state.next_view();
        }
        let content = render_to_string(&mut app);
        assert!(
            content.contains("Conceptual Model:"),
            "missing Conceptual Model section: {content}"
        );
    }

    // ── Search view render tests ──────────────────────────────────────────────

    #[test]
    fn test_search_view_shows_input_bar() {
        let mut app = make_loaded_app();
        // Navigate to Search (last view, index 19)
        for _ in 0..19 {
            app.app_state.next_view();
        }
        let content = render_to_string(&mut app);
        assert!(content.contains("Search"), "missing Search bar: {content}");
    }

    #[test]
    fn test_search_view_empty_state_prompt() {
        let mut app = make_loaded_app();
        for _ in 0..19 {
            app.app_state.next_view();
        }
        let content = render_to_string(&mut app);
        assert!(
            content.contains("Press / to enter search mode"),
            "missing empty search prompt: {content}"
        );
    }

    // ── Auto-refresh logic tests ──────────────────────────────────────────────

    #[test]
    fn test_auto_refresh_disabled_when_interval_zero() {
        let storage = MemoryStorage::new("test-agent");
        let backend: Box<dyn LocusTuiBackend> = Box::new(FixedTestBackend);
        let mut app = LocusTuiApp::new_with_refresh_interval(storage, backend, 0);
        // With interval=0 should_auto_refresh must always be false
        assert!(
            !app.app_state.should_auto_refresh(),
            "refresh should be disabled with interval=0"
        );
        assert!(
            !app.app_state.should_auto_refresh(),
            "second call still disabled"
        );
    }

    #[test]
    fn test_auto_refresh_not_fired_before_interval_elapses() {
        let storage = MemoryStorage::new("test-agent");
        let backend: Box<dyn LocusTuiBackend> = Box::new(FixedTestBackend);
        // Very long interval — should never fire in test execution
        let mut app = LocusTuiApp::new_with_refresh_interval(storage, backend, 9999);
        assert!(
            !app.app_state.should_auto_refresh(),
            "refresh must not fire before interval"
        );
    }

    #[test]
    fn test_auto_refresh_fires_when_interval_elapsed() {
        use std::time::Duration;
        let storage = MemoryStorage::new("test-agent");
        let backend: Box<dyn LocusTuiBackend> = Box::new(FixedTestBackend);
        let mut app = LocusTuiApp::new_with_refresh_interval(storage, backend, 30);
        // Backdate the timer so it looks stale
        app.app_state.last_refresh = std::time::Instant::now()
            .checked_sub(Duration::from_secs(60))
            .unwrap_or(std::time::Instant::now());
        assert!(
            app.app_state.should_auto_refresh(),
            "refresh must fire after interval"
        );
    }

    #[test]
    fn test_auto_refresh_resets_timer_on_fire() {
        use std::time::Duration;
        let storage = MemoryStorage::new("test-agent");
        let backend: Box<dyn LocusTuiBackend> = Box::new(FixedTestBackend);
        let mut app = LocusTuiApp::new_with_refresh_interval(storage, backend, 30);
        // Backdate so timer fires
        app.app_state.last_refresh = std::time::Instant::now()
            .checked_sub(Duration::from_secs(60))
            .unwrap_or(std::time::Instant::now());
        // First call: fires and resets
        assert!(app.app_state.should_auto_refresh());
        // Second call immediately after: should NOT fire (timer was reset)
        assert!(
            !app.app_state.should_auto_refresh(),
            "refresh must not fire again immediately after firing"
        );
    }

    // ── Insta snapshot tests ──────────────────────────────────────────────────
    //
    // Task IDs are UUID v4 prefixes (8 hex chars), which change each run.
    // We use insta's `filters` setting to redact them so snapshots are stable.

    /// Helper: render a view and assert a redacted snapshot.
    ///
    /// Replaces 8-char hex ID strings (task row IDs) with `[ID]` so the
    /// snapshots are deterministic across runs.
    fn snapshot_with_redacted_ids(name: &str, content: &str) {
        insta::with_settings!({
            filters => vec![
                // Redact full UUIDs first (36-char form: 8-4-4-4-12)
                (r"[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}", "[UUID]"),
                // Redact remaining 8-hex-char short IDs (task row prefixes)
                (r"[0-9a-f]{8}", "[ID]"),
            ],
        }, {
            insta::assert_snapshot!(name, content);
        });
    }

    #[test]
    fn snapshot_dashboard_view() {
        let mut app = make_loaded_app();
        let content = render_to_string(&mut app);
        snapshot_with_redacted_ids("dashboard_view", &content);
    }

    #[test]
    fn snapshot_tasks_view() {
        let mut app = make_loaded_app();
        app.app_state.next_view(); // Tasks
        let content = render_to_string(&mut app);
        snapshot_with_redacted_ids("tasks_view", &content);
    }

    #[test]
    fn snapshot_reasoning_view() {
        let mut app = make_loaded_app();
        app.app_state.next_view();
        app.app_state.next_view(); // Reasoning
        let content = render_to_string(&mut app);
        snapshot_with_redacted_ids("reasoning_view", &content);
    }

    #[test]
    fn snapshot_relationships_view() {
        let mut app = make_loaded_app();
        for _ in 0..3 {
            app.app_state.next_view();
        } // Relationships
        let content = render_to_string(&mut app);
        snapshot_with_redacted_ids("relationships_view", &content);
    }

    #[test]
    fn snapshot_contexts_view() {
        let mut app = make_loaded_app();
        for _ in 0..4 {
            app.app_state.next_view();
        } // Contexts
        let content = render_to_string(&mut app);
        snapshot_with_redacted_ids("contexts_view", &content);
    }

    #[test]
    fn snapshot_adrs_view() {
        let mut app = make_loaded_app();
        for _ in 0..5 {
            app.app_state.next_view();
        } // ADRs
        let content = render_to_string(&mut app);
        snapshot_with_redacted_ids("adrs_view", &content);
    }

    #[test]
    fn snapshot_theories_view() {
        let mut app = make_loaded_app();
        for _ in 0..6 {
            app.app_state.next_view();
        } // Theories
        let content = render_to_string(&mut app);
        snapshot_with_redacted_ids("theories_view", &content);
    }

    #[test]
    fn snapshot_search_view_empty() {
        let mut app = make_loaded_app();
        for _ in 0..19 {
            app.app_state.next_view();
        } // Search (index 19)
        let content = render_to_string(&mut app);
        snapshot_with_redacted_ids("search_view_empty", &content);
    }

    // ── Sync view render tests ────────────────────────────────────────────────

    /// Navigate from Dashboard to Sync view (index 20).
    fn make_sync_app() -> LocusTuiApp<MemoryStorage> {
        let mut app = make_loaded_app();
        for _ in 0..20 {
            app.app_state.next_view();
        }
        app
    }

    #[test]
    fn test_sync_view_renders_remotes_pane() {
        let mut app = make_sync_app();
        let content = render_to_string(&mut app);
        assert!(
            content.contains("Remotes"),
            "missing Remotes pane in Sync view: {content}"
        );
    }

    #[test]
    fn test_sync_view_empty_state_shows_no_remotes_message() {
        let mut app = make_sync_app();
        let content = render_to_string(&mut app);
        assert!(
            content.contains("no remotes configured"),
            "missing empty-remotes message in Sync view: {content}"
        );
    }

    #[test]
    fn test_sync_view_renders_sync_status_pane() {
        let mut app = make_sync_app();
        let content = render_to_string(&mut app);
        assert!(
            content.contains("Sync Status"),
            "missing Sync Status pane: {content}"
        );
    }

    #[test]
    fn test_sync_view_renders_last_operation_pane() {
        let mut app = make_sync_app();
        let content = render_to_string(&mut app);
        assert!(
            content.contains("Last Operation"),
            "missing Last Operation pane: {content}"
        );
    }

    #[test]
    fn test_sync_view_shows_key_hints() {
        let mut app = make_sync_app();
        let content = render_to_string(&mut app);
        // Key hints line rendered in the Last Operation pane
        assert!(content.contains("ull"), "missing pull hint: {content}");
        assert!(content.contains("push"), "missing push hint: {content}");
        assert!(content.contains("oth"), "missing both hint: {content}");
    }

    #[test]
    fn test_sync_view_initial_no_op_result() {
        let mut app = make_sync_app();
        let content = render_to_string(&mut app);
        assert!(
            content.contains("No operation yet"),
            "expected 'No operation yet' in fresh Sync view: {content}"
        );
    }

    #[test]
    fn test_sync_view_shows_last_op_result_after_set() {
        let mut app = make_sync_app();
        app.app_state.sync_view.last_op_result = Some("push: 4 refs pushed".to_string());
        let content = render_to_string(&mut app);
        assert!(
            content.contains("push: 4 refs pushed"),
            "expected op result in Sync view: {content}"
        );
    }

    #[test]
    fn test_sync_view_with_remotes_shows_remote_name() {
        let mut app = make_sync_app();
        app.app_state.sync_view.remotes = vec!["origin".to_string(), "backup".to_string()];
        let content = render_to_string(&mut app);
        assert!(
            content.contains("origin"),
            "expected remote name 'origin' in Sync view: {content}"
        );
    }

    #[test]
    fn test_sync_view_status_rows_render_in_table() {
        use crate::locus_tui::app::SyncStatusRow;
        let mut app = make_sync_app();
        app.app_state.sync_view.status_rows = vec![
            SyncStatusRow {
                entity_type: "task".to_string(),
                local_count: 12,
                remote_count: 10,
                conflicts: 2,
            },
            SyncStatusRow {
                entity_type: "context".to_string(),
                local_count: 5,
                remote_count: 5,
                conflicts: 0,
            },
        ];
        let content = render_to_string(&mut app);
        assert!(content.contains("task"), "expected 'task' row: {content}");
        assert!(
            content.contains("context"),
            "expected 'context' row: {content}"
        );
        assert!(content.contains("12"), "expected local count 12: {content}");
    }

    #[test]
    fn snapshot_sync_view_empty() {
        let mut app = make_sync_app();
        let content = render_to_string(&mut app);
        snapshot_with_redacted_ids("sync_view_empty", &content);
    }

    #[test]
    fn snapshot_sync_view_with_remotes() {
        use crate::locus_tui::app::SyncStatusRow;
        let mut app = make_sync_app();
        app.app_state.sync_view.remotes = vec!["origin".to_string()];
        app.app_state.sync_view.remotes_selected = 0;
        app.app_state.sync_view.status_rows = vec![
            SyncStatusRow {
                entity_type: "task".to_string(),
                local_count: 7,
                remote_count: 6,
                conflicts: 1,
            },
            SyncStatusRow {
                entity_type: "context".to_string(),
                local_count: 3,
                remote_count: 3,
                conflicts: 0,
            },
        ];
        app.app_state.sync_view.last_op_result = Some("pull: 1 fetched, 1 conflicts".to_string());
        let content = render_to_string(&mut app);
        snapshot_with_redacted_ids("sync_view_with_remotes", &content);
    }

    // ── selected_remote_name unit tests ──────────────────────────────────────

    #[test]
    fn test_selected_remote_name_empty_returns_none() {
        let mut app = make_loaded_app();
        // No remotes loaded — should return None
        assert!(app.app_state.sync_view.remotes.is_empty());
        // Directly inspect via test_dispatch so we exercise the no-remote path
        app.test_dispatch(crate::locus_tui::events::Action::SyncPull);
        assert_eq!(
            app.app_state.sync_view.last_op_result.as_deref(),
            Some("No remote selected"),
            "expected 'No remote selected' when remotes list is empty"
        );
    }

    #[test]
    fn test_selected_remote_name_returns_first_by_default() {
        let mut app = make_loaded_app();
        app.app_state.sync_view.remotes = vec!["origin".to_string(), "backup".to_string()];
        app.app_state.sync_view.remotes_selected = 0;
        // selected_remote_name() is private; verify via SyncPull result message
        // (with no remotes.json on disk it will hit auth error, not "No remote selected")
        app.test_dispatch(crate::locus_tui::events::Action::SyncPull);
        let result = app
            .app_state
            .sync_view
            .last_op_result
            .clone()
            .unwrap_or_default();
        assert!(
            !result.contains("No remote selected"),
            "should have attempted pull on 'origin', got: {result}"
        );
    }

    #[test]
    fn test_selected_remote_name_respects_selection_index() {
        let mut app = make_loaded_app();
        app.app_state.sync_view.remotes = vec!["origin".to_string(), "backup".to_string()];
        app.app_state.sync_view.remotes_selected = 1; // "backup" selected
        app.test_dispatch(crate::locus_tui::events::Action::SyncPull);
        let result = app
            .app_state
            .sync_view
            .last_op_result
            .clone()
            .unwrap_or_default();
        assert!(
            !result.contains("No remote selected"),
            "should have attempted pull on 'backup', got: {result}"
        );
    }

    // ── Sync action handler tests ─────────────────────────────────────────────

    #[test]
    fn test_sync_pull_no_remote_sets_result() {
        let mut app = make_loaded_app();
        // No remotes → "No remote selected"
        app.test_dispatch(crate::locus_tui::events::Action::SyncPull);
        assert_eq!(
            app.app_state.sync_view.last_op_result.as_deref(),
            Some("No remote selected")
        );
        assert!(!app.app_state.sync_view.op_in_flight);
    }

    #[test]
    fn test_sync_push_no_remote_sets_result() {
        let mut app = make_loaded_app();
        app.test_dispatch(crate::locus_tui::events::Action::SyncPush);
        assert_eq!(
            app.app_state.sync_view.last_op_result.as_deref(),
            Some("No remote selected")
        );
        assert!(!app.app_state.sync_view.op_in_flight);
    }

    #[test]
    fn test_sync_both_no_remote_sets_result() {
        let mut app = make_loaded_app();
        app.test_dispatch(crate::locus_tui::events::Action::SyncBoth);
        assert_eq!(
            app.app_state.sync_view.last_op_result.as_deref(),
            Some("No remote selected")
        );
        assert!(!app.app_state.sync_view.op_in_flight);
    }

    #[test]
    fn test_refresh_sync_status_no_remote_clears_status() {
        let mut app = make_loaded_app();
        app.app_state.set_status("stale".to_string());
        app.test_dispatch(crate::locus_tui::events::Action::RefreshSyncStatus);
        // op_in_flight must not be set by RefreshSyncStatus
        assert!(!app.app_state.sync_view.op_in_flight);
    }

    #[test]
    fn test_sync_pull_clears_op_in_flight_after_completion() {
        let mut app = make_loaded_app();
        // Even with no remote, op_in_flight must be false after dispatch returns
        app.test_dispatch(crate::locus_tui::events::Action::SyncPull);
        assert!(
            !app.app_state.sync_view.op_in_flight,
            "op_in_flight must be cleared after SyncPull returns"
        );
    }

    #[test]
    fn test_sync_push_clears_op_in_flight_after_completion() {
        let mut app = make_loaded_app();
        app.test_dispatch(crate::locus_tui::events::Action::SyncPush);
        assert!(!app.app_state.sync_view.op_in_flight);
    }

    #[test]
    fn test_sync_both_clears_op_in_flight_after_completion() {
        let mut app = make_loaded_app();
        app.test_dispatch(crate::locus_tui::events::Action::SyncBoth);
        assert!(!app.app_state.sync_view.op_in_flight);
    }

    #[test]
    fn test_load_sync_data_populates_from_backend() {
        let mut app = make_loaded_app();
        // FixedTestBackend returns empty remotes — verify state is consistent
        app.test_load_sync_data();
        assert!(
            app.app_state.sync_view.remotes.is_empty(),
            "FixedTestBackend has no remotes"
        );
        assert_eq!(app.app_state.sync_view.remotes_selected, 0);
    }
}

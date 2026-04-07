pub mod app;
pub mod backend;
pub mod events;
pub mod theme;
pub mod ui;

#[cfg(test)]
mod tui_tests;

use crate::entities::TaskStatus;
use crate::locus_integration::LocusIntegration;
use crate::locus_tui::app::{
    build_relationship_nodes, build_title_map, compute_summary, reasoning_to_node, task_to_row,
    AppState,
};
use crate::locus_tui::backend::{GitEngramBackend, LocusTuiBackend};
use crate::locus_tui::events::Action;
use crate::storage::{RelationshipStorage, Storage};
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;

/// Drop guard that restores the terminal to its original state.
struct TerminalGuard;

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture);
    }
}

pub struct LocusTuiApp<S: Storage + RelationshipStorage> {
    integration: LocusIntegration<S>,
    backend: Box<dyn LocusTuiBackend>,
    app_state: AppState,
}

impl<S: Storage + RelationshipStorage + Send + 'static> LocusTuiApp<S> {
    pub fn new(storage: S) -> Self {
        // `new_with_refresh_interval` is preferred for production; this
        // constructor is retained for call sites that don't need a custom
        // backend. It builds a GitRefsStorage-backed backend pointing at CWD,
        // matching what every CLI command uses.
        let backend: Box<dyn LocusTuiBackend> = match GitEngramBackend::new() {
            Ok(b) => Box::new(b),
            Err(e) => {
                // Surface the error as a warning — empty data is always wrong
                // in production. Fallback to memory keeps the TUI usable in
                // CI / test environments where there is no git repo.
                eprintln!("locus: warning: could not open git storage: {e}");
                let mem = crate::storage::memory_only_storage::MemoryStorage::new("locus-tui");
                let fallback = crate::locus_tui::backend::EngramBackend::from_storage(mem);
                Box::new(fallback)
            }
        };

        // Read refresh interval from workspace config; default to 30s if unavailable.
        let refresh_interval_secs =
            crate::config::workspace_config::WorkspaceConfig::default().refresh_interval_secs;

        let mut app_state = AppState::new();
        app_state.refresh_interval_secs = refresh_interval_secs;

        Self {
            integration: LocusIntegration::new(storage),
            backend,
            app_state,
        }
    }

    /// Create a TUI app using a specific backend (useful for tests).
    pub fn new_with_backend(storage: S, backend: Box<dyn LocusTuiBackend>) -> Self {
        Self {
            integration: LocusIntegration::new(storage),
            backend,
            app_state: AppState::new(),
        }
    }

    /// Create a TUI app with a specific refresh interval (seconds; 0 = disabled).
    pub fn new_with_refresh_interval(
        storage: S,
        backend: Box<dyn LocusTuiBackend>,
        refresh_interval_secs: u64,
    ) -> Self {
        let mut app_state = AppState::new();
        app_state.refresh_interval_secs = refresh_interval_secs;
        Self {
            integration: LocusIntegration::new(storage),
            backend,
            app_state,
        }
    }

    /// Load all data from the backend into AppState before the render loop.
    fn load_all_data(&mut self) {
        let tasks = self.backend.list_tasks().unwrap_or_default();
        let recent_tasks: Vec<_> = tasks.iter().map(task_to_row).collect();
        let task_summary = compute_summary(&recent_tasks);
        self.app_state.all_tasks = tasks;
        self.app_state.recent_tasks = recent_tasks;
        self.app_state.task_summary = task_summary;

        let contexts = self.backend.list_contexts().unwrap_or_default();
        self.app_state.contexts = contexts;

        let reasoning = self.backend.list_reasoning().unwrap_or_default();
        self.app_state.reasoning_nodes = reasoning.iter().map(reasoning_to_node).collect();
        self.app_state.all_reasoning = reasoning;

        let adrs = self.backend.list_adrs().unwrap_or_default();
        self.app_state.all_adrs = adrs;

        let theories = self.backend.list_theories().unwrap_or_default();
        self.app_state.all_theories = theories;

        // New entity types (must be loaded before build_title_map)
        self.app_state.all_workflows = self.backend.list_workflows().unwrap_or_default();
        self.app_state.workflow_count = self.app_state.all_workflows.len();
        self.app_state.all_workflow_instances =
            self.backend.list_workflow_instances().unwrap_or_default();
        self.app_state.all_knowledge = self.backend.list_knowledge().unwrap_or_default();
        self.app_state.all_sessions = self.backend.list_sessions().unwrap_or_default();
        self.app_state.all_compliance = self.backend.list_compliance().unwrap_or_default();
        self.app_state.all_rules = self.backend.list_rules().unwrap_or_default();
        self.app_state.all_standards = self.backend.list_standards().unwrap_or_default();
        self.app_state.all_state_reflections =
            self.backend.list_state_reflections().unwrap_or_default();
        self.app_state.all_escalations = self.backend.list_escalations().unwrap_or_default();
        self.app_state.all_sandboxes = self.backend.list_sandboxes().unwrap_or_default();
        self.app_state.all_execution_results =
            self.backend.list_execution_results().unwrap_or_default();
        self.app_state.all_progressive_configs =
            self.backend.list_progressive_configs().unwrap_or_default();

        let rels = self.backend.list_relationships().unwrap_or_default();
        let title_map = build_title_map(
            &self.app_state.all_tasks,
            &self.app_state.contexts,
            &self.app_state.all_reasoning,
            &self.app_state.all_adrs,
            &self.app_state.all_theories,
            &self.app_state.all_workflows,
            &self.app_state.all_workflow_instances,
            &self.app_state.all_knowledge,
            &self.app_state.all_sessions,
            &self.app_state.all_compliance,
            &self.app_state.all_rules,
            &self.app_state.all_standards,
            &self.app_state.all_state_reflections,
            &self.app_state.all_escalations,
        );
        self.app_state.relationship_nodes = build_relationship_nodes(&rels, &title_map);
    }

    /// Dispatch a high-level Action returned by handle_input.
    fn dispatch_action(&mut self, action: Action) {
        match action {
            Action::Refresh => {
                self.load_all_data();
                self.app_state.reset_refresh_timer();
                self.app_state.clear_status();
            }
            Action::OpenTaskDetail => {
                self.app_state.open_task_detail();
            }
            Action::CloseDetail => {
                self.app_state.close_task_detail();
            }
            Action::CycleTaskStatus => {
                self.cycle_selected_task_status();
            }
            Action::CycleAdrStatus => {
                if let Some((id, new_status)) = self.app_state.cycle_selected_adr_status() {
                    let _ = self.backend.update_adr_status(&id, new_status);
                    self.app_state.set_status("ADR status updated".to_string());
                }
            }
            Action::EnterSearchMode => {
                self.app_state.search_mode = true;
                self.app_state.search_query.clear();
                self.app_state.search_results.clear();
            }
            Action::ExitSearchMode => {
                self.app_state.search_mode = false;
            }
            Action::SearchQueryChar(_) | Action::RunSearch => {
                self.app_state.run_search();
            }
            Action::OpenEntityDetail => {
                // Set a status message describing the selected entity.
                // Full detail is rendered by ui.rs based on active view + selected index.
                self.app_state
                    .set_status("Press Esc to go back".to_string());
            }
            Action::OpenSearchResult => {
                if let Some(result) = self
                    .app_state
                    .search_results
                    .get(self.app_state.search_result_selected)
                {
                    self.app_state
                        .set_status(format!("[{}] {}", result.entity_type, result.title));
                }
            }
        }
    }

    /// Cycle the status of the currently selected task: Todo -> InProgress -> Done -> Todo.
    fn cycle_selected_task_status(&mut self) {
        let idx = self.app_state.selected_index;
        let persist: Option<(String, TaskStatus)> =
            if let Some(row) = self.app_state.recent_tasks.get_mut(idx) {
                let next_status = match row.status.as_str() {
                    "todo" => "in_progress",
                    "in_progress" => "done",
                    "done" => "todo",
                    _ => "todo",
                };
                row.status = next_status.to_string();

                // Also update the full entity if available
                let row_id = row.id.clone();
                if let Some(task) = self.app_state.all_tasks.iter_mut().find(|t| {
                    t.id == row_id
                        || t.id.starts_with(&row_id)
                        || row_id.starts_with(&t.id[..8.min(t.id.len())])
                }) {
                    task.status = match next_status {
                        "todo" => TaskStatus::Todo,
                        "in_progress" => TaskStatus::InProgress,
                        "done" => TaskStatus::Done,
                        _ => TaskStatus::Todo,
                    };
                }

                // Recompute summary
                let rows: Vec<_> = self.app_state.recent_tasks.clone();
                self.app_state.task_summary = compute_summary(&rows);

                // Capture values needed for backend persistence (released after this block)
                let status_enum = match next_status {
                    "todo" => TaskStatus::Todo,
                    "in_progress" => TaskStatus::InProgress,
                    "done" => TaskStatus::Done,
                    _ => TaskStatus::Todo,
                };
                Some((row_id, status_enum))
            } else {
                None
            };

        // Persist to backend (borrow of app_state released above)
        if let Some((row_id, status_enum)) = persist {
            let _ = self.backend.update_task_status(&row_id, status_enum);
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        // Set up terminal raw mode and alternate screen
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let _guard = TerminalGuard;

        let crossterm_backend = CrosstermBackend::new(io::stdout());
        let mut terminal = Terminal::new(crossterm_backend)?;

        // Load data before the first render
        self.load_all_data();

        loop {
            // Check tick-based auto-refresh before drawing.
            // `should_auto_refresh` resets the timer internally when it fires.
            if self.app_state.should_auto_refresh() {
                self.load_all_data();
                let interval = self.app_state.refresh_interval_secs;
                self.app_state
                    .set_status(format!("Auto-refreshed (every {}s)", interval));
            }

            // Split the borrows explicitly so the borrow checker is satisfied
            // inside the closure: integration and app_state are separate fields.
            let integration = &self.integration;
            let app_state = &mut self.app_state;
            terminal.draw(|f| ui::draw(integration, app_state, f))?;

            let (keep_running, action) = events::handle_input(&mut self.app_state);
            if let Some(action) = action {
                self.dispatch_action(action);
            }
            if !keep_running {
                break;
            }
        }

        Ok(())
    }

    #[cfg(test)]
    fn draw(&mut self, f: &mut ratatui::Frame<'_>) {
        ui::draw(&self.integration, &mut self.app_state, f);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::locus_tui::backend::EngramBackend;
    use crate::storage::memory_only_storage::MemoryStorage;

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

    fn make_app() -> LocusTuiApp<MemoryStorage> {
        let storage = MemoryStorage::new("test-agent");
        let backend: Box<dyn LocusTuiBackend> = Box::new(EngramBackend::from_storage(
            MemoryStorage::new("test-agent"),
        ));
        LocusTuiApp::new_with_backend(storage, backend)
    }

    #[test]
    fn test_new() {
        let storage = MemoryStorage::new("test-agent");
        let _app = LocusTuiApp::new(storage);
    }

    #[test]
    fn test_new_with_integration() {
        let app = make_app();
        let workflows = app.integration.get_workflows().unwrap();
        assert!(workflows.is_empty());
    }

    #[test]
    fn test_draw_with_empty_storage() {
        let mut app = make_app();

        let backend = ratatui::backend::TestBackend::new(80, 24);
        let mut terminal = ratatui::Terminal::new(backend).unwrap();
        terminal.draw(|f| app.draw(f)).unwrap();

        let buf = terminal.backend().buffer();
        let content = buffer_to_string(buf);
        assert!(content.contains("Tasks: 0"));
        assert!(content.contains("Workflows: 0"));
    }

    #[test]
    fn test_draw_title_bar() {
        let mut app = make_app();

        let backend = ratatui::backend::TestBackend::new(80, 24);
        let mut terminal = ratatui::Terminal::new(backend).unwrap();
        terminal.draw(|f| app.draw(f)).unwrap();

        let buf = terminal.backend().buffer();
        let content = buffer_to_string(buf);
        assert!(content.contains("Engram Locus"));
    }

    #[test]
    fn test_draw_help_bar() {
        let mut app = make_app();

        let backend = ratatui::backend::TestBackend::new(80, 24);
        let mut terminal = ratatui::Terminal::new(backend).unwrap();
        terminal.draw(|f| app.draw(f)).unwrap();

        let buf = terminal.backend().buffer();
        let content = buffer_to_string(buf);
        assert!(content.contains("q:quit"));
    }

    #[test]
    fn test_load_all_data_with_empty_storage() {
        let mut app = make_app();
        app.load_all_data();
        assert!(app.app_state.recent_tasks.is_empty());
        assert_eq!(app.app_state.task_summary.total, 0);
        assert!(app.app_state.contexts.is_empty());
        assert!(app.app_state.reasoning_nodes.is_empty());
        assert!(app.app_state.relationship_nodes.is_empty());
    }

    #[test]
    fn test_load_all_data_with_tasks() {
        use crate::entities::{GenericEntity, Task, TaskPriority};
        use crate::storage::Storage;

        let mut backend_storage = MemoryStorage::new("test-agent");
        let task = Task::new(
            "Test task".to_string(),
            "desc".to_string(),
            "test-agent".to_string(),
            TaskPriority::High,
            None,
        );
        let entity = GenericEntity {
            id: task.id.clone(),
            entity_type: "task".to_string(),
            agent: task.agent.clone(),
            timestamp: task.start_time,
            data: serde_json::to_value(&task).unwrap(),
        };
        backend_storage.store(&entity).unwrap();

        let storage = MemoryStorage::new("test-agent");
        let backend: Box<dyn LocusTuiBackend> =
            Box::new(EngramBackend::from_storage(backend_storage));
        let mut app = LocusTuiApp::new_with_backend(storage, backend);
        app.load_all_data();

        assert_eq!(app.app_state.recent_tasks.len(), 1);
        assert_eq!(app.app_state.task_summary.total, 1);
        assert_eq!(app.app_state.task_summary.todo, 1);
        assert_eq!(app.app_state.all_tasks.len(), 1);
    }

    #[test]
    fn test_new_with_refresh_interval_sets_interval() {
        let storage = MemoryStorage::new("test-agent");
        let backend: Box<dyn LocusTuiBackend> = Box::new(EngramBackend::from_storage(
            MemoryStorage::new("test-agent"),
        ));
        let app = LocusTuiApp::new_with_refresh_interval(storage, backend, 60);
        assert_eq!(app.app_state.refresh_interval_secs, 60);
    }

    #[test]
    fn test_new_with_refresh_interval_zero_disables_refresh() {
        let storage = MemoryStorage::new("test-agent");
        let backend: Box<dyn LocusTuiBackend> = Box::new(EngramBackend::from_storage(
            MemoryStorage::new("test-agent"),
        ));
        let app = LocusTuiApp::new_with_refresh_interval(storage, backend, 0);
        assert_eq!(app.app_state.refresh_interval_secs, 0);
    }
}

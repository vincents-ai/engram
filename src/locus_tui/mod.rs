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
use crate::storage::{RelationshipStorage, RemoteAuth, Storage};
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;
use std::sync::mpsc;

#[derive(Debug)]
pub(crate) struct SyncResult {
    pub message: String,
}

pub(crate) enum SyncOperation {
    Pull { remote_name: String },
    Push { remote_name: String },
    Both { remote_name: String },
}

fn execute_sync_op(op: SyncOperation) -> SyncResult {
    let message = match op {
        SyncOperation::Pull { remote_name } => match build_auth_for_remote(&remote_name) {
            Ok(auth) => crate::cli::sync::pull_from_remote(remote_name.clone(), auth, false)
                .map(|outcomes| {
                    let conflicts = outcomes
                        .iter()
                        .filter(|o| {
                            matches!(o, crate::cli::sync::PullEntityOutcome::Conflict { .. })
                        })
                        .count();
                    format!("pull: {} fetched, {} conflicts", outcomes.len(), conflicts)
                })
                .unwrap_or_else(|e| format!("pull error: {}", e)),
            Err(e) => format!("auth error: {}", e),
        },
        SyncOperation::Push { remote_name } => match build_auth_for_remote(&remote_name) {
            Ok(auth) => crate::cli::sync::push_to_remote(remote_name.clone(), auth, false)
                .map(|count| format!("push: {} refs pushed", count))
                .unwrap_or_else(|e| format!("push error: {}", e)),
            Err(e) => format!("auth error: {}", e),
        },
        SyncOperation::Both { remote_name } => match build_auth_for_remote(&remote_name) {
            Ok(auth) => crate::cli::sync::sync_both(remote_name.clone(), auth, false)
                .map(|r| {
                    format!(
                        "sync: {} fetched, {} pushed, {} conflicts",
                        r.pull_outcomes.len(),
                        r.push_count,
                        r.conflicts
                    )
                })
                .unwrap_or_else(|e| format!("sync error: {}", e)),
            Err(e) => format!("auth error: {}", e),
        },
    };
    SyncResult { message }
}

fn build_auth_for_remote(remote_name: &str) -> Result<RemoteAuth, String> {
    use std::collections::HashMap;
    use std::fs;
    let content = fs::read_to_string(".engram/remotes.json")
        .map_err(|e| format!("cannot read remotes.json: {}", e))?;
    let remotes: HashMap<String, crate::cli::sync::RemoteConfig> =
        serde_json::from_str(&content).map_err(|e| format!("cannot parse remotes.json: {}", e))?;
    let cfg = remotes
        .get(remote_name)
        .ok_or_else(|| format!("remote '{}' not found", remote_name))?;
    Ok(RemoteAuth {
        auth_type: cfg.auth_type.clone().unwrap_or_else(|| "none".to_string()),
        username: cfg.username.clone(),
        password: None,
        key_path: cfg.ssh_key_path.clone(),
    })
}

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
    sync_tx: mpsc::Sender<Option<SyncResult>>,
    sync_rx: mpsc::Receiver<Option<SyncResult>>,
}

impl<S: Storage + RelationshipStorage + Send + 'static> LocusTuiApp<S> {
    pub fn new(storage: S) -> Self {
        let backend: Box<dyn LocusTuiBackend> = match GitEngramBackend::new() {
            Ok(b) => Box::new(b),
            Err(e) => {
                eprintln!("locus: warning: could not open git storage: {e}");
                let mem = crate::storage::memory_only_storage::MemoryStorage::new("locus-tui");
                let fallback = crate::locus_tui::backend::EngramBackend::from_storage(mem);
                Box::new(fallback)
            }
        };

        let refresh_interval_secs =
            crate::config::workspace_config::WorkspaceConfig::default().refresh_interval_secs;

        let mut app_state = AppState::new();
        app_state.refresh_interval_secs = refresh_interval_secs;

        let (sync_tx, sync_rx) = mpsc::channel();

        Self {
            integration: LocusIntegration::new(storage),
            backend,
            app_state,
            sync_tx,
            sync_rx,
        }
    }

    pub fn new_with_backend(storage: S, backend: Box<dyn LocusTuiBackend>) -> Self {
        let (sync_tx, sync_rx) = mpsc::channel();

        Self {
            integration: LocusIntegration::new(storage),
            backend,
            app_state: AppState::new(),
            sync_tx,
            sync_rx,
        }
    }

    pub fn new_with_refresh_interval(
        storage: S,
        backend: Box<dyn LocusTuiBackend>,
        refresh_interval_secs: u64,
    ) -> Self {
        let mut app_state = AppState::new();
        app_state.refresh_interval_secs = refresh_interval_secs;

        let (sync_tx, sync_rx) = mpsc::channel();

        Self {
            integration: LocusIntegration::new(storage),
            backend,
            app_state,
            sync_tx,
            sync_rx,
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
        self.app_state.all_personas = self.backend.list_personas().unwrap_or_default();

        let dora_reports = self.backend.list_dora_metrics_reports().unwrap_or_default();
        self.app_state.analytics_view.populate_dora(&dora_reports);
        self.app_state
            .analytics_view
            .populate_quality_gate(&self.app_state.all_tasks);

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
                if self.app_state.persona_detail.is_some() {
                    self.app_state.close_persona_detail();
                } else {
                    self.app_state.close_task_detail();
                }
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
                if self.app_state.active_view == crate::locus_tui::app::ActiveView::Personas {
                    self.app_state.open_persona_detail();
                } else {
                    self.app_state
                        .set_status("Press Esc to go back".to_string());
                }
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
            Action::SyncPull => {
                if self.app_state.sync_view.op_in_flight {
                    return;
                }
                let remote = self.selected_remote_name();
                match remote {
                    Some(name) => {
                        self.app_state.sync_view.op_in_flight = true;
                        self.app_state
                            .set_status("Pulling from remote…".to_string());
                        let tx = self.sync_tx.clone();
                        std::thread::spawn(move || {
                            let result = execute_sync_op(SyncOperation::Pull { remote_name: name });
                            let _ = tx.send(Some(result));
                        });
                    }
                    None => {
                        self.app_state.sync_view.last_op_result =
                            Some("No remote selected".to_string());
                        self.app_state.set_status("No remote selected".to_string());
                    }
                }
            }
            Action::SyncPush => {
                if self.app_state.sync_view.op_in_flight {
                    return;
                }
                let remote = self.selected_remote_name();
                match remote {
                    Some(name) => {
                        self.app_state.sync_view.op_in_flight = true;
                        self.app_state.set_status("Pushing to remote…".to_string());
                        let tx = self.sync_tx.clone();
                        std::thread::spawn(move || {
                            let result = execute_sync_op(SyncOperation::Push { remote_name: name });
                            let _ = tx.send(Some(result));
                        });
                    }
                    None => {
                        self.app_state.sync_view.last_op_result =
                            Some("No remote selected".to_string());
                        self.app_state.set_status("No remote selected".to_string());
                    }
                }
            }
            Action::SyncBoth => {
                if self.app_state.sync_view.op_in_flight {
                    return;
                }
                let remote = self.selected_remote_name();
                match remote {
                    Some(name) => {
                        self.app_state.sync_view.op_in_flight = true;
                        self.app_state
                            .set_status("Syncing (pull+push)…".to_string());
                        let tx = self.sync_tx.clone();
                        std::thread::spawn(move || {
                            let result = execute_sync_op(SyncOperation::Both { remote_name: name });
                            let _ = tx.send(Some(result));
                        });
                    }
                    None => {
                        self.app_state.sync_view.last_op_result =
                            Some("No remote selected".to_string());
                        self.app_state.set_status("No remote selected".to_string());
                    }
                }
            }
            Action::RefreshSyncStatus => {
                self.app_state
                    .set_status("Refreshing sync status…".to_string());
                self.load_sync_data();
                self.app_state.clear_status();
            }
            Action::EscalationApprove => {
                self.approve_deny_escalation(true);
            }
            Action::EscalationDeny => {
                self.approve_deny_escalation(false);
            }
        }
    }

    fn poll_sync_results(&mut self) {
        while let Ok(Some(result)) = self.sync_rx.try_recv() {
            self.app_state.sync_view.op_in_flight = false;
            self.app_state.sync_view.last_op_result = Some(result.message.clone());
            self.app_state.set_status(result.message);
            self.refresh_sync_status();
        }
    }

    /// Load sync remotes and status into app state.
    fn load_sync_data(&mut self) {
        let remotes = self.backend.list_remote_names();
        // Reset selected index if the list shrank.
        if self.app_state.sync_view.remotes_selected >= remotes.len() {
            self.app_state.sync_view.remotes_selected = 0;
        }
        // Fetch status for the selected remote (if any).
        let selected_name = remotes
            .get(self.app_state.sync_view.remotes_selected)
            .cloned();
        if let Some(ref name) = selected_name {
            self.app_state.sync_view.status_rows =
                self.backend.get_sync_status_data(name).unwrap_or_default();
        } else {
            self.app_state.sync_view.status_rows.clear();
        }
        self.app_state.sync_view.remotes = remotes;
    }

    /// Refresh the sync status pane (does not re-fetch remote names).
    fn refresh_sync_status(&mut self) {
        let selected_name = self
            .app_state
            .sync_view
            .remotes
            .get(self.app_state.sync_view.remotes_selected)
            .cloned();
        if let Some(ref name) = selected_name {
            self.app_state.sync_view.status_rows =
                self.backend.get_sync_status_data(name).unwrap_or_default();
        }
    }

    /// Return the name of the currently selected remote, if any.
    fn selected_remote_name(&self) -> Option<String> {
        self.app_state
            .sync_view
            .remotes
            .get(self.app_state.sync_view.remotes_selected)
            .cloned()
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

    fn approve_deny_escalation(&mut self, approve: bool) {
        let idx = self.app_state.escalations_selected;
        let esc = match self.app_state.all_escalations.get(idx) {
            Some(e) => e.clone(),
            None => return,
        };
        if esc.status != crate::entities::EscalationStatus::Pending {
            self.app_state.set_status(format!(
                "Escalation {} is not pending ({:?})",
                &esc.id[..8.min(esc.id.len())],
                esc.status
            ));
            return;
        }
        let id = esc.id.clone();
        let new_status = if approve {
            crate::entities::EscalationStatus::Approved
        } else {
            crate::entities::EscalationStatus::Denied
        };
        let reason = if approve {
            "Approved via Locus TUI".to_string()
        } else {
            "Denied via Locus TUI".to_string()
        };
        match self.backend.update_escalation_status(
            &id,
            new_status,
            "locus-tui",
            "Locus TUI Reviewer",
            &reason,
        ) {
            Ok(()) => {
                let action_word = if approve { "Approved" } else { "Denied" };
                self.app_state.set_status(format!(
                    "{} escalation {}",
                    action_word,
                    &id[..8.min(id.len())]
                ));
                self.load_all_data();
            }
            Err(e) => {
                self.app_state.set_status(format!("Error: {}", e));
            }
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let _guard = TerminalGuard;

        let crossterm_backend = CrosstermBackend::new(io::stdout());
        let mut terminal = Terminal::new(crossterm_backend)?;

        self.load_all_data();

        loop {
            if self.app_state.should_auto_refresh() {
                self.load_all_data();
                let interval = self.app_state.refresh_interval_secs;
                self.app_state
                    .set_status(format!("Auto-refreshed (every {}s)", interval));
            }

            self.poll_sync_results();

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

    /// Test-only: dispatch an action directly without going through handle_input.
    #[cfg(test)]
    pub(crate) fn test_dispatch(&mut self, action: Action) {
        self.dispatch_action(action);
    }

    /// Test-only: load sync data from the backend into app_state.
    #[cfg(test)]
    pub(crate) fn test_load_sync_data(&mut self) {
        self.load_sync_data();
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

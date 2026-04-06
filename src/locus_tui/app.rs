/// Summary counts for tasks across all statuses.
#[derive(Debug, Clone, Default)]
pub struct TaskSummary {
    pub total: usize,
    pub todo: usize,
    pub in_progress: usize,
    pub done: usize,
}

/// A single row for the recent-tasks table.
#[derive(Debug, Clone)]
pub struct TaskRow {
    pub id: String,
    pub title: String,
    pub status: String,
    pub priority: String,
}

/// The active view currently displayed in the TUI.
#[derive(Debug, Clone, PartialEq)]
pub enum ActiveView {
    Dashboard,
    Tasks,
    Reasoning,
    Relationships,
    Contexts,
    Search,
}

impl ActiveView {
    /// All variants in cycle order.
    fn all() -> &'static [ActiveView] {
        use ActiveView::*;
        &[Dashboard, Tasks, Reasoning, Relationships, Contexts, Search]
    }

    fn index(&self) -> usize {
        Self::all().iter().position(|v| v == self).unwrap_or(0)
    }
}

/// Top-level application state for the Locus TUI.
pub struct AppState {
    pub active_view: ActiveView,
    pub should_quit: bool,
    pub status_message: Option<String>,
    /// Selected row index for list views.
    pub selected_index: usize,
    /// Search query string (used in the Search view).
    pub search_query: String,
    /// Active colour theme.
    pub theme: crate::locus_tui::theme::AppTheme,
    /// Summary counts shown on the Dashboard.
    pub task_summary: TaskSummary,
    /// Recent tasks shown in the Dashboard table (up to 10).
    pub recent_tasks: Vec<TaskRow>,
}

impl AppState {
    pub fn new() -> Self {
        let recent_tasks = vec![
            TaskRow {
                id: "a1b2c3d4".to_string(),
                title: "Implement CLI argument parser".to_string(),
                status: "done".to_string(),
                priority: "high".to_string(),
            },
            TaskRow {
                id: "e5f6a7b8".to_string(),
                title: "Add storage backend abstraction".to_string(),
                status: "done".to_string(),
                priority: "high".to_string(),
            },
            TaskRow {
                id: "c9d0e1f2".to_string(),
                title: "Phase 5: Dashboard view".to_string(),
                status: "in_progress".to_string(),
                priority: "high".to_string(),
            },
            TaskRow {
                id: "3a4b5c6d".to_string(),
                title: "Write integration tests".to_string(),
                status: "todo".to_string(),
                priority: "medium".to_string(),
            },
            TaskRow {
                id: "7e8f9a0b".to_string(),
                title: "Publish crate to crates.io".to_string(),
                status: "todo".to_string(),
                priority: "low".to_string(),
            },
        ];

        let task_summary = TaskSummary {
            total: recent_tasks.len(),
            todo: recent_tasks.iter().filter(|t| t.status == "todo").count(),
            in_progress: recent_tasks
                .iter()
                .filter(|t| t.status == "in_progress")
                .count(),
            done: recent_tasks.iter().filter(|t| t.status == "done").count(),
        };

        Self {
            active_view: ActiveView::Dashboard,
            should_quit: false,
            status_message: None,
            selected_index: 0,
            search_query: String::new(),
            theme: crate::locus_tui::theme::AppTheme::dark(),
            task_summary,
            recent_tasks,
        }
    }

    /// Cycle forward through all ActiveView variants.
    pub fn next_view(&mut self) {
        let variants = ActiveView::all();
        let next = (self.active_view.index() + 1) % variants.len();
        self.active_view = variants[next].clone();
    }

    /// Cycle backward through all ActiveView variants.
    pub fn prev_view(&mut self) {
        let variants = ActiveView::all();
        let current = self.active_view.index();
        let prev = if current == 0 {
            variants.len() - 1
        } else {
            current - 1
        };
        self.active_view = variants[prev].clone();
    }

    /// Increment the selected row index.
    pub fn select_next(&mut self) {
        self.selected_index = self.selected_index.saturating_add(1);
    }

    /// Decrement the selected row index, saturating at 0.
    pub fn select_prev(&mut self) {
        self.selected_index = self.selected_index.saturating_sub(1);
    }

    /// Move selection to the last row in a list of known length.
    pub fn select_bottom_of(&mut self, len: usize) {
        if len > 0 {
            self.selected_index = len - 1;
        }
    }

    /// Signal that the application should quit.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    /// Set a status bar message.
    pub fn set_status(&mut self, msg: impl Into<String>) {
        self.status_message = Some(msg.into());
    }

    /// Clear the status bar message.
    pub fn clear_status(&mut self) {
        self.status_message = None;
    }

    /// Toggle between dark and light themes.
    pub fn toggle_theme(&mut self) {
        self.theme = self.theme.toggle();
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::locus_tui::theme::AppTheme;

    #[test]
    fn test_next_view_cycles_through_all_variants() {
        let mut state = AppState::new();
        assert_eq!(state.active_view, ActiveView::Dashboard);
        state.next_view();
        assert_eq!(state.active_view, ActiveView::Tasks);
        state.next_view();
        assert_eq!(state.active_view, ActiveView::Reasoning);
        state.next_view();
        assert_eq!(state.active_view, ActiveView::Relationships);
        state.next_view();
        assert_eq!(state.active_view, ActiveView::Contexts);
        state.next_view();
        assert_eq!(state.active_view, ActiveView::Search);
        // Should wrap back to Dashboard
        state.next_view();
        assert_eq!(state.active_view, ActiveView::Dashboard);
    }

    #[test]
    fn test_prev_view_cycles_backward() {
        let mut state = AppState::new();
        assert_eq!(state.active_view, ActiveView::Dashboard);
        // Going backward from Dashboard should wrap to Search
        state.prev_view();
        assert_eq!(state.active_view, ActiveView::Search);
        state.prev_view();
        assert_eq!(state.active_view, ActiveView::Contexts);
        state.prev_view();
        assert_eq!(state.active_view, ActiveView::Relationships);
        state.prev_view();
        assert_eq!(state.active_view, ActiveView::Reasoning);
        state.prev_view();
        assert_eq!(state.active_view, ActiveView::Tasks);
        state.prev_view();
        assert_eq!(state.active_view, ActiveView::Dashboard);
    }

    #[test]
    fn test_select_next_increments() {
        let mut state = AppState::new();
        assert_eq!(state.selected_index, 0);
        state.select_next();
        assert_eq!(state.selected_index, 1);
        state.select_next();
        assert_eq!(state.selected_index, 2);
    }

    #[test]
    fn test_select_prev_saturates_at_zero() {
        let mut state = AppState::new();
        assert_eq!(state.selected_index, 0);
        state.select_prev();
        assert_eq!(state.selected_index, 0);
        state.select_next();
        state.select_next();
        assert_eq!(state.selected_index, 2);
        state.select_prev();
        assert_eq!(state.selected_index, 1);
        state.select_prev();
        assert_eq!(state.selected_index, 0);
        state.select_prev();
        assert_eq!(state.selected_index, 0);
    }

    #[test]
    fn test_quit_sets_should_quit() {
        let mut state = AppState::new();
        assert!(!state.should_quit);
        state.quit();
        assert!(state.should_quit);
    }

    #[test]
    fn test_set_and_clear_status() {
        let mut state = AppState::new();
        assert!(state.status_message.is_none());
        state.set_status("hello");
        assert_eq!(state.status_message, Some("hello".to_string()));
        state.clear_status();
        assert!(state.status_message.is_none());
    }

    #[test]
    fn test_default_theme_is_dark() {
        let state = AppState::new();
        assert!(matches!(state.theme, AppTheme::Dark(_)));
    }

    #[test]
    fn test_toggle_theme_switches_dark_to_light() {
        let mut state = AppState::new();
        assert!(matches!(state.theme, AppTheme::Dark(_)));
        state.toggle_theme();
        assert!(matches!(state.theme, AppTheme::Light(_)));
    }

    #[test]
    fn test_toggle_theme_switches_light_to_dark() {
        let mut state = AppState::new();
        state.toggle_theme(); // dark -> light
        state.toggle_theme(); // light -> dark
        assert!(matches!(state.theme, AppTheme::Dark(_)));
    }

    #[test]
    fn test_task_summary_defaults_populated() {
        let state = AppState::new();
        // Stub data: 5 tasks total, 2 todo, 1 in_progress, 2 done
        assert_eq!(state.task_summary.total, 5);
        assert_eq!(state.task_summary.todo, 2);
        assert_eq!(state.task_summary.in_progress, 1);
        assert_eq!(state.task_summary.done, 2);
    }

    #[test]
    fn test_recent_tasks_populated() {
        let state = AppState::new();
        assert_eq!(state.recent_tasks.len(), 5);
    }

    #[test]
    fn test_recent_tasks_have_required_fields() {
        let state = AppState::new();
        for task in &state.recent_tasks {
            assert!(!task.id.is_empty());
            assert!(!task.title.is_empty());
            assert!(!task.status.is_empty());
            assert!(!task.priority.is_empty());
        }
    }

    #[test]
    fn test_select_bottom_of_sets_last_index() {
        let mut state = AppState::new();
        state.select_bottom_of(5);
        assert_eq!(state.selected_index, 4);
    }

    #[test]
    fn test_select_bottom_of_zero_length_noop() {
        let mut state = AppState::new();
        state.selected_index = 3;
        state.select_bottom_of(0);
        assert_eq!(state.selected_index, 3);
    }

    #[test]
    fn test_select_bottom_of_single_element() {
        let mut state = AppState::new();
        state.select_bottom_of(1);
        assert_eq!(state.selected_index, 0);
    }

    #[test]
    fn test_task_summary_default_is_zero() {
        let s = TaskSummary::default();
        assert_eq!(s.total, 0);
        assert_eq!(s.todo, 0);
        assert_eq!(s.in_progress, 0);
        assert_eq!(s.done, 0);
    }

    #[test]
    fn test_task_row_fields() {
        let row = TaskRow {
            id: "abc12345".to_string(),
            title: "Test task".to_string(),
            status: "todo".to_string(),
            priority: "high".to_string(),
        };
        assert_eq!(row.id, "abc12345");
        assert_eq!(row.title, "Test task");
        assert_eq!(row.status, "todo");
        assert_eq!(row.priority, "high");
    }
}

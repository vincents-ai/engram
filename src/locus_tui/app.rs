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
}

impl AppState {
    pub fn new() -> Self {
        Self {
            active_view: ActiveView::Dashboard,
            should_quit: false,
            status_message: None,
            selected_index: 0,
            search_query: String::new(),
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
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}

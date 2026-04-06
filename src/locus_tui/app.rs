/// A directed edge in the relationships graph.
#[derive(Debug, Clone)]
pub struct RelationshipEdge {
    pub from_id: String,
    pub to_id: String,
    pub relationship_type: String, // e.g. "depends_on", "relates_to", "explains"
    pub to_title: String,
}

/// A node in the relationships graph (adjacency-list representation).
#[derive(Debug, Clone)]
pub struct RelationshipNode {
    pub id: String,
    pub title: String,
    pub entity_type: String, // "task", "context", "reasoning", "adr"
    pub edges: Vec<RelationshipEdge>,
}

/// A single node in the reasoning tree.
#[derive(Debug, Clone)]
pub struct ReasoningNode {
    pub id: String,
    pub title: String,
    pub content_preview: String, // first 80 chars of content
    pub task_id: Option<String>,
    pub depth: usize, // for indentation (0 = root)
    pub expanded: bool,
}

impl ReasoningNode {
    /// Return the indentation prefix string for this node's depth.
    pub fn indent_prefix(&self) -> String {
        " ".repeat(self.depth * 2)
    }

    /// Return the expand/collapse indicator glyph.
    pub fn expand_glyph(&self) -> &'static str {
        if self.expanded {
            "▼"
        } else {
            "▶"
        }
    }
}

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
    pub created: String,
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
    /// Status filter: None = show all, Some("todo") etc.
    pub filter_status: Option<String>,
    /// Substring filter on task title (case-insensitive).
    pub filter_text: String,
    /// Reasoning tree nodes.
    pub reasoning_nodes: Vec<ReasoningNode>,
    /// Selected index within the reasoning view list.
    pub reasoning_selected: usize,
    /// Nodes in the relationships graph (adjacency list).
    pub relationship_nodes: Vec<RelationshipNode>,
    /// Selected index within the relationships view list.
    pub relationship_selected: usize,
}

impl AppState {
    pub fn new() -> Self {
        let recent_tasks = vec![
            TaskRow {
                id: "a1b2c3d4".to_string(),
                title: "Implement CLI argument parser".to_string(),
                status: "done".to_string(),
                priority: "high".to_string(),
                created: "2026-01-01".to_string(),
            },
            TaskRow {
                id: "e5f6a7b8".to_string(),
                title: "Add storage backend abstraction".to_string(),
                status: "done".to_string(),
                priority: "high".to_string(),
                created: "2026-01-15".to_string(),
            },
            TaskRow {
                id: "c9d0e1f2".to_string(),
                title: "Phase 5: Dashboard view".to_string(),
                status: "in_progress".to_string(),
                priority: "high".to_string(),
                created: "2026-02-01".to_string(),
            },
            TaskRow {
                id: "3a4b5c6d".to_string(),
                title: "Write integration tests".to_string(),
                status: "todo".to_string(),
                priority: "medium".to_string(),
                created: "2026-03-01".to_string(),
            },
            TaskRow {
                id: "7e8f9a0b".to_string(),
                title: "Publish crate to crates.io".to_string(),
                status: "todo".to_string(),
                priority: "low".to_string(),
                created: "2026-03-15".to_string(),
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
            filter_status: None,
            filter_text: String::new(),
            reasoning_nodes: vec![
                ReasoningNode {
                    id: "rsn-0001".to_string(),
                    title: "Goal: Locus TUI rewrite".to_string(),
                    content_preview: "High-level goal: rebuild the Locus TUI using ratatui with a modular, testable architecture."
                        [..80.min(
                            "High-level goal: rebuild the Locus TUI using ratatui with a modular, testable architecture.".len()
                        )]
                        .to_string(),
                    task_id: None,
                    depth: 0,
                    expanded: true,
                },
                ReasoningNode {
                    id: "rsn-0002".to_string(),
                    title: "Phase 0: ratatui upgrade".to_string(),
                    content_preview: "Upgrade ratatui dependency and resolve breaking API changes from 0.26 to 0.29.".to_string(),
                    task_id: None,
                    depth: 1,
                    expanded: false,
                },
                ReasoningNode {
                    id: "rsn-0003".to_string(),
                    title: "Phase 1: modularise".to_string(),
                    content_preview: "Split monolithic tui.rs into app, events, theme, ui, backend sub-modules.".to_string(),
                    task_id: None,
                    depth: 1,
                    expanded: false,
                },
                ReasoningNode {
                    id: "rsn-0004".to_string(),
                    title: "Detail: module split decisions".to_string(),
                    content_preview: "Chose separate files per concern: app state owns data, ui owns rendering, events owns input.".to_string(),
                    task_id: None,
                    depth: 2,
                    expanded: false,
                },
            ],
            reasoning_selected: 0,
            relationship_nodes: vec![
                RelationshipNode {
                    id: "rel-0001".to_string(),
                    title: "Goal: Locus TUI rewrite".to_string(),
                    entity_type: "task".to_string(),
                    edges: vec![
                        RelationshipEdge {
                            from_id: "rel-0001".to_string(),
                            to_id: "rel-0002".to_string(),
                            relationship_type: "depends_on".to_string(),
                            to_title: "Phase 0: ratatui upgrade".to_string(),
                        },
                        RelationshipEdge {
                            from_id: "rel-0001".to_string(),
                            to_id: "rel-0003".to_string(),
                            relationship_type: "depends_on".to_string(),
                            to_title: "Phase 1: modularise".to_string(),
                        },
                    ],
                },
                RelationshipNode {
                    id: "rel-0002".to_string(),
                    title: "Phase 0: ratatui upgrade".to_string(),
                    entity_type: "task".to_string(),
                    edges: vec![],
                },
                RelationshipNode {
                    id: "rel-0003".to_string(),
                    title: "master brief".to_string(),
                    entity_type: "context".to_string(),
                    edges: vec![RelationshipEdge {
                        from_id: "rel-0003".to_string(),
                        to_id: "rel-0001".to_string(),
                        relationship_type: "relates_to".to_string(),
                        to_title: "Goal: Locus TUI rewrite".to_string(),
                    }],
                },
                RelationshipNode {
                    id: "rel-0004".to_string(),
                    title: "Phase 0 complete".to_string(),
                    entity_type: "reasoning".to_string(),
                    edges: vec![RelationshipEdge {
                        from_id: "rel-0004".to_string(),
                        to_id: "rel-0002".to_string(),
                        relationship_type: "explains".to_string(),
                        to_title: "Phase 0: ratatui upgrade".to_string(),
                    }],
                },
            ],
            relationship_selected: 0,
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

    /// Set the status filter (None = show all).
    pub fn set_filter_status(&mut self, status: Option<String>) {
        self.filter_status = status;
    }

    /// Set the text substring filter.
    pub fn set_filter_text(&mut self, text: String) {
        self.filter_text = text;
    }

    /// Return tasks that match both the active status filter and text filter.
    pub fn filtered_tasks(&self) -> Vec<&TaskRow> {
        self.recent_tasks
            .iter()
            .filter(|t| {
                let status_ok = self
                    .filter_status
                    .as_ref()
                    .map(|s| t.status.to_lowercase() == s.to_lowercase())
                    .unwrap_or(true);
                let text_ok = self.filter_text.is_empty()
                    || t.title
                        .to_lowercase()
                        .contains(&self.filter_text.to_lowercase());
                status_ok && text_ok
            })
            .collect()
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

    /// Toggle the expanded state of the currently selected reasoning node.
    pub fn toggle_reasoning_node(&mut self) {
        if let Some(node) = self.reasoning_nodes.get_mut(self.reasoning_selected) {
            node.expanded = !node.expanded;
        }
    }

    /// Return all reasoning nodes as a flat list (depth drives visual indent).
    pub fn visible_reasoning_nodes(&self) -> Vec<&ReasoningNode> {
        self.reasoning_nodes.iter().collect()
    }

    /// Return the currently selected relationship node, if any.
    pub fn selected_relationship_node(&self) -> Option<&RelationshipNode> {
        self.relationship_nodes.get(self.relationship_selected)
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
            created: "2026-01-01".to_string(),
        };
        assert_eq!(row.id, "abc12345");
        assert_eq!(row.title, "Test task");
        assert_eq!(row.status, "todo");
        assert_eq!(row.priority, "high");
        assert_eq!(row.created, "2026-01-01");
    }

    #[test]
    fn test_filtered_tasks_returns_all_when_no_filter() {
        let state = AppState::new();
        let filtered = state.filtered_tasks();
        assert_eq!(filtered.len(), state.recent_tasks.len());
    }

    #[test]
    fn test_filtered_tasks_filters_by_status() {
        let mut state = AppState::new();
        state.set_filter_status(Some("todo".to_string()));
        let filtered = state.filtered_tasks();
        assert!(filtered.iter().all(|t| t.status.to_lowercase() == "todo"));
        let todo_count = state
            .recent_tasks
            .iter()
            .filter(|t| t.status == "todo")
            .count();
        assert_eq!(filtered.len(), todo_count);
    }

    #[test]
    fn test_filtered_tasks_filters_by_text_case_insensitive() {
        let mut state = AppState::new();
        state.set_filter_text("CLI".to_string());
        let filtered = state.filtered_tasks();
        assert!(!filtered.is_empty());
        assert!(filtered
            .iter()
            .all(|t| t.title.to_lowercase().contains("cli")));
    }

    #[test]
    fn test_filtered_tasks_combines_both_filters() {
        let mut state = AppState::new();
        state.set_filter_status(Some("done".to_string()));
        state.set_filter_text("storage".to_string());
        let filtered = state.filtered_tasks();
        assert!(!filtered.is_empty());
        for t in &filtered {
            assert_eq!(t.status.to_lowercase(), "done");
            assert!(t.title.to_lowercase().contains("storage"));
        }
    }

    // ── Reasoning node tests ────────────────────────────────────────────────

    #[test]
    fn test_visible_reasoning_nodes_returns_all() {
        let state = AppState::new();
        let visible = state.visible_reasoning_nodes();
        assert_eq!(visible.len(), state.reasoning_nodes.len());
        assert_eq!(visible.len(), 4);
    }

    #[test]
    fn test_toggle_reasoning_node_flips_expanded() {
        let mut state = AppState::new();
        // Node 0 is expanded=true initially
        assert!(state.reasoning_nodes[0].expanded);
        state.reasoning_selected = 0;
        state.toggle_reasoning_node();
        assert!(!state.reasoning_nodes[0].expanded);
        state.toggle_reasoning_node();
        assert!(state.reasoning_nodes[0].expanded);
    }

    #[test]
    fn test_toggle_reasoning_node_on_collapsed_node() {
        let mut state = AppState::new();
        // Node 1 starts collapsed
        assert!(!state.reasoning_nodes[1].expanded);
        state.reasoning_selected = 1;
        state.toggle_reasoning_node();
        assert!(state.reasoning_nodes[1].expanded);
    }

    #[test]
    fn test_toggle_reasoning_node_out_of_bounds_noop() {
        let mut state = AppState::new();
        state.reasoning_selected = 999;
        // Should not panic
        state.toggle_reasoning_node();
    }

    #[test]
    fn test_reasoning_node_indent_prefix_depth_zero() {
        let node = ReasoningNode {
            id: "x".to_string(),
            title: "t".to_string(),
            content_preview: "c".to_string(),
            task_id: None,
            depth: 0,
            expanded: false,
        };
        assert_eq!(node.indent_prefix(), "");
    }

    #[test]
    fn test_reasoning_node_indent_prefix_depth_one() {
        let node = ReasoningNode {
            id: "x".to_string(),
            title: "t".to_string(),
            content_preview: "c".to_string(),
            task_id: None,
            depth: 1,
            expanded: false,
        };
        assert_eq!(node.indent_prefix(), "  ");
    }

    #[test]
    fn test_reasoning_node_indent_prefix_depth_two() {
        let node = ReasoningNode {
            id: "x".to_string(),
            title: "t".to_string(),
            content_preview: "c".to_string(),
            task_id: None,
            depth: 2,
            expanded: true,
        };
        assert_eq!(node.indent_prefix(), "    ");
    }

    #[test]
    fn test_reasoning_node_expand_glyph_collapsed() {
        let node = ReasoningNode {
            id: "x".to_string(),
            title: "t".to_string(),
            content_preview: "c".to_string(),
            task_id: None,
            depth: 0,
            expanded: false,
        };
        assert_eq!(node.expand_glyph(), "▶");
    }

    #[test]
    fn test_reasoning_node_expand_glyph_expanded() {
        let node = ReasoningNode {
            id: "x".to_string(),
            title: "t".to_string(),
            content_preview: "c".to_string(),
            task_id: None,
            depth: 0,
            expanded: true,
        };
        assert_eq!(node.expand_glyph(), "▼");
    }

    #[test]
    fn test_stub_reasoning_nodes_depths() {
        let state = AppState::new();
        let nodes = &state.reasoning_nodes;
        assert_eq!(nodes[0].depth, 0);
        assert_eq!(nodes[1].depth, 1);
        assert_eq!(nodes[2].depth, 1);
        assert_eq!(nodes[3].depth, 2);
    }

    #[test]
    fn test_stub_reasoning_node_titles() {
        let state = AppState::new();
        assert!(state.reasoning_nodes[0].title.contains("Goal"));
        assert!(state.reasoning_nodes[1].title.contains("Phase 0"));
        assert!(state.reasoning_nodes[2].title.contains("Phase 1"));
        assert!(state.reasoning_nodes[3].title.contains("Detail"));
    }

    // ── Relationship node tests ──────────────────────────────────────────────

    #[test]
    fn test_selected_relationship_node_returns_correct_node() {
        let mut state = AppState::new();
        state.relationship_selected = 0;
        let node = state
            .selected_relationship_node()
            .expect("should have node");
        assert_eq!(node.title, "Goal: Locus TUI rewrite");

        state.relationship_selected = 2;
        let node = state
            .selected_relationship_node()
            .expect("should have node");
        assert_eq!(node.title, "master brief");
    }

    #[test]
    fn test_selected_relationship_node_returns_none_when_empty() {
        let mut state = AppState::new();
        state.relationship_nodes.clear();
        state.relationship_selected = 0;
        assert!(state.selected_relationship_node().is_none());
    }

    #[test]
    fn test_selected_relationship_node_returns_none_out_of_bounds() {
        let mut state = AppState::new();
        state.relationship_selected = 999;
        assert!(state.selected_relationship_node().is_none());
    }

    #[test]
    fn test_stub_relationship_node_goal_has_two_depends_on_edges() {
        let state = AppState::new();
        let goal = state
            .relationship_nodes
            .iter()
            .find(|n| n.title == "Goal: Locus TUI rewrite")
            .expect("goal node should exist");
        assert_eq!(goal.edges.len(), 2);
        assert!(goal
            .edges
            .iter()
            .all(|e| e.relationship_type == "depends_on"));
        let targets: Vec<&str> = goal.edges.iter().map(|e| e.to_title.as_str()).collect();
        assert!(targets.contains(&"Phase 0: ratatui upgrade"));
        assert!(targets.contains(&"Phase 1: modularise"));
    }

    #[test]
    fn test_stub_relationship_node_master_brief_relates_to_goal() {
        let state = AppState::new();
        let brief = state
            .relationship_nodes
            .iter()
            .find(|n| n.title == "master brief")
            .expect("master brief node should exist");
        assert_eq!(brief.entity_type, "context");
        assert_eq!(brief.edges.len(), 1);
        assert_eq!(brief.edges[0].relationship_type, "relates_to");
        assert_eq!(brief.edges[0].to_title, "Goal: Locus TUI rewrite");
    }

    #[test]
    fn test_stub_relationship_node_phase0_complete_explains() {
        let state = AppState::new();
        let reasoning = state
            .relationship_nodes
            .iter()
            .find(|n| n.title == "Phase 0 complete")
            .expect("Phase 0 complete node should exist");
        assert_eq!(reasoning.entity_type, "reasoning");
        assert_eq!(reasoning.edges.len(), 1);
        assert_eq!(reasoning.edges[0].relationship_type, "explains");
        assert_eq!(reasoning.edges[0].to_title, "Phase 0: ratatui upgrade");
    }

    #[test]
    fn test_stub_relationship_nodes_count() {
        let state = AppState::new();
        assert_eq!(state.relationship_nodes.len(), 4);
    }
}

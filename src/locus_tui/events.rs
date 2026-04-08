use crate::locus_tui::app::{ActiveView, AppState, RelationshipFocus};
use crossterm::event::{self, Event, KeyCode, KeyEvent, MouseButton, MouseEventKind};
use std::time::Duration;

/// High-level action derived from a raw key or mouse event.
#[derive(Debug, Clone, PartialEq)]
pub enum KeyAction {
    Quit,
    NextView,
    PrevView,
    SelectNext,
    SelectPrev,
    SelectTop,
    SelectBottom,
    ToggleTheme,
    Confirm,
    Back,
    Search,
    Refresh,
    FilterStatus,
    CycleTaskStatus,
    ToggleHelp,
    FocusNext, // Tab inside a view (e.g. node->edge pane)
    Char(char),
    Unknown,
}

/// Application-level action dispatched from handle_input to the app.
#[derive(Debug, Clone)]
pub enum Action {
    Refresh,
    OpenTaskDetail,
    CloseDetail,
    CycleTaskStatus,
    CycleAdrStatus,
    EnterSearchMode,
    ExitSearchMode,
    SearchQueryChar(char),
    RunSearch,
    OpenEntityDetail,
    OpenSearchResult,
    // Sync view actions
    SyncPull,
    SyncPush,
    SyncBoth,
    RefreshSyncStatus,
    // Escalation view actions
    EscalationApprove,
    EscalationDeny,
}

/// Map a raw crossterm `KeyEvent` to a `KeyAction`.
pub fn map_key(key: KeyEvent) -> KeyAction {
    match key.code {
        KeyCode::Char('q') | KeyCode::Char('Q') => KeyAction::Quit,
        KeyCode::Tab => KeyAction::FocusNext,
        KeyCode::BackTab => KeyAction::PrevView,
        KeyCode::Down | KeyCode::Char('j') => KeyAction::SelectNext,
        KeyCode::Up | KeyCode::Char('k') => KeyAction::SelectPrev,
        KeyCode::Char('g') => KeyAction::SelectTop,
        KeyCode::Char('G') => KeyAction::SelectBottom,
        KeyCode::Char('t') => KeyAction::ToggleTheme,
        KeyCode::Enter => KeyAction::Confirm,
        KeyCode::Esc => KeyAction::Back,
        KeyCode::Char('/') => KeyAction::Search,
        KeyCode::Char('r') => KeyAction::Refresh,
        KeyCode::Char('f') => KeyAction::FilterStatus,
        KeyCode::Char('s') => KeyAction::CycleTaskStatus,
        KeyCode::Char('?') => KeyAction::ToggleHelp,
        KeyCode::Char(c) => KeyAction::Char(c),
        _ => KeyAction::Unknown,
    }
}

/// Poll for input events, update app state, and return (keep_running, Option<Action>).
/// `true` = keep running, `false` = quit.
pub fn handle_input(app: &mut AppState) -> (bool, Option<Action>) {
    if event::poll(Duration::from_millis(50)).unwrap_or(false) {
        match event::read() {
            Ok(Event::Key(key)) => {
                // If in search mode, handle characters specially
                if app.search_mode {
                    return handle_search_input(app, key);
                }
                return handle_key(app, key);
            }
            Ok(Event::Mouse(mouse)) => {
                return handle_mouse(app, mouse);
            }
            _ => {}
        }
    }
    (!app.should_quit, None)
}

fn handle_key(app: &mut AppState, key: KeyEvent) -> (bool, Option<Action>) {
    match map_key(key) {
        KeyAction::Quit => {
            app.quit();
            return (false, None);
        }
        KeyAction::ToggleHelp => {
            app.show_help = !app.show_help;
        }
        KeyAction::FocusNext => {
            if app.show_help {
                app.show_help = false;
            } else if app.active_view == ActiveView::Relationships {
                // Tab cycles focus: Nodes → Edges → next view
                match app.relationship_focus {
                    RelationshipFocus::Nodes => {
                        app.relationship_focus = RelationshipFocus::Edges;
                        app.relationship_edge_selected = 0;
                    }
                    RelationshipFocus::Edges => {
                        app.relationship_focus = RelationshipFocus::Nodes;
                        app.next_view();
                    }
                }
            } else {
                app.next_view();
            }
        }
        KeyAction::PrevView => app.prev_view(),
        KeyAction::SelectNext => {
            if app.active_view == ActiveView::Relationships
                && app.relationship_focus == RelationshipFocus::Edges
            {
                // Navigate edges within the selected node
                if let Some(node) = app.relationship_nodes.get(app.relationship_selected) {
                    let len = node.edges.len();
                    if len > 0 {
                        app.relationship_edge_selected =
                            (app.relationship_edge_selected + 1).min(len - 1);
                    }
                }
            } else if app.active_view == ActiveView::Reasoning {
                let len = app.reasoning_nodes.len();
                if len > 0 {
                    app.reasoning_selected = (app.reasoning_selected + 1).min(len - 1);
                }
            } else if app.active_view == ActiveView::Relationships {
                let len = app.relationship_nodes.len();
                if len > 0 {
                    app.relationship_selected = (app.relationship_selected + 1).min(len - 1);
                    app.relationship_edge_selected = 0;
                }
            } else if app.active_view == ActiveView::Contexts {
                let len = app.contexts.len();
                if len > 0 {
                    app.contexts_selected = (app.contexts_selected + 1).min(len - 1);
                }
            } else if app.active_view == ActiveView::Adrs {
                let len = app.all_adrs.len();
                if len > 0 {
                    app.adrs_selected = (app.adrs_selected + 1).min(len - 1);
                }
            } else if app.active_view == ActiveView::Theories {
                let len = app.all_theories.len();
                if len > 0 {
                    app.theories_selected = (app.theories_selected + 1).min(len - 1);
                }
            } else if app.active_view == ActiveView::Workflows {
                let len = app.all_workflows.len();
                if len > 0 {
                    app.workflows_selected = (app.workflows_selected + 1).min(len - 1);
                }
            } else if app.active_view == ActiveView::WorkflowInstances {
                let len = app.all_workflow_instances.len();
                if len > 0 {
                    app.workflow_instances_selected =
                        (app.workflow_instances_selected + 1).min(len - 1);
                }
            } else if app.active_view == ActiveView::Knowledge {
                let len = app.all_knowledge.len();
                if len > 0 {
                    app.knowledge_selected = (app.knowledge_selected + 1).min(len - 1);
                }
            } else if app.active_view == ActiveView::Sessions {
                let len = app.all_sessions.len();
                if len > 0 {
                    app.sessions_selected = (app.sessions_selected + 1).min(len - 1);
                }
            } else if app.active_view == ActiveView::Compliance {
                let len = app.all_compliance.len();
                if len > 0 {
                    app.compliance_selected = (app.compliance_selected + 1).min(len - 1);
                }
            } else if app.active_view == ActiveView::Rules {
                let len = app.all_rules.len();
                if len > 0 {
                    app.rules_selected = (app.rules_selected + 1).min(len - 1);
                }
            } else if app.active_view == ActiveView::Standards {
                let len = app.all_standards.len();
                if len > 0 {
                    app.standards_selected = (app.standards_selected + 1).min(len - 1);
                }
            } else if app.active_view == ActiveView::StateReflections {
                let len = app.all_state_reflections.len();
                if len > 0 {
                    app.state_reflections_selected =
                        (app.state_reflections_selected + 1).min(len - 1);
                }
            } else if app.active_view == ActiveView::Escalations {
                let len = app.all_escalations.len();
                if len > 0 {
                    app.escalations_selected = (app.escalations_selected + 1).min(len - 1);
                }
            } else if app.active_view == ActiveView::Sandboxes {
                let len = app.all_sandboxes.len();
                if len > 0 {
                    app.sandboxes_selected = (app.sandboxes_selected + 1).min(len - 1);
                }
            } else if app.active_view == ActiveView::ExecutionResults {
                let len = app.all_execution_results.len();
                if len > 0 {
                    app.execution_results_selected =
                        (app.execution_results_selected + 1).min(len - 1);
                }
            } else if app.active_view == ActiveView::ProgressiveConfigs {
                let len = app.all_progressive_configs.len();
                if len > 0 {
                    app.progressive_configs_selected =
                        (app.progressive_configs_selected + 1).min(len - 1);
                }
            } else if app.active_view == ActiveView::Search {
                let len = app.search_results.len();
                if len > 0 {
                    app.search_result_selected = (app.search_result_selected + 1).min(len - 1);
                }
            } else if app.active_view == ActiveView::Sync {
                let len = app.sync_view.remotes.len();
                if len > 0 {
                    app.sync_view.remotes_selected =
                        (app.sync_view.remotes_selected + 1).min(len - 1);
                }
            } else {
                app.select_next();
            }
        }
        KeyAction::SelectPrev => {
            if app.active_view == ActiveView::Relationships
                && app.relationship_focus == RelationshipFocus::Edges
            {
                app.relationship_edge_selected = app.relationship_edge_selected.saturating_sub(1);
            } else if app.active_view == ActiveView::Reasoning {
                app.reasoning_selected = app.reasoning_selected.saturating_sub(1);
            } else if app.active_view == ActiveView::Relationships {
                app.relationship_selected = app.relationship_selected.saturating_sub(1);
                app.relationship_edge_selected = 0;
            } else if app.active_view == ActiveView::Contexts {
                app.contexts_selected = app.contexts_selected.saturating_sub(1);
            } else if app.active_view == ActiveView::Adrs {
                app.adrs_selected = app.adrs_selected.saturating_sub(1);
            } else if app.active_view == ActiveView::Theories {
                app.theories_selected = app.theories_selected.saturating_sub(1);
            } else if app.active_view == ActiveView::Workflows {
                app.workflows_selected = app.workflows_selected.saturating_sub(1);
            } else if app.active_view == ActiveView::WorkflowInstances {
                app.workflow_instances_selected = app.workflow_instances_selected.saturating_sub(1);
            } else if app.active_view == ActiveView::Knowledge {
                app.knowledge_selected = app.knowledge_selected.saturating_sub(1);
            } else if app.active_view == ActiveView::Sessions {
                app.sessions_selected = app.sessions_selected.saturating_sub(1);
            } else if app.active_view == ActiveView::Compliance {
                app.compliance_selected = app.compliance_selected.saturating_sub(1);
            } else if app.active_view == ActiveView::Rules {
                app.rules_selected = app.rules_selected.saturating_sub(1);
            } else if app.active_view == ActiveView::Standards {
                app.standards_selected = app.standards_selected.saturating_sub(1);
            } else if app.active_view == ActiveView::StateReflections {
                app.state_reflections_selected = app.state_reflections_selected.saturating_sub(1);
            } else if app.active_view == ActiveView::Escalations {
                app.escalations_selected = app.escalations_selected.saturating_sub(1);
            } else if app.active_view == ActiveView::Sandboxes {
                app.sandboxes_selected = app.sandboxes_selected.saturating_sub(1);
            } else if app.active_view == ActiveView::ExecutionResults {
                app.execution_results_selected = app.execution_results_selected.saturating_sub(1);
            } else if app.active_view == ActiveView::ProgressiveConfigs {
                app.progressive_configs_selected =
                    app.progressive_configs_selected.saturating_sub(1);
            } else if app.active_view == ActiveView::Search {
                app.search_result_selected = app.search_result_selected.saturating_sub(1);
            } else if app.active_view == ActiveView::Sync {
                app.sync_view.remotes_selected = app.sync_view.remotes_selected.saturating_sub(1);
            } else {
                app.select_prev();
            }
        }
        KeyAction::SelectTop => {
            if app.active_view == ActiveView::Relationships
                && app.relationship_focus == RelationshipFocus::Edges
            {
                app.relationship_edge_selected = 0;
            } else if app.active_view == ActiveView::Reasoning {
                app.reasoning_selected = 0;
            } else if app.active_view == ActiveView::Relationships {
                app.relationship_selected = 0;
                app.relationship_edge_selected = 0;
            } else if app.active_view == ActiveView::Contexts {
                app.contexts_selected = 0;
            } else if app.active_view == ActiveView::Adrs {
                app.adrs_selected = 0;
            } else if app.active_view == ActiveView::Theories {
                app.theories_selected = 0;
            } else if app.active_view == ActiveView::Workflows {
                app.workflows_selected = 0;
            } else if app.active_view == ActiveView::WorkflowInstances {
                app.workflow_instances_selected = 0;
            } else if app.active_view == ActiveView::Knowledge {
                app.knowledge_selected = 0;
            } else if app.active_view == ActiveView::Sessions {
                app.sessions_selected = 0;
            } else if app.active_view == ActiveView::Compliance {
                app.compliance_selected = 0;
            } else if app.active_view == ActiveView::Rules {
                app.rules_selected = 0;
            } else if app.active_view == ActiveView::Standards {
                app.standards_selected = 0;
            } else if app.active_view == ActiveView::StateReflections {
                app.state_reflections_selected = 0;
            } else if app.active_view == ActiveView::Escalations {
                app.escalations_selected = 0;
            } else if app.active_view == ActiveView::Sandboxes {
                app.sandboxes_selected = 0;
            } else if app.active_view == ActiveView::ExecutionResults {
                app.execution_results_selected = 0;
            } else if app.active_view == ActiveView::ProgressiveConfigs {
                app.progressive_configs_selected = 0;
            } else if app.active_view == ActiveView::Sync {
                app.sync_view.remotes_selected = 0;
            } else {
                app.selected_index = 0;
            }
        }
        KeyAction::SelectBottom => {
            if app.active_view == ActiveView::Relationships
                && app.relationship_focus == RelationshipFocus::Edges
            {
                if let Some(node) = app.relationship_nodes.get(app.relationship_selected) {
                    let len = node.edges.len();
                    if len > 0 {
                        app.relationship_edge_selected = len - 1;
                    }
                }
            } else if app.active_view == ActiveView::Reasoning {
                let len = app.reasoning_nodes.len();
                if len > 0 {
                    app.reasoning_selected = len - 1;
                }
            } else if app.active_view == ActiveView::Relationships {
                let len = app.relationship_nodes.len();
                if len > 0 {
                    app.relationship_selected = len - 1;
                    app.relationship_edge_selected = 0;
                }
            } else if app.active_view == ActiveView::Contexts {
                let len = app.contexts.len();
                if len > 0 {
                    app.contexts_selected = len - 1;
                }
            } else if app.active_view == ActiveView::Adrs {
                let len = app.all_adrs.len();
                if len > 0 {
                    app.adrs_selected = len - 1;
                }
            } else if app.active_view == ActiveView::Theories {
                let len = app.all_theories.len();
                if len > 0 {
                    app.theories_selected = len - 1;
                }
            } else if app.active_view == ActiveView::Workflows {
                let len = app.all_workflows.len();
                if len > 0 {
                    app.workflows_selected = len - 1;
                }
            } else if app.active_view == ActiveView::WorkflowInstances {
                let len = app.all_workflow_instances.len();
                if len > 0 {
                    app.workflow_instances_selected = len - 1;
                }
            } else if app.active_view == ActiveView::Knowledge {
                let len = app.all_knowledge.len();
                if len > 0 {
                    app.knowledge_selected = len - 1;
                }
            } else if app.active_view == ActiveView::Sessions {
                let len = app.all_sessions.len();
                if len > 0 {
                    app.sessions_selected = len - 1;
                }
            } else if app.active_view == ActiveView::Compliance {
                let len = app.all_compliance.len();
                if len > 0 {
                    app.compliance_selected = len - 1;
                }
            } else if app.active_view == ActiveView::Rules {
                let len = app.all_rules.len();
                if len > 0 {
                    app.rules_selected = len - 1;
                }
            } else if app.active_view == ActiveView::Standards {
                let len = app.all_standards.len();
                if len > 0 {
                    app.standards_selected = len - 1;
                }
            } else if app.active_view == ActiveView::StateReflections {
                let len = app.all_state_reflections.len();
                if len > 0 {
                    app.state_reflections_selected = len - 1;
                }
            } else if app.active_view == ActiveView::Escalations {
                let len = app.all_escalations.len();
                if len > 0 {
                    app.escalations_selected = len - 1;
                }
            } else if app.active_view == ActiveView::Sandboxes {
                let len = app.all_sandboxes.len();
                if len > 0 {
                    app.sandboxes_selected = len - 1;
                }
            } else if app.active_view == ActiveView::ExecutionResults {
                let len = app.all_execution_results.len();
                if len > 0 {
                    app.execution_results_selected = len - 1;
                }
            } else if app.active_view == ActiveView::ProgressiveConfigs {
                let len = app.all_progressive_configs.len();
                if len > 0 {
                    app.progressive_configs_selected = len - 1;
                }
            } else if app.active_view == ActiveView::Sync {
                let len = app.sync_view.remotes.len();
                if len > 0 {
                    app.sync_view.remotes_selected = len - 1;
                }
            } else {
                let len = app.recent_tasks.len();
                app.select_bottom_of(len);
            }
        }
        KeyAction::ToggleTheme => app.toggle_theme(),
        KeyAction::Confirm => {
            if app.show_help {
                app.show_help = false;
            } else if app.active_view == ActiveView::Relationships
                && app.relationship_focus == RelationshipFocus::Nodes
            {
                // Enter on a node focuses its edge pane
                app.relationship_focus = RelationshipFocus::Edges;
                app.relationship_edge_selected = 0;
            } else if app.active_view == ActiveView::Reasoning {
                app.toggle_reasoning_node();
            } else if app.active_view == ActiveView::Tasks
                || app.active_view == ActiveView::Dashboard
            {
                if app.task_detail.is_none() {
                    return (true, Some(Action::OpenTaskDetail));
                }
            } else if app.active_view == ActiveView::Adrs
                || app.active_view == ActiveView::Contexts
                || app.active_view == ActiveView::Theories
            {
                return (true, Some(Action::OpenEntityDetail));
            } else if app.active_view == ActiveView::Search {
                return (true, Some(Action::OpenSearchResult));
            }
        }
        KeyAction::Back => {
            if app.show_help {
                app.show_help = false;
            } else if app.task_detail.is_some() {
                return (true, Some(Action::CloseDetail));
            } else if app.active_view == ActiveView::Relationships
                && app.relationship_focus == RelationshipFocus::Edges
            {
                // Esc in edge pane returns focus to nodes
                app.relationship_focus = RelationshipFocus::Nodes;
            } else {
                app.clear_status();
            }
        }
        KeyAction::Search => {
            if app.active_view == ActiveView::Search || app.active_view == ActiveView::Tasks {
                app.search_mode = true;
                app.search_query.clear();
                app.active_view = ActiveView::Search;
                return (true, Some(Action::EnterSearchMode));
            } else {
                app.set_status(String::from("Search mode"));
            }
        }
        KeyAction::Refresh => {
            app.set_status(String::from("Refreshing\u{2026}"));
            return (true, Some(Action::Refresh));
        }
        KeyAction::FilterStatus => {
            let next = match app.filter_status.as_deref() {
                None => Some("todo".to_string()),
                Some("todo") => Some("in_progress".to_string()),
                Some("in_progress") => Some("done".to_string()),
                _ => None,
            };
            app.set_filter_status(next);
        }
        KeyAction::CycleTaskStatus => {
            if app.active_view == ActiveView::Tasks || app.active_view == ActiveView::Dashboard {
                return (true, Some(Action::CycleTaskStatus));
            } else if app.active_view == ActiveView::Adrs {
                return (true, Some(Action::CycleAdrStatus));
            }
        }
        KeyAction::NextView => app.next_view(),
        KeyAction::Char(c) => {
            if app.active_view == ActiveView::Sync {
                match c {
                    'p' => return (true, Some(Action::SyncPull)),
                    'u' => return (true, Some(Action::SyncPush)),
                    'b' => return (true, Some(Action::SyncBoth)),
                    'R' => return (true, Some(Action::RefreshSyncStatus)),
                    'j' => {
                        let len = app.sync_view.remotes.len();
                        if len > 0 {
                            app.sync_view.remotes_selected =
                                (app.sync_view.remotes_selected + 1).min(len - 1);
                        }
                    }
                    'k' => {
                        app.sync_view.remotes_selected =
                            app.sync_view.remotes_selected.saturating_sub(1);
                    }
                    _ => {}
                }
            } else if app.active_view == ActiveView::Escalations {
                match c {
                    'a' => return (true, Some(Action::EscalationApprove)),
                    'd' => return (true, Some(Action::EscalationDeny)),
                    _ => {}
                }
            }
        }
        KeyAction::Unknown => {}
    }
    (!app.should_quit, None)
}

/// Handle a mouse event: click to select rows.
fn handle_mouse(app: &mut AppState, mouse: crossterm::event::MouseEvent) -> (bool, Option<Action>) {
    match mouse.kind {
        MouseEventKind::Down(MouseButton::Left) => {
            let row = mouse.row;
            let col = mouse.column;
            // Rows 0-2 are the title bar, row 3 onwards is the content area.
            // We do a simple heuristic: the content area starts at y=3, so
            // a click at row R maps to list item (R - 3) approximately.
            // Each view's list starts after its own header rows; we use a
            // conservative offset of 4 to account for borders.
            if row >= 4 {
                let item_idx = (row - 4) as usize;
                match app.active_view {
                    ActiveView::Tasks | ActiveView::Dashboard => {
                        let len = app.recent_tasks.len();
                        if item_idx < len {
                            app.selected_index = item_idx;
                        }
                    }
                    ActiveView::Reasoning => {
                        let len = app.reasoning_nodes.len();
                        if item_idx < len {
                            app.reasoning_selected = item_idx;
                        }
                    }
                    ActiveView::Relationships => {
                        // Left 40% = nodes pane, right 60% = edges pane
                        // We don't know the exact terminal width here, but we
                        // use a heuristic of 80 cols as minimum.
                        // Better: track last rendered widths in AppState (future work).
                        // For now: col < 40% of 120 ≈ 48 = nodes, else edges.
                        let approx_node_width = 48u16;
                        if col < approx_node_width {
                            let len = app.relationship_nodes.len();
                            if item_idx < len {
                                app.relationship_selected = item_idx;
                                app.relationship_focus = RelationshipFocus::Nodes;
                                app.relationship_edge_selected = 0;
                            }
                        } else if app.relationship_focus == RelationshipFocus::Edges {
                            if let Some(node) =
                                app.relationship_nodes.get(app.relationship_selected)
                            {
                                let len = node.edges.len();
                                if item_idx < len {
                                    app.relationship_edge_selected = item_idx;
                                }
                            }
                        } else {
                            // Clicking the edge pane activates it
                            app.relationship_focus = RelationshipFocus::Edges;
                            app.relationship_edge_selected = 0;
                        }
                    }
                    ActiveView::Contexts => {
                        let len = app.contexts.len();
                        if item_idx < len {
                            app.contexts_selected = item_idx;
                        }
                    }
                    ActiveView::Adrs => {
                        let len = app.all_adrs.len();
                        if item_idx < len {
                            app.adrs_selected = item_idx;
                        }
                    }
                    ActiveView::Theories => {
                        let len = app.all_theories.len();
                        if item_idx < len {
                            app.theories_selected = item_idx;
                        }
                    }
                    ActiveView::Workflows => {
                        let len = app.all_workflows.len();
                        if item_idx < len {
                            app.workflows_selected = item_idx;
                        }
                    }
                    ActiveView::WorkflowInstances => {
                        let len = app.all_workflow_instances.len();
                        if item_idx < len {
                            app.workflow_instances_selected = item_idx;
                        }
                    }
                    ActiveView::Knowledge => {
                        let len = app.all_knowledge.len();
                        if item_idx < len {
                            app.knowledge_selected = item_idx;
                        }
                    }
                    ActiveView::Sessions => {
                        let len = app.all_sessions.len();
                        if item_idx < len {
                            app.sessions_selected = item_idx;
                        }
                    }
                    ActiveView::Compliance => {
                        let len = app.all_compliance.len();
                        if item_idx < len {
                            app.compliance_selected = item_idx;
                        }
                    }
                    ActiveView::Rules => {
                        let len = app.all_rules.len();
                        if item_idx < len {
                            app.rules_selected = item_idx;
                        }
                    }
                    ActiveView::Standards => {
                        let len = app.all_standards.len();
                        if item_idx < len {
                            app.standards_selected = item_idx;
                        }
                    }
                    ActiveView::StateReflections => {
                        let len = app.all_state_reflections.len();
                        if item_idx < len {
                            app.state_reflections_selected = item_idx;
                        }
                    }
                    ActiveView::Escalations => {
                        let len = app.all_escalations.len();
                        if item_idx < len {
                            app.escalations_selected = item_idx;
                        }
                    }
                    ActiveView::Sandboxes => {
                        let len = app.all_sandboxes.len();
                        if item_idx < len {
                            app.sandboxes_selected = item_idx;
                        }
                    }
                    ActiveView::ExecutionResults => {
                        let len = app.all_execution_results.len();
                        if item_idx < len {
                            app.execution_results_selected = item_idx;
                        }
                    }
                    ActiveView::ProgressiveConfigs => {
                        let len = app.all_progressive_configs.len();
                        if item_idx < len {
                            app.progressive_configs_selected = item_idx;
                        }
                    }
                    ActiveView::Search => {}
                    ActiveView::Analytics => {}
                    ActiveView::Sync => {
                        let len = app.sync_view.remotes.len();
                        if item_idx < len {
                            app.sync_view.remotes_selected = item_idx;
                        }
                    }
                }
            }
        }
        MouseEventKind::ScrollDown => {
            // Delegate to SelectNext logic via a synthetic key
            return handle_key(
                app,
                crossterm::event::KeyEvent::new(
                    KeyCode::Down,
                    crossterm::event::KeyModifiers::empty(),
                ),
            );
        }
        MouseEventKind::ScrollUp => {
            return handle_key(
                app,
                crossterm::event::KeyEvent::new(
                    KeyCode::Up,
                    crossterm::event::KeyModifiers::empty(),
                ),
            );
        }
        _ => {}
    }
    (!app.should_quit, None)
}

/// Handle key input when in search mode.
fn handle_search_input(app: &mut AppState, key: KeyEvent) -> (bool, Option<Action>) {
    match key.code {
        KeyCode::Esc => {
            app.search_mode = false;
            return (true, Some(Action::ExitSearchMode));
        }
        KeyCode::Enter => {
            return (true, Some(Action::RunSearch));
        }
        KeyCode::Backspace => {
            app.search_query.pop();
            return (true, Some(Action::RunSearch));
        }
        KeyCode::Char(c) => {
            app.search_query.push(c);
            return (true, Some(Action::SearchQueryChar(c)));
        }
        _ => {}
    }
    (true, None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::empty())
    }

    #[test]
    fn test_map_key_q_quit() {
        assert_eq!(map_key(key(KeyCode::Char('q'))), KeyAction::Quit);
    }

    #[test]
    fn test_map_key_q_upper_quit() {
        assert_eq!(map_key(key(KeyCode::Char('Q'))), KeyAction::Quit);
    }

    #[test]
    fn test_map_key_tab_focus_next() {
        assert_eq!(map_key(key(KeyCode::Tab)), KeyAction::FocusNext);
    }

    #[test]
    fn test_map_key_backtab_prev_view() {
        assert_eq!(map_key(key(KeyCode::BackTab)), KeyAction::PrevView);
    }

    #[test]
    fn test_map_key_down_select_next() {
        assert_eq!(map_key(key(KeyCode::Down)), KeyAction::SelectNext);
    }

    #[test]
    fn test_map_key_j_select_next() {
        assert_eq!(map_key(key(KeyCode::Char('j'))), KeyAction::SelectNext);
    }

    #[test]
    fn test_map_key_up_select_prev() {
        assert_eq!(map_key(key(KeyCode::Up)), KeyAction::SelectPrev);
    }

    #[test]
    fn test_map_key_k_select_prev() {
        assert_eq!(map_key(key(KeyCode::Char('k'))), KeyAction::SelectPrev);
    }

    #[test]
    fn test_map_key_g_select_top() {
        assert_eq!(map_key(key(KeyCode::Char('g'))), KeyAction::SelectTop);
    }

    #[test]
    fn test_map_key_g_upper_select_bottom() {
        assert_eq!(map_key(key(KeyCode::Char('G'))), KeyAction::SelectBottom);
    }

    #[test]
    fn test_map_key_t_toggle_theme() {
        assert_eq!(map_key(key(KeyCode::Char('t'))), KeyAction::ToggleTheme);
    }

    #[test]
    fn test_map_key_enter_confirm() {
        assert_eq!(map_key(key(KeyCode::Enter)), KeyAction::Confirm);
    }

    #[test]
    fn test_map_key_esc_back() {
        assert_eq!(map_key(key(KeyCode::Esc)), KeyAction::Back);
    }

    #[test]
    fn test_map_key_slash_search() {
        assert_eq!(map_key(key(KeyCode::Char('/'))), KeyAction::Search);
    }

    #[test]
    fn test_map_key_r_refresh() {
        assert_eq!(map_key(key(KeyCode::Char('r'))), KeyAction::Refresh);
    }

    #[test]
    fn test_map_key_f_filter_status() {
        assert_eq!(map_key(key(KeyCode::Char('f'))), KeyAction::FilterStatus);
    }

    #[test]
    fn test_map_key_s_cycle_task_status() {
        assert_eq!(map_key(key(KeyCode::Char('s'))), KeyAction::CycleTaskStatus);
    }

    #[test]
    fn test_map_key_question_toggle_help() {
        assert_eq!(map_key(key(KeyCode::Char('?'))), KeyAction::ToggleHelp);
    }

    #[test]
    fn test_map_key_z_char() {
        assert_eq!(map_key(key(KeyCode::Char('z'))), KeyAction::Char('z'));
    }

    #[test]
    fn test_relationship_edge_navigation() {
        let mut app = AppState::new();
        app.active_view = ActiveView::Relationships;
        use crate::locus_tui::app::{RelationshipEdge, RelationshipNode};
        app.relationship_nodes = vec![RelationshipNode {
            id: "n1".to_string(),
            title: "Node 1".to_string(),
            entity_type: "task".to_string(),
            edges: vec![
                RelationshipEdge {
                    from_id: "n1".to_string(),
                    to_id: "n2".to_string(),
                    to_type: "context".to_string(),
                    relationship_type: "relates_to".to_string(),
                    to_title: "Node 2".to_string(),
                    agent: "test".to_string(),
                },
                RelationshipEdge {
                    from_id: "n1".to_string(),
                    to_id: "n3".to_string(),
                    to_type: "reasoning".to_string(),
                    relationship_type: "explains".to_string(),
                    to_title: "Node 3".to_string(),
                    agent: "test".to_string(),
                },
            ],
        }];
        // Initially in Nodes focus; Enter moves to Edges
        assert_eq!(app.relationship_focus, RelationshipFocus::Nodes);
        handle_key(
            &mut app,
            crossterm::event::KeyEvent::new(KeyCode::Enter, KeyModifiers::empty()),
        );
        assert_eq!(app.relationship_focus, RelationshipFocus::Edges);
        assert_eq!(app.relationship_edge_selected, 0);
        // j moves to edge 1
        handle_key(
            &mut app,
            crossterm::event::KeyEvent::new(KeyCode::Char('j'), KeyModifiers::empty()),
        );
        assert_eq!(app.relationship_edge_selected, 1);
        // k moves back to edge 0
        handle_key(
            &mut app,
            crossterm::event::KeyEvent::new(KeyCode::Char('k'), KeyModifiers::empty()),
        );
        assert_eq!(app.relationship_edge_selected, 0);
        // Esc returns to Nodes focus
        handle_key(
            &mut app,
            crossterm::event::KeyEvent::new(KeyCode::Esc, KeyModifiers::empty()),
        );
        assert_eq!(app.relationship_focus, RelationshipFocus::Nodes);
    }

    #[test]
    fn test_help_toggle() {
        let mut app = AppState::new();
        assert!(!app.show_help);
        handle_key(
            &mut app,
            crossterm::event::KeyEvent::new(KeyCode::Char('?'), KeyModifiers::empty()),
        );
        assert!(app.show_help);
        handle_key(
            &mut app,
            crossterm::event::KeyEvent::new(KeyCode::Char('?'), KeyModifiers::empty()),
        );
        assert!(!app.show_help);
    }
}

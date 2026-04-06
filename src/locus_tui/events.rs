use crate::locus_tui::app::{ActiveView, AppState};
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::time::Duration;

/// High-level action derived from a raw key event.
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
    EnterSearchMode,
    ExitSearchMode,
    SearchQueryChar(char),
    RunSearch,
}

/// Map a raw crossterm `KeyEvent` to a `KeyAction`.
pub fn map_key(key: KeyEvent) -> KeyAction {
    match key.code {
        KeyCode::Char('q') | KeyCode::Char('Q') => KeyAction::Quit,
        KeyCode::Tab => KeyAction::NextView,
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
        KeyCode::Char(c) => KeyAction::Char(c),
        _ => KeyAction::Unknown,
    }
}

/// Poll for input events, update app state, and return (keep_running, Option<Action>).
/// `true` = keep running, `false` = quit.
pub fn handle_input(app: &mut AppState) -> (bool, Option<Action>) {
    if event::poll(Duration::from_millis(100)).unwrap_or(false) {
        if let Ok(Event::Key(key)) = event::read() {
            // If in search mode, handle characters specially
            if app.search_mode {
                return handle_search_input(app, key);
            }

            match map_key(key) {
                KeyAction::Quit => {
                    app.quit();
                    return (false, None);
                }
                KeyAction::NextView => app.next_view(),
                KeyAction::PrevView => app.prev_view(),
                KeyAction::SelectNext => {
                    if app.active_view == ActiveView::Reasoning {
                        let len = app.reasoning_nodes.len();
                        if len > 0 {
                            app.reasoning_selected = (app.reasoning_selected + 1).min(len - 1);
                        }
                    } else if app.active_view == ActiveView::Relationships {
                        let len = app.relationship_nodes.len();
                        if len > 0 {
                            app.relationship_selected =
                                (app.relationship_selected + 1).min(len - 1);
                        }
                    } else if app.active_view == ActiveView::Contexts {
                        let len = app.contexts.len();
                        if len > 0 {
                            app.contexts_selected = (app.contexts_selected + 1).min(len - 1);
                        }
                    } else {
                        app.select_next();
                    }
                }
                KeyAction::SelectPrev => {
                    if app.active_view == ActiveView::Reasoning {
                        app.reasoning_selected = app.reasoning_selected.saturating_sub(1);
                    } else if app.active_view == ActiveView::Relationships {
                        app.relationship_selected = app.relationship_selected.saturating_sub(1);
                    } else if app.active_view == ActiveView::Contexts {
                        app.contexts_selected = app.contexts_selected.saturating_sub(1);
                    } else {
                        app.select_prev();
                    }
                }
                KeyAction::SelectTop => {
                    if app.active_view == ActiveView::Reasoning {
                        app.reasoning_selected = 0;
                    } else if app.active_view == ActiveView::Relationships {
                        app.relationship_selected = 0;
                    } else if app.active_view == ActiveView::Contexts {
                        app.contexts_selected = 0;
                    } else {
                        app.selected_index = 0;
                    }
                }
                KeyAction::SelectBottom => {
                    if app.active_view == ActiveView::Reasoning {
                        let len = app.reasoning_nodes.len();
                        if len > 0 {
                            app.reasoning_selected = len - 1;
                        }
                    } else if app.active_view == ActiveView::Relationships {
                        let len = app.relationship_nodes.len();
                        if len > 0 {
                            app.relationship_selected = len - 1;
                        }
                    } else if app.active_view == ActiveView::Contexts {
                        let len = app.contexts.len();
                        if len > 0 {
                            app.contexts_selected = len - 1;
                        }
                    } else {
                        let len = app.recent_tasks.len();
                        app.select_bottom_of(len);
                    }
                }
                KeyAction::ToggleTheme => app.toggle_theme(),
                KeyAction::Confirm => {
                    if app.active_view == ActiveView::Reasoning {
                        app.toggle_reasoning_node();
                    } else if app.active_view == ActiveView::Tasks
                        || app.active_view == ActiveView::Dashboard
                    {
                        // Open task detail if task_detail is not shown
                        if app.task_detail.is_none() {
                            return (true, Some(Action::OpenTaskDetail));
                        }
                    } else {
                        app.set_status(String::from("Confirm"));
                    }
                }
                KeyAction::Back => {
                    if app.task_detail.is_some() {
                        return (true, Some(Action::CloseDetail));
                    }
                    app.clear_status();
                }
                KeyAction::Search => {
                    if app.active_view == ActiveView::Search || app.active_view == ActiveView::Tasks
                    {
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
                    // Cycle: None → todo → in_progress → done → None
                    let next = match app.filter_status.as_deref() {
                        None => Some("todo".to_string()),
                        Some("todo") => Some("in_progress".to_string()),
                        Some("in_progress") => Some("done".to_string()),
                        _ => None,
                    };
                    app.set_filter_status(next);
                }
                KeyAction::CycleTaskStatus => {
                    if app.active_view == ActiveView::Tasks {
                        return (true, Some(Action::CycleTaskStatus));
                    }
                }
                KeyAction::Char(_) | KeyAction::Unknown => {}
            }
        }
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
    fn test_map_key_tab_next_view() {
        assert_eq!(map_key(key(KeyCode::Tab)), KeyAction::NextView);
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
    fn test_map_key_z_char() {
        assert_eq!(map_key(key(KeyCode::Char('z'))), KeyAction::Char('z'));
    }
}

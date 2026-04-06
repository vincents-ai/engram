use crate::locus_tui::app::AppState;
use crossterm::event::{self, Event, KeyCode};
use std::io;

/// Poll for input events, update app state, and return true if the application
/// should continue running (false = quit).
pub fn handle_input(app_state: &mut AppState) -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => {
                    app_state.quit();
                    return Ok(false);
                }
                KeyCode::Tab => {
                    app_state.next_view();
                }
                KeyCode::BackTab => {
                    app_state.prev_view();
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    app_state.select_next();
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    app_state.select_prev();
                }
                _ => {}
            }
        }
    }
    Ok(!app_state.should_quit)
}

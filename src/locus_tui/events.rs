use crossterm::event::{self, Event, KeyCode};
use std::io;

/// Poll for input events and return true if the application should quit.
pub fn handle_input() -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

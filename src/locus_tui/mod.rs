pub mod app;
pub mod backend;
pub mod events;
pub mod ui;

use crate::locus_integration::LocusIntegration;
use crate::storage::{RelationshipStorage, Storage};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;

pub struct LocusTuiApp<S: Storage + RelationshipStorage> {
    integration: LocusIntegration<S>,
}

impl<S: Storage + RelationshipStorage> LocusTuiApp<S> {
    pub fn new(storage: S) -> Self {
        let integration = LocusIntegration::new(storage);
        Self { integration }
    }

    pub fn run(&mut self) -> io::Result<()> {
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        loop {
            terminal.draw(|f| ui::draw(&self.integration, f))?;

            if events::handle_input()? {
                break;
            }
        }

        Ok(())
    }

    #[cfg(test)]
    fn draw(&self, f: &mut ratatui::Frame<'_>) {
        ui::draw(&self.integration, f);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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

    #[test]
    fn test_new() {
        let storage = MemoryStorage::new("test-agent");
        let _app = LocusTuiApp::new(storage);
    }

    #[test]
    fn test_new_with_integration() {
        let storage = MemoryStorage::new("test-agent");
        let app = LocusTuiApp::new(storage);
        let workflows = app.integration.get_workflows().unwrap();
        assert!(workflows.is_empty());
    }

    #[test]
    fn test_draw_with_empty_storage() {
        let storage = MemoryStorage::new("test-agent");
        let app = LocusTuiApp::new(storage);

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
        let storage = MemoryStorage::new("test-agent");
        let app = LocusTuiApp::new(storage);

        let backend = ratatui::backend::TestBackend::new(80, 24);
        let mut terminal = ratatui::Terminal::new(backend).unwrap();
        terminal.draw(|f| app.draw(f)).unwrap();

        let buf = terminal.backend().buffer();
        let content = buffer_to_string(buf);
        assert!(content.contains("Locus TUI"));
    }

    #[test]
    fn test_draw_help_bar() {
        let storage = MemoryStorage::new("test-agent");
        let app = LocusTuiApp::new(storage);

        let backend = ratatui::backend::TestBackend::new(80, 24);
        let mut terminal = ratatui::Terminal::new(backend).unwrap();
        terminal.draw(|f| app.draw(f)).unwrap();

        let buf = terminal.backend().buffer();
        let content = buffer_to_string(buf);
        assert!(content.contains("'q' to quit"));
    }
}

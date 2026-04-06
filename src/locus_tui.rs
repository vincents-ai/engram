use crate::locus_integration::LocusIntegration;
use crate::storage::{RelationshipStorage, Storage};
use crossterm::event::{self, Event, KeyCode};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
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
            terminal.draw(|f| self.draw(f))?;

            if self.handle_input()? {
                break;
            }
        }

        Ok(())
    }

    fn draw(&self, f: &mut ratatui::Frame<'_>) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(10),
                Constraint::Length(3),
            ])
            .split(f.area());

        let tasks = self.integration.get_tasks(None).unwrap_or_default();
        let workflows = self.integration.get_workflows().unwrap_or_default();

        let task_count = tasks.len();
        let workflow_count = workflows.len();

        let title = Paragraph::new(format!(
            "Locus TUI - Engram System Interface | Tasks: {} | Workflows: {}",
            task_count, workflow_count
        ))
        .style(Style::default().fg(Color::Cyan));

        f.render_widget(title, chunks[0]);

        let tasks_list: Vec<ListItem> = tasks
            .iter()
            .take(10)
            .map(|task| {
                let status_str = format!("{:?}", task.status).to_lowercase();
                ListItem::new(format!(
                    "[{}] {} - {} ({})",
                    task.id.split_at(8).0,
                    task.title,
                    status_str,
                    task.agent
                ))
            })
            .collect();

        let tasks_widget =
            List::new(tasks_list).block(Block::default().title("Tasks").borders(Borders::ALL));

        let workflows_list: Vec<ListItem> = workflows
            .iter()
            .take(5)
            .map(|workflow| {
                ListItem::new(format!(
                    "[{}] {}",
                    workflow.id.split_at(8).0,
                    workflow.title
                ))
            })
            .collect();

        let workflows_widget = List::new(workflows_list)
            .block(Block::default().title("Workflows").borders(Borders::ALL));

        let center_chunk = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
            .split(chunks[1]);

        f.render_widget(tasks_widget, center_chunk[0]);
        f.render_widget(workflows_widget, center_chunk[1]);

        let help = Paragraph::new("Press 'q' to quit").style(Style::default().fg(Color::Yellow));
        f.render_widget(help, chunks[2]);
    }

    fn handle_input(&self) -> io::Result<bool> {
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    return Ok(true);
                }
            }
        }
        Ok(false)
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

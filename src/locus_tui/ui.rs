use crate::locus_integration::LocusIntegration;
use crate::storage::{RelationshipStorage, Storage};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};

/// Render the TUI to the given frame.
pub fn draw<S: Storage + RelationshipStorage>(
    integration: &LocusIntegration<S>,
    f: &mut ratatui::Frame<'_>,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(f.area());

    let tasks = integration.get_tasks(None).unwrap_or_default();
    let workflows = integration.get_workflows().unwrap_or_default();

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

    let workflows_widget =
        List::new(workflows_list).block(Block::default().title("Workflows").borders(Borders::ALL));

    let center_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(chunks[1]);

    f.render_widget(tasks_widget, center_chunk[0]);
    f.render_widget(workflows_widget, center_chunk[1]);

    let help = Paragraph::new("Press 'q' to quit").style(Style::default().fg(Color::Yellow));
    f.render_widget(help, chunks[2]);
}

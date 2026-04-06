use crate::locus_integration::LocusIntegration;
use crate::locus_tui::app::{ActiveView, AppState};
use crate::storage::{RelationshipStorage, Storage};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};

/// Render the TUI to the given frame.
pub fn draw<S: Storage + RelationshipStorage>(
    integration: &LocusIntegration<S>,
    app_state: &mut AppState,
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

    // Render view-specific content in the main panel
    match &app_state.active_view {
        ActiveView::Dashboard => {
            draw_dashboard(integration, &tasks, &workflows, f, chunks[1]);
        }
        ActiveView::Tasks => {
            let panel = Block::default().title("Tasks").borders(Borders::ALL);
            let items: Vec<ListItem> = tasks
                .iter()
                .take(20)
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
            f.render_widget(List::new(items).block(panel), chunks[1]);
        }
        ActiveView::Reasoning => {
            let panel = Paragraph::new("Reasoning view — no data loaded")
                .block(Block::default().title("Reasoning").borders(Borders::ALL));
            f.render_widget(panel, chunks[1]);
        }
        ActiveView::Relationships => {
            let panel = Paragraph::new("Relationships view — no data loaded").block(
                Block::default()
                    .title("Relationships")
                    .borders(Borders::ALL),
            );
            f.render_widget(panel, chunks[1]);
        }
        ActiveView::Contexts => {
            let panel = Paragraph::new("Contexts view — no data loaded")
                .block(Block::default().title("Contexts").borders(Borders::ALL));
            f.render_widget(panel, chunks[1]);
        }
        ActiveView::Search => {
            let query = app_state.search_query.clone();
            let panel = Paragraph::new(format!("Search: {}", query))
                .block(Block::default().title("Search").borders(Borders::ALL));
            f.render_widget(panel, chunks[1]);
        }
    }

    // Status / help bar
    let help_text = if let Some(ref msg) = app_state.status_message {
        format!("Press 'q' to quit | {}", msg)
    } else {
        "Press 'q' to quit".to_string()
    };
    let help = Paragraph::new(help_text).style(Style::default().fg(Color::Yellow));
    f.render_widget(help, chunks[2]);
}

fn draw_dashboard<S: Storage + RelationshipStorage>(
    _integration: &LocusIntegration<S>,
    tasks: &[crate::entities::Task],
    workflows: &[crate::entities::Workflow],
    f: &mut ratatui::Frame<'_>,
    area: ratatui::layout::Rect,
) {
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
        .split(area);

    f.render_widget(tasks_widget, center_chunk[0]);
    f.render_widget(workflows_widget, center_chunk[1]);
}

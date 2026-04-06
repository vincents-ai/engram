use crate::locus_integration::LocusIntegration;
use crate::locus_tui::app::{ActiveView, AppState};
#[allow(unused_imports)]
use crate::locus_tui::theme::Theme;
use crate::storage::{RelationshipStorage, Storage};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Cell, List, ListItem, Paragraph, Row, Table, TableState};

/// Render the TUI to the given frame.
pub fn draw<S: Storage + RelationshipStorage>(
    integration: &LocusIntegration<S>,
    app_state: &mut AppState,
    f: &mut ratatui::Frame<'_>,
) {
    let theme = app_state.theme.as_theme();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(1),
        ])
        .split(f.area());

    let tasks = integration.get_tasks(None).unwrap_or_default();
    let workflows = integration.get_workflows().unwrap_or_default();

    let task_count = tasks.len();
    let workflow_count = workflows.len();

    // Top bar: title, current view, key hints
    let view_name = match &app_state.active_view {
        ActiveView::Dashboard => "Dashboard",
        ActiveView::Tasks => "Tasks",
        ActiveView::Reasoning => "Reasoning",
        ActiveView::Relationships => "Relationships",
        ActiveView::Contexts => "Contexts",
        ActiveView::Search => "Search",
    };
    let title_text = format!(
        "Engram Locus  [{view_name}]  Tasks: {task_count}  Workflows: {workflow_count}  Tab:next  q:quit  t:theme"
    );
    let title = Paragraph::new(title_text).style(Style::default().fg(theme.title()));
    f.render_widget(title, chunks[0]);

    // Border style derived from the active theme
    let border_style = Style::default().fg(theme.border());

    // Render view-specific content in the main panel
    match &app_state.active_view {
        ActiveView::Dashboard => {
            draw_dashboard(app_state, border_style, f, chunks[1]);
        }
        ActiveView::Tasks => {
            let panel = Block::default()
                .title("Tasks")
                .borders(Borders::ALL)
                .border_style(border_style);
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
            let panel = Paragraph::new("Reasoning view — no data loaded").block(
                Block::default()
                    .title("Reasoning")
                    .borders(Borders::ALL)
                    .border_style(border_style),
            );
            f.render_widget(panel, chunks[1]);
        }
        ActiveView::Relationships => {
            let panel = Paragraph::new("Relationships view — no data loaded").block(
                Block::default()
                    .title("Relationships")
                    .borders(Borders::ALL)
                    .border_style(border_style),
            );
            f.render_widget(panel, chunks[1]);
        }
        ActiveView::Contexts => {
            let panel = Paragraph::new("Contexts view — no data loaded").block(
                Block::default()
                    .title("Contexts")
                    .borders(Borders::ALL)
                    .border_style(border_style),
            );
            f.render_widget(panel, chunks[1]);
        }
        ActiveView::Search => {
            let query = app_state.search_query.clone();
            let panel = Paragraph::new(format!("Search: {}", query)).block(
                Block::default()
                    .title("Search")
                    .borders(Borders::ALL)
                    .border_style(border_style),
            );
            f.render_widget(panel, chunks[1]);
        }
    }

    // Status bar (1 row at bottom)
    let status_text = if let Some(ref msg) = app_state.status_message {
        format!("  {}  |  Tab:next view  q:quit  t:theme", msg)
    } else {
        "  Tab:next view  q:quit  t:theme  j/k:select  g/G:top/bottom  r:refresh".to_string()
    };
    let status_bar = Paragraph::new(status_text).style(Style::default().fg(Color::Yellow));
    f.render_widget(status_bar, chunks[2]);
}

fn draw_dashboard(
    app_state: &AppState,
    border_style: Style,
    f: &mut ratatui::Frame<'_>,
    area: ratatui::layout::Rect,
) {
    let theme = app_state.theme.as_theme();
    let summary = &app_state.task_summary;

    // Split the area: summary row on top, table below
    let vert = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(5), Constraint::Min(5)])
        .split(area);

    // ── Summary row: 4 equal columns ────────────────────────────────────────
    let summary_cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ])
        .split(vert[0]);

    // Helper: build a summary stat widget with a count and label
    let make_stat = |label: &'static str, value: usize, color: Color| -> Paragraph<'static> {
        let text = format!("\n  {}", value);
        Paragraph::new(text)
            .style(Style::default().fg(color))
            .block(
                Block::default()
                    .title(label)
                    .borders(Borders::ALL)
                    .border_style(border_style),
            )
    };

    f.render_widget(
        make_stat("Total", summary.total, theme.fg()),
        summary_cols[0],
    );
    f.render_widget(
        make_stat("Todo", summary.todo, theme.status_warn()),
        summary_cols[1],
    );
    f.render_widget(
        make_stat("In Progress", summary.in_progress, theme.highlight_fg()),
        summary_cols[2],
    );
    f.render_widget(
        make_stat("Done", summary.done, theme.status_ok()),
        summary_cols[3],
    );

    // ── Recent tasks table ──────────────────────────────────────────────────
    let header_cells =
        ["ID", "Title", "Status", "Priority"].map(|h| Cell::from(h).style(theme.header_row()));
    let header = Row::new(header_cells).height(1);

    let rows: Vec<Row> = app_state
        .recent_tasks
        .iter()
        .take(10)
        .enumerate()
        .map(|(i, task)| {
            let style = if i == app_state.selected_index {
                theme.selected_row()
            } else {
                theme.normal_row()
            };
            Row::new([
                Cell::from(task.id.clone()),
                Cell::from(task.title.clone()),
                Cell::from(task.status.clone()),
                Cell::from(task.priority.clone()),
            ])
            .style(style)
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Length(10),
            Constraint::Length(42),
            Constraint::Length(12),
            Constraint::Min(8),
        ],
    )
    .header(header)
    .block(
        Block::default()
            .title("Recent Tasks")
            .borders(Borders::ALL)
            .border_style(border_style),
    )
    .row_highlight_style(theme.selected_row());

    let mut table_state = TableState::default();
    table_state.select(Some(app_state.selected_index));
    f.render_stateful_widget(table, vert[1], &mut table_state);
}

use crate::locus_integration::LocusIntegration;
use crate::locus_tui::app::{ActiveView, AppState};
#[allow(unused_imports)]
use crate::locus_tui::theme::Theme;
use crate::storage::{RelationshipStorage, Storage};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState};

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
            let theme = app_state.theme.as_theme();
            let border_style = Style::default().fg(theme.border());
            draw_tasks_view(f, chunks[1], app_state, border_style);
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

fn draw_tasks_view(
    f: &mut ratatui::Frame<'_>,
    area: ratatui::layout::Rect,
    app: &mut AppState,
    border_style: Style,
) {
    let theme = app.theme.as_theme();

    // Split area: filter bar (3) | table (flex) | help row (1)
    let vert = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(1),
        ])
        .split(area);

    // ── Filter bar ────────────────────────────────────────────────────────────
    let filter_labels = [
        ("All", None),
        ("Todo", Some("todo")),
        ("In Progress", Some("in_progress")),
        ("Done", Some("done")),
    ];

    let active_status = app.filter_status.clone();
    let chips: Vec<Span> = filter_labels
        .iter()
        .flat_map(|(label, val)| {
            let is_active = active_status.as_deref() == *val;
            let style = if is_active {
                Style::default()
                    .bg(theme.highlight_bg())
                    .fg(theme.highlight_fg())
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme.fg())
            };
            let chip = Span::styled(format!(" [{}] ", label), style);
            [chip, Span::raw(" ")]
        })
        .collect();

    let text_hint = if app.filter_text.is_empty() {
        Span::styled("  /: search", Style::default().fg(theme.border()))
    } else {
        Span::styled(
            format!("  search: \"{}\"", app.filter_text),
            Style::default().fg(theme.highlight_fg()),
        )
    };

    let mut filter_spans = chips;
    filter_spans.push(text_hint);

    let filter_bar = Paragraph::new(Line::from(filter_spans)).block(
        Block::default()
            .title("Filters")
            .borders(Borders::ALL)
            .border_style(border_style),
    );
    f.render_widget(filter_bar, vert[0]);

    // ── Task table ────────────────────────────────────────────────────────────
    let filtered = app.filtered_tasks();
    let filtered_count = filtered.len();
    let total_count = app.recent_tasks.len();

    let header_cells = ["ID", "Title", "Status", "Priority", "Created"]
        .map(|h| Cell::from(h).style(theme.header_row()));
    let header = Row::new(header_cells).height(1);

    let rows: Vec<Row> = filtered
        .iter()
        .enumerate()
        .map(|(i, task)| {
            let style = if i == app.selected_index {
                theme.selected_row()
            } else {
                theme.normal_row()
            };
            Row::new([
                Cell::from(task.id.clone()),
                Cell::from(task.title.clone()),
                Cell::from(task.status.clone()),
                Cell::from(task.priority.clone()),
                Cell::from(task.created.clone()),
            ])
            .style(style)
        })
        .collect();

    let table_title = format!("Tasks ({} / {})", filtered_count, total_count);
    let table = Table::new(
        rows,
        [
            Constraint::Length(10),
            Constraint::Min(30),
            Constraint::Length(13),
            Constraint::Length(10),
            Constraint::Length(12),
        ],
    )
    .header(header)
    .block(
        Block::default()
            .title(table_title)
            .borders(Borders::ALL)
            .border_style(border_style),
    )
    .row_highlight_style(theme.selected_row());

    let mut table_state = TableState::default();
    table_state.select(Some(app.selected_index));
    f.render_stateful_widget(table, vert[1], &mut table_state);

    // ── Help row ──────────────────────────────────────────────────────────────
    let help =
        Paragraph::new("  f: filter by status   /: search   Enter: details   Tab: next view")
            .style(Style::default().fg(Color::DarkGray));
    f.render_widget(help, vert[2]);
}

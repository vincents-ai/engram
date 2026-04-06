use crate::entities::ContextRelevance;
use crate::locus_integration::LocusIntegration;
use crate::locus_tui::app::{ActiveView, AppState, TaskDetail};
#[allow(unused_imports)]
use crate::locus_tui::theme::Theme;
use crate::storage::{RelationshipStorage, Storage};
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{
    Block, Borders, Cell, Clear, List, ListItem, Paragraph, Row, Table, TableState, Wrap,
};

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
        ActiveView::Adrs => "ADRs",
        ActiveView::Theories => "Theories",
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
            let theme = app_state.theme.as_theme();
            let border_style = Style::default().fg(theme.border());
            draw_reasoning_view(f, chunks[1], app_state, border_style);
        }
        ActiveView::Relationships => {
            let theme = app_state.theme.as_theme();
            let border_style = Style::default().fg(theme.border());
            draw_relationships_view(f, chunks[1], app_state, border_style);
        }
        ActiveView::Contexts => {
            let theme = app_state.theme.as_theme();
            let border_style = Style::default().fg(theme.border());
            draw_contexts_view(f, chunks[1], app_state, border_style);
        }
        ActiveView::Adrs => {
            let theme = app_state.theme.as_theme();
            let border_style = Style::default().fg(theme.border());
            draw_adrs_view(f, chunks[1], app_state, border_style);
        }
        ActiveView::Theories => {
            let theme = app_state.theme.as_theme();
            let border_style = Style::default().fg(theme.border());
            draw_theories_view(f, chunks[1], app_state, border_style);
        }
        ActiveView::Search => {
            let theme = app_state.theme.as_theme();
            let border_style = Style::default().fg(theme.border());
            draw_search_view(f, chunks[1], app_state, border_style);
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

    // Draw task detail overlay on top of everything (if active)
    if let Some(ref detail) = app_state.task_detail.clone() {
        draw_task_detail(f, detail, f.area());
    }
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
        Paragraph::new("  f:filter   /:search   Enter:detail   s:cycle-status   Tab:next view")
            .style(Style::default().fg(Color::DarkGray));
    f.render_widget(help, vert[2]);
}

fn draw_reasoning_view(
    f: &mut ratatui::Frame<'_>,
    area: ratatui::layout::Rect,
    app: &mut AppState,
    border_style: Style,
) {
    let theme = app.theme.as_theme();
    let nodes = app.visible_reasoning_nodes();
    let node_count = nodes.len();

    // Layout: header (3) | tree list (flex) | detail pane (6)
    let vert = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(6),
        ])
        .split(area);

    // ── Header ────────────────────────────────────────────────────────────────
    let header_text = format!(
        "Reasoning  [{} nodes]  Enter: expand/collapse  j/k: navigate",
        node_count
    );
    let header = Paragraph::new(header_text)
        .style(Style::default().fg(theme.title()))
        .block(
            Block::default()
                .title("Reasoning")
                .borders(Borders::ALL)
                .border_style(border_style),
        );
    f.render_widget(header, vert[0]);

    // ── Tree list ─────────────────────────────────────────────────────────────
    let selected = app.reasoning_selected;
    let mut list_items: Vec<ListItem> = Vec::new();

    for (i, node) in nodes.iter().enumerate() {
        let prefix = format!("{}{} ", node.indent_prefix(), node.expand_glyph());
        let row_style = if i == selected {
            theme.selected_row()
        } else {
            theme.normal_row()
        };
        let title_line = Line::from(vec![Span::styled(
            format!("{}{}", prefix, node.title.clone()),
            row_style,
        )]);

        if node.expanded {
            let preview_indent = " ".repeat(node.depth * 2 + 4);
            let preview_line = Line::from(vec![Span::styled(
                format!("{}{}", preview_indent, node.content_preview.clone()),
                Style::default()
                    .fg(theme.border())
                    .add_modifier(Modifier::DIM),
            )]);
            list_items.push(ListItem::new(vec![title_line, preview_line]));
        } else {
            list_items.push(ListItem::new(vec![title_line]));
        }
    }

    let list = List::new(list_items).block(
        Block::default()
            .title("Nodes")
            .borders(Borders::ALL)
            .border_style(border_style),
    );
    f.render_widget(list, vert[1]);

    // ── Detail pane ───────────────────────────────────────────────────────────
    let detail_text = if let Some(node) = app.reasoning_nodes.get(selected) {
        format!(
            "Title: {}\nPreview: {}\nDepth: {}  Expanded: {}  ID: {}",
            node.title, node.content_preview, node.depth, node.expanded, node.id,
        )
    } else {
        "No node selected".to_string()
    };

    let detail = Paragraph::new(detail_text)
        .style(Style::default().fg(theme.fg()))
        .block(
            Block::default()
                .title("Detail")
                .borders(Borders::ALL)
                .border_style(border_style),
        );
    f.render_widget(detail, vert[2]);
}

fn draw_relationships_view(
    f: &mut ratatui::Frame<'_>,
    area: ratatui::layout::Rect,
    app: &mut AppState,
    border_style: Style,
) {
    let theme = app.theme.as_theme();

    // Horizontal split: left 40% nodes, right 60% edges
    let horiz = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(area);

    // ── Left pane: Nodes list ────────────────────────────────────────────────
    let selected = app.relationship_selected;
    let node_items: Vec<ListItem> = app
        .relationship_nodes
        .iter()
        .enumerate()
        .map(|(i, node)| {
            let label = format!("[{}] {}", node.entity_type, node.title);
            let style = if i == selected {
                theme.selected_row()
            } else {
                theme.normal_row()
            };
            ListItem::new(Line::from(vec![Span::styled(label, style)]))
        })
        .collect();

    let nodes_list = List::new(node_items).block(
        Block::default()
            .title("Nodes")
            .borders(Borders::ALL)
            .border_style(border_style),
    );
    f.render_widget(nodes_list, horiz[0]);

    // ── Right pane: Edges + help bar ─────────────────────────────────────────
    let right_vert = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3), Constraint::Length(1)])
        .split(horiz[1]);

    let selected_title = app
        .relationship_nodes
        .get(selected)
        .map(|n| n.title.as_str())
        .unwrap_or("none");

    let edges_title = format!("Edges from: {}", selected_title);

    let edge_items: Vec<ListItem> = if let Some(node) = app.relationship_nodes.get(selected) {
        if node.edges.is_empty() {
            vec![ListItem::new(Line::from(vec![Span::styled(
                "No outgoing edges",
                Style::default().fg(theme.border()),
            )]))]
        } else {
            node.edges
                .iter()
                .map(|edge| {
                    let label = format!("──[{}]──▶ {}", edge.relationship_type, edge.to_title);
                    ListItem::new(Line::from(vec![Span::styled(label, theme.normal_row())]))
                })
                .collect()
        }
    } else {
        vec![ListItem::new(Line::from(vec![Span::styled(
            "No outgoing edges",
            Style::default().fg(theme.border()),
        )]))]
    };

    let edges_list = List::new(edge_items).block(
        Block::default()
            .title(edges_title)
            .borders(Borders::ALL)
            .border_style(border_style),
    );
    f.render_widget(edges_list, right_vert[0]);

    let help = Paragraph::new("  j/k: navigate nodes   Tab: next view")
        .style(Style::default().fg(Color::DarkGray));
    f.render_widget(help, right_vert[1]);
}

fn draw_contexts_view(
    f: &mut ratatui::Frame<'_>,
    area: Rect,
    app: &mut AppState,
    border_style: Style,
) {
    let theme = app.theme.as_theme();

    // Layout: list (flex) | detail pane (8) | help (1)
    let vert = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(5),
            Constraint::Length(8),
            Constraint::Length(1),
        ])
        .split(area);

    // ── Context list ──────────────────────────────────────────────────────────
    let selected = app.contexts_selected;
    let header_cells = ["Title", "Relevance", "Source", "Created"]
        .map(|h| Cell::from(h).style(theme.header_row()));
    let header = Row::new(header_cells).height(1);

    let rows: Vec<Row> = app
        .contexts
        .iter()
        .enumerate()
        .map(|(i, ctx)| {
            let style = if i == selected {
                theme.selected_row()
            } else {
                theme.normal_row()
            };
            let relevance = match ctx.relevance {
                ContextRelevance::Low => "low",
                ContextRelevance::Medium => "medium",
                ContextRelevance::High => "high",
                ContextRelevance::Critical => "critical",
            };
            Row::new([
                Cell::from(ctx.title.clone()),
                Cell::from(relevance),
                Cell::from(ctx.source.clone()),
                Cell::from(ctx.created_at.format("%Y-%m-%d").to_string()),
            ])
            .style(style)
        })
        .collect();

    let table_title = format!("Contexts ({})", app.contexts.len());
    let table = Table::new(
        rows,
        [
            Constraint::Min(30),
            Constraint::Length(10),
            Constraint::Length(20),
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
    table_state.select(Some(selected));
    f.render_stateful_widget(table, vert[0], &mut table_state);

    // ── Detail pane: selected context content ────────────────────────────────
    let detail_text = app
        .contexts
        .get(selected)
        .map(|ctx| ctx.content.clone())
        .unwrap_or_else(|| "No context selected".to_string());

    let detail = Paragraph::new(detail_text)
        .wrap(Wrap { trim: true })
        .style(Style::default().fg(theme.fg()))
        .block(
            Block::default()
                .title("Content")
                .borders(Borders::ALL)
                .border_style(border_style),
        );
    f.render_widget(detail, vert[1]);

    // ── Help row ─────────────────────────────────────────────────────────────
    let help = Paragraph::new("  j/k: navigate   Tab: next view")
        .style(Style::default().fg(Color::DarkGray));
    f.render_widget(help, vert[2]);
}

fn draw_search_view(
    f: &mut ratatui::Frame<'_>,
    area: Rect,
    app: &mut AppState,
    border_style: Style,
) {
    let theme = app.theme.as_theme();

    // Layout: input bar (3) | results (flex) | help (1)
    let vert = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(1),
        ])
        .split(area);

    // ── Input bar ─────────────────────────────────────────────────────────────
    let cursor = if app.search_mode { "_" } else { "" };
    let input_text = format!("Search: {}{}", app.search_query, cursor);
    let input_style = if app.search_mode {
        Style::default()
            .fg(theme.highlight_fg())
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(theme.fg())
    };
    let input_bar = Paragraph::new(input_text).style(input_style).block(
        Block::default()
            .title("Search")
            .borders(Borders::ALL)
            .border_style(border_style),
    );
    f.render_widget(input_bar, vert[0]);

    // ── Results list ──────────────────────────────────────────────────────────
    let result_items: Vec<ListItem> = if app.search_results.is_empty() {
        if app.search_query.is_empty() {
            vec![ListItem::new(Line::from(vec![Span::styled(
                "Press / to enter search mode, type query, Enter to search",
                Style::default().fg(theme.border()),
            )]))]
        } else {
            vec![ListItem::new(Line::from(vec![Span::styled(
                "No results found",
                Style::default().fg(theme.border()),
            )]))]
        }
    } else {
        app.search_results
            .iter()
            .map(|r| {
                let label = format!("[{}]  {}  —  {}", r.entity_type, r.title, r.preview);
                ListItem::new(Line::from(vec![Span::styled(label, theme.normal_row())]))
            })
            .collect()
    };

    let results_title = format!("Results ({})", app.search_results.len());
    let results_list = List::new(result_items).block(
        Block::default()
            .title(results_title)
            .borders(Borders::ALL)
            .border_style(border_style),
    );
    f.render_widget(results_list, vert[1]);

    // ── Help row ─────────────────────────────────────────────────────────────
    let help_text = if app.search_mode {
        "  type to search   Enter:confirm   Esc:exit search mode"
    } else {
        "  /:enter search   Tab:next view"
    };
    let help = Paragraph::new(help_text).style(Style::default().fg(Color::DarkGray));
    f.render_widget(help, vert[2]);
}

fn draw_adrs_view(f: &mut ratatui::Frame<'_>, area: Rect, app: &mut AppState, border_style: Style) {
    use crate::entities::AdrStatus;
    let theme = app.theme.as_theme();

    // Layout: list (flex) | detail pane (10) | help (1)
    let vert = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(5),
            Constraint::Length(10),
            Constraint::Length(1),
        ])
        .split(area);

    // ── ADR list ──────────────────────────────────────────────────────────────
    let selected = app.adrs_selected;
    let header_cells = ["#", "Title", "Status", "Agent", "Created"]
        .map(|h| Cell::from(h).style(theme.header_row()));
    let header = Row::new(header_cells).height(1);

    let rows: Vec<Row> = app
        .all_adrs
        .iter()
        .enumerate()
        .map(|(i, adr)| {
            let style = if i == selected {
                theme.selected_row()
            } else {
                theme.normal_row()
            };
            let status = match adr.status {
                AdrStatus::Proposed => "proposed",
                AdrStatus::Accepted => "accepted",
                AdrStatus::Deprecated => "deprecated",
                AdrStatus::Superseded => "superseded",
            };
            Row::new([
                Cell::from(adr.number.to_string()),
                Cell::from(adr.title.clone()),
                Cell::from(status),
                Cell::from(adr.agent.chars().take(20).collect::<String>()),
                Cell::from(adr.created_at.format("%Y-%m-%d").to_string()),
            ])
            .style(style)
        })
        .collect();

    let table_title = format!("ADRs ({})", app.all_adrs.len());
    let table = Table::new(
        rows,
        [
            Constraint::Length(5),
            Constraint::Min(30),
            Constraint::Length(12),
            Constraint::Length(20),
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
    table_state.select(Some(selected));
    f.render_stateful_widget(table, vert[0], &mut table_state);

    // ── Detail pane: selected ADR context + decision ──────────────────────────
    let detail_text = app
        .all_adrs
        .get(selected)
        .map(|adr| {
            let decision = if adr.decision.is_empty() {
                "(none yet)".to_string()
            } else {
                adr.decision.clone()
            };
            let consequences = if adr.consequences.is_empty() {
                "(none yet)".to_string()
            } else {
                adr.consequences.clone()
            };
            format!(
                "Context:\n{}\n\nDecision:\n{}\n\nConsequences:\n{}",
                adr.context, decision, consequences,
            )
        })
        .unwrap_or_else(|| "No ADR selected".to_string());

    let detail = Paragraph::new(detail_text)
        .wrap(Wrap { trim: true })
        .style(Style::default().fg(theme.fg()))
        .block(
            Block::default()
                .title("Detail")
                .borders(Borders::ALL)
                .border_style(border_style),
        );
    f.render_widget(detail, vert[1]);

    // ── Help row ─────────────────────────────────────────────────────────────
    let help = Paragraph::new("  j/k: navigate   Tab: next view")
        .style(Style::default().fg(Color::DarkGray));
    f.render_widget(help, vert[2]);
}

fn draw_theories_view(
    f: &mut ratatui::Frame<'_>,
    area: Rect,
    app: &mut AppState,
    border_style: Style,
) {
    let theme = app.theme.as_theme();

    // Horizontal split: left 40% list, right 60% detail
    let horiz = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(area);

    // ── Left pane: Theory list ────────────────────────────────────────────────
    let selected = app.theories_selected;
    let theory_items: Vec<ListItem> = app
        .all_theories
        .iter()
        .enumerate()
        .map(|(i, theory)| {
            let label = format!("{} (iter: {})", theory.domain_name, theory.iteration_count);
            let style = if i == selected {
                theme.selected_row()
            } else {
                theme.normal_row()
            };
            ListItem::new(Line::from(vec![Span::styled(label, style)]))
        })
        .collect();

    let list_title = format!("Theories ({})", app.all_theories.len());
    let theory_list = List::new(theory_items).block(
        Block::default()
            .title(list_title)
            .borders(Borders::ALL)
            .border_style(border_style),
    );
    f.render_widget(theory_list, horiz[0]);

    // ── Right pane: Detail + help ─────────────────────────────────────────────
    let right_vert = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3), Constraint::Length(1)])
        .split(horiz[1]);

    let detail_text = app
        .all_theories
        .get(selected)
        .map(|theory| {
            let mut lines = vec![
                format!("Domain:    {}", theory.domain_name),
                format!("Agent:     {}", theory.agent),
                format!("Iteration: {}", theory.iteration_count),
                format!("Updated:   {}", theory.last_updated.format("%Y-%m-%d")),
                String::new(),
            ];

            if !theory.conceptual_model.is_empty() {
                lines.push("Conceptual Model:".to_string());
                for (k, v) in &theory.conceptual_model {
                    lines.push(format!("  {}: {}", k, v));
                }
                lines.push(String::new());
            }

            if !theory.design_rationale.is_empty() {
                lines.push("Design Rationale:".to_string());
                for (k, v) in &theory.design_rationale {
                    lines.push(format!("  {}: {}", k, v));
                }
                lines.push(String::new());
            }

            if !theory.invariants.is_empty() {
                lines.push("Invariants:".to_string());
                for inv in &theory.invariants {
                    lines.push(format!("  - {}", inv));
                }
            }

            lines.join("\n")
        })
        .unwrap_or_else(|| "No theory selected".to_string());

    let detail = Paragraph::new(detail_text)
        .wrap(Wrap { trim: true })
        .style(Style::default().fg(theme.fg()))
        .block(
            Block::default()
                .title("Detail")
                .borders(Borders::ALL)
                .border_style(border_style),
        );
    f.render_widget(detail, right_vert[0]);

    let help = Paragraph::new("  j/k: navigate   Tab: next view")
        .style(Style::default().fg(Color::DarkGray));
    f.render_widget(help, right_vert[1]);
}

/// Render the task detail modal overlay centered on the screen.
fn draw_task_detail(f: &mut ratatui::Frame<'_>, detail: &TaskDetail, area: Rect) {
    // Centre a 70%×70% modal
    let modal_area = centered_rect(70, 80, area);

    // Clear the background area first
    f.render_widget(Clear, modal_area);

    let tags_str = if detail.tags.is_empty() {
        "(none)".to_string()
    } else {
        detail.tags.join(", ")
    };
    let outcome_str = detail.outcome.as_deref().unwrap_or("(none)");

    let text = format!(
        "ID:          {}\nTitle:       {}\nStatus:      {}\nPriority:    {}\nAgent:       {}\nCreated:     {}\nTags:        {}\nOutcome:     {}\n\nDescription:\n{}",
        detail.id,
        detail.title,
        detail.status,
        detail.priority,
        detail.agent,
        detail.created,
        tags_str,
        outcome_str,
        detail.description,
    );

    let modal = Paragraph::new(text)
        .wrap(Wrap { trim: true })
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .title("Task Detail  (Esc to close)")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        );

    f.render_widget(modal, modal_area);
}

/// Helper: return a rectangle centred within `r` with the given width/height percentages.
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

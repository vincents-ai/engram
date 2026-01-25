use prettytable::{format, Table};

/// Create a standard table format for CLI output
pub fn create_table() -> Table {
    let mut table = Table::new();
    let format = format::FormatBuilder::new()
        .column_separator('│')
        .borders('│')
        .separator(
            format::LinePosition::Top,
            format::LineSeparator::new('─', '─', '╭', '╮'),
        )
        .separator(
            format::LinePosition::Bottom,
            format::LineSeparator::new('─', '─', '╰', '╯'),
        )
        .separator(
            format::LinePosition::Title,
            format::LineSeparator::new('─', '─', '├', '┤'),
        )
        .padding(1, 1)
        .build();
    table.set_format(format);
    table
}

/// Truncate string to a maximum length with ellipsis
pub fn truncate(s: &str, max_len: usize) -> String {
    if s.len() > max_len {
        format!("{}...", &s[..max_len - 3])
    } else {
        s.to_string()
    }
}

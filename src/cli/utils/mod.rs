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
    if max_len < 4 || s.len() <= max_len {
        return s.to_string();
    }
    let mut end = max_len - 3;
    while end > 0 && !s.is_char_boundary(end) {
        end -= 1;
    }
    if end == 0 {
        s.to_string()
    } else {
        format!("{}...", &s[..end])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncate_ascii() {
        assert_eq!(truncate("hello world", 8), "hello...");
    }

    #[test]
    fn truncate_short_string_unchanged() {
        assert_eq!(truncate("hi", 10), "hi");
    }

    #[test]
    fn truncate_empty_string() {
        assert_eq!(truncate("", 5), "");
    }

    #[test]
    fn truncate_max_len_less_than_3() {
        assert_eq!(truncate("hello", 2), "hello");
    }

    #[test]
    fn truncate_exact_length() {
        assert_eq!(truncate("hello", 5), "hello");
    }

    #[test]
    fn truncate_multibyte_char_at_boundary() {
        let s = "Gap analysis: Accessibility domain — 2 covered, 8 gaps (A1-A8)";
        let result = truncate(s, 40);
        assert!(result.ends_with("..."));
        assert!(result.len() <= 40);
    }

    #[test]
    fn truncate_emoji_at_slice_point() {
        let s = "Hello 🌍 World! This is a longer string for testing";
        let result = truncate(s, 20);
        assert!(result.ends_with("..."));
        assert!(result.len() <= 20);
    }

    #[test]
    fn truncate_cjk_characters() {
        let s = "日本語のテスト文字列です";
        let result = truncate(s, 15);
        assert!(result.ends_with("..."));
        assert!(result.len() <= 15);
        assert!(s.is_char_boundary(result.len() - 3));
    }

    #[test]
    fn truncate_all_multibyte() {
        let s = "——";
        assert_eq!(truncate(s, 3), "——");
    }

    #[test]
    fn truncate_preserves_valid_utf8() {
        let s = "abcdé—fgh";
        let result = truncate(s, 7);
        let bytes = result.as_bytes();
        assert!(std::str::from_utf8(bytes).is_ok());
    }
}

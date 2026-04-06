use ratatui::style::{Color, Modifier, Style};

/// Core colour/style contract every theme must fulfil.
pub trait Theme {
    fn bg(&self) -> Color;
    fn fg(&self) -> Color;
    fn highlight_bg(&self) -> Color;
    fn highlight_fg(&self) -> Color;
    fn border(&self) -> Color;
    fn title(&self) -> Color;
    fn status_ok(&self) -> Color;
    fn status_warn(&self) -> Color;
    fn status_err(&self) -> Color;
    fn tab_active(&self) -> Style;
    fn tab_inactive(&self) -> Style;
    fn selected_row(&self) -> Style;
    fn normal_row(&self) -> Style;
    fn header_row(&self) -> Style;
}

// ---------------------------------------------------------------------------
// Dark theme
// ---------------------------------------------------------------------------

pub struct DarkTheme;

impl Theme for DarkTheme {
    fn bg(&self) -> Color {
        Color::Black
    }
    fn fg(&self) -> Color {
        Color::White
    }
    fn highlight_bg(&self) -> Color {
        Color::DarkGray
    }
    fn highlight_fg(&self) -> Color {
        Color::Cyan
    }
    fn border(&self) -> Color {
        Color::DarkGray
    }
    fn title(&self) -> Color {
        Color::Cyan
    }
    fn status_ok(&self) -> Color {
        Color::Green
    }
    fn status_warn(&self) -> Color {
        Color::Yellow
    }
    fn status_err(&self) -> Color {
        Color::Red
    }
    fn tab_active(&self) -> Style {
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    }
    fn tab_inactive(&self) -> Style {
        Style::default().fg(Color::DarkGray)
    }
    fn selected_row(&self) -> Style {
        Style::default()
            .bg(Color::DarkGray)
            .fg(Color::White)
            .add_modifier(Modifier::BOLD)
    }
    fn normal_row(&self) -> Style {
        Style::default().fg(Color::White)
    }
    fn header_row(&self) -> Style {
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::UNDERLINED)
    }
}

// ---------------------------------------------------------------------------
// Light theme
// ---------------------------------------------------------------------------

pub struct LightTheme;

impl Theme for LightTheme {
    fn bg(&self) -> Color {
        Color::White
    }
    fn fg(&self) -> Color {
        Color::Black
    }
    fn highlight_bg(&self) -> Color {
        Color::Gray
    }
    fn highlight_fg(&self) -> Color {
        Color::Blue
    }
    fn border(&self) -> Color {
        Color::Gray
    }
    fn title(&self) -> Color {
        Color::Blue
    }
    fn status_ok(&self) -> Color {
        Color::Green
    }
    fn status_warn(&self) -> Color {
        Color::Rgb(200, 120, 0)
    }
    fn status_err(&self) -> Color {
        Color::Red
    }
    fn tab_active(&self) -> Style {
        Style::default()
            .fg(Color::Blue)
            .add_modifier(Modifier::BOLD)
    }
    fn tab_inactive(&self) -> Style {
        Style::default().fg(Color::Gray)
    }
    fn selected_row(&self) -> Style {
        Style::default()
            .bg(Color::Gray)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD)
    }
    fn normal_row(&self) -> Style {
        Style::default().fg(Color::Black)
    }
    fn header_row(&self) -> Style {
        Style::default()
            .fg(Color::Blue)
            .add_modifier(Modifier::UNDERLINED)
    }
}

// ---------------------------------------------------------------------------
// AppTheme — boxed-like enum that can be stored in AppState
// ---------------------------------------------------------------------------

pub enum AppTheme {
    Dark(DarkTheme),
    Light(LightTheme),
}

impl AppTheme {
    pub fn dark() -> Self {
        AppTheme::Dark(DarkTheme)
    }

    pub fn light() -> Self {
        AppTheme::Light(LightTheme)
    }

    pub fn as_theme(&self) -> &dyn Theme {
        match self {
            AppTheme::Dark(t) => t,
            AppTheme::Light(t) => t,
        }
    }

    /// Toggle between dark and light.
    pub fn toggle(&self) -> Self {
        match self {
            AppTheme::Dark(_) => AppTheme::light(),
            AppTheme::Light(_) => AppTheme::dark(),
        }
    }
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: assert every Theme method returns without panicking and the
    /// returned Color/Style values are the expected concrete types.  We call
    /// every method so that if a future implementation accidentally panics we
    /// catch it here.
    fn smoke_theme(theme: &dyn Theme) {
        let _ = theme.bg();
        let _ = theme.fg();
        let _ = theme.highlight_bg();
        let _ = theme.highlight_fg();
        let _ = theme.border();
        let _ = theme.title();
        let _ = theme.status_ok();
        let _ = theme.status_warn();
        let _ = theme.status_err();
        let _ = theme.tab_active();
        let _ = theme.tab_inactive();
        let _ = theme.selected_row();
        let _ = theme.normal_row();
        let _ = theme.header_row();
    }

    #[test]
    fn test_dark_theme_implements_theme() {
        let t = DarkTheme;
        smoke_theme(&t);
        // Spot-check a few concrete values
        assert_eq!(t.bg(), Color::Black);
        assert_eq!(t.fg(), Color::White);
        assert_eq!(t.border(), Color::DarkGray);
        assert_eq!(t.title(), Color::Cyan);
        assert_eq!(t.status_ok(), Color::Green);
        assert_eq!(t.status_err(), Color::Red);
    }

    #[test]
    fn test_dark_theme_styles_carry_modifiers() {
        let t = DarkTheme;
        let active = t.tab_active();
        assert!(active.add_modifier.contains(Modifier::BOLD));
        let header = t.header_row();
        assert!(header.add_modifier.contains(Modifier::UNDERLINED));
        let sel = t.selected_row();
        assert!(sel.add_modifier.contains(Modifier::BOLD));
    }

    #[test]
    fn test_light_theme_implements_theme() {
        let t = LightTheme;
        smoke_theme(&t);
        // Spot-check a few concrete values
        assert_eq!(t.bg(), Color::White);
        assert_eq!(t.fg(), Color::Black);
        assert_eq!(t.border(), Color::Gray);
        assert_eq!(t.title(), Color::Blue);
        assert_eq!(t.status_ok(), Color::Green);
        assert_eq!(t.status_err(), Color::Red);
    }

    #[test]
    fn test_light_theme_styles_carry_modifiers() {
        let t = LightTheme;
        let active = t.tab_active();
        assert!(active.add_modifier.contains(Modifier::BOLD));
        let header = t.header_row();
        assert!(header.add_modifier.contains(Modifier::UNDERLINED));
        let sel = t.selected_row();
        assert!(sel.add_modifier.contains(Modifier::BOLD));
    }

    #[test]
    fn test_app_theme_toggle_dark_to_light() {
        let dark = AppTheme::dark();
        // Verify it is dark first
        assert!(matches!(dark, AppTheme::Dark(_)));
        let light = dark.toggle();
        assert!(matches!(light, AppTheme::Light(_)));
    }

    #[test]
    fn test_app_theme_toggle_light_to_dark() {
        let light = AppTheme::light();
        assert!(matches!(light, AppTheme::Light(_)));
        let dark = light.toggle();
        assert!(matches!(dark, AppTheme::Dark(_)));
    }

    #[test]
    fn test_app_theme_toggle_round_trip() {
        let original = AppTheme::dark();
        let toggled = original.toggle().toggle();
        assert!(matches!(toggled, AppTheme::Dark(_)));
    }

    #[test]
    fn test_app_theme_as_theme_dark() {
        let at = AppTheme::dark();
        let t = at.as_theme();
        // Should behave like DarkTheme
        assert_eq!(t.bg(), Color::Black);
        assert_eq!(t.border(), Color::DarkGray);
    }

    #[test]
    fn test_app_theme_as_theme_light() {
        let at = AppTheme::light();
        let t = at.as_theme();
        // Should behave like LightTheme
        assert_eq!(t.bg(), Color::White);
        assert_eq!(t.border(), Color::Gray);
    }
}

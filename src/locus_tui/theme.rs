use ratatui::style::{Color, Modifier, Style};

/// Core colour/style contract every theme must fulfil.
pub trait Theme {
    fn bg(&self) -> Color;
    fn fg(&self) -> Color;
    fn highlight_bg(&self) -> Color;
    fn highlight_fg(&self) -> Color;
    fn border(&self) -> Color;
    fn border_focused(&self) -> Color;
    fn title(&self) -> Color;
    fn status_ok(&self) -> Color;
    fn status_warn(&self) -> Color;
    fn status_err(&self) -> Color;
    fn tab_active(&self) -> Style;
    fn tab_inactive(&self) -> Style;
    fn selected_row(&self) -> Style;
    fn normal_row(&self) -> Style;
    fn header_row(&self) -> Style;
    fn dim(&self) -> Style;
    fn badge_todo(&self) -> Style;
    fn badge_in_progress(&self) -> Style;
    fn badge_done(&self) -> Style;
    fn badge_high(&self) -> Style;
    fn badge_medium(&self) -> Style;
    fn badge_low(&self) -> Style;
}

// ---------------------------------------------------------------------------
// Dark theme — k9s-inspired deep blue-grey palette
// ---------------------------------------------------------------------------

pub struct DarkTheme;

// Palette constants
const BG: Color = Color::Rgb(13, 17, 23); // GitHub dark bg
const BG_PANEL: Color = Color::Rgb(22, 27, 34); // panel bg
const BG_SEL: Color = Color::Rgb(31, 111, 235); // bright selection blue
const FG: Color = Color::Rgb(201, 209, 217); // default text
const FG_DIM: Color = Color::Rgb(110, 118, 129); // muted text
const CYAN: Color = Color::Rgb(121, 192, 255); // titles / accents
const CYAN_BRIGHT: Color = Color::Rgb(56, 189, 248); // focused border
const GREEN: Color = Color::Rgb(63, 185, 80); // ok / done
const YELLOW: Color = Color::Rgb(210, 153, 34); // warn / todo
const ORANGE: Color = Color::Rgb(255, 123, 114); // in-progress
const RED: Color = Color::Rgb(248, 81, 73); // error
const PURPLE: Color = Color::Rgb(188, 140, 255); // high priority
const BORDER: Color = Color::Rgb(48, 54, 61); // subtle border

impl Theme for DarkTheme {
    fn bg(&self) -> Color {
        BG
    }
    fn fg(&self) -> Color {
        FG
    }
    fn highlight_bg(&self) -> Color {
        BG_SEL
    }
    fn highlight_fg(&self) -> Color {
        Color::Rgb(255, 255, 255)
    }
    fn border(&self) -> Color {
        BORDER
    }
    fn border_focused(&self) -> Color {
        CYAN_BRIGHT
    }
    fn title(&self) -> Color {
        CYAN
    }
    fn status_ok(&self) -> Color {
        GREEN
    }
    fn status_warn(&self) -> Color {
        YELLOW
    }
    fn status_err(&self) -> Color {
        RED
    }

    fn tab_active(&self) -> Style {
        Style::default()
            .fg(CYAN_BRIGHT)
            .add_modifier(Modifier::BOLD)
    }
    fn tab_inactive(&self) -> Style {
        Style::default().fg(FG_DIM)
    }
    fn selected_row(&self) -> Style {
        Style::default()
            .bg(BG_SEL)
            .fg(Color::Rgb(255, 255, 255))
            .add_modifier(Modifier::BOLD)
    }
    fn normal_row(&self) -> Style {
        Style::default().fg(FG)
    }
    fn header_row(&self) -> Style {
        Style::default()
            .fg(CYAN)
            .bg(BG_PANEL)
            .add_modifier(Modifier::BOLD)
    }
    fn dim(&self) -> Style {
        Style::default().fg(FG_DIM)
    }
    fn badge_todo(&self) -> Style {
        Style::default().fg(YELLOW).add_modifier(Modifier::BOLD)
    }
    fn badge_in_progress(&self) -> Style {
        Style::default().fg(ORANGE).add_modifier(Modifier::BOLD)
    }
    fn badge_done(&self) -> Style {
        Style::default().fg(GREEN).add_modifier(Modifier::BOLD)
    }
    fn badge_high(&self) -> Style {
        Style::default().fg(PURPLE).add_modifier(Modifier::BOLD)
    }
    fn badge_medium(&self) -> Style {
        Style::default().fg(CYAN)
    }
    fn badge_low(&self) -> Style {
        Style::default().fg(FG_DIM)
    }
}

// ---------------------------------------------------------------------------
// Light theme — high-contrast light palette
// ---------------------------------------------------------------------------

pub struct LightTheme;

const LBG: Color = Color::Rgb(255, 255, 255);
const LBG_PANEL: Color = Color::Rgb(246, 248, 250);
const LBG_SEL: Color = Color::Rgb(0, 92, 197);
const LFG: Color = Color::Rgb(36, 41, 47);
const LFG_DIM: Color = Color::Rgb(110, 119, 129);
const LBLUE: Color = Color::Rgb(0, 70, 174);
const LBLUE_BRIGHT: Color = Color::Rgb(9, 105, 218);
const LGREEN: Color = Color::Rgb(26, 127, 55);
const LYELLOW: Color = Color::Rgb(154, 103, 0);
const LORANGE: Color = Color::Rgb(207, 34, 46);
const LRED: Color = Color::Rgb(185, 28, 28);
const LPURPLE: Color = Color::Rgb(130, 80, 223);
const LBORDER: Color = Color::Rgb(208, 215, 222);

impl Theme for LightTheme {
    fn bg(&self) -> Color {
        LBG
    }
    fn fg(&self) -> Color {
        LFG
    }
    fn highlight_bg(&self) -> Color {
        LBG_SEL
    }
    fn highlight_fg(&self) -> Color {
        Color::Rgb(255, 255, 255)
    }
    fn border(&self) -> Color {
        LBORDER
    }
    fn border_focused(&self) -> Color {
        LBLUE_BRIGHT
    }
    fn title(&self) -> Color {
        LBLUE
    }
    fn status_ok(&self) -> Color {
        LGREEN
    }
    fn status_warn(&self) -> Color {
        LYELLOW
    }
    fn status_err(&self) -> Color {
        LRED
    }

    fn tab_active(&self) -> Style {
        Style::default()
            .fg(LBLUE_BRIGHT)
            .add_modifier(Modifier::BOLD)
    }
    fn tab_inactive(&self) -> Style {
        Style::default().fg(LFG_DIM)
    }
    fn selected_row(&self) -> Style {
        Style::default()
            .bg(LBG_SEL)
            .fg(Color::Rgb(255, 255, 255))
            .add_modifier(Modifier::BOLD)
    }
    fn normal_row(&self) -> Style {
        Style::default().fg(LFG)
    }
    fn header_row(&self) -> Style {
        Style::default()
            .fg(LBLUE)
            .bg(LBG_PANEL)
            .add_modifier(Modifier::BOLD)
    }
    fn dim(&self) -> Style {
        Style::default().fg(LFG_DIM)
    }
    fn badge_todo(&self) -> Style {
        Style::default().fg(LYELLOW).add_modifier(Modifier::BOLD)
    }
    fn badge_in_progress(&self) -> Style {
        Style::default().fg(LORANGE).add_modifier(Modifier::BOLD)
    }
    fn badge_done(&self) -> Style {
        Style::default().fg(LGREEN).add_modifier(Modifier::BOLD)
    }
    fn badge_high(&self) -> Style {
        Style::default().fg(LPURPLE).add_modifier(Modifier::BOLD)
    }
    fn badge_medium(&self) -> Style {
        Style::default().fg(LBLUE)
    }
    fn badge_low(&self) -> Style {
        Style::default().fg(LFG_DIM)
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

    fn smoke_theme(theme: &dyn Theme) {
        let _ = theme.bg();
        let _ = theme.fg();
        let _ = theme.highlight_bg();
        let _ = theme.highlight_fg();
        let _ = theme.border();
        let _ = theme.border_focused();
        let _ = theme.title();
        let _ = theme.status_ok();
        let _ = theme.status_warn();
        let _ = theme.status_err();
        let _ = theme.tab_active();
        let _ = theme.tab_inactive();
        let _ = theme.selected_row();
        let _ = theme.normal_row();
        let _ = theme.header_row();
        let _ = theme.dim();
        let _ = theme.badge_todo();
        let _ = theme.badge_in_progress();
        let _ = theme.badge_done();
        let _ = theme.badge_high();
        let _ = theme.badge_medium();
        let _ = theme.badge_low();
    }

    #[test]
    fn test_dark_theme_implements_theme() {
        let t = DarkTheme;
        smoke_theme(&t);
        assert_eq!(t.bg(), BG);
        assert_eq!(t.border(), BORDER);
        assert_eq!(t.title(), CYAN);
        assert_eq!(t.status_ok(), GREEN);
        assert_eq!(t.status_err(), RED);
    }

    #[test]
    fn test_dark_theme_styles_carry_modifiers() {
        let t = DarkTheme;
        let active = t.tab_active();
        assert!(active.add_modifier.contains(Modifier::BOLD));
        let header = t.header_row();
        assert!(header.add_modifier.contains(Modifier::BOLD));
        let sel = t.selected_row();
        assert!(sel.add_modifier.contains(Modifier::BOLD));
    }

    #[test]
    fn test_light_theme_implements_theme() {
        let t = LightTheme;
        smoke_theme(&t);
        assert_eq!(t.bg(), LBG);
        assert_eq!(t.border(), LBORDER);
        assert_eq!(t.title(), LBLUE);
        assert_eq!(t.status_ok(), LGREEN);
        assert_eq!(t.status_err(), LRED);
    }

    #[test]
    fn test_light_theme_styles_carry_modifiers() {
        let t = LightTheme;
        let active = t.tab_active();
        assert!(active.add_modifier.contains(Modifier::BOLD));
        let header = t.header_row();
        assert!(header.add_modifier.contains(Modifier::BOLD));
        let sel = t.selected_row();
        assert!(sel.add_modifier.contains(Modifier::BOLD));
    }

    #[test]
    fn test_app_theme_toggle_dark_to_light() {
        let dark = AppTheme::dark();
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
        assert_eq!(t.bg(), BG);
        assert_eq!(t.border(), BORDER);
    }

    #[test]
    fn test_app_theme_as_theme_light() {
        let at = AppTheme::light();
        let t = at.as_theme();
        assert_eq!(t.bg(), LBG);
        assert_eq!(t.border(), LBORDER);
    }
}

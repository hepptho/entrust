use ratatui::prelude::{Style, Stylize};
use std::borrow::Cow;
use std::ops::Deref;
use std::sync::LazyLock;

#[derive(Clone, Debug)]
pub struct Theme {
    pub cursor_on_style: Style,
    pub cursor_off_style: Style,
    pub header_style: Style,
    pub select_indicator: String,
    pub placeholder_style: Style,
    pub prompt_style: Style,
    pub selected_style: Style,
    pub match_style: Style,
    pub completion_style: Style,
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            cursor_on_style: Style::new().black().on_light_blue(),
            cursor_off_style: Style::new(),
            header_style: Style::new().light_blue().bold(),
            placeholder_style: Style::new().dim(),
            prompt_style: Style::new().light_yellow().bold(),
            select_indicator: "• ".to_string(),
            selected_style: Style::new().bold(),
            match_style: Style::new().light_blue().underlined(),
            completion_style: Style::new().dim().italic(),
        }
    }
}

impl Default for &'static Theme {
    fn default() -> Self {
        Theme::default_ref()
    }
}

impl Theme {
    pub fn default_ref() -> &'static Theme {
        static THEME: LazyLock<Theme> = LazyLock::new(Theme::default);
        THEME.deref()
    }
}

impl From<Theme> for Cow<'static, Theme> {
    fn from(value: Theme) -> Self {
        Cow::Owned(value)
    }
}

impl<'a> From<&'a Theme> for Cow<'a, Theme> {
    fn from(value: &'a Theme) -> Self {
        Cow::Borrowed(value)
    }
}

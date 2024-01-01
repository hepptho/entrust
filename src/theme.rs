use clap::builder::styling::AnsiColor;
use clap::builder::Styles;
use inquire::ui::{
    Attributes, Color, ErrorMessageRenderConfig, IndexPrefix, RenderConfig, StyleSheet, Styled,
};
use once_cell::sync::Lazy;

pub static INQUIRE_RENDER_CONFIG: Lazy<RenderConfig> = Lazy::new(|| RenderConfig {
    prompt_prefix: Styled::new("?")
        .with_fg(Color::rgb(255, 184, 108))
        .with_attr(Attributes::BOLD),
    answered_prompt_prefix: Styled::new("❯").with_fg(Color::LightYellow),
    prompt: StyleSheet::new().with_fg(Color::LightBlue),
    default_value: StyleSheet::empty(),
    placeholder: StyleSheet::new().with_fg(Color::DarkGrey),
    help_message: StyleSheet::empty().with_fg(Color::DarkGrey),
    text_input: StyleSheet::empty(),
    error_message: ErrorMessageRenderConfig::default_colored(),
    password_mask: '*',
    answer: StyleSheet::empty().with_fg(Color::LightYellow),
    canceled_prompt_indicator: Styled::new("<canceled>").with_fg(Color::DarkRed),
    highlighted_option_prefix: Styled::new("❯").with_fg(Color::LightYellow),
    scroll_up_prefix: Styled::new("^"),
    scroll_down_prefix: Styled::new("v"),
    selected_checkbox: Styled::new("[x]").with_fg(Color::LightGreen),
    unselected_checkbox: Styled::new("[ ]"),
    option_index_prefix: IndexPrefix::None,
    option: StyleSheet::empty(),
    selected_option: Some(
        StyleSheet::new()
            .with_fg(Color::LightYellow)
            .with_attr(Attributes::BOLD),
    ),
});

pub fn clap_theme() -> Styles {
    Styles::styled()
        .usage(AnsiColor::BrightYellow.on_default().bold().underline())
        .header(AnsiColor::BrightYellow.on_default().bold())
        .literal(AnsiColor::BrightBlue.on_default().italic())
        .placeholder(AnsiColor::BrightBlack.on_default())
}

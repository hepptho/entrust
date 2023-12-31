use clap::builder::styling::AnsiColor;
use clap::builder::Styles;
use console::{style, Style};
use dialoguer::theme::ColorfulTheme;
use once_cell::sync::Lazy;

pub static DIALOGUER_THEME: Lazy<ColorfulTheme> = Lazy::new(|| ColorfulTheme {
    defaults_style: Style::new().for_stderr().cyan(),
    prompt_style: Style::new().for_stderr().bold().bright().blue(),
    prompt_prefix: style("".to_string()).for_stderr().bright().yellow(),
    prompt_suffix: style("❯".to_string()).for_stderr().black().bright(),
    success_prefix: style("".to_string()).for_stderr(),
    success_suffix: style("".to_string()).for_stderr(),
    error_prefix: style("✘".to_string()).for_stderr().red(),
    error_style: Style::new().for_stderr().red(),
    hint_style: Style::new().for_stderr().black().bright(),
    values_style: Style::new().for_stderr().green(),
    active_item_style: Style::new().for_stderr().bright().blue().underlined(),
    inactive_item_style: Style::new().for_stderr(),
    active_item_prefix: style("❯".to_string()).for_stderr().bright().yellow(),
    inactive_item_prefix: style(" ".to_string()).for_stderr(),
    checked_item_prefix: style("✔".to_string()).for_stderr().green(),
    unchecked_item_prefix: style("⬚".to_string()).for_stderr().magenta(),
    picked_item_prefix: style("❯".to_string()).for_stderr().green(),
    unpicked_item_prefix: style(" ".to_string()).for_stderr(),
    fuzzy_cursor_style: Style::new().for_stderr().black().on_white(),
    fuzzy_match_highlight_style: Style::new().for_stderr().bright().yellow().bold(),
});

pub fn clap_theme() -> Styles {
    Styles::styled()
        .usage(AnsiColor::BrightYellow.on_default().bold().underline())
        .header(AnsiColor::BrightYellow.on_default().bold())
        .literal(AnsiColor::BrightBlue.on_default().italic())
        .placeholder(AnsiColor::BrightBlack.on_default())
}

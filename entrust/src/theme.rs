use clap::builder::Styles;
use clap::builder::styling::{AnsiColor, Color as ClapColor, Style as ClapStyle};
use entrust_dialog::style::{Color as DialogColor, Modifier, Style as DialogStyle};
use entrust_dialog::theme::Theme;
use std::io::IsTerminal;
use std::str::FromStr;
use std::sync::LazyLock;
use std::{env, io};

pub fn color() -> bool {
    env::var("NO_COLOR").is_err() && io::stdout().is_terminal()
}

pub static DIALOG_THEME: LazyLock<Theme> = LazyLock::new(load_dialog_theme);

pub fn load_clap_theme() -> Styles {
    env::var("ENT_THEME")
        .map(parse_clap_theme)
        .unwrap_or_else(|_| clap_theme_default())
}

fn parse_clap_theme(string: String) -> Styles {
    let usage = get_setting(string.as_str(), "help_usage:")
        .map(clap_style_from_dialog_style)
        .unwrap_or(AnsiColor::BrightYellow.on_default().bold().underline());
    let header = get_setting(string.as_str(), "help_header:")
        .map(clap_style_from_dialog_style)
        .unwrap_or(AnsiColor::BrightYellow.on_default().bold());
    let literal = get_setting(string.as_str(), "help_literal:")
        .map(clap_style_from_dialog_style)
        .unwrap_or(AnsiColor::BrightBlue.on_default().italic());
    let placeholder = get_setting(string.as_str(), "help_placeholder:")
        .map(clap_style_from_dialog_style)
        .unwrap_or(AnsiColor::BrightBlack.on_default());
    Styles::styled()
        .usage(usage)
        .header(header)
        .literal(literal)
        .placeholder(placeholder)
}

fn load_dialog_theme() -> Theme {
    env::var("ENT_THEME")
        .map(parse_dialog_theme)
        .unwrap_or_else(|_| Theme::default())
}

fn parse_dialog_theme(string: String) -> Theme {
    Theme {
        cursor_on_style: get_setting(string.as_str(), "dialog_cursor_on:")
            .unwrap_or(Theme::default_ref().cursor_on_style),
        cursor_off_style: get_setting(string.as_str(), "dialog_cursor_off:")
            .unwrap_or(Theme::default_ref().cursor_off_style),
        header_style: get_setting(string.as_str(), "dialog_header:")
            .unwrap_or(Theme::default_ref().header_style),
        placeholder_style: get_setting(string.as_str(), "dialog_placeholder:")
            .unwrap_or(Theme::default_ref().placeholder_style),
        prompt_style: get_setting(string.as_str(), "dialog_prompt:")
            .unwrap_or(Theme::default_ref().prompt_style),
        selected_style: get_setting(string.as_str(), "dialog_selected:")
            .unwrap_or(Theme::default_ref().selected_style),
        match_style: get_setting(string.as_str(), "dialog_match:")
            .unwrap_or(Theme::default_ref().match_style),
        completion_style: get_setting(string.as_str(), "dialog_completion:")
            .unwrap_or(Theme::default_ref().completion_style),
        ..Default::default()
    }
}

fn get_setting(theme_string: &str, prefix: &str) -> Option<DialogStyle> {
    theme_string
        .split(';')
        .filter_map(|s| s.split_once(prefix))
        .map(|(_, usage)| usage)
        .map(parse_ratatui_style)
        .next()
}

fn parse_ratatui_style(string: &str) -> DialogStyle {
    let mut style = DialogStyle::new();
    let fg = string
        .split(',')
        .filter_map(|p| p.split_once("fg:"))
        .filter_map(|(_, color)| DialogColor::from_str(color).ok())
        .next();
    if let Some(fg) = fg {
        style = style.fg(fg);
    }
    let bg = string
        .split(',')
        .filter_map(|p| p.split_once("bg:"))
        .filter_map(|(_, color)| DialogColor::from_str(color).ok())
        .next();
    if let Some(bg) = bg {
        style = style.bg(bg);
    }
    if string.split(',').any(|p| p == "bold") {
        style = style.bold();
    }
    if string.split(',').any(|p| p == "italic") {
        style = style.italic();
    }
    if string.split(',').any(|p| p == "underlined") {
        style = style.underlined();
    }
    style
}

fn clap_style_from_dialog_style(style: DialogStyle) -> ClapStyle {
    let mut ret = ClapStyle::new();
    ret = ret.fg_color(clap_color_from_dialog_color(style.fg));
    ret = ret.bg_color(clap_color_from_dialog_color(style.bg));
    if style.add_modifier.contains(Modifier::BOLD) {
        ret = ret.bold();
    }
    if style.add_modifier.contains(Modifier::ITALIC) {
        ret = ret.italic();
    }
    if style.add_modifier.contains(Modifier::UNDERLINED) {
        ret = ret.underline();
    }
    ret
}

fn clap_color_from_dialog_color(color: Option<DialogColor>) -> Option<ClapColor> {
    match color {
        None => None,
        Some(color) => match color {
            DialogColor::Reset => ClapColor::Ansi(AnsiColor::White),
            DialogColor::Black => ClapColor::Ansi(AnsiColor::Black),
            DialogColor::Red => ClapColor::Ansi(AnsiColor::Red),
            DialogColor::Green => ClapColor::Ansi(AnsiColor::Green),
            DialogColor::Yellow => ClapColor::Ansi(AnsiColor::Yellow),
            DialogColor::Blue => ClapColor::Ansi(AnsiColor::Blue),
            DialogColor::Magenta => ClapColor::Ansi(AnsiColor::Magenta),
            DialogColor::Cyan => ClapColor::Ansi(AnsiColor::Cyan),
            DialogColor::Gray => ClapColor::Ansi(AnsiColor::White),
            DialogColor::DarkGray => ClapColor::Ansi(AnsiColor::BrightBlack),
            DialogColor::LightRed => ClapColor::Ansi(AnsiColor::BrightRed),
            DialogColor::LightGreen => ClapColor::Ansi(AnsiColor::BrightGreen),
            DialogColor::LightYellow => ClapColor::Ansi(AnsiColor::BrightYellow),
            DialogColor::LightBlue => ClapColor::Ansi(AnsiColor::BrightBlue),
            DialogColor::LightMagenta => ClapColor::Ansi(AnsiColor::BrightMagenta),
            DialogColor::LightCyan => ClapColor::Ansi(AnsiColor::BrightCyan),
            DialogColor::White => ClapColor::Ansi(AnsiColor::BrightWhite),
            DialogColor::Rgb(r, g, b) => ClapColor::from((r, g, b)),
            DialogColor::Indexed(i) => ClapColor::from(i),
        }
        .into(),
    }
}

fn clap_theme_default() -> Styles {
    Styles::styled()
        .usage(AnsiColor::BrightYellow.on_default().bold().underline())
        .header(AnsiColor::BrightYellow.on_default().bold())
        .literal(AnsiColor::BrightBlue.on_default().italic())
        .placeholder(AnsiColor::BrightBlack.on_default())
}

pub const CHEVRON: &str = "❯";

macro_rules! chevron_prompt {
    ($text: literal) => {
        const_format::concatcp!($text, " ", crate::theme::CHEVRON, " ")
    };
}
pub(crate) use chevron_prompt;

#[cfg(test)]
mod tests {
    use super::*;
    use clap::builder::styling::Effects;

    #[test]
    fn test_parse_clap_theme() {
        let styles = parse_clap_theme(
            "help_usage:fg:red,bg:blue,bold;help_header:fg:bright cyan,italic".to_string(),
        );

        assert_eq!(
            Some(ClapColor::Ansi(AnsiColor::Red)),
            styles.get_usage().get_fg_color()
        );
        assert_eq!(
            Some(ClapColor::Ansi(AnsiColor::Blue)),
            styles.get_usage().get_bg_color()
        );
        assert!(styles.get_usage().get_effects().contains(Effects::BOLD));
        assert!(!styles.get_usage().get_effects().contains(Effects::ITALIC));

        assert_eq!(
            Some(ClapColor::Ansi(AnsiColor::BrightCyan)),
            styles.get_header().get_fg_color()
        );
        assert!(!styles.get_header().get_effects().contains(Effects::BOLD));
        assert!(styles.get_header().get_effects().contains(Effects::ITALIC));
    }

    #[test]
    fn test_parse_dialog_theme() {
        let theme = parse_dialog_theme(
            "dialog_cursor_on:fg:magenta,bg:light gray,underlined;dialog_cursor_off:fg:#123456"
                .to_string(),
        );
        let expected = Theme {
            cursor_on_style: DialogStyle::new()
                .fg(DialogColor::Magenta)
                .bg(DialogColor::White)
                .underlined(),
            cursor_off_style: DialogStyle::new().fg(DialogColor::Rgb(18, 52, 86)),
            ..Default::default()
        };
        assert_eq!(expected, theme);
    }
}

use clap::builder::styling::AnsiColor;
use clap::builder::Styles;
use par_dialog::dialog::Theme;
use std::sync::LazyLock;

pub static DIALOG_THEME: LazyLock<Theme> = LazyLock::new(Theme::default);

pub fn clap_theme() -> Styles {
    Styles::styled()
        .usage(AnsiColor::BrightYellow.on_default().bold().underline())
        .header(AnsiColor::BrightYellow.on_default().bold())
        .literal(AnsiColor::BrightBlue.on_default().italic())
        .placeholder(AnsiColor::BrightBlack.on_default())
}

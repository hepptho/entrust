pub mod dialog;
pub mod input;
pub mod select;
pub mod theme;
pub mod yes_no;

pub use ratatui::style;
pub use ratatui::text;

macro_rules! key_event_pattern {
    ($code:pat) => {
        ratatui::crossterm::event::KeyEvent {
            code: $code,
            modifiers: _,
            kind: ratatui::crossterm::event::KeyEventKind::Press,
            state: _,
        }
    };
    ($code:pat, $modifier:pat) => {
        ratatui::crossterm::event::KeyEvent {
            code: $code,
            modifiers: $modifier,
            kind: ratatui::crossterm::event::KeyEventKind::Press,
            state: _,
        }
    };
    ($code:pat, $modifier:pat, $kind:pat) => {
        ratatui::crossterm::event::KeyEvent {
            code: $code,
            modifiers: $modifier,
            kind: $kind,
            state: _,
        }
    };
    ($code:pat, $modifier:pat, $kind:pat, $state:pat) => {
        ratatui::crossterm::event::KeyEvent {
            code: $code,
            modifiers: $modifier,
            kind: $kind,
            state: $state,
        }
    };
}
pub(crate) use key_event_pattern;

macro_rules! cancel_key_event {
    () => {
        crate::key_event_pattern!(KeyCode::Esc)
            | crate::key_event_pattern!(KeyCode::Char('c'), KeyModifiers::CONTROL)
    };
}
pub(crate) use cancel_key_event;

use crate::dialog::Theme;
use ratatui::prelude::Style;

#[derive(Debug, Default, Clone)]
pub(super) struct Cursor {
    index: usize,
    pub(super) style: CursorStyle,
    pub(super) mode: CursorMode,
}

impl Cursor {
    pub(super) fn on(&mut self) {
        self.style.on = true;
    }
    pub(super) fn off(&mut self) {
        self.style.on = false;
    }
    fn toggle(&mut self) {
        self.style.on = !self.style.on
    }

    pub(super) fn index(&self) -> usize {
        self.index
    }

    pub(crate) fn set_index(&mut self, index: usize) {
        self.on_move();
        self.index = index
    }

    pub(super) fn move_by(&mut self, steps: isize) {
        self.on_move();
        if steps.is_positive() {
            self.index = self.index.saturating_add(steps.unsigned_abs())
        } else {
            self.index = self.index.saturating_sub(steps.unsigned_abs())
        }
    }

    fn on_move(&mut self) {
        if self.mode != CursorMode::Hide {
            self.on();
        }
    }

    pub(super) fn set_cursor_on_style(&mut self, on_style: Style) {
        self.style.on_style = on_style
    }

    pub(super) fn set_cursor_off_style(&mut self, off_style: Style) {
        self.style.off_style = off_style
    }

    pub(super) fn set_cursor_mode(&mut self, cursor_mode: CursorMode) {
        self.mode = cursor_mode;
        if cursor_mode == CursorMode::Hide {
            self.off();
        }
    }
    pub(super) fn tick(&mut self) -> bool {
        if self.mode == CursorMode::Blink {
            self.toggle();
            true
        } else {
            false
        }
    }
    pub(super) fn current_style(&self) -> Style {
        if self.mode == CursorMode::Hide {
            Style::default()
        } else {
            self.style.current_style()
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(super) struct CursorStyle {
    on_style: Style,
    off_style: Style,
    on: bool,
}

impl CursorStyle {
    fn current_style(&self) -> Style {
        if self.on {
            self.on_style
        } else {
            self.off_style
        }
    }
}

impl Default for CursorStyle {
    fn default() -> Self {
        let theme = Theme::default();
        let on_style = theme.cursor_style;
        let off_style = Style::default();
        let on = true;
        CursorStyle {
            on_style,
            off_style,
            on,
        }
    }
}

#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub enum CursorMode {
    #[default]
    Blink,
    Hide,
    Static,
}

#[cfg(test)]
mod tests {
    use crate::dialog::Dialog;
    use crate::input::{InputDialog, Update};

    #[test]
    fn test() {
        let mut state = InputDialog::default();
        state.perform_update(Update::InsertChar('1')).unwrap();
        assert_eq!(vec!['1'], state.content);
        assert_eq!(1, state.cursor.index());

        state.perform_update(Update::InsertChar('2')).unwrap();
        assert_eq!(vec!['1', '2'], state.content);
        assert_eq!(2, state.cursor.index());

        state.perform_update(Update::MoveCursorRight).unwrap();
        assert_eq!(2, state.cursor.index());

        state.perform_update(Update::MoveCursorLeft).unwrap();
        state.perform_update(Update::MoveCursorLeft).unwrap();
        assert_eq!(0, state.cursor.index());
        state.perform_update(Update::MoveCursorLeft).unwrap();
        assert_eq!(0, state.cursor.index());
        state.perform_update(Update::InsertChar('0')).unwrap();
        assert_eq!(vec!['0', '1', '2'], state.content);
        assert_eq!(1, state.cursor.index());
    }
}

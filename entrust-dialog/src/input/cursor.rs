use crate::theme::Theme;
use ratatui::prelude::Style;

#[derive(Debug, Default, Clone)]
pub(super) struct Cursor {
    index: usize,
    pub(super) state: CursorState,
    pub(super) mode: CursorMode,
}

impl Cursor {
    pub(super) fn set_state(&mut self, state: CursorState) {
        self.state = state;
    }
    fn toggle_state(&mut self) {
        match self.state {
            CursorState::On => self.state = CursorState::Off,
            CursorState::Off => self.state = CursorState::On,
        }
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
            self.set_state(CursorState::On);
        }
    }

    pub(super) fn set_cursor_mode(&mut self, cursor_mode: CursorMode) {
        self.mode = cursor_mode;
        if cursor_mode == CursorMode::Hide {
            self.set_state(CursorState::Off);
        }
    }
    pub(super) fn tick(&mut self) -> bool {
        if self.mode == CursorMode::Blink {
            self.toggle_state();
            true
        } else {
            false
        }
    }
    pub(super) fn current_style(&self, theme: &Theme) -> Style {
        if self.mode == CursorMode::Hide {
            Style::default()
        } else {
            match self.state {
                CursorState::On => theme.cursor_on_style,
                CursorState::Off => theme.cursor_off_style,
            }
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub(super) enum CursorState {
    #[default]
    On,
    Off,
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
    use super::*;
    use crate::dialog::Dialog;
    use crate::input::{InputDialog, Update};

    #[test]
    fn test_index() {
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

    #[test]
    fn test_state() {
        let mut blink_cursor = Cursor {
            index: 0,
            state: CursorState::On,
            mode: CursorMode::Blink,
        };
        let theme = Theme::default_ref();
        assert_eq!(theme.cursor_on_style, blink_cursor.current_style(theme));
        blink_cursor.tick();
        assert_eq!(theme.cursor_off_style, blink_cursor.current_style(theme));

        let mut hidden_cursor = Cursor {
            index: 0,
            state: CursorState::On,
            mode: CursorMode::Hide,
        };
        assert_eq!(theme.cursor_off_style, hidden_cursor.current_style(theme));
        hidden_cursor.tick();
        assert_eq!(theme.cursor_off_style, hidden_cursor.current_style(theme));

        let mut static_cursor = Cursor {
            index: 0,
            state: CursorState::On,
            mode: CursorMode::Static,
        };
        assert_eq!(theme.cursor_on_style, static_cursor.current_style(theme));
        static_cursor.tick();
        assert_eq!(theme.cursor_on_style, static_cursor.current_style(theme));
    }
}

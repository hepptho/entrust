use ratatui::prelude::Style;

#[derive(Debug, Default, Clone)]
pub(super) struct Cursor {
    index: usize,
    pub(super) style: Style,
    pub(super) style_display: Style,
    pub(super) mode: CursorMode,
    style_state: CursorStyleState,
}

impl Cursor {
    pub(super) fn on(&mut self) {
        self.style_display = self.style;
        self.style_state = CursorStyleState::On;
    }
    pub(super) fn off(&mut self) {
        self.style_display = Style::default();
        self.style_state = CursorStyleState::Off;
    }
    fn toggle(&mut self) {
        match self.style_state {
            CursorStyleState::On => {
                self.style_display = Style::default();
                self.style_state = CursorStyleState::Off;
            }
            CursorStyleState::Off => {
                self.style_display = self.style;
                self.style_state = CursorStyleState::On;
            }
        }
    }

    pub(super) fn index(&self) -> usize {
        self.index
    }

    pub(super) fn set_index(&mut self, index: usize) {
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

    pub(super) fn set_cursor_style(&mut self, style: Style) {
        self.style = style;
        if self.mode != CursorMode::Hide {
            self.style_display = style;
        }
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
}

#[derive(Debug, Default, Clone, Copy)]
enum CursorStyleState {
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

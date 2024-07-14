pub mod cursor;
mod widget;

use std::io;
use std::time::Duration;

use ratatui::crossterm::event::Event::Key;
use ratatui::crossterm::event::{Event, KeyCode, KeyModifiers};
use ratatui::prelude::*;
use ratatui::Viewport;

use crate::dialog::Dialog;
use crate::input::cursor::{Cursor, CursorMode};
use crate::key_event_pattern as kep;

#[derive(Debug, Default, Clone)]
pub struct InputDialog {
    content: Vec<char>,
    cursor: Cursor,
    hidden: bool,
    header: Line<'static>,
    prompt: Span<'static>,
    placeholder: Span<'static>,
    timeout: Option<Duration>,
    completed: bool,
}

impl InputDialog {
    pub fn with_content(mut self, text: &str) -> Self {
        let text: Vec<char> = text.chars().collect();
        let cursor_index = text.len();
        self.content = text;
        self.cursor.set_index(cursor_index);
        self
    }
    pub fn with_cursor_style(mut self, cursor_style: Style) -> Self {
        self.cursor.set_cursor_style(cursor_style);
        self
    }
    pub fn with_header(mut self, header: Line<'static>) -> Self {
        self.header = header;
        self
    }
    pub fn with_prompt(mut self, prompt: Span<'static>) -> Self {
        self.prompt = prompt;
        self
    }
    pub fn with_placeholder(mut self, placeholder: Span<'static>) -> Self {
        self.placeholder = placeholder;
        self
    }
    pub fn with_timeout(mut self, timeout: Option<Duration>) -> Self {
        self.timeout = timeout;
        self
    }
    pub fn with_cursor_mode(mut self, cursor_mode: CursorMode) -> Self {
        self.cursor.set_cursor_mode(cursor_mode);
        self
    }
    pub fn hidden(mut self, hidden: bool) -> Self {
        self.hidden = hidden;
        self
    }

    pub fn current_content(&self) -> String {
        self.content.iter().collect()
    }
}

pub enum Update {
    InsertChar(char),
    DeleteBeforeCursor,
    DeleteAfterCursor,
    MoveCursorLeft,
    MoveCursorRight,
    ToggleHidden,
    Enter,
    Noop,
}

impl Dialog for InputDialog {
    type Update = Update;
    type Output = String;

    fn update_for_event(event: Event) -> Self::Update {
        match event {
            Key(ke) => match ke {
                kep!(KeyCode::Char('h'), KeyModifiers::ALT) => Update::ToggleHidden,
                kep!(KeyCode::Char(char)) => Update::InsertChar(char),
                kep!(KeyCode::Backspace) => Update::DeleteBeforeCursor,
                kep!(KeyCode::Delete) => Update::DeleteAfterCursor,
                kep!(KeyCode::Left) => Update::MoveCursorLeft,
                kep!(KeyCode::Right) => Update::MoveCursorRight,
                kep!(KeyCode::Enter) => Update::Enter,
                _ => Update::Noop,
            },
            _ => Update::Noop,
        }
    }

    fn perform_update(&mut self, change: Self::Update) -> io::Result<()> {
        match change {
            Update::InsertChar(char) => {
                self.content.insert(self.cursor.index(), char);
                self.cursor.move_by(1);
            }
            Update::DeleteBeforeCursor => {
                if self.cursor.index() > 0 {
                    self.content.remove(self.cursor.index() - 1);
                    self.cursor.move_by(-1);
                }
            }
            Update::DeleteAfterCursor => {
                if self.cursor.index() < self.content.len() {
                    self.content.remove(self.cursor.index());
                }
            }
            Update::MoveCursorLeft => {
                if self.cursor.index() > 0 {
                    self.cursor.move_by(-1);
                }
            }
            Update::MoveCursorRight => {
                if self.cursor.index() < self.content.len() {
                    self.cursor.move_by(1);
                }
            }
            Update::ToggleHidden => self.hidden = !self.hidden,
            Update::Enter => self.completed = true,
            Update::Noop => {}
        };
        Ok(())
    }

    fn completed(&self) -> bool {
        self.completed
    }

    fn output(self) -> Self::Output {
        self.content.iter().collect()
    }

    fn viewport(&self) -> Viewport {
        Viewport::Inline(2)
    }

    fn draw(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.size())
    }

    fn tick(&mut self) -> bool {
        self.cursor.tick()
    }

    fn timeout(&self) -> Option<Duration> {
        self.timeout
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

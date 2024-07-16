pub mod confirmation;
pub mod cursor;
pub mod prompt;
pub mod validator;
mod widget;

use std::io;
use std::time::Duration;

use ratatui::crossterm::event::Event::Key;
use ratatui::crossterm::event::{Event, KeyCode, KeyModifiers};
use ratatui::prelude::*;
use ratatui::Viewport;

use crate::dialog::{Dialog, Theme};
use crate::input::confirmation::Confirmation;
use crate::input::cursor::{Cursor, CursorMode};
use crate::input::prompt::Prompt;
use crate::input::validator::Validator;
use crate::key_event_pattern as kep;

#[derive(Debug, Default, Clone)]
pub struct InputDialog {
    content: Vec<char>,
    cursor: Cursor,
    hidden: bool,
    prompt: Prompt,
    placeholder: &'static str,
    timeout: Option<Duration>,
    validator: Validator,
    confirmation: Option<Confirmation>,
    completed: bool,
    theme: Theme,
}

impl InputDialog {
    pub fn with_content(mut self, text: &str) -> Self {
        let text: Vec<char> = text.chars().collect();
        let cursor_index = text.len();
        self.content = text;
        self.cursor.set_index(cursor_index);
        self
    }
    pub fn with_cursor_on_style(mut self, cursor_on_style: Style) -> Self {
        self.cursor.set_cursor_on_style(cursor_on_style);
        self
    }
    pub fn with_cursor_off_style(mut self, cursor_off_style: Style) -> Self {
        self.cursor.set_cursor_off_style(cursor_off_style);
        self
    }
    pub fn with_prompt(mut self, prompt: Prompt) -> Self {
        self.prompt = prompt;
        self
    }
    pub fn with_placeholder(mut self, placeholder: &'static str) -> Self {
        self.placeholder = placeholder;
        self
    }
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }
    pub fn with_cursor_mode(mut self, cursor_mode: CursorMode) -> Self {
        self.cursor.set_cursor_mode(cursor_mode);
        self
    }
    pub fn with_hidden(mut self, hidden: bool) -> Self {
        self.hidden = hidden;
        self
    }
    pub fn with_validator(mut self, validator: Validator) -> Self {
        self.validator = validator;
        self
    }
    pub fn with_confirmation(mut self, confirmation: Confirmation) -> Self {
        self.confirmation = Some(confirmation);
        self
    }
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    pub(crate) fn current_content(&self) -> String {
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
            Update::Enter => {
                if self.validation_message().is_none() {
                    Confirmation::confirm(self)
                }
            }
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
        Viewport::Inline(3)
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

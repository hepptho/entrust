pub mod confirmation;
pub mod cursor;
pub mod mask;
pub mod prompt;
pub mod validator;
mod widget;

use std::borrow::Cow;
use std::io;
use std::time::Duration;

use crate::dialog::{Dialog, DialogState};
use crate::input::confirmation::Confirmation;
use crate::input::cursor::{Cursor, CursorMode};
use crate::input::mask::InputMask;
use crate::input::prompt::Prompt;
use crate::input::validator::Validator;
use crate::theme::Theme;
use crate::{cancel_key_event, key_event_pattern as kep};
use ratatui::crossterm::event::Event::Key;
use ratatui::crossterm::event::{Event, KeyCode, KeyModifiers};
use ratatui::prelude::*;
use ratatui::Viewport;
use tracing::debug;

#[derive(Debug, Default)]
pub struct InputDialog {
    content: Vec<char>,
    cursor: Cursor,
    mask: InputMask,
    prompt: Prompt,
    placeholder: &'static str,
    timeout: Option<Duration>,
    validator: Validator,
    confirmation: Option<Confirmation>,
    state: DialogState,
    theme: Cow<'static, Theme>,
}

impl InputDialog {
    pub fn with_content(mut self, text: &str) -> Self {
        let text: Vec<char> = text.chars().collect();
        let cursor_index = text.len();
        self.content = text;
        self.cursor.set_index(cursor_index);
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
    pub fn with_mask(mut self, mask: InputMask) -> Self {
        self.mask = mask;
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
    pub fn with_theme<T: Into<Cow<'static, Theme>>>(mut self, theme: T) -> Self {
        self.theme = theme.into();
        self
    }

    pub(crate) fn current_content(&self) -> String {
        self.content.iter().collect()
    }
}

#[derive(Debug)]
pub enum Update {
    InsertChar(char),
    DeleteBeforeCursor,
    DeleteAfterCursor,
    MoveCursorLeft,
    MoveCursorRight,
    ToggleMask,
    Confirm,
    Cancel,
}

impl Dialog for InputDialog {
    type Update = Update;
    type Output = String;

    fn update_for_event(event: Event) -> Option<Self::Update> {
        match event {
            Key(ke) => match ke {
                cancel_key_event!() => Update::Cancel.into(),
                kep!(KeyCode::Char('h'), KeyModifiers::ALT) => Update::ToggleMask.into(),
                kep!(KeyCode::Char(char)) => Update::InsertChar(char).into(),
                kep!(KeyCode::Backspace) => Update::DeleteBeforeCursor.into(),
                kep!(KeyCode::Delete) => Update::DeleteAfterCursor.into(),
                kep!(KeyCode::Left) => Update::MoveCursorLeft.into(),
                kep!(KeyCode::Right) => Update::MoveCursorRight.into(),
                kep!(KeyCode::Enter) => Update::Confirm.into(),
                _ => None,
            },
            _ => None,
        }
    }

    fn perform_update(&mut self, update: Self::Update) -> io::Result<()> {
        debug!(?update, ?self.content, ?self.cursor);
        match update {
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
            Update::ToggleMask => self.mask.toggle(),
            Update::Confirm => {
                if self.validation_message().is_none() {
                    Confirmation::confirm(self)
                }
            }
            Update::Cancel => self.state = DialogState::Cancelled,
        };
        Ok(())
    }

    fn state(&self) -> DialogState {
        self.state
    }

    fn output(self) -> Self::Output {
        self.content.iter().collect()
    }

    fn viewport(&self) -> Viewport {
        Viewport::Inline(3)
    }

    fn draw(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.area())
    }

    fn tick(&mut self) -> bool {
        self.cursor.tick()
    }

    fn timeout(&self) -> Option<Duration> {
        self.timeout
    }
}

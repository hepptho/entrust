mod completions;
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
use crate::input::completions::Completions;
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
pub struct InputDialog<'p, 'c> {
    content: Vec<char>,
    cursor: Cursor,
    completions: Completions,
    mask: InputMask,
    prompt: Prompt<'p>,
    placeholder: &'static str,
    timeout: Option<Duration>,
    validator: Validator<'static>,
    confirmation: Option<Confirmation<'c>>,
    state: DialogState,
    theme: Cow<'static, Theme>,
}

impl<'p, 'c> InputDialog<'p, 'c> {
    pub fn with_content(mut self, text: &str) -> Self {
        let text: Vec<char> = text.chars().collect();
        let cursor_index = text.len();
        self.content = text;
        self.cursor.set_index(cursor_index);
        self
    }
    pub fn with_prompt(mut self, prompt: Prompt<'p>) -> Self {
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
    pub fn with_validator(mut self, validator: Validator<'static>) -> Self {
        self.validator = validator;
        self
    }
    pub fn with_confirmation(mut self, confirmation: Confirmation<'c>) -> Self {
        self.confirmation = Some(confirmation);
        self
    }
    pub fn with_theme<T: Into<Cow<'static, Theme>>>(mut self, theme: T) -> Self {
        self.theme = theme.into();
        self
    }
    pub fn with_completions(mut self, completions: Vec<Cow<'static, str>>) -> Self {
        self.completions.list = completions;
        self.completions.list.sort();
        self
    }

    pub(crate) fn current_content(&self) -> String {
        self.content.iter().collect()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Update {
    InsertChar(char),
    DeleteBeforeCursor,
    DeleteAfterCursor,
    MoveCursorLeft,
    MoveCursorRight,
    ToggleMask,
    Confirm,
    Cancel,
    CycleCompletions,
    CycleCompletionsBackward,
}

impl<'p, 'c> Dialog for InputDialog<'p, 'c> {
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
                kep!(KeyCode::Tab, KeyModifiers::SHIFT) | kep!(KeyCode::Up) => {
                    Update::CycleCompletionsBackward.into()
                }
                kep!(KeyCode::Tab) | kep!(KeyCode::Down) => Update::CycleCompletions.into(),
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
                } else if let Some(completion) = self.get_full_completion() {
                    self.content = completion.chars().collect();
                    self.cursor.set_index(self.content.len());
                }
            }
            Update::ToggleMask => self.mask.toggle(),
            Update::Confirm => {
                if self.validation_message().is_none() {
                    self.confirm()
                }
            }
            Update::Cancel => self.state = DialogState::Cancelled,
            Update::CycleCompletions | Update::CycleCompletionsBackward => {
                self.update_completions(update)
            }
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

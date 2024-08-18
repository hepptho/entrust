use crate::dialog::{Dialog, DialogState};
use crate::input;
use crate::input::prompt::Prompt;
use crate::input::validator::Validator;
use crate::input::InputDialog;
use crate::theme::Theme;
use ratatui::crossterm::event::Event;
use ratatui::prelude::{Color, Span};
use ratatui::{Frame, Viewport};
use std::borrow::Cow;

#[derive(Debug)]
pub struct YesNoDialog {
    inner: InputDialog<'static, 'static>,
}

const ORANGE: Color = Color::Rgb(255, 184, 108);

impl Default for YesNoDialog {
    fn default() -> Self {
        let inner = InputDialog::default()
            .with_prompt(Prompt::inline(Span::styled("[y/N] > ", ORANGE)))
            .with_validator(Validator::new(|vec| {
                let mut string: String = vec.iter().collect();
                string.make_ascii_lowercase();
                if "yes".starts_with(string.as_str()) || "no".starts_with(string.as_str()) {
                    None
                } else {
                    Some("Choose yes or no".into())
                }
            }))
            .with_completions(vec!["yes".into(), "no".into()]);
        YesNoDialog { inner }
    }
}

impl YesNoDialog {
    pub fn with_message<M: Into<Cow<'static, str>>>(mut self, message: M) -> Self {
        let prompt = Prompt::inline(vec![
            Span::styled(message.into(), Color::LightBlue),
            Span::styled(" [y/N] > ", ORANGE),
        ]);
        self.inner = self.inner.with_prompt(prompt);
        self
    }

    pub fn with_theme<T: Into<Cow<'static, Theme>>>(mut self, theme: T) -> Self {
        self.inner = self.inner.with_theme(theme);
        self
    }
}

impl Dialog for YesNoDialog {
    type Update = input::Update;
    type Output = bool;

    fn update_for_event(event: Event) -> Option<Self::Update> {
        InputDialog::update_for_event(event)
    }

    fn perform_update(&mut self, update: Self::Update) -> std::io::Result<()> {
        self.inner.perform_update(update)
    }

    fn state(&self) -> DialogState {
        self.inner.state()
    }

    fn output(self) -> Self::Output {
        let mut string = self.inner.output();
        if string.is_empty() {
            false
        } else {
            string.make_ascii_lowercase();
            "yes".starts_with(string.as_str())
        }
    }

    fn viewport(&self) -> Viewport {
        self.inner.viewport()
    }

    fn draw(&mut self, frame: &mut Frame) {
        self.inner.draw(frame)
    }
}

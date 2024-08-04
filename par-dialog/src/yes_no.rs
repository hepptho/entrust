use crate::dialog::{Dialog, DialogState};
use crate::input;
use crate::input::prompt::Prompt;
use crate::input::validator::Validator;
use crate::input::InputDialog;
use crate::theme::Theme;
use ratatui::crossterm::event::Event;
use ratatui::{Frame, Viewport};

#[derive(Debug)]
pub struct YesNoDialog {
    inner: InputDialog,
}

impl Default for YesNoDialog {
    fn default() -> Self {
        let inner =
            InputDialog::default().with_validator(Validator::new("Choose yes or no", |vec| {
                let mut string: String = vec.iter().collect();
                string.make_ascii_lowercase();
                "yes".starts_with(string.as_str()) || "no".starts_with(string.as_str())
            }));
        YesNoDialog { inner }
    }
}

impl YesNoDialog {
    pub fn with_message(mut self, message: &str) -> Self {
        let prompt = Prompt::inline(format!("{message} [y/N] > "));
        self.inner = self.inner.with_prompt(prompt);
        self
    }

    pub fn with_theme(mut self, theme: &'static Theme) -> Self {
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

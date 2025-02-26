use crate::dialog::DialogState;
use crate::input::InputDialog;
use crate::input::prompt::Prompt;
use crate::input::validator::Validator;
use std::mem;

#[derive(Debug, Clone)]
pub struct Confirmation<'c> {
    prompt: Prompt<'c>,
    validation_message: &'static str,
    is_active: bool,
}

impl<'c> Confirmation<'c> {
    pub fn new(prompt: Prompt<'c>) -> Self {
        Confirmation {
            prompt,
            validation_message: "no match",
            is_active: false,
        }
    }

    pub fn with_validation_message(mut self, validation_message: &'static str) -> Self {
        self.validation_message = validation_message;
        self
    }
}

impl<'p, 'c> InputDialog<'p, 'c> {
    pub(super) fn confirm(&mut self) {
        match self.confirmation.as_mut() {
            None => self.state = DialogState::Completed,
            Some(confirmation) if confirmation.is_active => self.state = DialogState::Completed,
            Some(confirmation) => {
                let first = mem::take(&mut self.content);
                self.cursor.set_index(0);
                let message = confirmation.validation_message;
                self.validator = Validator::new(move |chars| {
                    if chars == first {
                        None
                    } else {
                        Some(message.into())
                    }
                });
                confirmation.is_active = true;
            }
        }
    }

    pub(super) fn prompt_with_confirmation(&self) -> Prompt {
        self.confirmation
            .as_ref()
            .and_then(|c| {
                if c.is_active {
                    Some(c.prompt.clone())
                } else {
                    None
                }
            })
            .unwrap_or(self.prompt.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dialog::Dialog;
    use crate::input::Update;
    use crate::input::prompt::Prompt;
    use ratatui::prelude::Line;

    #[test]
    fn test() {
        let prompt_header = "header prompt";
        let prompt_inline = "inline prompt";
        let confirmation_header = "repeat";
        let validation_message = "no match";
        let mut dialog = InputDialog::default()
            .with_prompt(Prompt::new(prompt_header, prompt_inline))
            .with_confirmation(
                Confirmation::new(Prompt::header(confirmation_header))
                    .with_validation_message(validation_message),
            );

        dialog.perform_update(Update::InsertChar('a')).unwrap();
        assert!(!dialog.confirmation.as_ref().unwrap().is_active);
        assert_eq!(
            Line::raw(prompt_header),
            dialog.prompt_with_confirmation().header
        );

        dialog.perform_update(Update::Confirm).unwrap();
        assert!(dialog.content.is_empty());
        assert!(dialog.confirmation.as_ref().unwrap().is_active);
        assert_eq!(
            Line::raw(confirmation_header),
            dialog.prompt_with_confirmation().header
        );

        dialog.perform_update(Update::InsertChar('b')).unwrap();
        dialog.perform_update(Update::Confirm).unwrap();
        assert_eq!(Some(validation_message.into()), dialog.validation_message());
        assert_eq!(DialogState::Pending, dialog.state);

        dialog.perform_update(Update::DeleteBeforeCursor).unwrap();
        dialog.perform_update(Update::InsertChar('a')).unwrap();
        assert_eq!(None, dialog.validation_message());
        dialog.perform_update(Update::Confirm).unwrap();
        assert_eq!(DialogState::Completed, dialog.state);
    }
}

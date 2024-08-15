use crate::dialog::DialogState;
use crate::input::InputDialog;
use ratatui::text::Line;
use std::mem;

#[derive(Debug, Clone)]
pub struct Confirmation<'c> {
    initial_message: Line<'c>,
    incorrect_message: Line<'c>,
    pub(super) message_type: ConfirmationMessageType,
    pub(super) answered_incorrectly: bool,
    pub(super) first_input: Option<Vec<char>>,
}

impl<'c> Confirmation<'c> {
    pub fn new<M, I>(
        initial_message: M,
        incorrect_message: I,
        message_type: ConfirmationMessageType,
    ) -> Self
    where
        M: Into<Line<'c>>,
        I: Into<Line<'c>>,
    {
        Confirmation {
            initial_message: initial_message.into(),
            incorrect_message: incorrect_message.into(),
            message_type,
            answered_incorrectly: false,
            first_input: None,
        }
    }

    pub(super) fn message(&self) -> &Line {
        if self.answered_incorrectly {
            &self.incorrect_message
        } else {
            &self.initial_message
        }
    }

    pub(super) fn confirm(dialog: &mut InputDialog) {
        if let Some(confirmation) = &dialog.confirmation {
            if let Some(first_input) = &confirmation.first_input {
                if first_input == &dialog.content {
                    dialog.state = DialogState::Completed
                } else {
                    dialog.confirmation.as_mut().unwrap().answered_incorrectly = true
                }
            } else {
                dialog.confirmation.as_mut().unwrap().first_input =
                    Some(mem::take(&mut dialog.content));
                dialog.cursor.set_index(0)
            }
        } else {
            dialog.state = DialogState::Completed
        }
    }
}

impl InputDialog {
    pub(super) fn prompt_with_confirmation(&self) -> (Line, Line) {
        self.confirmation
            .as_ref()
            .and_then(|c| {
                if c.first_input.is_some() {
                    Some(c)
                } else {
                    None
                }
            })
            .map(|c| match c.message_type {
                ConfirmationMessageType::Header => {
                    (c.message().clone(), self.prompt.inline.clone())
                }
                ConfirmationMessageType::Inline => {
                    (self.prompt.header.clone(), c.message().clone())
                }
            })
            .unwrap_or((self.prompt.header.clone(), self.prompt.inline.clone()))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConfirmationMessageType {
    Header,
    Inline,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dialog::Dialog;
    use crate::input::prompt::Prompt;
    use crate::input::Update;

    #[test]
    fn test() {
        let prompt_header = "header prompt";
        let prompt_inline = "inline prompt";
        let confirmation_message = "repeat";
        let incorrect_message = "no match";
        let mut dialog = InputDialog::default()
            .with_prompt(Prompt::new(prompt_header, prompt_inline))
            .with_confirmation(Confirmation::new(
                confirmation_message,
                incorrect_message,
                ConfirmationMessageType::Header,
            ));

        dialog.perform_update(Update::InsertChar('a')).unwrap();
        assert_eq!(None, dialog.confirmation.as_ref().unwrap().first_input);
        assert_eq!(
            Line::raw(prompt_header),
            dialog.prompt_with_confirmation().0
        );

        dialog.perform_update(Update::Confirm).unwrap();
        assert_eq!(
            Some(vec!['a']),
            dialog.confirmation.as_ref().unwrap().first_input
        );
        assert!(dialog.content.is_empty());
        assert!(!dialog.confirmation.as_ref().unwrap().answered_incorrectly);
        assert_eq!(
            Line::raw(confirmation_message),
            dialog.prompt_with_confirmation().0
        );

        dialog.perform_update(Update::InsertChar('b')).unwrap();
        dialog.perform_update(Update::Confirm).unwrap();
        assert!(dialog.confirmation.as_ref().unwrap().answered_incorrectly);
        assert_eq!(
            Line::raw(incorrect_message),
            dialog.prompt_with_confirmation().0
        );
        assert_eq!(DialogState::Pending, dialog.state);

        dialog.perform_update(Update::DeleteBeforeCursor).unwrap();
        dialog.perform_update(Update::InsertChar('a')).unwrap();
        dialog.perform_update(Update::Confirm).unwrap();
        assert_eq!(DialogState::Completed, dialog.state);
    }
}

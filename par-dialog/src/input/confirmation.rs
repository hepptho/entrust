use crate::dialog::DialogState;
use crate::input::InputDialog;
use std::mem;

#[derive(Debug, Clone)]
pub struct Confirmation {
    initial_message: &'static str,
    incorrect_message: &'static str,
    pub(super) message_type: ConfirmationMessageType,
    pub(super) answered_incorrectly: bool,
    pub(super) first_input: Option<Vec<char>>,
}

impl Confirmation {
    pub fn new(
        initial_message: &'static str,
        incorrect_message: &'static str,
        message_type: ConfirmationMessageType,
    ) -> Self {
        Confirmation {
            initial_message,
            incorrect_message,
            message_type,
            answered_incorrectly: false,
            first_input: None,
        }
    }

    pub(super) fn message(&self) -> &'static str {
        if self.answered_incorrectly {
            self.incorrect_message
        } else {
            self.initial_message
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
    pub(super) fn prompt_with_confirmation(&self) -> (&str, &str) {
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
                ConfirmationMessageType::Header => (c.message(), self.prompt.inline.as_ref()),
                ConfirmationMessageType::Inline => (self.prompt.header.as_ref(), c.message()),
            })
            .unwrap_or((self.prompt.header.as_ref(), self.prompt.inline.as_ref()))
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
        assert_eq!(prompt_header, dialog.prompt_with_confirmation().0);

        dialog.perform_update(Update::Confirm).unwrap();
        assert_eq!(
            Some(vec!['a']),
            dialog.confirmation.as_ref().unwrap().first_input
        );
        assert!(dialog.content.is_empty());
        assert!(!dialog.confirmation.as_ref().unwrap().answered_incorrectly);
        assert_eq!(confirmation_message, dialog.prompt_with_confirmation().0);

        dialog.perform_update(Update::InsertChar('b')).unwrap();
        dialog.perform_update(Update::Confirm).unwrap();
        assert!(dialog.confirmation.as_ref().unwrap().answered_incorrectly);
        assert_eq!(incorrect_message, dialog.prompt_with_confirmation().0);
        assert_eq!(DialogState::Pending, dialog.state);

        dialog.perform_update(Update::DeleteBeforeCursor).unwrap();
        dialog.perform_update(Update::InsertChar('a')).unwrap();
        dialog.perform_update(Update::Confirm).unwrap();
        assert_eq!(DialogState::Completed, dialog.state);
    }
}

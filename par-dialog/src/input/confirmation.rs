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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConfirmationMessageType {
    Header,
    Inline,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dialog::Dialog;
    use crate::input::Update;

    #[test]
    fn test() {
        let mut dialog = InputDialog::default().with_confirmation(Confirmation::new(
            "repeat",
            "no match",
            ConfirmationMessageType::Header,
        ));

        dialog.perform_update(Update::InsertChar('a')).unwrap();
        assert_eq!(None, dialog.confirmation.as_ref().unwrap().first_input);

        dialog.perform_update(Update::Confirm).unwrap();
        assert_eq!(
            Some(vec!['a']),
            dialog.confirmation.as_ref().unwrap().first_input
        );
        assert!(dialog.content.is_empty());
        assert!(!dialog.confirmation.as_ref().unwrap().answered_incorrectly);

        dialog.perform_update(Update::InsertChar('b')).unwrap();
        dialog.perform_update(Update::Confirm).unwrap();
        assert!(dialog.confirmation.as_ref().unwrap().answered_incorrectly);
        assert_eq!(DialogState::Pending, dialog.state);

        dialog.perform_update(Update::DeleteBeforeCursor).unwrap();
        dialog.perform_update(Update::InsertChar('a')).unwrap();
        dialog.perform_update(Update::Confirm).unwrap();
        assert_eq!(DialogState::Completed, dialog.state);
    }
}

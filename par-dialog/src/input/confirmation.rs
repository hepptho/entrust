use crate::input::InputDialog;
use std::mem;

#[derive(Debug, Clone)]
pub struct Confirmation {
    initial_message: &'static str,
    incorrect_message: &'static str,
    pub(super) answered_incorrectly: bool,
    pub(super) first_input: Option<Vec<char>>,
}

impl Confirmation {
    pub fn new(initial_message: &'static str, incorrect_message: &'static str) -> Self {
        Confirmation {
            initial_message,
            incorrect_message,
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
                    dialog.completed = true
                } else {
                    dialog.confirmation.as_mut().unwrap().answered_incorrectly = true
                }
            } else {
                dialog.confirmation.as_mut().unwrap().first_input =
                    Some(mem::take(&mut dialog.content));
                dialog.cursor.set_index(0)
            }
        } else {
            dialog.completed = true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dialog::Dialog;
    use crate::input::Update;

    #[test]
    fn test() {
        let mut state =
            InputDialog::default().with_confirmation(Confirmation::new("repeat", "no match"));

        state.perform_update(Update::InsertChar('a')).unwrap();
        assert_eq!(None, state.confirmation.as_ref().unwrap().first_input);

        state.perform_update(Update::Enter).unwrap();
        assert_eq!(
            Some(vec!['a']),
            state.confirmation.as_ref().unwrap().first_input
        );
        assert!(state.content.is_empty());
        assert!(!state.confirmation.as_ref().unwrap().answered_incorrectly);

        state.perform_update(Update::InsertChar('b')).unwrap();
        state.perform_update(Update::Enter).unwrap();
        assert!(state.confirmation.as_ref().unwrap().answered_incorrectly);
        assert!(!state.completed);

        state.perform_update(Update::DeleteBeforeCursor).unwrap();
        state.perform_update(Update::InsertChar('a')).unwrap();
        state.perform_update(Update::Enter).unwrap();
        assert!(state.completed);
    }
}

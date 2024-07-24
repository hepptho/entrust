use crate::input::InputDialog;
use std::fmt::{Debug, Formatter};

pub trait Predicate: 'static + Fn(&Vec<char>) -> bool {}
impl<P: 'static + Fn(&Vec<char>) -> bool> Predicate for P {}

pub struct Validator {
    message: &'static str,
    predicate: Box<dyn Predicate>,
}

impl Validator {
    pub fn new(message: &'static str, predicate: impl Predicate) -> Self {
        let predicate = Box::new(predicate);
        Validator { message, predicate }
    }
}

impl Default for Validator {
    fn default() -> Self {
        Validator {
            message: "",
            predicate: Box::new(|_| true),
        }
    }
}

impl Validator {
    pub fn not_empty(message: &'static str) -> Self {
        Validator {
            message,
            predicate: Box::new(|vec| !vec.is_empty()),
        }
    }
}

impl InputDialog {
    pub fn validation_message(&self) -> Option<&'static str> {
        if (self.validator.predicate)(&self.content) {
            None
        } else {
            Some(self.validator.message)
        }
    }
}

impl Debug for Validator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Validator {{ message: {} }}", self.message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dialog::{Dialog, DialogState};
    use crate::input::Update;
    use crate::input::Update::InsertChar;

    #[test]
    fn test_not_empty() {
        let mut state =
            InputDialog::default().with_validator(Validator::not_empty("must not be empty"));

        assert_eq!(Some("must not be empty"), state.validation_message());

        state.perform_update(Update::Confirm).unwrap();
        assert_eq!(DialogState::Pending, state.state);

        state.perform_update(InsertChar('a')).unwrap();
        assert_eq!(None, state.validation_message());

        state.perform_update(Update::Confirm).unwrap();
        assert_eq!(DialogState::Completed, state.state);
    }
}

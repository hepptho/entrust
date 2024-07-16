use crate::input::InputDialog;

#[derive(Debug, Clone)]
pub struct Validator {
    message: &'static str,
    predicate: fn(&Vec<char>) -> bool,
}

impl Validator {
    pub fn new(message: &'static str, predicate: fn(&Vec<char>) -> bool) -> Self {
        Validator { message, predicate }
    }
}

impl Default for Validator {
    fn default() -> Self {
        Validator {
            message: "",
            predicate: |_| true,
        }
    }
}

impl Validator {
    pub fn not_empty(message: &'static str) -> Self {
        Validator {
            message,
            predicate: |vec| !vec.is_empty(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dialog::Dialog;
    use crate::input::Update;
    use crate::input::Update::InsertChar;

    #[test]
    fn test_not_empty() {
        let mut state =
            InputDialog::default().with_validator(Validator::not_empty("must not be empty"));

        assert_eq!(Some("must not be empty"), state.validation_message());

        state.perform_update(Update::Enter).unwrap();
        assert!(!state.completed);

        state.perform_update(InsertChar('a')).unwrap();
        assert_eq!(None, state.validation_message());

        state.perform_update(Update::Enter).unwrap();
        assert!(state.completed);
    }
}

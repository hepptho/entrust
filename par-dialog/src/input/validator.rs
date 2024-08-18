use crate::input::InputDialog;
use std::borrow::Cow;
use std::fmt::{Debug, Formatter};
use std::ops::Add;

pub trait ValidatorFn<'m>: 'static + Fn(&[char]) -> Option<Cow<'m, str>> {}
impl<'m, P: 'static + Fn(&[char]) -> Option<Cow<'m, str>>> ValidatorFn<'m> for P {}

pub struct Validator {
    function: Box<dyn ValidatorFn<'static>>,
}

impl Validator {
    pub fn new(function: impl ValidatorFn<'static>) -> Self {
        Validator {
            function: Box::new(function),
        }
    }
}

impl<F: ValidatorFn<'static>> From<F> for Validator {
    fn from(value: F) -> Self {
        Validator::new(value)
    }
}

impl Default for Validator {
    fn default() -> Self {
        Validator::new(|_| None)
    }
}

impl Validator {
    pub fn not_empty(message: &'static str) -> Self {
        Validator::new(validate_not_empty(message))
    }
    pub fn filename() -> Self {
        Validator::new(validate_filename())
    }
}

pub fn validate_not_empty(message: &'static str) -> impl ValidatorFn<'static> {
    move |vec| {
        if vec.is_empty() {
            Some(message.into())
        } else {
            None
        }
    }
}

pub fn validate_filename() -> impl ValidatorFn<'static> {
    const WINDOWS_ILLEGAL_CHARS: &str = r#":*?"<>|"#;
    |vec| {
        if vec.is_empty() {
            return Some("Filename must not be empty".into());
        } else if vec.last() == Some(&'/') {
            return Some("Filename must not end with '/'".into());
        }
        if cfg!(windows) {
            let contains_invalid = vec.iter().any(|char| WINDOWS_ILLEGAL_CHARS.contains(*char));
            if contains_invalid {
                return Some(format!("Filename must not contain any of the following characters: {WINDOWS_ILLEGAL_CHARS}").into());
            }
        }
        None
    }
}

impl Add for Validator {
    type Output = Validator;

    fn add(self, rhs: Self) -> Self::Output {
        let combined = move |chars: &[char]| (self.function)(chars).or((rhs.function)(chars));
        Validator::new(combined)
    }
}

impl<'p, 'c> InputDialog<'p, 'c> {
    pub fn validation_message(&self) -> Option<Cow<'static, str>> {
        (self.validator.function)(&self.content)
    }
}

impl Debug for Validator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Validator")
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

        assert_eq!(Some("must not be empty".into()), state.validation_message());

        state.perform_update(Update::Confirm).unwrap();
        assert_eq!(DialogState::Pending, state.state);

        state.perform_update(InsertChar('a')).unwrap();
        assert_eq!(None, state.validation_message());

        state.perform_update(Update::Confirm).unwrap();
        assert_eq!(DialogState::Completed, state.state);
    }
}

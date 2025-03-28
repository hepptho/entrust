use crate::input::InputDialog;
use std::borrow::Cow;
use std::fmt::{Debug, Formatter};
use std::ops::Add;

pub trait ValidatorFn<'f>: 'f + Fn(&[char]) -> Option<Cow<'f, str>> {}
impl<'f, F> ValidatorFn<'f> for F where F: 'f + Fn(&[char]) -> Option<Cow<'f, str>> {}

pub struct Validator<'f> {
    function: Box<dyn ValidatorFn<'f>>,
}

impl<'f> Validator<'f> {
    pub fn new(function: impl ValidatorFn<'f>) -> Self {
        Validator {
            function: Box::new(function),
        }
    }
    pub fn not_empty(message: &'f str) -> Self {
        Validator::new(validate_not_empty(message))
    }
    pub fn filename() -> Validator<'static> {
        Validator::new(validate_filename(false))
    }
    pub fn filename_cross_platform() -> Validator<'static> {
        Validator::new(validate_filename(true))
    }
}

pub fn validate_not_empty(message: &str) -> impl ValidatorFn {
    |chars| {
        if chars.is_empty() {
            Some(Cow::Borrowed(message))
        } else {
            None
        }
    }
}

pub fn validate_filename(cross_platform: bool) -> impl ValidatorFn<'static> {
    const WINDOWS_ILLEGAL_CHARS: &str = r#":*?"<>|"#;
    move |chars| {
        if chars.is_empty() {
            return Some("Filename must not be empty".into());
        }
        if chars.last() == Some(&'/') {
            return Some("Filename must not end with '/'".into());
        }
        if cross_platform || cfg!(windows) {
            let contains_invalid = chars
                .iter()
                .any(|char| WINDOWS_ILLEGAL_CHARS.contains(*char));
            if contains_invalid {
                return Some(format!("Filename must not contain any of the following characters: {WINDOWS_ILLEGAL_CHARS}").into());
            }
        }
        if cross_platform || cfg!(unix) {
            let bytes_len = chars.iter().fold(0, |acc, e| acc + e.len_utf8());
            if bytes_len > 255 {
                return Some("Filename must not be longer than 255 bytes".into());
            }
        }
        None
    }
}

impl<'f, F> From<F> for Validator<'f>
where
    F: ValidatorFn<'f>,
{
    fn from(value: F) -> Self {
        Validator::new(value)
    }
}

impl Default for Validator<'_> {
    fn default() -> Self {
        Validator::new(|_| None)
    }
}

impl<'f> Add for Validator<'f> {
    type Output = Validator<'f>;

    fn add(self, rhs: Self) -> Self::Output {
        Validator::new(combine(self.function, rhs.function))
    }
}

pub fn combine<'a>(
    val_fn_1: impl ValidatorFn<'a>,
    val_fn_2: impl ValidatorFn<'a>,
) -> impl ValidatorFn<'a> {
    move |chars| val_fn_1(chars).or(val_fn_2(chars))
}

impl<'p, 'c> InputDialog<'p, 'c> {
    pub fn validation_message(&self) -> Option<Cow<'static, str>> {
        (self.validator.function)(&self.content)
    }
}

impl Debug for Validator<'_> {
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

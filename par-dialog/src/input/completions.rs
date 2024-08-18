use crate::input::InputDialog;

impl<'p, 'c> InputDialog<'p, 'c> {
    pub(super) fn get_full_completion(&self) -> Option<&str> {
        if self.completions.is_empty() {
            return None;
        }
        let current = self.current_content();
        self.completions
            .iter()
            .filter(|&c| c.len() > current.len())
            .filter(|&c| c.starts_with(current.as_str()))
            .min_by_key(|c| c.len())
            .map(|c| c.as_ref())
    }

    pub(super) fn get_end_completion(&self) -> Option<&str> {
        self.get_full_completion()
            .map(|c| c.trim_start_matches(self.current_content().as_str()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dialog::Dialog;
    use crate::input::Update;

    #[test]
    fn test_get_completion() {
        let dialog = InputDialog::default()
            .with_content("I held it truth")
            .with_completions(vec![
                "I held it truth, with him who sings".into(),
                "I held it truth, with him who sings / To one clear harp in divers tones".into(),
            ]);
        assert_eq!(
            Some("I held it truth, with him who sings"),
            dialog.get_full_completion(),
        );
        assert_eq!(Some(", with him who sings"), dialog.get_end_completion());

        let dialog = InputDialog::default()
            .with_content("That men may rise on stepping stones")
            .with_completions(vec!["Of their dead selves to higher things".into()]);
        assert_eq!(None, dialog.get_full_completion());

        let dialog = InputDialog::default()
            .with_content("But who shall forecast the years / And find in loss a gain to match")
            .with_completions(vec![
                "But who shall forecast the years / And find in loss a gain to match".into(),
            ]);
        assert_eq!(None, dialog.get_full_completion());
    }

    #[test]
    fn test_accept_completion() {
        let mut dialog = InputDialog::default()
            .with_content("Or reach a hand through")
            .with_completions(vec![
                "Or reach a hand thro' time to catch / The far-off interest of tears".into(),
            ]);
        dialog.perform_update(Update::MoveCursorRight).unwrap();
        assert_eq!("Or reach a hand through", dialog.current_content().as_str());

        dialog.perform_update(Update::DeleteBeforeCursor).unwrap();
        dialog.perform_update(Update::DeleteBeforeCursor).unwrap();
        dialog.perform_update(Update::DeleteBeforeCursor).unwrap();
        dialog.perform_update(Update::MoveCursorRight).unwrap();
        assert_eq!(
            "Or reach a hand thro' time to catch / The far-off interest of tears",
            dialog.current_content().as_str()
        );

        dialog.perform_update(Update::DeleteBeforeCursor).unwrap();
        assert_eq!(
            "Or reach a hand thro' time to catch / The far-off interest of tear",
            dialog.current_content().as_str()
        );
    }
}

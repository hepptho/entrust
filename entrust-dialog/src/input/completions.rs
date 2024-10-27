use crate::input::{InputDialog, Update};
use std::borrow::Cow;
use std::cell::Cell;

#[derive(Debug, Default)]
pub(super) struct Completions {
    pub list: Vec<Cow<'static, str>>,
    index: Cell<usize>,
}

impl Completions {
    fn get_full_completion(&self, current_content: &str) -> Option<&str> {
        if self.list.is_empty() {
            return None;
        }
        let options: Vec<_> = self
            .list
            .iter()
            .filter(|&c| c.len() > current_content.len())
            .filter(|&c| c.starts_with(current_content))
            .map(|c| c.as_ref())
            .collect();
        if options.len() > self.index.get() {
            Some(options[self.index.get()])
        } else {
            self.index.set(0);
            options.into_iter().next()
        }
    }

    fn get_end_completion(&self, current_content: &str) -> Option<&str> {
        self.get_full_completion(current_content)
            .map(|c| c.trim_start_matches(current_content))
    }
}

impl<'p, 'c> InputDialog<'p, 'c> {
    pub(super) fn update_completions(&mut self, update: Update) {
        if update == Update::CycleCompletions {
            self.completions.index.set(self.completions.index.get() + 1)
        } else if update == Update::CycleCompletionsBackward {
            self.completions
                .index
                .set(self.completions.index.get().saturating_sub(1))
        }
    }
    pub(super) fn get_full_completion(&self) -> Option<&str> {
        self.completions
            .get_full_completion(self.current_content().as_str())
    }

    pub(super) fn get_end_completion(&self) -> Option<&str> {
        self.completions
            .get_end_completion(self.current_content().as_str())
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

    #[test]
    fn test_cycle_completion() {
        let mut dialog = InputDialog::default()
            .with_content("I h")
            .with_completions(vec![
                "I held it truth, with him who sings".into(),
                "I hold it true, whate'er befall".into(),
            ]);
        assert_eq!(
            Some("I held it truth, with him who sings"),
            dialog.get_full_completion()
        );
        dialog.perform_update(Update::CycleCompletions).unwrap();
        assert_eq!(
            Some("I hold it true, whate'er befall"),
            dialog.get_full_completion()
        );
        dialog.perform_update(Update::CycleCompletions).unwrap();
        assert_eq!(
            Some("I held it truth, with him who sings"),
            dialog.get_full_completion()
        );

        dialog.perform_update(Update::InsertChar('a')).unwrap();
        assert_eq!(None, dialog.get_full_completion());
        dialog.perform_update(Update::CycleCompletions).unwrap();
        assert_eq!(None, dialog.get_full_completion());
    }
}

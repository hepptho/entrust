use crate::theme::{chevron_prompt, DIALOG_THEME};
use par_core::get_existing_locations;
use par_dialog::dialog::Dialog;
use par_dialog::input::confirmation::{Confirmation, ConfirmationMessageType};
use par_dialog::input::mask::InputMask;
use par_dialog::input::prompt::Prompt;
use par_dialog::input::validator::Validator;
use par_dialog::input::InputDialog;
use par_dialog::select::SelectDialog;
use std::borrow::Cow;
use std::ops::Deref;
use std::path::Path;

pub fn select_existing_key(store: &Path) -> anyhow::Result<String> {
    let vec = get_existing_locations(store)?
        .files
        .into_iter()
        .map(Cow::Owned)
        .collect();
    let selected = SelectDialog::new(vec).run()?;
    match selected {
        None => select_existing_key(store),
        Some(sel) => Ok(sel.to_string()),
    }
}

pub fn read_password_interactive(initial: &str) -> anyhow::Result<String> {
    let mask = if initial.is_empty() {
        InputMask::dots()
    } else {
        InputMask::none()
    };
    let mut dialog = InputDialog::default()
        .with_content(initial)
        .with_prompt(Prompt::inline(chevron_prompt!("Enter new password")))
        .with_validator(Validator::not_empty("The password must not be empty."))
        .with_mask(mask)
        .with_theme(DIALOG_THEME.deref());
    if initial.is_empty() {
        dialog = dialog.with_confirmation(match_confirmation())
    }
    let pass = dialog.run()?;
    Ok(pass)
}

fn match_confirmation() -> Confirmation<'static> {
    Confirmation::new(
        chevron_prompt!("Confirm password  "),
        chevron_prompt!("The entered passwords do not match"),
        ConfirmationMessageType::Inline,
    )
}

pub fn read_new_key_interactive(prompt: &'static str, store: &Path) -> anyhow::Result<String> {
    let existing = get_existing_locations(store)?;
    let predicate = move |vec: &Vec<char>| !existing.files.contains(&vec.iter().collect());
    let new_key = InputDialog::default()
        .with_prompt(Prompt::inline(prompt))
        .with_validator(Validator::new("Key already exists", predicate))
        .with_completions(existing.dirs.into_iter().map(Cow::Owned).collect())
        .with_theme(DIALOG_THEME.deref())
        .run()?;
    Ok(new_key)
}

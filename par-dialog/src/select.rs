use crate::dialog::{Dialog, Theme};
use crate::input::prompt::Prompt;
use crate::input::InputDialog;
use crate::select::filter::apply_filter;
use crate::{input, key_event_pattern as kep};
use ratatui::crossterm::event::Event::Key;
use ratatui::crossterm::event::{Event, KeyCode};
use ratatui::widgets::ListState;
use ratatui::{Frame, Viewport};
use std::borrow::Cow;
use std::mem;

mod filter;
mod widget;

#[derive(Clone, Debug)]
pub struct SelectDialog<'a> {
    items: Vec<Item<'a>>,
    list_state: ListState,
    height: u16,
    theme: &'static Theme,
    filter_dialog: Option<InputDialog>,
    completed: bool,
}

#[derive(Clone, Debug, Default)]
struct Item<'a> {
    content: Cow<'a, str>,
    index: usize,
}

impl<'a> SelectDialog<'a> {
    pub fn new(options: Vec<Cow<'a, str>>) -> Self {
        let list_state = if options.is_empty() {
            ListState::default()
        } else {
            ListState::default().with_selected(Some(0))
        };
        let items = options
            .into_iter()
            .enumerate()
            .map(|(index, content)| Item { index, content })
            .collect();
        let theme = Theme::default_ref();
        let filter = InputDialog::default()
            .with_prompt(Prompt::inline("  "))
            .with_placeholder("type to filter...")
            .into();
        SelectDialog {
            items,
            list_state,
            height: 10,
            theme,
            filter_dialog: filter,
            completed: false,
        }
    }
    pub fn with_theme(mut self, theme: &'static Theme) -> Self {
        self.theme = theme;
        self
    }
    pub fn with_height(mut self, height: u16) -> Self {
        self.height = height;
        self
    }
    pub fn without_filter_dialog(mut self) -> Self {
        self.filter_dialog = None;
        self
    }
}

pub enum Update {
    Previous,
    Next,
    Confirm,
    FilterInput(input::Update),
    Noop,
}

impl<'a> Dialog for SelectDialog<'a> {
    type Update = Update;
    type Output = Option<Cow<'a, str>>;

    fn update_for_event(event: Event) -> Self::Update {
        match event {
            Key(ke) => match ke {
                kep!(KeyCode::Up) => Update::Previous,
                kep!(KeyCode::Down) => Update::Next,
                kep!(KeyCode::Enter) => Update::Confirm,
                kep!(KeyCode::Backspace) => Update::FilterInput(input::Update::DeleteBeforeCursor),
                kep!(KeyCode::Left) => Update::FilterInput(input::Update::MoveCursorLeft),
                kep!(KeyCode::Right) => Update::FilterInput(input::Update::MoveCursorRight),
                _ => Update::FilterInput(InputDialog::update_for_event(event)),
            },
            _ => Update::Noop,
        }
    }

    fn perform_update(&mut self, update: Self::Update) -> std::io::Result<()> {
        match update {
            Update::Previous => self.list_state.select_previous(),
            Update::Next => self.list_state.select_next(),
            Update::Confirm => self.completed = true,
            Update::FilterInput(input) => {
                if let Some(ref mut filter_dialog) = self.filter_dialog {
                    filter_dialog.perform_update(input)?;
                }
            }
            Update::Noop => {}
        };
        Ok(())
    }

    fn completed(&self) -> bool {
        self.completed
    }

    fn output(mut self) -> Self::Output {
        let mut state = mem::take(&mut self.list_state);
        let mut items = mem::take(&mut self.items);
        if let Some(ref mut filter_dialog) = self.filter_dialog {
            let filtered = apply_filter(
                items.as_slice(),
                &mut state,
                filter_dialog.current_content().as_str(),
            );
            state
                .selected()
                .map(|s| filtered[s].item.index)
                .map(|i| items.swap_remove(i).content)
        } else {
            state.selected().map(|s| items.swap_remove(s).content)
        }
    }

    fn viewport(&self) -> Viewport {
        Viewport::Inline(self.height)
    }

    fn draw(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    fn tick(&mut self) -> bool {
        if let Some(ref mut filter_dialog) = self.filter_dialog {
            filter_dialog.tick()
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_filter() {
        let mut dialog = SelectDialog::new(
            "I heard the trailing garments of the Night sweep through her marble halls"
                .split(' ')
                .map(Cow::Borrowed)
                .collect(),
        );

        dialog.perform_update(Update::Next).unwrap();
        let filtered = apply_filter(
            dialog.items.as_slice(),
            &mut dialog.list_state,
            dialog
                .filter_dialog
                .as_ref()
                .unwrap()
                .current_content()
                .as_str(),
        );
        assert_eq!(13, filtered.len());
        assert_eq!(Some("heard".into()), dialog.clone().output());

        dialog
            .perform_update(Update::FilterInput(input::Update::InsertChar('t')))
            .unwrap();
        let filtered = apply_filter(
            dialog.items.as_slice(),
            &mut dialog.list_state,
            dialog
                .filter_dialog
                .as_ref()
                .unwrap()
                .current_content()
                .as_str(),
        );
        assert_eq!(6, filtered.len());
        assert_eq!(Some("trailing".into()), dialog.clone().output());

        dialog
            .perform_update(Update::FilterInput(input::Update::InsertChar('x')))
            .unwrap();
        let filtered = apply_filter(
            dialog.items.as_slice(),
            &mut dialog.list_state,
            dialog
                .filter_dialog
                .as_ref()
                .unwrap()
                .current_content()
                .as_str(),
        );
        assert_eq!(0, filtered.len());
        assert_eq!(None, dialog.clone().output());

        dialog
            .perform_update(Update::FilterInput(input::Update::DeleteBeforeCursor))
            .unwrap();
        let filtered = apply_filter(
            dialog.items.as_slice(),
            &mut dialog.list_state,
            dialog
                .filter_dialog
                .as_ref()
                .unwrap()
                .current_content()
                .as_str(),
        );
        assert_eq!(6, filtered.len());
        assert_eq!(Some("the".into()), dialog.clone().output());
    }

    #[test]
    fn test_without_filter() {
        let mut dialog = SelectDialog::new(
            "I saw her sable skirts all fringed with light from the celestial walls"
                .split(' ')
                .map(Cow::Borrowed)
                .collect(),
        )
        .without_filter_dialog();

        dialog.perform_update(Update::Next).unwrap();
        assert_eq!(Some("saw".into()), dialog.clone().output());

        dialog
            .perform_update(Update::FilterInput(input::Update::InsertChar('a')))
            .unwrap();
        assert_eq!(Some("saw".into()), dialog.clone().output());

        dialog.perform_update(Update::Previous).unwrap();
        assert_eq!(Some("I".into()), dialog.clone().output());

        for _ in 0..11 {
            dialog.perform_update(Update::Next).unwrap();
        }
        assert_eq!(Some("celestial".into()), dialog.clone().output());

        dialog.perform_update(Update::Next).unwrap();
        assert_eq!(Some("walls".into()), dialog.clone().output());
    }
}

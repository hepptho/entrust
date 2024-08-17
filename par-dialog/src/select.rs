use crate::dialog::{Dialog, DialogState};
use crate::input::prompt::Prompt;
use crate::input::InputDialog;
use crate::select::filter::get_filtered;
use crate::theme::Theme;
use crate::{cancel_key_event, input, key_event_pattern as kep};
use ratatui::crossterm::event::Event::Key;
use ratatui::crossterm::event::{Event, KeyCode, KeyModifiers};
use ratatui::widgets::ListState;
use ratatui::{Frame, Viewport};
use std::borrow::Cow;
use std::env;
use tracing::debug;

mod filter;
mod widget;

#[derive(Debug)]
pub struct SelectDialog<'a> {
    items: Vec<Item<'a>>,
    list_state: ListState,
    height: u8,
    theme: Cow<'static, Theme>,
    filter_dialog: Option<InputDialog<'static, 'static>>,
    state: DialogState,
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
        let height =
            env::var("PAR_SELECT_HEIGHT").map_or(10, |var| var.parse::<u8>().unwrap_or(10));
        let items = options
            .into_iter()
            .enumerate()
            .map(|(index, content)| Item { index, content })
            .collect();
        let theme = Cow::Borrowed(Theme::default_ref());
        let filter = InputDialog::default()
            .with_prompt(Prompt::inline("  "))
            .with_placeholder("type to filter...")
            .into();
        SelectDialog {
            items,
            list_state,
            height,
            theme,
            filter_dialog: filter,
            state: DialogState::Pending,
        }
    }
    pub fn with_theme<T: Into<Cow<'static, Theme>>>(mut self, theme: T) -> Self {
        self.theme = theme.into();
        self
    }
    pub fn with_height(mut self, height: u8) -> Self {
        self.height = height;
        self
    }
    pub fn without_filter_dialog(mut self) -> Self {
        self.filter_dialog = None;
        self
    }

    fn current_item_index(&self) -> Option<usize> {
        if let Some(filter_dialog) = &self.filter_dialog {
            let filtered = get_filtered(&self.items, &filter_dialog.current_content());
            self.list_state.selected().map(|s| filtered[s].item.index)
        } else {
            self.list_state.selected()
        }
    }
}

#[derive(Debug)]
pub enum Update {
    Previous,
    Next,
    FilterInput(input::Update),
    Confirm,
    Cancel,
}

impl<'a> Dialog for SelectDialog<'a> {
    type Update = Update;
    type Output = Option<Cow<'a, str>>;

    fn update_for_event(event: Event) -> Option<Self::Update> {
        match event {
            Key(ke) => match ke {
                cancel_key_event!() => Update::Cancel.into(),
                kep!(KeyCode::Up) => Update::Previous.into(),
                kep!(KeyCode::Down) => Update::Next.into(),
                kep!(KeyCode::Backspace) => {
                    Update::FilterInput(input::Update::DeleteBeforeCursor).into()
                }
                kep!(KeyCode::Left) => Update::FilterInput(input::Update::MoveCursorLeft).into(),
                kep!(KeyCode::Right) => Update::FilterInput(input::Update::MoveCursorRight).into(),
                kep!(KeyCode::Enter) => Update::Confirm.into(),
                _ => InputDialog::update_for_event(event).map(Update::FilterInput),
            },
            _ => None,
        }
    }

    fn perform_update(&mut self, update: Self::Update) -> std::io::Result<()> {
        debug!(?update, ?self.items, ?self.list_state);
        match update {
            Update::Previous => self.list_state.select_previous(),
            Update::Next => self.list_state.select_next(),
            Update::FilterInput(input) => {
                if let Some(ref mut filter_dialog) = self.filter_dialog {
                    filter_dialog.perform_update(input)?;
                }
            }
            Update::Confirm => self.state = DialogState::Completed,
            Update::Cancel => self.state = DialogState::Cancelled,
        };
        Ok(())
    }

    fn state(&self) -> DialogState {
        self.state
    }

    fn output(mut self) -> Self::Output {
        self.current_item_index()
            .map(|i| self.items.swap_remove(i).content)
    }

    fn viewport(&self) -> Viewport {
        Viewport::Inline(self.height as u16)
    }

    fn draw(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
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
    use crate::select::filter::apply_filter;

    fn current_selected<'a>(dialog: &'a SelectDialog) -> Option<&'a str> {
        dialog
            .current_item_index()
            .map(|i| &dialog.items[i].content)
            .map(|c| c.as_ref())
    }

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
        assert_eq!(Some("heard"), current_selected(&dialog));

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
        assert_eq!(Some("trailing"), current_selected(&dialog));

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
        assert_eq!(None, current_selected(&dialog));

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
        assert_eq!(Some("the"), current_selected(&dialog));
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
        assert_eq!(Some("saw"), current_selected(&dialog));

        dialog
            .perform_update(Update::FilterInput(input::Update::InsertChar('a')))
            .unwrap();
        assert_eq!(Some("saw"), current_selected(&dialog));

        dialog.perform_update(Update::Previous).unwrap();
        assert_eq!(Some("I"), current_selected(&dialog));

        for _ in 0..11 {
            dialog.perform_update(Update::Next).unwrap();
        }
        assert_eq!(Some("celestial"), current_selected(&dialog));

        dialog.perform_update(Update::Next).unwrap();
        assert_eq!(Some("walls"), current_selected(&dialog));
    }
}

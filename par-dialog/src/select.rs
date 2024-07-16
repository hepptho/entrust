use crate::dialog::{Dialog, Theme};
use crate::input::prompt::Prompt;
use crate::input::InputDialog;
use crate::select::filter::apply_filter;
use crate::{input, key_event_pattern as kep};
use ratatui::crossterm::event::Event::Key;
use ratatui::crossterm::event::{Event, KeyCode};
use ratatui::widgets::ListState;
use ratatui::{Frame, Viewport};
use std::mem;

mod filter;
mod widget;

#[derive(Clone, Debug, Default)]
pub struct SelectDialog<'a> {
    items: Vec<Item<'a>>,
    list_state: ListState,
    height: u16,
    theme: Theme,
    completed: bool,
    filter: InputDialog,
}

#[derive(Clone, Debug, Default)]
struct Item<'a> {
    content: &'a str,
    index: usize,
}

impl<'a> SelectDialog<'a> {
    pub fn new(options: Vec<&'a str>) -> Self {
        let list_state = if options.is_empty() {
            ListState::default()
        } else {
            ListState::default().with_selected(Some(0))
        };
        let items = options
            .iter()
            .enumerate()
            .map(|(index, &content)| Item { index, content })
            .collect();
        let theme = Theme::default();
        let filter = InputDialog::default()
            .with_prompt(Prompt::inline("  "))
            .with_placeholder("type to filter...");
        SelectDialog {
            items,
            list_state,
            height: 10,
            filter,
            theme,
            ..Self::default()
        }
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
    type Output = Option<&'a str>;

    fn update_for_event(event: Event) -> Self::Update {
        match event {
            Key(ke) => match ke {
                kep!(KeyCode::Up) => Update::Previous,
                kep!(KeyCode::Down) => Update::Next,
                kep!(KeyCode::Enter) => Update::Confirm,
                kep!(KeyCode::Char(char)) => Update::FilterInput(input::Update::InsertChar(char)),
                kep!(KeyCode::Backspace) => Update::FilterInput(input::Update::DeleteBeforeCursor),
                kep!(KeyCode::Left) => Update::FilterInput(input::Update::MoveCursorLeft),
                kep!(KeyCode::Right) => Update::FilterInput(input::Update::MoveCursorRight),
                _ => Update::Noop,
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
                self.filter.perform_update(input)?;
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
        let options = mem::take(&mut self.items);
        let filtered = apply_filter(
            options.as_slice(),
            &mut state,
            self.filter.current_content().as_str(),
        );
        state
            .selected()
            .map(|s| filtered[s].item.index)
            .map(|i| options[i].content)
    }

    fn viewport(&self) -> Viewport {
        Viewport::Inline(self.height)
    }

    fn draw(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    fn tick(&mut self) -> bool {
        self.filter.tick()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut dialog = SelectDialog::new(vec!["abc", "def", "ghi"]);

        dialog.perform_update(Update::Next).unwrap();
        let filtered = apply_filter(
            dialog.items.as_slice(),
            &mut dialog.list_state,
            dialog.filter.current_content().as_str(),
        );
        assert_eq!(3, filtered.len());
        assert_eq!(Some("def"), dialog.clone().output());

        dialog
            .perform_update(Update::FilterInput(input::Update::InsertChar('g')))
            .unwrap();
        let filtered = apply_filter(
            dialog.items.as_slice(),
            &mut dialog.list_state,
            dialog.filter.current_content().as_str(),
        );
        assert_eq!(1, filtered.len());
        assert_eq!(Some("ghi"), dialog.clone().output());

        dialog
            .perform_update(Update::FilterInput(input::Update::InsertChar('x')))
            .unwrap();
        let filtered = apply_filter(
            dialog.items.as_slice(),
            &mut dialog.list_state,
            dialog.filter.current_content().as_str(),
        );
        assert_eq!(0, filtered.len());
        assert_eq!(None, dialog.clone().output());

        dialog
            .perform_update(Update::FilterInput(input::Update::DeleteBeforeCursor))
            .unwrap();
        let filtered = apply_filter(
            dialog.items.as_slice(),
            &mut dialog.list_state,
            dialog.filter.current_content().as_str(),
        );
        assert_eq!(1, filtered.len());
        assert_eq!(Some("ghi"), dialog.clone().output());
    }
}

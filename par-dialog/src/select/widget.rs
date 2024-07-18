use crate::dialog::Theme;
use crate::select::filter::{apply_filter, FilteredItem};
use crate::select::SelectDialog;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::{Line, Span, StatefulWidget, Widget};
use ratatui::widgets::{HighlightSpacing, List, Scrollbar, ScrollbarOrientation, ScrollbarState};

impl<'a> Widget for &mut SelectDialog<'a> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let (header_area, list_area, scrollbar_area) = {
            let (header_area, list_scroll_area) = {
                let filter_height = if self.filter_dialog.is_some() { 1 } else { 0 };
                let rects = Layout::vertical(vec![
                    Constraint::Length(filter_height),
                    Constraint::Percentage(100),
                ])
                .split(area);
                (rects[0], rects[1])
            };
            let rects = Layout::horizontal(vec![Constraint::Percentage(100), Constraint::Min(1)])
                .split(list_scroll_area);
            (header_area, rects[0], rects[1])
        };

        if let Some(ref mut filter_dialog) = self.filter_dialog {
            filter_dialog.render(header_area, buf);
        }

        let lines: Vec<Line> = if let Some(ref mut filter_dialog) = self.filter_dialog {
            let filtered = apply_filter(
                self.items.as_slice(),
                &mut self.list_state,
                filter_dialog.current_content().as_str(),
            );
            filtered
                .iter()
                .map(|s| render_filtered_item(s, self.theme))
                .collect()
        } else {
            self.items.iter().map(|i| i.content.into()).collect()
        };
        let len = lines.len();

        let list = List::new(lines)
            .highlight_symbol(self.theme.input_prefix.as_str())
            .highlight_style(self.theme.selected_style)
            .highlight_spacing(HighlightSpacing::Always);
        StatefulWidget::render(list, list_area, buf, &mut self.list_state);

        if len > list_area.height as usize {
            let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight);
            let mut scrollbar_state = ScrollbarState::default()
                .content_length(len)
                .position(self.list_state.selected().unwrap_or(0));
            StatefulWidget::render(scrollbar, scrollbar_area, buf, &mut scrollbar_state);
        }
    }
}

fn render_filtered_item(item: &FilteredItem, theme: &Theme) -> Line<'static> {
    let char_spans: Vec<Span> = item
        .item
        .content
        .chars()
        .enumerate()
        .map(|(index, char)| {
            let string = char.to_string();
            if item.matching_chars.contains(&index) {
                Span::styled(string, theme.match_style)
            } else {
                Span::raw(string)
            }
        })
        .collect();
    char_spans.into()
}

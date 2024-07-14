use crate::input::InputDialog;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::{Line, Span, Text, Widget};
use ratatui::widgets::Paragraph;

impl Widget for &mut InputDialog {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let (header_area, input_area) = {
            if self.header.iter().as_slice().is_empty() {
                (None, area)
            } else {
                let rects = Layout::vertical(vec![Constraint::Min(1), Constraint::Percentage(100)])
                    .split(area);
                (Some(rects[0]), rects[1])
            }
        };

        let line = if self.hidden {
            Line::from(vec![
                self.prompt.clone(),
                String::from_iter(vec!['•'; self.content.len()]).into(),
            ])
        } else if self.content.is_empty() {
            Line::from(vec![self.prompt.clone(), self.placeholder.clone()])
        } else {
            let (before_cursor, from_cursor) = self.content.split_at(self.cursor.index());
            let at_cursor = from_cursor.iter().next().unwrap_or(&' ');
            let after_cursor: String = from_cursor.iter().skip(1).collect();
            Line::from(vec![
                self.prompt.clone(),
                before_cursor.iter().collect::<String>().into(),
                Span::styled(at_cursor.to_string(), self.cursor.style_display),
                after_cursor.into(),
            ])
        };

        if let Some(header_area) = header_area {
            Paragraph::new(Text::from(self.header.clone())).render(header_area, buf);
        }

        Paragraph::new(line).render(input_area, buf);
    }
}

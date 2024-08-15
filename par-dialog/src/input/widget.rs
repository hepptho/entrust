use crate::input::InputDialog;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::{Line, Span, Widget};
use ratatui::style::{Color, Style};
use ratatui::widgets::Paragraph;

impl Widget for &mut InputDialog {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let (header_prompt, inline_prompt) = self.prompt_with_confirmation();

        let validation_message = self.validation_message();

        let (header_area, input_area, validation_area) = {
            let header_height = if header_prompt.spans.is_empty() { 0 } else { 1 };
            let validation_height = validation_message.map(|m| m.lines().count()).unwrap_or(0);
            let rects = Layout::vertical(vec![
                Constraint::Length(header_height),
                Constraint::Length(1),
                Constraint::Length(validation_height as u16),
            ])
            .split(area);
            (rects[0], rects[1], rects[2])
        };

        let header_prompt = header_prompt.patch_style(self.theme.header_style);
        Paragraph::new(header_prompt).render(header_area, buf);

        self.render_input(buf, inline_prompt, input_area);

        if let Some(message) = validation_message {
            Paragraph::new(Line::styled(message, Style::from(Color::LightRed)))
                .render(validation_area, buf);
        }
    }
}

impl InputDialog {
    fn render_input(&self, buf: &mut Buffer, inline_prompt: Line, input_area: Rect) {
        let mut line = inline_prompt.patch_style(self.theme.prompt_style);
        if self.mask.active {
            line.push_span(String::from_iter(vec![self.mask.char; self.content.len()]))
        } else if self.content.is_empty() {
            if self.placeholder.is_empty() {
                line.push_span(Span::styled(" ", self.cursor.current_style(&self.theme)))
            } else {
                line.push_span(Span::styled(self.placeholder, self.theme.placeholder_style))
            }
        } else {
            let (before_cursor, from_cursor) = self.content.split_at(self.cursor.index());
            let at_cursor = from_cursor.iter().next().unwrap_or(&' ');
            let after_cursor: String = from_cursor.iter().skip(1).collect();
            line.push_span(Span::styled(
                before_cursor.iter().collect::<String>(),
                Style::reset(),
            ));
            line.push_span(Span::styled(
                at_cursor.to_string(),
                Style::reset().patch(self.cursor.current_style(&self.theme)),
            ));
            line.push_span(Span::styled(after_cursor, Style::reset()));
        };
        Paragraph::new(line).render(input_area, buf);
    }
}

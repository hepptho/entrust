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
        let prompt = self.prompt_with_confirmation();

        let validation_message = self.validation_message();

        let (header_area, input_area, validation_area) = {
            let header_height = if prompt.header.is_empty() { 0 } else { 1 };
            let validation_height = validation_message.map(|m| m.lines().count()).unwrap_or(0);
            let rects = Layout::vertical(vec![
                Constraint::Length(header_height),
                Constraint::Length(1),
                Constraint::Length(validation_height as u16),
            ])
            .split(area);
            (rects[0], rects[1], rects[2])
        };

        Paragraph::new(Line::styled(prompt.header, self.theme.header_style))
            .render(header_area, buf);

        self.render_input(buf, prompt.inline, input_area);

        if let Some(message) = validation_message {
            Paragraph::new(Line::styled(message, Style::from(Color::LightRed)))
                .render(validation_area, buf);
        }
    }
}

impl InputDialog {
    fn render_input(&self, buf: &mut Buffer, inline_prompt: &str, input_area: Rect) {
        let styled_prompt = Span::styled(inline_prompt, self.theme.prompt_style);
        let line = if self.mask.active {
            Line::from(vec![
                styled_prompt,
                String::from_iter(vec![self.mask.char; self.content.len()]).into(),
            ])
        } else if self.content.is_empty() {
            if self.placeholder.is_empty() {
                Line::from(vec![
                    styled_prompt,
                    Span::styled(" ", self.cursor.current_style(self.theme)),
                ])
            } else {
                Line::from(vec![
                    styled_prompt,
                    Span::styled(self.placeholder, self.theme.placeholder_style),
                ])
            }
        } else {
            let (before_cursor, from_cursor) = self.content.split_at(self.cursor.index());
            let at_cursor = from_cursor.iter().next().unwrap_or(&' ');
            let after_cursor: String = from_cursor.iter().skip(1).collect();
            Line::from(vec![
                styled_prompt,
                before_cursor.iter().collect::<String>().into(),
                Span::styled(at_cursor.to_string(), self.cursor.current_style(self.theme)),
                after_cursor.into(),
            ])
        };
        Paragraph::new(line).render(input_area, buf);
    }
}

use crate::input::InputDialog;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::{Line, Span, Widget};
use ratatui::style::{Color, Style};
use ratatui::widgets::Paragraph;

impl<'p, 'c> Widget for &mut InputDialog<'p, 'c> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let (header_prompt, inline_prompt) = self.prompt_with_confirmation();

        let validation_message = self.validation_message();

        let (header_area, input_area, validation_area) = {
            let header_height = if header_prompt.spans.is_empty() { 0 } else { 1 };
            let validation_height = validation_message
                .as_ref()
                .map(|m| m.lines().count())
                .unwrap_or(0);
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
            Paragraph::new(Line::styled(message.as_ref(), Style::from(Color::LightRed)))
                .render(validation_area, buf);
        }
    }
}

impl<'p, 'c> InputDialog<'p, 'c> {
    fn render_input(&self, buf: &mut Buffer, inline_prompt: Line, input_area: Rect) {
        let mut line = inline_prompt.patch_style(self.theme.prompt_style);
        let completion = self.get_end_completion().unwrap_or("");
        let cursor_style = Style::reset().patch(self.cursor.current_style(&self.theme));
        if self.mask.active {
            line.push_span(String::from_iter(vec![self.mask.char; self.content.len()]))
        } else if self.content.is_empty() {
            if !self.placeholder.is_empty() {
                line.push_span(Span::styled(
                    self.placeholder,
                    Style::reset().patch(self.theme.placeholder_style),
                ))
            } else if !completion.is_empty() {
                line.push_span(Span::styled(
                    &completion[0..1],
                    cursor_style.patch(self.theme.completion_style),
                ));
                if completion.len() > 1 {
                    line.push_span(Span::styled(
                        &completion[1..],
                        Style::reset().patch(self.theme.completion_style),
                    ));
                }
            } else {
                line.push_span(Span::styled(" ", cursor_style));
            }
        } else {
            let (before_cursor, from_cursor) = self.content.split_at(self.cursor.index());
            let completion_first_char = completion.chars().next();
            let (at_cursor, is_cursor_at_completion) = {
                if from_cursor.is_empty() {
                    (completion_first_char.unwrap_or(' '), true)
                } else {
                    (*from_cursor.iter().next().unwrap(), false)
                }
            };
            let after_cursor: String = from_cursor.iter().skip(1).collect();
            line.push_span(Span::styled(
                before_cursor.iter().collect::<String>(),
                Style::reset(),
            ));
            let at_cursor_style = if is_cursor_at_completion {
                cursor_style.patch(self.theme.completion_style)
            } else {
                cursor_style
            };
            line.push_span(Span::styled(at_cursor.to_string(), at_cursor_style));
            line.push_span(Span::styled(after_cursor, Style::reset()));
            if !completion.is_empty() {
                let remaining_completion = if is_cursor_at_completion {
                    &completion[1..]
                } else {
                    completion
                };
                line.push_span(Span::styled(
                    remaining_completion,
                    Style::reset().patch(self.theme.completion_style),
                ))
            }
        };
        Paragraph::new(line).render(input_area, buf);
    }
}

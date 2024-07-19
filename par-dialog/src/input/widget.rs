use crate::input::confirmation::ConfirmationMessageType;
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
        let (header_prompt, inline_prompt) = self
            .confirmation
            .as_ref()
            .and_then(|c| {
                if c.first_input.is_some() {
                    Some(c)
                } else {
                    None
                }
            })
            .map(|c| match c.message_type {
                ConfirmationMessageType::Header => (c.message(), self.prompt.inline),
                ConfirmationMessageType::Inline => (self.prompt.header, c.message()),
            })
            .unwrap_or((self.prompt.header, self.prompt.inline));

        let validation_message = self.validation_message();

        let (header_area, input_area, validation_area) = {
            let header_height = if header_prompt.is_empty() { 0 } else { 1 };
            let validation_height = validation_message.map(|m| m.lines().count()).unwrap_or(0);
            let rects = Layout::vertical(vec![
                Constraint::Length(header_height),
                Constraint::Length(1),
                Constraint::Length(validation_height as u16),
            ])
            .split(area);
            (rects[0], rects[1], rects[2])
        };

        // region render header
        Paragraph::new(Line::styled(header_prompt, self.theme.header_style))
            .render(header_area, buf);
        // endregion

        // region render line
        let styled_prompt = Span::styled(inline_prompt, self.theme.prompt_style);
        let line = if self.hidden {
            Line::from(vec![
                styled_prompt,
                String::from_iter(vec!['•'; self.content.len()]).into(),
            ])
        } else if self.content.is_empty() {
            Line::from(vec![
                styled_prompt,
                Span::styled(self.placeholder, self.theme.placeholder_style),
            ])
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
        // endregion

        // region validation
        if let Some(message) = validation_message {
            Paragraph::new(Line::styled(message, Style::from(Color::LightRed)))
                .render(validation_area, buf);
        }
        // endregion
    }
}

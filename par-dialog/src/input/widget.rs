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

        let input_spans = self.input_spans(inline_prompt);
        Paragraph::new(Line::from(input_spans)).render(input_area, buf);

        if let Some(message) = validation_message {
            Paragraph::new(Line::styled(message.as_ref(), Style::from(Color::LightRed)))
                .render(validation_area, buf);
        }
    }
}

impl<'p, 'c> InputDialog<'p, 'c> {
    fn input_spans<'l>(&'l self, inline_prompt: Line<'l>) -> Vec<Span<'l>> {
        let mut spans = self.inline_prompt_spans(inline_prompt);
        let completion = self.get_end_completion().unwrap_or("");
        let cursor_style = self.cursor.current_style(&self.theme);
        if self.mask.active {
            spans.push(String::from_iter(vec![self.mask.char; self.content.len()]).into())
        } else if self.content.is_empty() {
            if !self.placeholder.is_empty() {
                spans.push(Span::styled(self.placeholder, self.theme.placeholder_style))
            } else if !completion.is_empty() {
                spans.push(Span::styled(
                    &completion[0..1],
                    cursor_style.patch(self.theme.completion_style),
                ));
                if completion.len() > 1 {
                    spans.push(Span::styled(&completion[1..], self.theme.completion_style));
                }
            } else {
                spans.push(Span::styled(" ", cursor_style));
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
            spans.push(Span::raw(before_cursor.iter().collect::<String>()));
            let at_cursor_style = if is_cursor_at_completion {
                cursor_style.patch(self.theme.completion_style)
            } else {
                cursor_style
            };
            spans.push(Span::styled(at_cursor.to_string(), at_cursor_style));
            spans.push(Span::raw(after_cursor));
            if !completion.is_empty() {
                let remaining_completion = if is_cursor_at_completion {
                    &completion[1..]
                } else {
                    completion
                };
                spans.push(Span::styled(
                    remaining_completion,
                    self.theme.completion_style,
                ))
            }
        };
        spans
    }

    fn inline_prompt_spans<'s>(&'s self, inline_prompt: Line<'s>) -> Vec<Span<'s>> {
        let prompt_line_style = inline_prompt.style;
        inline_prompt
            .spans
            .into_iter()
            .map(|s| {
                let span_style = s.style;
                s.style(
                    self.theme
                        .prompt_style
                        .patch(prompt_line_style)
                        .patch(span_style),
                )
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dialog::Dialog;
    use crate::input::prompt::Prompt;
    use crate::theme::Theme;
    use ratatui::prelude::*;

    #[test]
    fn test_input_line_prompt() {
        let dialog = InputDialog::default()
            .with_content("content")
            .with_prompt(Prompt::inline("inline".bold()));
        let spans = dialog.input_spans(dialog.prompt_with_confirmation().1);
        assert_eq!(4, spans.len());
        let inline_prompt_style = Theme::default().prompt_style.patch(Style::new().bold());
        assert_eq!(inline_prompt_style, spans[0].style);
        assert_eq!("inline", spans[0].content.as_ref());

        assert_eq!(Style::default(), spans[1].style);
        assert_eq!("content", spans[1].content.as_ref());
    }

    #[test]
    fn test_input_line_placeholder() {
        let dialog = InputDialog::default().with_placeholder("placeholder");
        let spans = dialog.input_spans(dialog.prompt_with_confirmation().1);
        assert_eq!(1, spans.len());
        assert_eq!(Theme::default().placeholder_style, spans[0].style);
        assert_eq!("placeholder", spans[0].content);
    }

    #[test]
    fn test_input_line_cursor() {
        let mut dialog = InputDialog::default();
        {
            let spans = dialog.input_spans(dialog.prompt_with_confirmation().1);
            assert_eq!(1, spans.len());
            assert_eq!(Theme::default().cursor_on_style, spans[0].style);
            assert_eq!(" ", spans[0].content);
        }
        dialog.tick();
        {
            let spans = dialog.input_spans(dialog.prompt_with_confirmation().1);
            assert_eq!(1, spans.len());
            assert_eq!(Theme::default().cursor_off_style, spans[0].style);
            assert_eq!(" ", spans[0].content);
        }
    }

    #[test]
    fn test_input_line_completion() {
        let theme = Theme::default();
        let dialog = InputDialog::default()
            .with_content("So it is, and so it will be")
            .with_completions(vec![
                "So it is, and so it will be, for so it has been, time out of mind".into(),
            ]);
        let spans = dialog.input_spans(dialog.prompt_with_confirmation().1);
        assert_eq!(4, spans.len());
        // before cursor
        assert_eq!(Style::default(), spans[0].style);
        assert_eq!("So it is, and so it will be", spans[0].content.as_ref());
        // cursor
        assert_eq!(
            theme.cursor_on_style.patch(theme.completion_style),
            spans[1].style
        );
        assert_eq!(",", spans[1].content.as_ref());
        // after cursor
        assert_eq!(Style::default(), spans[2].style);
        assert_eq!("", spans[2].content.as_ref());
        // remaining completion
        assert_eq!(theme.completion_style, spans[3].style);
        assert_eq!(
            " for so it has been, time out of mind",
            spans[3].content.as_ref()
        );
    }
}

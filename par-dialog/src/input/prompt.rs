use ratatui::text::Line;

#[derive(Debug, Default, Clone)]
pub struct Prompt<'p> {
    pub(super) header: Line<'p>,
    pub(super) inline: Line<'p>,
}

impl<'p> Prompt<'p> {
    pub fn new<H, I>(header: H, inline: I) -> Self
    where
        H: Into<Line<'p>>,
        I: Into<Line<'p>>,
    {
        Prompt {
            header: header.into(),
            inline: inline.into(),
        }
    }
    pub fn header<H>(header: H) -> Self
    where
        H: Into<Line<'p>>,
    {
        Prompt::new(header, "")
    }
    pub fn inline<I>(inline: I) -> Self
    where
        I: Into<Line<'p>>,
    {
        Prompt::new("", inline)
    }
}

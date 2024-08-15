use std::borrow::Cow;

#[derive(Debug, Default, Clone)]
pub struct Prompt<'p> {
    pub(super) header: Cow<'p, str>,
    pub(super) inline: Cow<'p, str>,
}

impl<'p> Prompt<'p> {
    pub fn new<H, I>(header: H, inline: I) -> Self
    where
        H: Into<Cow<'p, str>>,
        I: Into<Cow<'p, str>>,
    {
        Prompt {
            header: header.into(),
            inline: inline.into(),
        }
    }
    pub fn header<H>(header: H) -> Self
    where
        H: Into<Cow<'p, str>>,
    {
        Prompt::new(header, "")
    }
    pub fn inline<I>(inline: I) -> Self
    where
        I: Into<Cow<'p, str>>,
    {
        Prompt::new("", inline)
    }
}

use std::borrow::Cow;

#[derive(Debug, Default, Clone)]
pub struct Prompt {
    pub(super) header: Cow<'static, str>,
    pub(super) inline: Cow<'static, str>,
}

impl Prompt {
    pub fn new<H, I>(header: H, inline: I) -> Self
    where
        H: Into<Cow<'static, str>>,
        I: Into<Cow<'static, str>>,
    {
        Prompt {
            header: header.into(),
            inline: inline.into(),
        }
    }
    pub fn header<H>(header: H) -> Self
    where
        H: Into<Cow<'static, str>>,
    {
        Prompt::new(header, "")
    }
    pub fn inline<I>(inline: I) -> Self
    where
        I: Into<Cow<'static, str>>,
    {
        Prompt::new("", inline)
    }
}

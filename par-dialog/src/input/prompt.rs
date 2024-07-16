#[derive(Debug, Default, Clone)]
pub struct Prompt {
    pub(super) header: &'static str,
    pub(super) inline: &'static str,
}

impl Prompt {
    pub fn new(header: &'static str, inline: &'static str) -> Self {
        Prompt { header, inline }
    }
    pub fn header(header: &'static str) -> Self {
        Prompt { header, inline: "" }
    }
    pub fn inline(inline: &'static str) -> Self {
        Prompt { header: "", inline }
    }
}

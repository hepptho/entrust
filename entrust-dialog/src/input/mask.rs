#[derive(Clone, Copy, Debug)]
pub struct InputMask {
    pub(super) char: char,
    pub(super) active: bool,
}

impl InputMask {
    pub fn dots() -> Self {
        '•'.into()
    }

    pub fn whitespace() -> Self {
        ' '.into()
    }

    pub fn none() -> Self {
        Self {
            char: ' ',
            active: false,
        }
    }

    pub(super) fn toggle(&mut self) {
        self.active = !self.active;
    }
}

impl From<char> for InputMask {
    fn from(value: char) -> Self {
        InputMask {
            char: value,
            active: true,
        }
    }
}

impl Default for InputMask {
    fn default() -> Self {
        Self::none()
    }
}

use once_cell::sync::Lazy;
use ratatui::backend::CrosstermBackend;
use ratatui::crossterm::event;
use ratatui::crossterm::event::Event;
use ratatui::prelude::{Style, Stylize};
use ratatui::{Frame, Terminal, TerminalOptions, Viewport};
use std::io;
use std::ops::Deref;
use std::time::{Duration, Instant};

pub trait Dialog: Sized {
    type Update;
    type Output;
    fn update_for_event(event: Event) -> Option<Self::Update>;
    fn perform_update(&mut self, update: Self::Update) -> io::Result<()>;
    fn state(&self) -> DialogState;
    fn output(self) -> Self::Output;
    fn viewport(&self) -> Viewport;
    fn draw(&mut self, frame: &mut Frame);
    fn run(mut self) -> io::Result<Self::Output> {
        let backend = CrosstermBackend::new(io::stderr());
        let mut terminal = Terminal::with_options(
            backend,
            TerminalOptions {
                viewport: self.viewport(),
            },
        )?;
        let start = Instant::now();
        let mut up_to_date = false;
        let mut timed_out = false;
        while self.state() == DialogState::Pending {
            if !up_to_date {
                terminal.draw(|frame| self.draw(frame))?;
            }

            let event_available = event::poll(self.tick_rate())?;
            if self
                .timeout()
                .is_some_and(|timeout| start.elapsed() > timeout)
            {
                timed_out = true;
                break;
            }
            if !event_available {
                up_to_date = !self.tick();
                continue;
            }

            let event = event::read()?;
            if let Some(update) = Self::update_for_event(event) {
                self.perform_update(update)?;
                up_to_date = false;
            } else {
                up_to_date = true;
            }
        }
        terminal.clear()?;
        if timed_out {
            Err(io::Error::other("timed out"))
        } else if self.state() == DialogState::Cancelled {
            Err(io::Error::other("cancelled"))
        } else {
            Ok(self.output())
        }
    }
    fn tick(&mut self) -> bool {
        false
    }
    fn tick_rate(&self) -> Duration {
        Duration::from_millis(500)
    }
    fn timeout(&self) -> Option<Duration> {
        None
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum DialogState {
    #[default]
    Pending,
    Completed,
    Cancelled,
}

#[derive(Clone, Debug)]
pub struct Theme {
    pub cursor_on_style: Style,
    pub cursor_off_style: Style,
    pub header_style: Style,
    pub select_indicator: String,
    pub placeholder_style: Style,
    pub prompt_style: Style,
    pub selected_style: Style,
    pub match_style: Style,
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            cursor_on_style: Style::new().black().on_light_blue(),
            cursor_off_style: Style::new(),
            header_style: Style::new().light_blue().bold(),
            placeholder_style: Style::new().dim(),
            prompt_style: Style::new().light_yellow().bold(),
            select_indicator: "❯ ".to_string(),
            selected_style: Style::new().bold(),
            match_style: Style::new().light_yellow().underlined(),
        }
    }
}

impl Default for &'static Theme {
    fn default() -> Self {
        Theme::default_ref()
    }
}

impl Theme {
    pub fn default_ref() -> &'static Theme {
        static THEME: Lazy<Theme> = Lazy::new(Theme::default);
        THEME.deref()
    }
}

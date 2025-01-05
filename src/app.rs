use ratatui::{prelude::*, widgets::*};
use std::io::{self, stdout, Error};
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode};

#[derive(PartialEq, Default)]
pub enum CurrentScreen {
    #[default]
    Main,
    Editing,
    Quit,
}

pub enum CurrentlyEditing {
    Url,
    Verb,
}

pub struct App {
    pub url_input: String,
    pub verb_input: String,
    pub current_screen: CurrentScreen,
    pub currently_editing: Option<CurrentlyEditing>,
    pub response_data: Option<String>,
}

#[derive(Default)]
pub struct App2 {
    pub current_screen: CurrentScreen,
}

impl App2 {
    pub fn run(&mut self, terminal: &mut Terminal<impl Backend>) -> Result<(), Error> {
        while self.is_running() {
            self.draw(terminal)?;
            self.handle_input()?;
        }
        Ok(())
    }

    fn is_running(&self) -> bool {
        self.current_screen != CurrentScreen::Quit
    }

    fn draw(&self, terminal: &mut Terminal<impl Backend>) -> Result<(), Error> {
        terminal.draw(|frame| {
            frame.render_widget(self, frame.area());
        })?;
        Ok(())
    }

    pub fn handle_input(&mut self) -> Result<(), Error> {
        let timeout = Duration::from_secs_f64(1.0 / 50.0);
        if !event::poll(timeout)? {
            return Ok(());
        }

        if let Event::Key(key) = event::read()? {
            match self.current_screen {
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('q') => {
                        self.current_screen = CurrentScreen::Quit;
                    }
                    _ => {}
                },
                CurrentScreen::Editing => match key.code {
                    KeyCode::Char('q') => {
                        self.current_screen = CurrentScreen::Quit;
                    }
                    _ => {}
                },
                CurrentScreen::Quit => {
                    return Ok(());
                }
            }
        }

        Ok(())
    }
}

impl Widget for &App2 {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vertical = Layout::vertical([Constraint::Length(3), Constraint::Min(1)]);

        let [title, tabs] = vertical.areas(area);

        let title_block = Block::bordered();

        let title_text =
            Paragraph::new("POSTMAN TUI - You Can Request!".green().bold()).block(title_block);
        title_text.render(title, buf);

        let tabs_block = Block::bordered();

        let tabs_text = Paragraph::new("tabs".green().bold()).block(tabs_block);
        tabs_text.render(tabs, buf);
    }
}

impl App {
    pub fn new() -> App {
        App {
            url_input: String::new(),
            verb_input: String::new(),
            current_screen: CurrentScreen::Main,
            currently_editing: None,
            response_data: None,
        }
    }

    pub fn make_request(&mut self) {
        let response = reqwest::blocking::get(&self.url_input)
            .unwrap()
            .text()
            .unwrap();

        self.response_data = Some(response);

        self.current_screen = CurrentScreen::Main;
        self.currently_editing = None;
    }
}

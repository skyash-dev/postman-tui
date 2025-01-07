use ratatui::{prelude::*, widgets::*};
use std::io::Error;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode};

use crate::tabs::Tab;

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

pub struct App2 {
    pub current_screen: CurrentScreen,
    pub active_tab: usize,
    pub tabs: Vec<Tab>,
}

impl Default for App2 {
    fn default() -> Self {
        let tab1 = Tab {
            verb_input: "GET".to_string(),
            url_input: "https://jsonplaceholder.typicode.com/posts".to_string(),
            response_data: None,
        };
        let tab2 = Tab {
            verb_input: "POST".to_string(),
            url_input: "https://jsonplaceholder.typicode.com/comments".to_string(),
            response_data: None,
        };

        App2 {
            current_screen: CurrentScreen::Main,
            active_tab: 0,
            tabs: vec![tab1, tab2],
        }
    }
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
                    KeyCode::Tab => {
                        self.active_tab = (self.active_tab + 1) % self.tabs.len();
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

        self.render_header(title, buf);
        self.render_tabs(tabs, buf);
    }
}

impl App2 {
    pub fn render_tabs(&self, area: Rect, buf: &mut Buffer) {
        // let title_block = Block::bordered();

        // let title_text =
        //     Paragraph::new("POSTMAN TUI - You Can Request!".green().bold()).block(title_block);
        // title_text.render(area, buf);

        self.tabs[self.active_tab].render(area, buf);
    }

    pub fn render_header(&self, area: Rect, buf: &mut Buffer) {
        let tab_titles: Vec<&str> = self
            .tabs
            .iter()
            .map(|tab| tab.verb_input.as_str())
            .collect();

        let tabs_block = Block::bordered().title("Requests");
        let tabs = Tabs::new(tab_titles)
            .select(self.active_tab)
            .block(tabs_block);

        tabs.render(area, buf);
    }
}

pub struct App {
    pub url_input: String,
    pub verb_input: String,
    pub current_screen: CurrentScreen,
    pub currently_editing: Option<CurrentlyEditing>,
    pub response_data: Option<String>,
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

use ratatui::{prelude::*, widgets::*};
use std::io::Error;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode};

use crate::tabs::{RequestInformation, Tab};

#[derive(PartialEq, Default)]
pub enum CurrentScreen {
    #[default]
    Main,
    Editing,
    Quit,
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub active_tab: usize,
    pub tabs: Vec<Tab>,
}

impl Default for App {
    fn default() -> Self {
        let tab = Tab::default();
        App {
            current_screen: CurrentScreen::Main,
            active_tab: 0,
            tabs: vec![tab],
        }
    }
}

impl App {
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
                    KeyCode::Char('l') | KeyCode::Tab => {
                        self.active_tab = (self.active_tab + 1) % self.tabs.len();
                    }
                    KeyCode::Char('h') | KeyCode::BackTab => {
                        if self.active_tab == 0 {
                            self.active_tab = self.tabs.len() - 1;
                        } else {
                            self.active_tab = (self.active_tab - 1) % self.tabs.len();
                        }
                    }
                    KeyCode::Char('u') => {
                        self.current_screen = CurrentScreen::Editing;
                        self.tabs[self.active_tab].currently_editing =
                            Some(RequestInformation::Url);
                    }
                    KeyCode::Char('v') => {
                        self.current_screen = CurrentScreen::Editing;
                        self.tabs[self.active_tab].currently_editing =
                            Some(RequestInformation::Verb);
                    }

                    KeyCode::Char('n') => {
                        self.tabs.push(Tab::default());
                    }

                    KeyCode::Delete => {
                        if self.active_tab == (self.tabs.len() - 1) {
                            self.tabs.pop();
                            self.active_tab = 0;
                        } else {
                            self.tabs.remove(self.active_tab);
                        }
                    }
                    _ => {
                        self.tabs[self.active_tab].handle_event(Event::Key(key));
                    }
                },

                CurrentScreen::Editing => match key.code {
                    KeyCode::Esc => {
                        self.current_screen = CurrentScreen::Main;
                        self.tabs[self.active_tab].currently_editing = None;
                    }
                    _ => {
                        let tab = &mut self.tabs[self.active_tab];
                        tab.handle_event(Event::Key(key));
                    }
                },
                CurrentScreen::Quit => {
                    return Ok(());
                }
            }
        }

        Ok(())
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vertical = Layout::vertical([Constraint::Length(3), Constraint::Min(1)]);

        let [title, tabs] = vertical.areas(area);

        self.render_header(title, buf);
        self.render_tabs(tabs, buf);
    }
}

impl App {
    pub fn render_tabs(&self, area: Rect, buf: &mut Buffer) {
        // let title_block = Block::bordered();

        // let title_text =
        //     Paragraph::new("POSTMAN TUI - You Can Request!".green().bold()).block(title_block);
        // title_text.render(area, buf);

        self.tabs[self.active_tab].render(area, buf);
    }

    pub fn render_header(&self, area: Rect, buf: &mut Buffer) {
        let tab_titles: Vec<&str> = self.tabs.iter().map(|tab| tab.verb_input.value()).collect();

        let tabs_block = Block::bordered().title("Requests");
        let tabs = Tabs::new(tab_titles)
            .select(self.active_tab)
            .block(tabs_block);

        tabs.render(area, buf);
    }
}

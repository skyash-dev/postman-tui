use std::{error::Error, io};

use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};

mod app;
mod tabs;
mod ui;

use crate::{
    app::{App, App2, CurrentScreen},
    tabs::RequestInformation,
    ui::ui,
};

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr(); // This is a special case. Normally using stdout is fine
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    // let mut app = App::new();
    // let _res = run_app(&mut terminal, &mut app);

    let mut app = App2::default().run(&mut terminal)?;

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }
            match app.current_screen {
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('u') => {
                        app.current_screen = CurrentScreen::Editing;
                        app.currently_editing = Some(RequestInformation::Url);
                    }
                    KeyCode::Char('v') => {
                        app.current_screen = CurrentScreen::Editing;
                        app.currently_editing = Some(RequestInformation::Verb);
                    }
                    KeyCode::Char('q') => {
                        return Ok(true);
                    }
                    _ => {}
                },
                CurrentScreen::Editing if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Enter => {
                        app.current_screen = CurrentScreen::Main;
                        app.make_request();
                    }
                    KeyCode::Esc => {
                        app.current_screen = CurrentScreen::Main;
                        app.currently_editing = None;
                    }
                    KeyCode::Tab => match app.currently_editing {
                        Some(RequestInformation::Verb) => {
                            app.currently_editing = Some(RequestInformation::Url)
                        }
                        Some(RequestInformation::Url) => {
                            app.currently_editing = Some(RequestInformation::Verb)
                        }
                        None => app.currently_editing = None,
                    },
                    KeyCode::Char(value) => {
                        if let Some(editing) = &app.currently_editing {
                            match editing {
                                RequestInformation::Verb => {
                                    app.verb_input.push(value);
                                }
                                RequestInformation::Url => {
                                    app.url_input.push(value);
                                }
                            }
                        }
                    }
                    KeyCode::Backspace => {
                        if let Some(editing) = &app.currently_editing {
                            match editing {
                                RequestInformation::Verb => app.verb_input.pop(),
                                RequestInformation::Url => app.url_input.pop(),
                            };
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
}

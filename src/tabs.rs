use ratatui::{prelude::*, widgets::*};
use tui_input::{backend::crossterm::EventHandler, Input};

use crossterm::event::{Event, KeyCode};
use serde_json::{from_str, to_string_pretty, Value};

pub enum RequestInformation {
    Url,
    Verb,
}

#[derive(Default)]
pub struct Tab {
    pub verb_input: Input,
    pub url_input: Input,
    pub response_data: Option<String>,
    pub currently_editing: Option<RequestInformation>,
    pub scroll_position: (u16, u16),
}

impl Tab {
    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        let vertical = Layout::vertical([Constraint::Length(3), Constraint::Min(1)]);

        let [header, body] = vertical.areas(area);

        self.render_inputs(header, buf);
        self.render_body(body, buf);
    }

    pub fn render_body(&self, area: Rect, buf: &mut Buffer) {
        let text = self.response_data.clone().unwrap_or_default();

        let json: Value = from_str(text.as_str()).unwrap_or_default();
        let pretty_data = to_string_pretty(&json).unwrap();

        let data_box = Paragraph::new(pretty_data)
            .block(Block::default().borders(Borders::ALL))
            .wrap(Wrap { trim: false })
            .scroll(self.scroll_position);
        data_box.render(area, buf);
    }

    pub fn render_inputs(&self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::horizontal([Constraint::Length(10), Constraint::Min(1)]);

        let [verb_input, url_input] = layout.areas(area);

        let verb_text = self.verb_input.value();
        let get = Span::styled(
            verb_text,
            Style::default().fg(Color::White).bg(Color::Green),
        );
        let verb_select =
            Paragraph::new(Line::from(vec![get])).block(Block::default().borders(Borders::ALL));

        let url_text = self.url_input.value();

        let url = Span::styled(url_text, Style::default().fg(Color::White));
        let url_box =
            Paragraph::new(Line::from(vec![url])).block(Block::default().borders(Borders::ALL));

        verb_select.render(verb_input, buf);
        url_box.render(url_input, buf);
    }

    pub fn handle_event(&mut self, key_event: Event) {
        if let Some(editing) = &self.currently_editing {
            match editing {
                RequestInformation::Verb => {
                    self.verb_input.handle_event(&key_event);
                }
                RequestInformation::Url => {
                    self.url_input.handle_event(&key_event);
                }
            }
            if let Event::Key(key) = key_event {
                match key.code {
                    KeyCode::Enter => {
                        self.make_request();
                    }
                    KeyCode::Down => {
                        self.scroll_position = (self.scroll_position.0.saturating_add(1), 0)
                    }
                    KeyCode::Up => {
                        self.scroll_position = (self.scroll_position.0.saturating_sub(1), 0)
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn make_request(&mut self) {
        let response = reqwest::blocking::get(&self.url_input.to_string())
            .unwrap()
            .text()
            .unwrap();

        self.response_data = Some(response);

        self.currently_editing = None;
    }
}

use ratatui::{prelude::*, widgets::*};

pub enum RequestInformation {
    Url,
    Verb,
}

pub struct Tab {
    pub verb_input: String,
    pub url_input: String,
    pub response_data: Option<String>,
    pub currently_editing: Option<RequestInformation>,
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
        let url = Span::styled(text, Style::default().fg(Color::White));
        let url_box =
            Paragraph::new(Line::from(vec![url])).block(Block::default().borders(Borders::ALL));
        url_box.render(area, buf);
    }

    pub fn render_inputs(&self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::horizontal([Constraint::Length(10), Constraint::Min(1)]);

        let [verb_input, url_input] = layout.areas(area);

        let verb_text = self.verb_input.clone();
        let get = Span::styled(
            verb_text,
            Style::default().fg(Color::White).bg(Color::Green),
        );
        let verb_select =
            Paragraph::new(Line::from(vec![get])).block(Block::default().borders(Borders::ALL));

        let url_text = self.url_input.clone();

        let url = Span::styled(url_text, Style::default().fg(Color::White));
        let url_box =
            Paragraph::new(Line::from(vec![url])).block(Block::default().borders(Borders::ALL));

        verb_select.render(verb_input, buf);
        url_box.render(url_input, buf);
    }

    pub fn make_request(&mut self) {
        let response = reqwest::blocking::get(&self.url_input)
            .unwrap()
            .text()
            .unwrap();

        self.response_data = Some(response);

        self.currently_editing = None;
    }
}

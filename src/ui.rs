use std::vec;

use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::App;

pub fn ui(frame: &mut Frame, _app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(1),
        ])
        .split(frame.area());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "POSTMAN TUI - You Can Request!",
        Style::default().fg(Color::Green),
    ))
    .block(title_block);

    frame.render_widget(title, chunks[0]);

    // let tabs = Tabs::new(vec!["Request 1"])
    //     .block(Block::bordered())
    //     .style(Style::default().white())
    //     .highlight_style(Style::default().yellow())
    //     .divider("|");

    // frame.render_widget(tabs, chunks[1]);

    let request_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(10), Constraint::Percentage(90)])
        .split(chunks[1]);

    let verb_text = _app.verb_input.clone();
    let get = Span::styled(
        verb_text,
        Style::default().fg(Color::White).bg(Color::Green),
    );
    let verb_select =
        Paragraph::new(Line::from(vec![get])).block(Block::default().borders(Borders::ALL));

    let url_text = _app.url_input.clone();

    let url = Span::styled(url_text, Style::default().fg(Color::White));
    let url_box =
        Paragraph::new(Line::from(vec![url])).block(Block::default().borders(Borders::ALL));

    frame.render_widget(verb_select, request_layout[0]);
    frame.render_widget(url_box, request_layout[1]);

    let response_text = _app.response_data.clone().unwrap_or("".to_string());

    let response_data = Paragraph::new(Text::styled(
        response_text,
        Style::default().fg(Color::Green),
    ))
    .block(Block::default().borders(Borders::ALL));

    frame.render_widget(response_data, chunks[2]);
}

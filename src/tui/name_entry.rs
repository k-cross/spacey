//! Name entry UI for high scores

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    prelude::*,
    widgets::{Block, Paragraph},
};

const PHOSPHOR_GREEN: Color = Color::Rgb(0, 200, 0);
const PHOSPHOR_GREEN_DIM: Color = Color::Rgb(0, 120, 0);

/// Run the name entry screen
pub fn run<B: Backend>(terminal: &mut Terminal<B>, score: u32) -> Result<Option<String>> {
    let mut name = String::new();

    loop {
        terminal.draw(|frame| render(frame, score, &name))?;

        if event::poll(std::time::Duration::from_millis(100))?
            && let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
        {
            match key.code {
                KeyCode::Char(c) => {
                    if name.len() < 10 {
                        name.push(c.to_ascii_uppercase());
                    }
                }
                KeyCode::Backspace => {
                    name.pop();
                }
                KeyCode::Enter => {
                    if !name.is_empty() {
                        return Ok(Some(name));
                    }
                }
                KeyCode::Esc => {
                    return Ok(None);
                }
                _ => {}
            }
        }
    }
}

fn render(frame: &mut Frame, score: u32, name: &str) {
    let area = frame.area();

    let block = Block::default().style(Style::default().bg(Color::Black));
    frame.render_widget(block, area);

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(area.height.saturating_sub(5) / 2),
            Constraint::Length(5),
            Constraint::Min(0),
        ])
        .split(area);

    let content_area = layout[1];

    let lines = vec![
        Line::from(Span::styled(
            "NEW HIGH SCORE!",
            Style::default().fg(PHOSPHOR_GREEN).bold(),
        )),
        Line::from(Span::styled(
            format!("SCORE: {}", score),
            Style::default().fg(PHOSPHOR_GREEN),
        )),
        Line::from(Span::raw("")),
        Line::from(vec![
            Span::styled("ENTER NAME: ", Style::default().fg(PHOSPHOR_GREEN_DIM)),
            Span::styled(
                format!("{}_", name),
                Style::default().fg(PHOSPHOR_GREEN).bold(),
            ),
        ]),
    ];

    let para = Paragraph::new(lines).alignment(Alignment::Center);
    frame.render_widget(para, content_area);
}

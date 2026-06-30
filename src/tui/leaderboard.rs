//! Leaderboard UI for high scores

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    prelude::*,
    widgets::{Block, Paragraph},
};

const PHOSPHOR_GREEN: Color = Color::Rgb(0, 200, 0);
const PHOSPHOR_GREEN_DIM: Color = Color::Rgb(0, 120, 0);

/// Run the leaderboard screen
pub fn run<B: Backend>(terminal: &mut Terminal<B>, high_scores: &[(String, u32)]) -> Result<()> {
    loop {
        terminal.draw(|frame| render(frame, high_scores))?;

        if event::poll(std::time::Duration::from_millis(100))?
            && let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
        {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc | KeyCode::Enter => return Ok(()),
                _ => {}
            }
        }
    }
}

fn render(frame: &mut Frame, high_scores: &[(String, u32)]) {
    let area = frame.area();

    let block = Block::default().style(Style::default().bg(Color::Black));
    frame.render_widget(block, area);

    let list_height = (high_scores.len() as u16).max(1);
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(area.height.saturating_sub(list_height + 4) / 2),
            Constraint::Length(2), // Title
            Constraint::Length(list_height),
            Constraint::Length(2), // Footer padding
            Constraint::Min(0),
        ])
        .split(area);

    let title = Paragraph::new("HIGH SCORES")
        .style(Style::default().fg(PHOSPHOR_GREEN).bold())
        .alignment(Alignment::Center);
    frame.render_widget(title, layout[1]);

    let mut lines = Vec::new();
    if high_scores.is_empty() {
        lines.push(Line::from(Span::styled(
            "NO SCORES YET",
            Style::default().fg(PHOSPHOR_GREEN_DIM),
        )));
    } else {
        for (i, (name, score)) in high_scores.iter().enumerate() {
            lines.push(Line::from(vec![
                Span::styled(
                    format!("{:2}. ", i + 1),
                    Style::default().fg(PHOSPHOR_GREEN_DIM),
                ),
                Span::styled(
                    format!("{:<10} ", name),
                    Style::default().fg(PHOSPHOR_GREEN),
                ),
                Span::styled(
                    format!("{:>6}", score),
                    Style::default().fg(PHOSPHOR_GREEN).bold(),
                ),
            ]));
        }
    }

    let scores = Paragraph::new(lines).alignment(Alignment::Center);
    frame.render_widget(scores, layout[2]);

    let footer = Paragraph::new("PRESS ENTER TO RETURN")
        .style(Style::default().fg(PHOSPHOR_GREEN_DIM))
        .alignment(Alignment::Center);
    frame.render_widget(footer, layout[4]);
}

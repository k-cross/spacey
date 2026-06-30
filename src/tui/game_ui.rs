//! Game UI rendering

use ratatui::{
    prelude::*,
    widgets::{Block, Paragraph},
};

use super::game::GameState;

/// Retro phosphor green colors
const PHOSPHOR_GREEN_DIM: Color = Color::Rgb(0, 100, 0);
const PHOSPHOR_GREEN_BRIGHT: Color = Color::Rgb(50, 255, 50);

/// Render the entire game screen
pub fn render(frame: &mut Frame, game: &GameState) {
    let area = frame.area();

    // Dark background
    let block = Block::default().style(Style::default().bg(Color::Black));
    frame.render_widget(block, area);

    // Layout: Game view, HUD
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(10),   // Game view
            Constraint::Length(1), // HUD
        ])
        .split(area);

    let game_area = layout[0];
    let hud_area = layout[1];

    render_starfield(frame, game_area, game);
    render_enemies(frame, game_area, game);
    render_lasers(frame, game_area, game);
    render_ship(frame, game_area, game);
    render_hud(frame, hud_area, game);

    // Pause overlay
    if game.paused {
        render_pause_overlay(frame, area);
    }
}

/// Render scrolling starfield
fn render_starfield(frame: &mut Frame, area: Rect, game: &GameState) {
    let width = area.width as usize;
    let height = area.height as usize;

    let mut buffer = vec![vec![' '; width]; height];

    // Scroll offset increases over time, making stars move downwards
    let scroll_offset = (game.frame as usize) / 2;

    for (y, row) in buffer.iter_mut().enumerate().take(height) {
        // Compute an absolute world Y coordinate for this row
        let world_y = y.wrapping_sub(scroll_offset);

        // Restore the original visually appealing pattern
        if world_y % 3 == 0 {
            let x1 = (world_y * 7) % width.max(1);
            let x2 = (world_y * 13) % width.max(1);
            if x1 < width {
                row[x1] = '.';
            }
            if x2 < width {
                row[x2] = '*';
            }
        }
    }

    let lines: Vec<Line> = buffer
        .into_iter()
        .map(|row| {
            let s: String = row.into_iter().collect();
            Line::from(Span::styled(s, Style::default().fg(PHOSPHOR_GREEN_DIM)))
        })
        .collect();

    frame.render_widget(Paragraph::new(lines), area);
}

/// Render the player's ship
fn render_ship(frame: &mut Frame, area: Rect, game: &GameState) {
    let width = area.width as f32;
    let height = area.height as f32;

    // Map -1.0 to 1.0 to screen coordinates
    let x_pos = ((game.ship_x + 1.0) * 0.5 * (width - 1.0).max(0.0)) as u16;
    let y_pos = ((game.ship_y + 1.0) * 0.5 * (height - 1.0).max(0.0)) as u16;

    let ship_sprite = [r" /| ", r"/__\"];

    let ship_width = 4;
    let ship_height = 2;

    let ship_x = x_pos.saturating_sub(ship_width / 2);
    let ship_y = y_pos.saturating_sub(ship_height / 2);

    for (i, line) in ship_sprite.iter().enumerate() {
        let draw_y = ship_y + i as u16;
        if draw_y < area.height && ship_x < area.width {
            let ship_area = Rect {
                x: area.x + ship_x,
                y: area.y + draw_y,
                width: line.len() as u16,
                height: 1,
            };

            let render_area = area.intersection(ship_area);
            if render_area.area() > 0 {
                frame.render_widget(
                    Paragraph::new(*line).style(Style::default().fg(PHOSPHOR_GREEN_BRIGHT)),
                    render_area,
                );
            }
        }
    }
}

/// Render enemies
fn render_enemies(frame: &mut Frame, area: Rect, game: &GameState) {
    let width = area.width as f32;
    let height = area.height as f32;

    for enemy in &game.enemies {
        let x_pos = ((enemy.x + 1.0) * 0.5 * (width - 1.0).max(0.0)) as i32;
        let y_pos = ((enemy.y + 1.0) * 0.5 * (height - 1.0).max(0.0)) as i32;

        let sprite = r"\-V-/"; // Simple fighter shape looking down
        let sprite_len = 5;

        let draw_x = x_pos - sprite_len / 2;
        let draw_y = y_pos;

        if draw_x >= 0 && draw_x < area.width as i32 && draw_y >= 0 && draw_y < area.height as i32 {
            let enemy_area = Rect {
                x: area.x + draw_x as u16,
                y: area.y + draw_y as u16,
                width: sprite.len() as u16,
                height: 1,
            };

            let render_area = area.intersection(enemy_area);
            if render_area.area() > 0 {
                frame.render_widget(
                    Paragraph::new(sprite).style(Style::default().fg(PHOSPHOR_GREEN_BRIGHT)),
                    render_area,
                );
            }
        }
    }
}

/// Render lasers
fn render_lasers(frame: &mut Frame, area: Rect, game: &GameState) {
    let width = area.width as f32;
    let height = area.height as f32;

    for laser in &game.lasers {
        let x_pos = ((laser.x + 1.0) * 0.5 * (width - 1.0).max(0.0)) as i32;
        let y_pos = ((laser.y + 1.0) * 0.5 * (height - 1.0).max(0.0)) as i32;

        if x_pos >= 0 && x_pos < area.width as i32 && y_pos >= 0 && y_pos < area.height as i32 {
            let laser_area = Rect {
                x: area.x + x_pos as u16,
                y: area.y + y_pos as u16,
                width: 1,
                height: 1,
            };

            let render_area = area.intersection(laser_area);
            if render_area.area() > 0 {
                frame.render_widget(
                    Paragraph::new("|").style(Style::default().fg(Color::Red)),
                    render_area,
                );
            }
        }
    }
}

/// Render HUD bar at bottom
fn render_hud(frame: &mut Frame, area: Rect, game: &GameState) {
    // Shield bar: "SHIELD: ||||||||"
    let shield_str: String = (0..8)
        .map(|i| if i < game.shield as usize { '|' } else { ' ' })
        .collect();

    let hud = format!(
        "SHIELD: {}   LASER: READY   ALTITUDE: {:>4}   SCORE: {:06}",
        shield_str, game.altitude, game.score
    );

    let hud_widget = Paragraph::new(hud)
        .style(Style::default().fg(PHOSPHOR_GREEN_BRIGHT))
        .alignment(Alignment::Center);
    frame.render_widget(hud_widget, area);
}

/// Render pause overlay
fn render_pause_overlay(frame: &mut Frame, area: Rect) {
    let pause_text = vec![
        Line::from(""),
        Line::from(Span::styled(
            "[ PAUSED ]",
            Style::default().fg(PHOSPHOR_GREEN_BRIGHT).bold(),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "Press ENTER to resume",
            Style::default().fg(PHOSPHOR_GREEN_DIM),
        )),
        Line::from(Span::styled(
            "Press Q to return to menu",
            Style::default().fg(PHOSPHOR_GREEN_DIM),
        )),
    ];

    let pause_height = 5u16;
    let pause_width = 30u16;
    let pause_area = Rect {
        x: area.x + (area.width.saturating_sub(pause_width)) / 2,
        y: area.y + (area.height.saturating_sub(pause_height)) / 2,
        width: pause_width,
        height: pause_height,
    };

    let pause_widget = Paragraph::new(pause_text)
        .style(Style::default().bg(Color::Black))
        .alignment(Alignment::Center);
    frame.render_widget(pause_widget, pause_area);
}

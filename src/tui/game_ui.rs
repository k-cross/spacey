//! Game UI rendering

use ratatui::{
    prelude::*,
    widgets::{Block, Paragraph},
};

use super::game::{EntityType, GameState, MAX_ENTITIES};

/// Retro phosphor green colors
const PHOSPHOR_GREEN_DIM: Color = Color::Rgb(0, 100, 0);
const PHOSPHOR_GREEN_BRIGHT: Color = Color::Rgb(50, 255, 50);

/// Render the entire game screen
pub fn render(frame: &mut Frame, game: &GameState, alpha: f32) {
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

    render_starfield(frame, game_area, game, alpha);
    render_asteroids(frame, game_area, game, alpha);
    render_enemies(frame, game_area, game, alpha);
    render_lasers(frame, game_area, game, alpha);
    render_explosions(frame, game_area, game, alpha);
    render_ship(frame, game_area, game, alpha);
    render_hud(frame, hud_area, game);

    // Overlays
    if game.game_over {
        render_game_over_overlay(frame, area);
    } else if game.paused {
        render_pause_overlay(frame, area);
    }
}

/// Render scrolling starfield
fn render_starfield(frame: &mut Frame, area: Rect, game: &GameState, _alpha: f32) {
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
fn render_ship(frame: &mut Frame, area: Rect, game: &GameState, alpha: f32) {
    let width = area.width as f32;
    let height = area.height as f32;

    if let Some(p_id) = game.player_id
        && game.active[p_id]
    {
        if game.invincibility_timer > 0 && (game.invincibility_timer / 4) & 1 == 1 {
            return;
        }

        let pos = game.positions[p_id];
        let interp_x = pos.prev_x + (pos.x - pos.prev_x) * alpha;
        let interp_y = pos.prev_y + (pos.y - pos.prev_y) * alpha;

        // Map -1.0 to 1.0 to screen coordinates
        let x_pos = ((interp_x + 1.0) * 0.5 * (width - 1.0).max(0.0)) as u16;
        let y_pos = ((interp_y + 1.0) * 0.5 * (height - 1.0).max(0.0)) as u16;

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
}

/// Render asteroids
fn render_asteroids(frame: &mut Frame, area: Rect, game: &GameState, alpha: f32) {
    let width = area.width as f32;
    let height = area.height as f32;

    for i in 0..MAX_ENTITIES {
        if game.active[i] && game.entity_types[i] == EntityType::Asteroid {
            let pos = game.positions[i];
            let interp_x = pos.prev_x + (pos.x - pos.prev_x) * alpha;
            let interp_y = pos.prev_y + (pos.y - pos.prev_y) * alpha;

            if let Some(ast) = &game.asteroids[i] {
                let cos_a = ast.angle.cos();
                let sin_a = ast.angle.sin();

                for &(px, py) in &ast.points {
                    let rx = px * cos_a - py * sin_a;
                    let ry = px * sin_a + py * cos_a;
                    let cx = interp_x + rx;
                    let cy = interp_y + ry;

                    let x_pos = ((cx + 1.0) * 0.5 * (width - 1.0).max(0.0)) as i32;
                    let y_pos = ((cy + 1.0) * 0.5 * (height - 1.0).max(0.0)) as i32;

                    if x_pos >= 0
                        && x_pos < area.width as i32
                        && y_pos >= 0
                        && y_pos < area.height as i32
                    {
                        let chunk_area = Rect {
                            x: area.x + x_pos as u16,
                            y: area.y + y_pos as u16,
                            width: 1,
                            height: 1,
                        };

                        let render_area = area.intersection(chunk_area);
                        if render_area.area() > 0 {
                            frame.render_widget(
                                Paragraph::new("O")
                                    .style(Style::default().fg(Color::Rgb(150, 100, 50))),
                                render_area,
                            );
                        }
                    }
                }
            }
        }
    }
}

/// Render enemies
fn render_enemies(frame: &mut Frame, area: Rect, game: &GameState, alpha: f32) {
    let width = area.width as f32;
    let height = area.height as f32;

    for i in 0..MAX_ENTITIES {
        if game.active[i] && game.entity_types[i] == EntityType::Enemy {
            let pos = game.positions[i];
            let interp_x = pos.prev_x + (pos.x - pos.prev_x) * alpha;
            let interp_y = pos.prev_y + (pos.y - pos.prev_y) * alpha;

            let x_pos = ((interp_x + 1.0) * 0.5 * (width - 1.0).max(0.0)) as i32;
            let y_pos = ((interp_y + 1.0) * 0.5 * (height - 1.0).max(0.0)) as i32;

            let sprite = r"\-V-/"; // Simple fighter shape looking down
            let sprite_len = 5;

            let draw_x = x_pos - sprite_len / 2;
            let draw_y = y_pos;

            if draw_x >= 0
                && draw_x < area.width as i32
                && draw_y >= 0
                && draw_y < area.height as i32
            {
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
}

/// Render lasers
fn render_lasers(frame: &mut Frame, area: Rect, game: &GameState, alpha: f32) {
    let width = area.width as f32;
    let height = area.height as f32;

    for i in 0..MAX_ENTITIES {
        if game.active[i] && game.entity_types[i] == EntityType::Laser {
            let pos = game.positions[i];
            let interp_x = pos.prev_x + (pos.x - pos.prev_x) * alpha;
            let interp_y = pos.prev_y + (pos.y - pos.prev_y) * alpha;

            let x_pos = ((interp_x + 1.0) * 0.5 * (width - 1.0).max(0.0)) as i32;
            let y_pos = ((interp_y + 1.0) * 0.5 * (height - 1.0).max(0.0)) as i32;

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
}

/// Render explosions
fn render_explosions(frame: &mut Frame, area: Rect, game: &GameState, alpha: f32) {
    let width = area.width as f32;
    let height = area.height as f32;

    for i in 0..MAX_ENTITIES {
        if game.active[i] && game.entity_types[i] == EntityType::Explosion {
            let pos = game.positions[i];
            let timer = game.lifetimes[i].map(|l| l.timer).unwrap_or(0);

            let interp_x = pos.prev_x + (pos.x - pos.prev_x) * alpha;
            let interp_y = pos.prev_y + (pos.y - pos.prev_y) * alpha;

            let x_pos = ((interp_x + 1.0) * 0.5 * (width - 1.0).max(0.0)) as i32;
            let y_pos = ((interp_y + 1.0) * 0.5 * (height - 1.0).max(0.0)) as i32;

            if x_pos >= 0 && x_pos < area.width as i32 && y_pos >= 0 && y_pos < area.height as i32 {
                let sprite = match timer {
                    0..=2 => "*",
                    3..=5 => "O",
                    6..=9 => ".",
                    _ => " ",
                };

                let explosion_area = Rect {
                    x: area.x + x_pos as u16,
                    y: area.y + y_pos as u16,
                    width: 1,
                    height: 1,
                };

                let render_area = area.intersection(explosion_area);
                if render_area.area() > 0 {
                    frame.render_widget(
                        Paragraph::new(sprite).style(Style::default().fg(Color::Yellow)),
                        render_area,
                    );
                }
            }
        }
    }
}

/// Render HUD bar at bottom
fn render_hud(frame: &mut Frame, area: Rect, game: &GameState) {
    // Shield bar: "SHIELD: |||"
    let shield_str: String = (0..3)
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

/// Render game over overlay
fn render_game_over_overlay(frame: &mut Frame, area: Rect) {
    let game_over_text = vec![
        Line::from(""),
        Line::from(Span::styled(
            "[ GAME OVER ]",
            Style::default().fg(Color::Red).bold(),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "Press ENTER to return to menu",
            Style::default().fg(Color::White),
        )),
    ];

    let overlay_height = 4u16;
    let overlay_width = 30u16;
    let overlay_area = Rect {
        x: area.x + (area.width.saturating_sub(overlay_width)) / 2,
        y: area.y + (area.height.saturating_sub(overlay_height)) / 2,
        width: overlay_width,
        height: overlay_height,
    };

    let overlay_widget = Paragraph::new(game_over_text)
        .style(Style::default().bg(Color::Black))
        .alignment(Alignment::Center);
    frame.render_widget(overlay_widget, overlay_area);
}

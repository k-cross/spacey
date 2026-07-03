//! Game UI rendering

use ratatui::{
    prelude::*,
    symbols::Marker,
    widgets::{
        Block, Paragraph,
        canvas::{Canvas, Circle, Line as CanvasLine},
    },
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
    render_world_canvas(frame, game_area, game, alpha);
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

/// Render all world entities as one braille wireframe canvas.
///
/// The canvas plane uses world coordinates directly (bounds `[-1, 1]`), but its
/// Y axis points *up* while the game's world Y points *down*, so every vertex is
/// negated at draw time. The previous per-type draw order (asteroids → enemies
/// → lasers → explosions → ship) is preserved via `ctx.layer()`, so later
/// groups win per-cell color conflicts.
fn render_world_canvas(frame: &mut Frame, area: Rect, game: &GameState, alpha: f32) {
    const ROCK_COLOR: Color = Color::Rgb(150, 100, 50);

    /// Queue a segment, clamping endpoints that poke past the world bounds so
    /// shapes flatten against the edge instead of vanishing — Canvas drops a
    /// `Line` entirely when an endpoint is out of bounds. Segments with both
    /// endpoints outside are skipped.
    fn push_seg(segs: &mut Vec<(f64, f64, f64, f64)>, x1: f64, y1: f64, x2: f64, y2: f64) {
        let inside = |x: f64, y: f64| (-1.0..=1.0).contains(&x) && (-1.0..=1.0).contains(&y);
        if !inside(x1, y1) && !inside(x2, y2) {
            return;
        }
        segs.push((
            x1.clamp(-1.0, 1.0),
            y1.clamp(-1.0, 1.0),
            x2.clamp(-1.0, 1.0),
            y2.clamp(-1.0, 1.0),
        ));
    }

    // (x, y, radius) — each asteroid collision point drawn as its collider.
    let mut rock_circles: Vec<(f64, f64, f64)> = Vec::new();
    let mut enemy_segs: Vec<(f64, f64, f64, f64)> = Vec::new();
    let mut laser_segs: Vec<(f64, f64, f64, f64)> = Vec::new();
    let mut explosion_circles: Vec<(f64, f64, f64, Color)> = Vec::new();
    let mut ship_segs: Vec<(f64, f64, f64, f64)> = Vec::new();

    for i in 0..MAX_ENTITIES {
        if !game.active[i] {
            continue;
        }
        let pos = game.positions[i];
        let cx = (pos.prev_x + (pos.x - pos.prev_x) * alpha) as f64;
        let cy = (pos.prev_y + (pos.y - pos.prev_y) * alpha) as f64;

        match game.entity_types[i] {
            EntityType::Asteroid => {
                if let Some(ast) = &game.asteroids[i] {
                    let cos_a = ast.angle.cos();
                    let sin_a = ast.angle.sin();
                    for &(px, py) in &ast.points {
                        // Apply the asteroid's spin, then translate to world position.
                        let rx = px * cos_a - py * sin_a;
                        let ry = px * sin_a + py * cos_a;
                        rock_circles.push((
                            cx + rx as f64,
                            cy + ry as f64,
                            ast.point_radius as f64,
                        ));
                    }
                }
            }
            EntityType::Enemy => {
                // Wireframe fighter, nose pointing down toward the player
                // (replaces the old `\-V-/` sprite).
                let wing_l = (cx - 0.06, cy - 0.03);
                let body_l = (cx - 0.025, cy - 0.005);
                let nose = (cx, cy + 0.045);
                let body_r = (cx + 0.025, cy - 0.005);
                let wing_r = (cx + 0.06, cy - 0.03);
                for &((x1, y1), (x2, y2)) in &[
                    (wing_l, body_l),
                    (body_l, nose),
                    (nose, body_r),
                    (body_r, wing_r),
                ] {
                    push_seg(&mut enemy_segs, x1, y1, x2, y2);
                }
            }
            EntityType::Laser => {
                push_seg(&mut laser_segs, cx, cy - 0.02, cx, cy + 0.02);
            }
            EntityType::Explosion => {
                // Expanding shockwave ring that dims as it dies (replaces the
                // `*` → `O` → `.` glyph sequence).
                let timer = game.lifetimes[i].map(|l| l.timer).unwrap_or(0);
                let radius = 0.012 + f64::from(timer) * 0.006;
                let color = if timer < 6 {
                    Color::Yellow
                } else {
                    Color::Rgb(130, 130, 0)
                };
                explosion_circles.push((cx, cy, radius, color));
            }
            EntityType::Player => {} // drawn below so it stays on top
        }
    }

    // Ship: wireframe fighter, nose pointing up (toward -y in world space).
    if let Some(p_id) = game.player_id
        && game.active[p_id]
        // Blink out on alternating windows during invincibility.
        && !(game.invincibility_timer > 0 && (game.invincibility_timer / 4) & 1 == 1)
    {
        let pos = game.positions[p_id];
        let cx = (pos.prev_x + (pos.x - pos.prev_x) * alpha) as f64;
        let cy = (pos.prev_y + (pos.y - pos.prev_y) * alpha) as f64;

        let nose = (cx, cy - 0.09);
        let left = (cx - 0.07, cy + 0.06);
        let right = (cx + 0.07, cy + 0.06);
        let tail = (cx, cy + 0.03); // rear notch so it reads as a ship, not a triangle
        for &((x1, y1), (x2, y2)) in &[(nose, left), (left, tail), (tail, right), (right, nose)] {
            push_seg(&mut ship_segs, x1, y1, x2, y2);
        }
    }

    let canvas = Canvas::default()
        .background_color(Color::Black)
        .marker(Marker::Braille)
        .x_bounds([-1.0, 1.0])
        .y_bounds([-1.0, 1.0])
        .paint(move |ctx| {
            // Each collision point is drawn as its actual collider circle, so
            // the rendered rock is exactly the area that causes impact — the
            // road through a large asteroid appears as the circle-free gap.
            for &(x, y, r) in &rock_circles {
                ctx.draw(&Circle {
                    x,
                    y: -y,
                    radius: r,
                    color: ROCK_COLOR,
                });
            }
            ctx.layer();
            for &(x1, y1, x2, y2) in &enemy_segs {
                ctx.draw(&CanvasLine {
                    x1,
                    y1: -y1,
                    x2,
                    y2: -y2,
                    color: PHOSPHOR_GREEN_BRIGHT,
                });
            }
            for &(x1, y1, x2, y2) in &laser_segs {
                ctx.draw(&CanvasLine {
                    x1,
                    y1: -y1,
                    x2,
                    y2: -y2,
                    color: Color::Red,
                });
            }
            ctx.layer();
            for &(x, y, r, color) in &explosion_circles {
                ctx.draw(&Circle {
                    x,
                    y: -y,
                    radius: r,
                    color,
                });
            }
            ctx.layer();
            for &(x1, y1, x2, y2) in &ship_segs {
                ctx.draw(&CanvasLine {
                    x1,
                    y1: -y1,
                    x2,
                    y2: -y2,
                    color: PHOSPHOR_GREEN_BRIGHT,
                });
            }
        });
    frame.render_widget(canvas, area);
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

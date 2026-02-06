//! Game UI rendering

use ratatui::{
    prelude::*,
    widgets::{Block, Paragraph},
};

use super::game::GameState;

/// Retro phosphor green colors
const PHOSPHOR_GREEN: Color = Color::Rgb(0, 200, 0);
const PHOSPHOR_GREEN_DIM: Color = Color::Rgb(0, 100, 0);
const PHOSPHOR_GREEN_BRIGHT: Color = Color::Rgb(50, 255, 50);

/// Cockpit ASCII art (ship from behind view)
const COCKPIT: &[&str] = &[
    r"          /\                    /\          ",
    r"         /  \                  /  \         ",
    r"        /    \                /    \        ",
    r"       /      \______________/      \       ",
    r"      /       |              |       \      ",
    r"     /________|              |________\     ",
    r"    |    _____|              |_____    |    ",
    r"    |   /     \______________/     \   |    ",
    r"    |  /                            \  |    ",
    r"    | /          ________            \ |    ",
    r"    |/          /   /\   \            \|    ",
    r"   _|__________/   /  \   \____________|_   ",
    r"  /            \__/    \__/              \  ",
    r" /                                        \ ",
];

/// Render the entire game screen
pub fn render(frame: &mut Frame, game: &GameState) {
    let area = frame.area();

    // Dark background
    let block = Block::default().style(Style::default().bg(Color::Black));
    frame.render_widget(block, area);

    // Layout: Stars, Game view, Cockpit, HUD
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(6),  // Top Sky/Stars
            Constraint::Min(10),    // Game view (perspective grid)
            Constraint::Length(14), // Cockpit
            Constraint::Length(1),  // HUD
        ])
        .split(area);

    render_sky(frame, layout[0], game);
    // Combine layout[1] and part of layout[0] logic for proper specific rendering if needed,
    // but here we treat layout[1] as the main viewport for the trench run.
    render_trench(frame, layout[1], game);
    render_enemies(frame, layout[1], game);
    render_cockpit(frame, layout[2], game);
    render_hud(frame, layout[3], game);

    // Pause overlay
    if game.paused {
        render_pause_overlay(frame, area);
    }
}

/// Render sky/stars
fn render_sky(frame: &mut Frame, area: Rect, game: &GameState) {
    let width = area.width as usize;
    let offset = (game.ship_x * 10.0) as i32;
    // Just a simple static star field for now
    let stars: String = (0..width)
        .map(|i| {
            let pos = (i as i32 + offset).rem_euclid(7);
            if pos == 0 || pos == 3 { '*' } else { ' ' }
        })
        .collect();

    let star_line = Paragraph::new(stars).style(Style::default().fg(PHOSPHOR_GREEN_DIM));
    frame.render_widget(star_line, area);
}

/// Render the "Trench Run" perspective grid
fn render_trench(frame: &mut Frame, area: Rect, game: &GameState) {
    let width = area.width as usize;
    let height = area.height as usize;
    let center_x = width / 2;
    let center_y = height / 2;

    // Animation phase
    let phase = (game.frame as f32 * 0.5) % 8.0;

    // Vanishing point moves with ship
    let vp_x = center_x as i32 - (game.ship_x * (width as f32 / 3.0)) as i32;
    let vp_y = center_y as i32 - (game.ship_y * (height as f32 / 3.0)) as i32;

    let mut buffer = vec![vec![' '; width]; height];

    for (y, row) in buffer.iter_mut().enumerate().take(height) {
        let dy = y as i32 - vp_y;
        if dy == 0 {
            continue;
        } // Horizon line

        // Simulating ceiling and floor
        let is_floor = dy > 0;
        let dist_factor = (height as f32 / dy.abs() as f32).max(1.0);

        // Perspective lines (Vertical walls/corridor)
        // We draw two main perspective lines defining the "trench"
        let trench_width_at_depth = (width as f32 / dist_factor) * 0.8;

        let left_wall_x = (vp_x as f32 - trench_width_at_depth) as i32;
        let right_wall_x = (vp_x as f32 + trench_width_at_depth) as i32;

        // Draw Side Walls
        if left_wall_x >= 0 && left_wall_x < width as i32 {
            row[left_wall_x as usize] = if is_floor { '/' } else { '\\' };
        }
        if right_wall_x >= 0 && right_wall_x < width as i32 {
            row[right_wall_x as usize] = if is_floor { '\\' } else { '/' };
        }

        // Horizontal Grid Lines (moving towards player)
        // distance Z calculation approximation
        let z_depth = 100.0 / dist_factor;
        let grid_pos = (z_depth + phase) % 10.0;

        if grid_pos < 1.0 {
            // Draw horizontal line
            let start = left_wall_x.max(0) as usize;
            let end = right_wall_x.min(width as i32) as usize;
            for (x, cell) in row.iter_mut().enumerate().take(end).skip(start) {
                // Gaps in the middle to simulate individual floor/ceiling tiles
                if x % 10 != 0 {
                    *cell = '-';
                }
            }
        }

        // Vertical pillars on the side walls passing by
        let pillar_interval = (z_depth + phase) % 20.0;
        if pillar_interval < 2.0 {
            // Draw "pillar" lines outside the trench
            // Left side pillars
            let outer_left = (left_wall_x - 10).max(0);
            if left_wall_x > 0 {
                for cell in row
                    .iter_mut()
                    .take(left_wall_x as usize)
                    .skip(outer_left as usize)
                {
                    *cell = '|';
                }
            }
            // Right side pillars
            let outer_right = (right_wall_x + 10).min(width as i32);
            if right_wall_x < width as i32 {
                for cell in row
                    .iter_mut()
                    .take(outer_right as usize)
                    .skip(right_wall_x as usize)
                {
                    *cell = '|';
                }
            }
        }
    }

    // Convert buffer to widgets
    let lines: Vec<Line> = buffer
        .into_iter()
        .map(|row| {
            let s: String = row.into_iter().collect();
            Line::from(Span::styled(s, Style::default().fg(PHOSPHOR_GREEN_DIM)))
        })
        .collect();

    frame.render_widget(Paragraph::new(lines), area);
}

/// Render enemies scaled by distance
fn render_enemies(frame: &mut Frame, area: Rect, game: &GameState) {
    let width = area.width as f32;
    let height = area.height as f32;
    let vp_x = width / 2.0 - (game.ship_x * (width / 3.0));
    let vp_y = height / 2.0 - (game.ship_y * (height / 3.0));

    for enemy in &game.enemies {
        // Simple perspective projection
        // z=0 is camera, z=100 is far
        if enemy.z <= 1.0 {
            continue;
        }

        let scale = 100.0 / enemy.z;

        // Projected Position relative to Vanishing Point
        // Enemy x/y is -1.0 to 1.0
        let world_x = enemy.x * width;
        let world_y = enemy.y * height;

        let proj_x = vp_x + (world_x * scale * 0.5);
        let proj_y = vp_y + (world_y * scale * 0.5);

        // Sprite Selection based on scale (distance)
        let sprite = if scale < 2.0 {
            "."
        } else if scale < 5.0 {
            "-o-"
        } else {
            r"/-\" // Simple fighter shape
        };

        // Don't render if out of bounds
        if proj_x < 0.0 || proj_x >= width || proj_y < 0.0 || proj_y >= height {
            continue;
        }

        let x = proj_x as u16;
        let y = proj_y as u16;

        // Using a centralized rect for the enemy widget
        let enemy_area = Rect {
            x: area.x + x,
            y: area.y + y,
            width: sprite.len() as u16,
            height: 1,
        };

        // Clip to game area
        let render_area = area.intersection(enemy_area);

        if render_area.area() > 0 {
            frame.render_widget(
                Paragraph::new(sprite).style(Style::default().fg(PHOSPHOR_GREEN_BRIGHT)),
                render_area,
            );
        }
    }
}

/// Render cockpit/ship view
fn render_cockpit(frame: &mut Frame, area: Rect, game: &GameState) {
    let width = area.width as usize;

    // Center the cockpit
    let cockpit_width = COCKPIT.first().map(|s| s.len()).unwrap_or(0);
    // Move cockpit slightly opposite to ship movement for parallax feel
    let offset_x = (game.ship_x * 4.0) as i32;
    let padding = ((width as i32 - cockpit_width as i32) / 2 + offset_x).max(0) as usize;

    let lines: Vec<Line> = COCKPIT
        .iter()
        .map(|&s| {
            let padded = format!("{:>width$}", s, width = padding + s.len());
            Line::from(Span::styled(padded, Style::default().fg(PHOSPHOR_GREEN)))
        })
        .collect();

    let cockpit = Paragraph::new(lines);
    frame.render_widget(cockpit, area);
}

/// Render HUD bar at bottom
fn render_hud(frame: &mut Frame, area: Rect, game: &GameState) {
    // Shield bar: "SHIELD: ||||||||"
    // Using simple pipe chars
    let shield_str: String = (0..8)
        .map(|i| if i < game.shield as usize { '|' } else { ' ' })
        .collect();

    // Layout: SHIELD  LASER  ALTITUDE  SCORE
    // Using distinct spacing
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

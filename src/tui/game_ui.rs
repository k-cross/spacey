//! Game UI rendering

use ratatui::{
    prelude::*,
    widgets::{Block, Paragraph},
};

use super::game::GameState;

/// Retro phosphor green colors
const PHOSPHOR_GREEN: Color = Color::Rgb(0, 200, 0);
const PHOSPHOR_GREEN_DIM: Color = Color::Rgb(0, 120, 0);
const PHOSPHOR_GREEN_BRIGHT: Color = Color::Rgb(0, 255, 0);

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
            Constraint::Length(2),  // Stars
            Constraint::Min(10),    // Game view (perspective grid)
            Constraint::Length(14), // Cockpit
            Constraint::Length(1),  // HUD
        ])
        .split(area);

    render_stars(frame, layout[0], game);
    render_perspective_grid(frame, layout[1], game);
    render_cockpit(frame, layout[2], game);
    render_hud(frame, layout[3], game);

    // Pause overlay
    if game.paused {
        render_pause_overlay(frame, area);
    }
}

/// Render star field
fn render_stars(frame: &mut Frame, area: Rect, game: &GameState) {
    let width = area.width as usize;
    
    // Create pseudo-random star pattern that shifts with movement
    let offset = (game.ship_x * 10.0) as i32;
    let stars: String = (0..width)
        .map(|i| {
            let pos = (i as i32 + offset).rem_euclid(7);
            if pos == 0 || pos == 3 { '*' } else { ' ' }
        })
        .collect();
    
    let star_line = Paragraph::new(stars)
        .style(Style::default().fg(PHOSPHOR_GREEN_DIM));
    frame.render_widget(star_line, area);
}

/// Render perspective grid creating forward motion illusion
fn render_perspective_grid(frame: &mut Frame, area: Rect, game: &GameState) {
    let width = area.width as usize;
    let height = area.height as usize;
    let center_x = width / 2;
    
    // Animation phase (0-7 cycle)
    let phase = (game.frame / 3) % 8;
    
    // Ship offset affects vanishing point
    let vp_offset_x = (game.ship_x * (width as f32 / 4.0)) as i32;
    let vp_offset_y = (game.ship_y * (height as f32 / 4.0)) as i32;
    
    let mut lines = Vec::new();
    
    for row in 0..height {
        let mut line = vec![' '; width];
        
        // Distance factor (further = closer to horizon)
        let y_from_bottom = height - row - 1;
        let distance = (y_from_bottom as f32 / height as f32).max(0.05);
        
        // Perspective lines converging to vanishing point
        let spread = ((1.0 - distance) * (width as f32 / 2.0)) as i32;
        
        // Left perspective line
        let left_x = ((center_x as i32 + vp_offset_x) - spread) as usize;
        if left_x < width {
            line[left_x] = '/';
        }
        
        // Right perspective line
        let right_x = ((center_x as i32 + vp_offset_x) + spread) as usize;
        if right_x < width {
            line[right_x] = '\\';
        }
        
        // Horizontal grid lines (animated based on phase)
        let grid_interval = (8.0 * distance) as usize + 1;
        let row_with_offset = (row as i32 - vp_offset_y).rem_euclid(height as i32) as usize;
        if (row_with_offset + phase as usize) % grid_interval == 0 && y_from_bottom > 2 {
            // Draw horizontal line between perspective lines
            let start = left_x.saturating_add(1);
            let end = right_x.min(width);
            for x in start..end {
                if line[x] == ' ' {
                    line[x] = '-';
                }
            }
        }
        
        let line_str: String = line.into_iter().collect();
        lines.push(Line::from(Span::styled(line_str, Style::default().fg(PHOSPHOR_GREEN_DIM))));
    }
    
    let grid = Paragraph::new(lines);
    frame.render_widget(grid, area);
}

/// Render cockpit/ship view
fn render_cockpit(frame: &mut Frame, area: Rect, game: &GameState) {
    let width = area.width as usize;
    
    // Center the cockpit
    let cockpit_width = COCKPIT.first().map(|s| s.len()).unwrap_or(0);
    let offset_x = (game.ship_x * 5.0) as i32;
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
    // Shield bar
    let shield_bar: String = (0..10)
        .map(|i| if i < game.shield { '|' } else { ' ' })
        .collect();
    
    let hud = format!(
        "SHIELD: {}  LASER: READY  ALTITUDE: {:>5}  SCORE: {:06}",
        shield_bar, game.altitude, game.score
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
        Line::from(Span::styled("[ PAUSED ]", Style::default().fg(PHOSPHOR_GREEN_BRIGHT).bold())),
        Line::from(""),
        Line::from(Span::styled("Press ENTER to resume", Style::default().fg(PHOSPHOR_GREEN_DIM))),
        Line::from(Span::styled("Press Q to return to menu", Style::default().fg(PHOSPHOR_GREEN_DIM))),
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

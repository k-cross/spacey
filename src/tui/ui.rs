//! UI rendering for the start menu

use ratatui::{
    prelude::*,
    widgets::{Block, Paragraph},
};

use super::{app::App, menu::MenuItem};

/// Retro phosphor green color
const PHOSPHOR_GREEN: Color = Color::Rgb(0, 200, 0);
const PHOSPHOR_GREEN_DIM: Color = Color::Rgb(0, 120, 0);

/// ASCII art title banner
const TITLE_ART: &str = r#"
  ███████╗██████╗  █████╗  ██████╗███████╗██╗   ██╗
  ██╔════╝██╔══██╗██╔══██╗██╔════╝██╔════╝╚██╗ ██╔╝
  ███████╗██████╔╝███████║██║     █████╗   ╚████╔╝ 
  ╚════██║██╔═══╝ ██╔══██║██║     ██╔══╝    ╚██╔╝  
  ███████║██║     ██║  ██║╚██████╗███████╗   ██║   
  ╚══════╝╚═╝     ╚═╝  ╚═╝ ╚═════╝╚══════╝   ╚═╝   
"#;

/// Render the entire UI
pub fn render(frame: &mut Frame, app: &App) {
    let area = frame.area();

    // Create dark background
    let block = Block::default().style(Style::default().bg(Color::Black));
    frame.render_widget(block, area);

    // Layout: Title at top, menu in center, footer at bottom
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(9), // Title
            Constraint::Min(8),    // Menu
            Constraint::Length(3), // Footer
        ])
        .split(area);

    render_title(frame, layout[0]);
    render_menu(frame, layout[1], app);
    render_footer(frame, layout[2]);
}

/// Render the ASCII art title
fn render_title(frame: &mut Frame, area: Rect) {
    let title = Paragraph::new(TITLE_ART)
        .style(Style::default().fg(PHOSPHOR_GREEN))
        .alignment(Alignment::Center);
    frame.render_widget(title, area);
}

/// Render the menu items
fn render_menu(frame: &mut Frame, area: Rect, app: &App) {
    let items = MenuItem::all();

    // Calculate vertical centering
    let menu_height = items.len() as u16;
    let vertical_padding = (area.height.saturating_sub(menu_height)) / 2;

    let menu_area = Rect {
        x: area.x,
        y: area.y + vertical_padding,
        width: area.width,
        height: menu_height,
    };

    // Build menu text with selection indicator
    let menu_lines: Vec<Line> = items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let is_selected = i == app.selected_index();
            let prefix = if is_selected { "> " } else { "  " };
            let text = format!("{}{}", prefix, item.label());

            let style = if is_selected {
                Style::default().fg(PHOSPHOR_GREEN).bold()
            } else {
                Style::default().fg(PHOSPHOR_GREEN_DIM)
            };

            Line::from(Span::styled(text, style))
        })
        .collect();

    let menu = Paragraph::new(menu_lines).alignment(Alignment::Center);

    frame.render_widget(menu, menu_area);
}

/// Render the footer prompt
fn render_footer(frame: &mut Frame, area: Rect) {
    let footer = Paragraph::new("PRESS ENTER.")
        .style(Style::default().fg(PHOSPHOR_GREEN_DIM))
        .alignment(Alignment::Center);
    frame.render_widget(footer, area);
}

mod tui;

use color_eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;

    // Run the TUI and get the selected menu action
    let selected = tui::run()?;

    // Handle the selected action
    if let Some(action) = selected {
        match action {
            tui::MenuItem::StartGame => {
                println!("Starting game... (not yet implemented)");
            }
            tui::MenuItem::Options => {
                println!("Options... (not yet implemented)");
            }
            tui::MenuItem::HighScores => {
                println!("High Scores... (not yet implemented)");
            }
            tui::MenuItem::Exit => {
                println!("Goodbye!");
            }
        }
    }

    Ok(())
}

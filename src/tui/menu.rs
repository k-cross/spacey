//! Menu items and configuration

/// Represents a menu option
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuItem {
    StartGame,
    Options,
    HighScores,
    Exit,
}

impl MenuItem {
    /// Get all menu items in display order
    pub fn all() -> &'static [MenuItem] {
        &[
            MenuItem::StartGame,
            MenuItem::Options,
            MenuItem::HighScores,
            MenuItem::Exit,
        ]
    }

    /// Get the display text for this menu item
    pub fn label(&self) -> &'static str {
        match self {
            MenuItem::StartGame => "START GAME",
            MenuItem::Options => "OPTIONS",
            MenuItem::HighScores => "HIGH SCORES",
            MenuItem::Exit => "EXIT",
        }
    }
}

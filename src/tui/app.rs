//! Application state management

use super::menu::MenuItem;

/// Application state
pub struct App {
    /// Current menu selection index
    selected_index: usize,
    /// Whether the app is still running
    running: bool,
    /// The action selected by the user (if any)
    selected_action: Option<MenuItem>,
}

impl App {
    /// Create a new App instance
    pub fn new() -> Self {
        Self {
            selected_index: 0,
            running: true,
            selected_action: None,
        }
    }

    /// Check if the app is still running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Get the currently selected menu index
    pub fn selected_index(&self) -> usize {
        self.selected_index
    }

    /// Get the selected action after app exits
    pub fn selected_action(&self) -> Option<MenuItem> {
        self.selected_action
    }

    /// Move selection to previous menu item
    pub fn previous(&mut self) {
        let menu_len = MenuItem::all().len();
        if self.selected_index > 0 {
            self.selected_index -= 1;
        } else {
            self.selected_index = menu_len - 1;
        }
    }

    /// Move selection to next menu item
    pub fn next(&mut self) {
        let menu_len = MenuItem::all().len();
        self.selected_index = (self.selected_index + 1) % menu_len;
    }

    /// Select the current menu item
    pub fn select(&mut self) {
        let items = MenuItem::all();
        if let Some(item) = items.get(self.selected_index) {
            self.selected_action = Some(*item);
            match item {
                MenuItem::Exit => self.quit(),
                _ => self.quit(), // For now, all selections exit
            }
        }
    }

    /// Quit the application
    pub fn quit(&mut self) {
        self.running = false;
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_state() {
        let app = App::new();
        assert!(app.is_running());
        assert_eq!(app.selected_index(), 0);
        assert!(app.selected_action().is_none());
    }

    #[test]
    fn test_navigation() {
        let mut app = App::new();
        let menu_len = MenuItem::all().len();

        // Previous from 0 wraps to end
        app.previous();
        assert_eq!(app.selected_index(), menu_len - 1);

        // Next wraps around
        app.next();
        assert_eq!(app.selected_index(), 0);

        app.next();
        assert_eq!(app.selected_index(), 1);
    }

    #[test]
    fn test_selection_and_quit() {
        let mut app = App::new();
        // Assuming "Exit" is the last item or we can find it
        // Check menu.rs for order. usually [Start, Exit].
        // Let's just select current (Start)
        app.select();
        // Current implementation quits on any selection (placeholder behavior logic in app.rs check line 63)
        assert!(!app.is_running());
        assert!(app.selected_action().is_some());
    }
}

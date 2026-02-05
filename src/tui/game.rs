//! Game state and logic

/// Game state during active gameplay
pub struct GameState {
    /// Ship view X offset (-1.0 to 1.0)
    pub ship_x: f32,
    /// Ship view Y offset (-1.0 to 1.0)
    pub ship_y: f32,
    /// Animation frame counter for grid motion
    pub frame: u64,
    /// Whether the game is paused
    pub paused: bool,
    /// Flag to return to main menu
    pub should_exit: bool,
    /// Current score
    pub score: u32,
    /// Current altitude
    pub altitude: u32,
    /// Shield level (0-10)
    pub shield: u8,
}

impl GameState {
    /// Create a new game state
    pub fn new() -> Self {
        Self {
            ship_x: 0.0,
            ship_y: 0.0,
            frame: 0,
            paused: false,
            should_exit: false,
            score: 0,
            altitude: 1500,
            shield: 10,
        }
    }

    /// Update game state each frame
    pub fn update(&mut self) {
        if !self.paused {
            self.frame = self.frame.wrapping_add(1);
            // Slowly increase altitude and score
            if self.frame % 10 == 0 {
                self.altitude = self.altitude.wrapping_add(1);
                self.score = self.score.wrapping_add(5);
            }
        }
    }

    /// Move ship view left
    pub fn move_left(&mut self) {
        if !self.paused {
            self.ship_x = (self.ship_x - 0.1).max(-1.0);
        }
    }

    /// Move ship view right
    pub fn move_right(&mut self) {
        if !self.paused {
            self.ship_x = (self.ship_x + 0.1).min(1.0);
        }
    }

    /// Move ship view up
    pub fn move_up(&mut self) {
        if !self.paused {
            self.ship_y = (self.ship_y - 0.1).max(-1.0);
        }
    }

    /// Move ship view down
    pub fn move_down(&mut self) {
        if !self.paused {
            self.ship_y = (self.ship_y + 0.1).min(1.0);
        }
    }

    /// Toggle pause state
    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused;
    }

    /// Exit to menu (only works when paused)
    pub fn exit_to_menu(&mut self) {
        if self.paused {
            self.should_exit = true;
        }
    }

    /// Check if game should continue running
    pub fn is_running(&self) -> bool {
        !self.should_exit
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

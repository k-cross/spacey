//! Game state and logic
#![allow(clippy::manual_is_multiple_of)]

use super::enemy::Enemy;

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
    /// Active enemies
    pub enemies: Vec<Enemy>,
}

impl GameState {
    /// Create a new game state
    pub fn new() -> Self {
        let mut state = Self {
            ship_x: 0.0,
            ship_y: 0.0,
            frame: 0,
            paused: false,
            should_exit: false,
            score: 0,
            altitude: 1500,
            shield: 10,
            enemies: Vec::new(),
        };
        // Add some initial visual enemies
        state.spawn_enemy();
        state
    }

    fn spawn_enemy(&mut self) {
        use std::time::{SystemTime, UNIX_EPOCH};
        // Use a simple seed for now
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_micros() as u64;

        self.enemies.push(Enemy::new(seed));
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

            // Spawn enemies occasionally
            if self.frame % 150 == 0 {
                self.spawn_enemy();
            }

            // Move enemies closer
            let speed = 1.5;
            for enemy in &mut self.enemies {
                enemy.update(speed);
            }

            // Remove enemies that passed the player
            self.enemies.retain(|e| e.is_visible());
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_state() {
        let game = GameState::new();
        assert_eq!(game.score, 0);
        assert!(!game.paused);
        assert!(!game.should_exit);
        // Should have at least one enemy spawned
        assert!(!game.enemies.is_empty());
    }

    #[test]
    fn test_movement_clamping() {
        let mut game = GameState::new();
        game.ship_x = 0.9;
        game.move_right(); // -> 1.0
        game.move_right(); // -> 1.0 (clamped)
        assert!((game.ship_x - 1.0).abs() < f32::EPSILON);

        game.ship_x = -0.9;
        game.move_left(); // -> -1.0
        game.move_left(); // -> -1.0 (clamped)
        assert!((game.ship_x - -1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_pause_toggle() {
        let mut game = GameState::new();
        assert!(!game.paused);
        game.toggle_pause();
        assert!(game.paused);
        game.toggle_pause();
        assert!(!game.paused);
    }

    #[test]
    fn test_update_while_paused() {
        let mut game = GameState::new();
        game.paused = true;
        let initial_frame = game.frame;
        game.update();
        assert_eq!(game.frame, initial_frame);
    }
}

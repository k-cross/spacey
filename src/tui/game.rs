//! Game state and logic
#![allow(clippy::manual_is_multiple_of)]

use super::enemy::Enemy;

/// Laser projectile
#[derive(Debug, Clone)]
pub struct Laser {
    /// View X offset (-1.0 to 1.0)
    pub x: f32,
    /// View Y offset (-1.0 to 1.0)
    pub y: f32,
}

/// Game state during active gameplay
pub struct GameState {
    /// Ship view X offset (-1.0 to 1.0)
    pub ship_x: f32,
    /// Ship view Y offset (-1.0 to 1.0)
    pub ship_y: f32,
    /// Whether the ship is currently moving left
    pub moving_left: bool,
    /// Whether the ship is currently moving right
    pub moving_right: bool,
    /// Whether the ship is currently moving up
    pub moving_up: bool,
    /// Whether the ship is currently moving down
    pub moving_down: bool,
    /// Whether the ship is currently firing
    pub firing: bool,
    /// Animation frame counter for grid motion
    pub frame: u64,
    /// Last frame a laser was fired
    pub last_fire_frame: u64,
    /// Active lasers
    pub lasers: Vec<Laser>,
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
            moving_left: false,
            moving_right: false,
            moving_up: false,
            moving_down: false,
            firing: false,
            frame: 0,
            last_fire_frame: 0,
            lasers: Vec::new(),
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

            // Continuous movement and firing
            if self.moving_left {
                self.ship_x = (self.ship_x - 0.04).max(-1.0);
            }
            if self.moving_right {
                self.ship_x = (self.ship_x + 0.04).min(1.0);
            }
            if self.moving_up {
                self.ship_y = (self.ship_y - 0.04).max(-1.0);
            }
            if self.moving_down {
                self.ship_y = (self.ship_y + 0.04).min(1.0);
            }
            if self.firing {
                self.fire_laser();
            }

            // Move enemies down
            let speed = 0.02;
            for enemy in &mut self.enemies {
                enemy.update(speed);
            }

            // Update lasers (move up)
            let laser_speed = 0.05;
            for laser in &mut self.lasers {
                laser.y -= laser_speed;
            }

            // Remove distant objects
            self.enemies.retain(|e| e.is_visible());
            self.lasers.retain(|l| l.y > -1.2);
        }
    }

    /// Fire a laser
    pub fn fire_laser(&mut self) {
        if !self.paused {
            // Cooldown check (e.g., every 8 frames)
            if self.frame > self.last_fire_frame + 8 {
                self.lasers.push(Laser {
                    x: self.ship_x,
                    y: self.ship_y,
                });
                self.last_fire_frame = self.frame;
            }
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
        assert!(game.lasers.is_empty());
        // Should have at least one enemy spawned
        assert!(!game.enemies.is_empty());
    }

    #[test]
    fn test_movement_clamping() {
        let mut game = GameState::new();
        game.ship_x = 0.9;
        game.moving_right = true;
        game.update(); // -> 0.94
        game.update(); // -> 0.98
        game.update(); // -> 1.0 (clamped)
        game.update(); // -> 1.0 (clamped)
        assert!((game.ship_x - 1.0).abs() < f32::EPSILON);

        game.ship_x = -0.9;
        game.moving_right = false;
        game.moving_left = true;
        game.update(); // -> -0.94
        game.update(); // -> -0.98
        game.update(); // -> -1.0 (clamped)
        game.update(); // -> -1.0 (clamped)
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
        game.fire_laser(); // Should ignore input
        game.update();
        assert_eq!(game.frame, initial_frame);
        assert!(game.lasers.is_empty());
    }

    #[test]
    fn test_fire_laser() {
        let mut game = GameState::new();
        // Move frames ahead to ensure cooldown pass relative to 0 if needed,
        // essentially first shot should always work if last_fire_frame is 0 and frame is > 8?
        // Actually initialized 0,0, condition is frame > last + 8.
        // Let's advance frame to 10
        for _ in 0..10 {
            game.update();
        }

        let prev_count = game.lasers.len();
        game.fire_laser();
        assert_eq!(game.lasers.len(), prev_count + 1);

        // Test cooldown
        game.fire_laser();
        assert_eq!(game.lasers.len(), prev_count + 1); // Should not increase yet
    }

    #[test]
    fn test_laser_movement() {
        let mut game = GameState::new();
        game.frame = 10;
        game.fire_laser();
        let initial_y = game.lasers[0].y;
        game.update();
        assert!(game.lasers[0].y < initial_y);
    }

    #[test]
    fn test_diagonal_movement() {
        let mut game = GameState::new();
        game.ship_x = 0.0;
        game.ship_y = 0.0;

        // Move down and right simultaneously
        game.moving_right = true;
        game.moving_down = true;
        game.update();

        assert!((game.ship_x - 0.04).abs() < f32::EPSILON);
        assert!((game.ship_y - 0.04).abs() < f32::EPSILON);
    }

    #[test]
    fn test_continuous_firing() {
        let mut game = GameState::new();
        // Advance frame to 10 to ensure we can fire immediately
        for _ in 0..10 {
            game.update();
        }

        let initial_lasers = game.lasers.len();

        // Start firing
        game.firing = true;
        game.update(); // Fired! Last fire frame is now 10
        assert_eq!(game.lasers.len(), initial_lasers + 1);

        game.update(); // Cooldown not met
        assert_eq!(game.lasers.len(), initial_lasers + 1);

        // Advance frames past cooldown (cooldown is 8 frames)
        for _ in 0..8 {
            game.update();
        }

        game.update(); // Cooldown met, fires again!
        assert_eq!(game.lasers.len(), initial_lasers + 2);
    }
}

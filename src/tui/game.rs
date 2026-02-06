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
    /// Depth (Starts at 0.0, moves away)
    pub z: f32,
}

/// Game state during active gameplay
pub struct GameState {
    /// Ship view X offset (-1.0 to 1.0)
    pub ship_x: f32,
    /// Ship view Y offset (-1.0 to 1.0)
    pub ship_y: f32,
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

            // Move enemies closer
            let speed = 1.5;
            for enemy in &mut self.enemies {
                enemy.update(speed);
            }

            // Update lasers
            let laser_speed = 2.0;
            for laser in &mut self.lasers {
                laser.z += laser_speed;
            }

            // Remove distant objects
            self.enemies.retain(|e| e.is_visible());
            self.lasers.retain(|l| l.z < 100.0);
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
                    z: 0.0,
                });
                self.last_fire_frame = self.frame;
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
        let initial_z = game.lasers[0].z;
        game.update();
        assert!(game.lasers[0].z > initial_z);
    }
}

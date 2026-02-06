//! Enemy logic and state

/// Enemy types
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EnemyType {
    Fighter,
    // Future types: Scout, Bomber, Boss
}

/// A visual enemy entity
#[derive(Clone, Debug)]
pub struct Enemy {
    /// X position normalized (-1.0 to 1.0)
    pub x: f32,
    /// Y position normalized (-1.0 to 1.0)
    pub y: f32,
    /// Z depth (0.0 to 100.0, where 0 is close and 100 is far)
    pub z: f32,
    /// Enemy type
    pub _kind: EnemyType,
}

impl Enemy {
    /// Create a new enemy at the given position scaling
    pub fn new(seed: u64) -> Self {
        // Deterministic-ish spawn based on seed
        let x = ((seed % 200) as f32 / 100.0) - 1.0;
        // Keep somewhat centered vertically
        let y = (((seed / 200) % 100) as f32 / 100.0) * 0.5 - 0.25;

        Self {
            x,
            y,
            z: 100.0, // Start far away
            _kind: EnemyType::Fighter,
        }
    }

    /// Update enemy position (move closer)
    pub fn update(&mut self, speed: f32) {
        self.z -= speed;
    }

    /// Check if enemy has passed the player
    pub fn is_visible(&self) -> bool {
        self.z > 0.0
    }
}

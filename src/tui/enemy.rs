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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enemy_spawn() {
        let enemy = Enemy::new(12345);
        assert_eq!(enemy.z, 100.0);
        // Ensure x, y within bounds
        assert!(enemy.x >= -1.0 && enemy.x <= 1.0);
    }

    #[test]
    fn test_enemy_update() {
        let mut enemy = Enemy::new(1);
        let start_z = enemy.z;
        enemy.update(1.0);
        assert_eq!(enemy.z, start_z - 1.0);
    }

    #[test]
    fn test_visibility() {
        let mut enemy = Enemy::new(1);
        enemy.z = 0.1;
        assert!(enemy.is_visible());

        enemy.update(0.2); // z becomes -0.1
        assert!(!enemy.is_visible());
    }
}

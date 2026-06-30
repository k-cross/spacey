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
    /// Enemy type
    pub _kind: EnemyType,
}

impl Enemy {
    /// Create a new enemy at the top of the screen
    pub fn new(seed: u64) -> Self {
        // Deterministic-ish spawn based on seed
        let x = ((seed % 200) as f32 / 100.0) - 1.0;

        Self {
            x,
            y: -1.2, // Start slightly above the screen
            _kind: EnemyType::Fighter,
        }
    }

    /// Update enemy position (move downwards)
    pub fn update(&mut self, speed: f32) {
        self.y += speed;
    }

    /// Check if enemy is still visible (hasn't passed the bottom of the screen)
    pub fn is_visible(&self) -> bool {
        self.y <= 1.2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enemy_spawn() {
        let enemy = Enemy::new(12345);
        assert_eq!(enemy.y, -1.2);
        // Ensure x within bounds
        assert!(enemy.x >= -1.0 && enemy.x <= 1.0);
    }

    #[test]
    fn test_enemy_update() {
        let mut enemy = Enemy::new(1);
        let start_y = enemy.y;
        enemy.update(0.1);
        assert_eq!(enemy.y, start_y + 0.1);
    }

    #[test]
    fn test_visibility() {
        let mut enemy = Enemy::new(1);
        enemy.y = 1.0;
        assert!(enemy.is_visible());

        enemy.update(0.3); // y becomes 1.3
        assert!(!enemy.is_visible());
    }
}

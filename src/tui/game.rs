//! Game state and logic (Component-Oriented Architecture)
#![allow(clippy::manual_is_multiple_of)]

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EntityType {
    Player,
    Enemy,
    Laser,
    Explosion,
}

#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub prev_x: f32,
    pub prev_y: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct Velocity {
    pub dx: f32,
    pub dy: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct Collider {
    pub radius: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct Lifetime {
    pub timer: u32,
    pub max: u32,
}

pub const MAX_ENTITIES: usize = 1000;

pub struct GameState {
    pub active: Vec<bool>,
    pub entity_types: Vec<EntityType>,
    pub positions: Vec<Position>,
    pub velocities: Vec<Option<Velocity>>,
    pub colliders: Vec<Option<Collider>>,
    pub lifetimes: Vec<Option<Lifetime>>,

    pub player_id: Option<usize>,

    pub moving_left: bool,
    pub moving_right: bool,
    pub moving_up: bool,
    pub moving_down: bool,
    pub firing: bool,

    pub frame: u64,
    pub last_fire_frame: u64,
    pub paused: bool,
    pub game_over: bool,
    pub should_exit: bool,
    pub score: u32,
    pub altitude: u32,
    pub shield: u8,
}

impl GameState {
    pub fn new() -> Self {
        let mut state = Self {
            active: vec![false; MAX_ENTITIES],
            entity_types: vec![EntityType::Player; MAX_ENTITIES],
            positions: vec![
                Position {
                    x: 0.0,
                    y: 0.0,
                    prev_x: 0.0,
                    prev_y: 0.0
                };
                MAX_ENTITIES
            ],
            velocities: vec![None; MAX_ENTITIES],
            colliders: vec![None; MAX_ENTITIES],
            lifetimes: vec![None; MAX_ENTITIES],

            player_id: None,
            moving_left: false,
            moving_right: false,
            moving_up: false,
            moving_down: false,
            firing: false,

            frame: 0,
            last_fire_frame: 0,
            paused: false,
            game_over: false,
            should_exit: false,
            score: 0,
            altitude: 1500,
            shield: 3,
        };

        let player = state.spawn_entity(EntityType::Player);
        state.positions[player] = Position {
            x: 0.0,
            y: 0.0,
            prev_x: 0.0,
            prev_y: 0.0,
        };
        state.colliders[player] = Some(Collider { radius: 0.08 });
        state.player_id = Some(player);

        state.spawn_enemy();
        state
    }

    fn spawn_entity(&mut self, kind: EntityType) -> usize {
        for i in 0..MAX_ENTITIES {
            if !self.active[i] {
                self.active[i] = true;
                self.entity_types[i] = kind;
                self.velocities[i] = None;
                self.colliders[i] = None;
                self.lifetimes[i] = None;
                return i;
            }
        }
        MAX_ENTITIES - 1
    }

    fn spawn_enemy(&mut self) {
        use std::time::{SystemTime, UNIX_EPOCH};
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_micros() as u64;
        let x = ((seed % 200) as f32 / 100.0) - 1.0;

        let id = self.spawn_entity(EntityType::Enemy);
        self.positions[id] = Position {
            x,
            y: -1.2,
            prev_x: x,
            prev_y: -1.2,
        };
        self.velocities[id] = Some(Velocity { dx: 0.0, dy: 0.02 });
        self.colliders[id] = Some(Collider { radius: 0.05 });
    }

    pub fn fire_laser(&mut self) {
        if !self.paused
            && !self.game_over
            && self.frame > self.last_fire_frame + 8
            && let Some(player_id) = self.player_id
        {
            let p = self.positions[player_id];
            let id = self.spawn_entity(EntityType::Laser);
            self.positions[id] = Position {
                x: p.x,
                y: p.y,
                prev_x: p.x,
                prev_y: p.y,
            };
            self.velocities[id] = Some(Velocity { dx: 0.0, dy: -0.05 });
            self.colliders[id] = Some(Collider { radius: 0.02 });
            self.last_fire_frame = self.frame;
        }
    }

    fn spawn_explosion(&mut self, x: f32, y: f32) {
        let id = self.spawn_entity(EntityType::Explosion);
        self.positions[id] = Position {
            x,
            y,
            prev_x: x,
            prev_y: y,
        };
        self.lifetimes[id] = Some(Lifetime { timer: 0, max: 10 });
    }

    pub fn update(&mut self) {
        if !self.paused {
            self.frame = self.frame.wrapping_add(1);

            if self.frame % 10 == 0 {
                self.altitude = self.altitude.wrapping_add(1);
                self.score = self.score.wrapping_add(5);
            }

            if self.frame % 150 == 0 {
                self.spawn_enemy();
            }

            for i in 0..MAX_ENTITIES {
                if self.active[i] {
                    self.positions[i].prev_x = self.positions[i].x;
                    self.positions[i].prev_y = self.positions[i].y;
                }
            }

            if let Some(p_id) = self.player_id
                && !self.game_over
            {
                let speed = 0.04;
                let mut p = self.positions[p_id];
                if self.moving_left {
                    p.x = (p.x - speed).max(-1.0);
                }
                if self.moving_right {
                    p.x = (p.x + speed).min(1.0);
                }
                if self.moving_up {
                    p.y = (p.y - speed).max(-1.0);
                }
                if self.moving_down {
                    p.y = (p.y + speed).min(1.0);
                }
                self.positions[p_id] = p;
            }

            if self.firing && !self.game_over {
                self.fire_laser();
            }

            for i in 0..MAX_ENTITIES {
                if self.active[i]
                    && let Some(vel) = self.velocities[i]
                {
                    self.positions[i].x += vel.dx;
                    self.positions[i].y += vel.dy;
                }
            }

            let mut to_destroy = Vec::new();
            let mut new_explosions = Vec::new();

            for i in 0..MAX_ENTITIES {
                if !self.active[i] || self.entity_types[i] != EntityType::Enemy {
                    continue;
                }
                let e_pos = self.positions[i];
                let e_col = self.colliders[i].unwrap_or(Collider { radius: 0.0 });

                for j in 0..MAX_ENTITIES {
                    if !self.active[j] || self.entity_types[j] != EntityType::Laser {
                        continue;
                    }
                    let l_pos = self.positions[j];
                    let dx = e_pos.x - l_pos.x;
                    let dy = e_pos.y - l_pos.y;
                    if dx * dx + dy * dy < e_col.radius * e_col.radius {
                        to_destroy.push(i);
                        to_destroy.push(j);
                        self.score = self.score.wrapping_add(10);
                        new_explosions.push((e_pos.x, e_pos.y));
                    }
                }

                if let Some(p_id) = self.player_id
                    && self.active[p_id]
                {
                    let p_pos = self.positions[p_id];
                    let p_col = self.colliders[p_id].unwrap_or(Collider { radius: 0.08 });
                    let dx = e_pos.x - p_pos.x;
                    let dy = e_pos.y - p_pos.y;
                    let rad = e_col.radius + p_col.radius;
                    if dx * dx + dy * dy < rad * rad {
                        to_destroy.push(i);
                        new_explosions.push((e_pos.x, e_pos.y));
                        if self.shield > 0 {
                            self.shield -= 1;
                        } else {
                            self.game_over = true;
                            self.active[p_id] = false;
                            new_explosions.push((p_pos.x, p_pos.y));
                        }
                    }
                }
            }

            for id in to_destroy {
                self.active[id] = false;
            }
            for (ex, ey) in new_explosions {
                self.spawn_explosion(ex, ey);
            }

            for i in 0..MAX_ENTITIES {
                if self.active[i] {
                    if let Some(mut life) = self.lifetimes[i] {
                        life.timer = life.timer.saturating_add(1);
                        if life.timer >= life.max {
                            self.active[i] = false;
                        } else {
                            self.lifetimes[i] = Some(life);
                        }
                    } else if (self.entity_types[i] == EntityType::Enemy
                        && self.positions[i].y > 1.2)
                        || (self.entity_types[i] == EntityType::Laser && self.positions[i].y < -1.2)
                    {
                        self.active[i] = false;
                    }
                }
            }
        }
    }

    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused;
    }

    pub fn exit_to_menu(&mut self) {
        if self.paused || self.game_over {
            self.should_exit = true;
        }
    }

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
        assert!(!game.game_over);
        assert!(!game.should_exit);
        assert!(game.player_id.is_some());
    }

    #[test]
    fn test_movement_clamping() {
        let mut game = GameState::new();
        game.moving_right = true;

        let p_id = game.player_id.unwrap();
        game.positions[p_id].x = 0.9;

        game.update();
        game.update();
        game.update();

        assert!((game.positions[p_id].x - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_fire_laser() {
        let mut game = GameState::new();

        // Wait for cooldown
        game.frame = 10;
        game.fire_laser();

        let mut lasers = 0;
        for i in 0..MAX_ENTITIES {
            if game.active[i] && game.entity_types[i] == EntityType::Laser {
                lasers += 1;
            }
        }
        assert_eq!(lasers, 1);
    }
}

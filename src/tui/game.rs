//! Game state and logic (Component-Oriented Architecture)
#![allow(clippy::manual_is_multiple_of)]

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EntityType {
    Player,
    Enemy,
    Laser,
    Explosion,
    Asteroid,
}

#[derive(Clone, Debug)]
pub struct AsteroidData {
    pub points: Vec<(f32, f32)>,
    pub point_radius: f32,
    pub angle: f32,
    pub spin_rate: f32,
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
    pub asteroids: Vec<Option<AsteroidData>>,

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
    pub invincibility_timer: u32,
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
            asteroids: vec![None; MAX_ENTITIES],

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
            invincibility_timer: 0,
        };

        let player = state.spawn_entity(EntityType::Player);
        state.positions[player] = Position {
            x: 0.0,
            y: 0.0,
            prev_x: 0.0,
            prev_y: 0.0,
        };
        state.colliders[player] = Some(Collider { radius: 0.08 });
        state.velocities[player] = Some(Velocity { dx: 0.0, dy: 0.0 });
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
                self.asteroids[i] = None;
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

    fn spawn_asteroid(&mut self) {
        use std::time::{SystemTime, UNIX_EPOCH};
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_micros() as u64;
        let x = ((seed % 200) as f32 / 100.0) - 1.0;

        let id = self.spawn_entity(EntityType::Asteroid);
        self.positions[id] = Position {
            x,
            y: -1.2,
            prev_x: x,
            prev_y: -1.2,
        };
        // Move downwards slowly
        self.velocities[id] = Some(Velocity { dx: 0.0, dy: 0.005 });

        // Generate points for the asteroid (large or small)
        let mut points = Vec::new();
        let is_large = seed % 3 != 0; // 66% chance to be a large one with a path

        if is_large {
            let num_points = 45;
            let radius = 0.45; // Make the asteroid physically much larger
            let road_width = 0.22; // Widen the road significantly so player (radius 0.08) + asteroid chunks (radius 0.03) can pass easily

            // Fill a circle rather than just the perimeter to make it look substantial.
            for r_step in 1..=4 {
                let r = radius * (r_step as f32 / 4.0);
                for i in 0..num_points {
                    let angle = (i as f32 / num_points as f32) * std::f32::consts::PI * 2.0;
                    let px = angle.cos() * r;
                    let py = angle.sin() * r;
                    // Leave a gap in the middle to form a road along the local Y axis
                    if px.abs() > road_width {
                        points.push((px, py));
                    }
                }
            }
        } else {
            let num_points = 12;
            let radius = 0.08;
            for r_step in 1..=2 {
                let r = radius * (r_step as f32 / 2.0);
                for i in 0..num_points {
                    let angle = (i as f32 / num_points as f32) * std::f32::consts::PI * 2.0;
                    let px = angle.cos() * r;
                    let py = angle.sin() * r;
                    points.push((px, py));
                }
            }
        }

        self.asteroids[id] = Some(AsteroidData {
            points,
            point_radius: 0.03,
            angle: 0.0,
            spin_rate: 0.01 * if seed % 2 == 0 { 1.0 } else { -1.0 },
        });
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

            if self.invincibility_timer > 0 {
                self.invincibility_timer -= 1;
            }

            if self.frame % 10 == 0 {
                self.altitude = self.altitude.wrapping_add(1);
                self.score = self.score.wrapping_add(5);
            }

            if self.frame % 150 == 0 {
                self.spawn_enemy();
            }
            if self.frame % 350 == 0 {
                self.spawn_asteroid();
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
                let accel = 0.005;
                let friction = 0.85;
                let max_speed = 0.06;
                let mut vel = self.velocities[p_id].unwrap_or(Velocity { dx: 0.0, dy: 0.0 });

                if self.moving_left {
                    vel.dx -= accel;
                }
                if self.moving_right {
                    vel.dx += accel;
                }
                if self.moving_up {
                    vel.dy -= accel;
                }
                if self.moving_down {
                    vel.dy += accel;
                }

                if !self.moving_left && !self.moving_right {
                    vel.dx *= friction;
                }
                if !self.moving_up && !self.moving_down {
                    vel.dy *= friction;
                }

                vel.dx = vel.dx.clamp(-max_speed, max_speed);
                vel.dy = vel.dy.clamp(-max_speed, max_speed);

                self.velocities[p_id] = Some(vel);
            }

            if self.firing && !self.game_over {
                self.fire_laser();
            }

            for i in 0..MAX_ENTITIES {
                if self.active[i] {
                    if let Some(vel) = self.velocities[i] {
                        self.positions[i].x += vel.dx;
                        self.positions[i].y += vel.dy;
                    }
                    if self.entity_types[i] == EntityType::Asteroid {
                        if let Some(data) = &mut self.asteroids[i] {
                            data.angle += data.spin_rate;
                        }
                    }
                }
            }

            if let Some(p_id) = self.player_id {
                self.positions[p_id].x = self.positions[p_id].x.clamp(-1.0, 1.0);
                self.positions[p_id].y = self.positions[p_id].y.clamp(-1.0, 1.0);
            }

            let mut to_destroy = Vec::new();
            let mut new_explosions = Vec::new();

            let mut active_enemies = Vec::with_capacity(64);
            let mut active_lasers = Vec::with_capacity(64);
            let mut active_asteroids = Vec::with_capacity(64);

            for i in 0..MAX_ENTITIES {
                if self.active[i] {
                    if self.entity_types[i] == EntityType::Enemy {
                        active_enemies.push(i);
                    } else if self.entity_types[i] == EntityType::Laser {
                        active_lasers.push(i);
                    } else if self.entity_types[i] == EntityType::Asteroid {
                        active_asteroids.push(i);
                    }
                }
            }

            for &i in &active_enemies {
                let e_pos = self.positions[i];
                let e_col = self.colliders[i].unwrap_or(Collider { radius: 0.0 });

                for &j in &active_lasers {
                    let l_pos = self.positions[j];
                    let dx = e_pos.x - l_pos.x;
                    let dy = e_pos.y - l_pos.y;
                    if dx * dx + dy * dy < e_col.radius * e_col.radius {
                        to_destroy.push(i);
                        to_destroy.push(j);
                        self.score = self.score.wrapping_add(100);
                        new_explosions.push((e_pos.x, e_pos.y));
                    }
                }

                if let Some(p_id) = self.player_id
                    && self.active[p_id]
                    && self.invincibility_timer == 0
                    && e_pos.y <= 1.0
                {
                    let p_pos = self.positions[p_id];
                    let p_col = self.colliders[p_id].unwrap_or(Collider { radius: 0.08 });
                    let dx = e_pos.x - p_pos.x;
                    let dy = e_pos.y - p_pos.y;
                    let rad = e_col.radius + p_col.radius;
                    if dx * dx + dy * dy < rad * rad {
                        to_destroy.push(i);
                        self.score = self.score.wrapping_add(100);
                        new_explosions.push((e_pos.x, e_pos.y));
                        if self.shield > 0 {
                            self.shield -= 1;
                            self.invincibility_timer = 60;
                        } else {
                            self.game_over = true;
                            self.active[p_id] = false;
                            new_explosions.push((p_pos.x, p_pos.y));
                        }
                    }
                }
            }

            for &a_id in &active_asteroids {
                if let Some(ast) = &self.asteroids[a_id] {
                    let a_pos = self.positions[a_id];
                    let cos_a = ast.angle.cos();
                    let sin_a = ast.angle.sin();
                    let radius = ast.point_radius;

                    for &(px, py) in &ast.points {
                        let rx = px * cos_a - py * sin_a;
                        let ry = px * sin_a + py * cos_a;
                        let cx = a_pos.x + rx;
                        let cy = a_pos.y + ry;

                        for &l_id in &active_lasers {
                            if to_destroy.contains(&l_id) {
                                continue;
                            }
                            let l_pos = self.positions[l_id];
                            let dx = cx - l_pos.x;
                            let dy = cy - l_pos.y;
                            let l_rad = self.colliders[l_id].map(|c| c.radius).unwrap_or(0.0);
                            if dx * dx + dy * dy < (radius + l_rad) * (radius + l_rad) {
                                to_destroy.push(l_id);
                                new_explosions.push((l_pos.x, l_pos.y));
                            }
                        }

                        for &e_id in &active_enemies {
                            if to_destroy.contains(&e_id) {
                                continue;
                            }
                            let e_pos = self.positions[e_id];
                            let dx = cx - e_pos.x;
                            let dy = cy - e_pos.y;
                            let e_rad = self.colliders[e_id].map(|c| c.radius).unwrap_or(0.0);
                            if dx * dx + dy * dy < (radius + e_rad) * (radius + e_rad) {
                                to_destroy.push(e_id);
                                new_explosions.push((e_pos.x, e_pos.y));
                            }
                        }

                        if let Some(p_id) = self.player_id {
                            if self.active[p_id]
                                && self.invincibility_timer == 0
                                && !to_destroy.contains(&p_id)
                                && cy <= 1.0
                            {
                                let p_pos = self.positions[p_id];
                                let p_rad = self.colliders[p_id].map(|c| c.radius).unwrap_or(0.08);
                                let dx = cx - p_pos.x;
                                let dy = cy - p_pos.y;
                                if dx * dx + dy * dy < (radius + p_rad) * (radius + p_rad) {
                                    new_explosions.push((p_pos.x, p_pos.y));
                                    if self.shield > 0 {
                                        self.shield -= 1;
                                        self.invincibility_timer = 60;
                                    } else {
                                        self.game_over = true;
                                        self.active[p_id] = false;
                                    }
                                }
                            }
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
                        || (self.entity_types[i] == EntityType::Asteroid
                            && self.positions[i].y > 1.6)
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
    fn test_diagonal_movement() {
        let mut game = GameState::new();
        let p_id = game.player_id.unwrap();
        game.positions[p_id].x = 0.0;
        game.positions[p_id].y = 0.0;

        // Move down and right simultaneously
        game.moving_right = true;
        game.moving_down = true;
        game.update();

        let expected = 0.005;
        assert!((game.positions[p_id].x - expected).abs() < f32::EPSILON);
        assert!((game.positions[p_id].y - expected).abs() < f32::EPSILON);
    }

    #[test]
    fn test_movement_clamping() {
        let mut game = GameState::new();
        game.moving_right = true;

        let p_id = game.player_id.unwrap();
        game.positions[p_id].x = 0.9;

        for _ in 0..100 {
            game.update();
        }

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

    #[test]
    fn test_update_performance() {
        let mut game = GameState::new();
        game.player_id = None; // Disable player to avoid player collision overhead

        // Fill game with many entities but spread them out so they don't collide!
        for i in 1..MAX_ENTITIES {
            game.active[i] = true;
            if i % 2 == 0 {
                game.entity_types[i] = EntityType::Enemy;
                game.colliders[i] = Some(Collider { radius: 0.05 });
                // Spread out horizontally and vertically
                game.positions[i] = Position {
                    x: (i as f32) * 0.01,
                    y: (i as f32) * 0.01,
                    prev_x: 0.0,
                    prev_y: 0.0,
                };
            } else {
                game.entity_types[i] = EntityType::Laser;
                game.colliders[i] = Some(Collider { radius: 0.02 });
                game.positions[i] = Position {
                    x: -(i as f32) * 0.01,
                    y: -(i as f32) * 0.01,
                    prev_x: 0.0,
                    prev_y: 0.0,
                };
            }
        }

        let start = std::time::Instant::now();
        for _ in 0..100 {
            game.update();
        }
        let elapsed = start.elapsed();
        // Ensure that 100 frames of 1000 entities without collisions is resolved very quickly (< 100ms)
        assert!(
            elapsed.as_millis() < 100,
            "Update loop is too slow: {}ms",
            elapsed.as_millis()
        );
    }

    #[test]
    fn test_collision_logic() {
        let mut game = GameState::new();
        game.active.fill(false); // Clear all
        game.player_id = None; // Disable player so we don't self-collide if slot 0 is reused

        // Set up one enemy
        let enemy = game.spawn_entity(EntityType::Enemy);
        game.positions[enemy] = Position {
            x: 0.0,
            y: 0.0,
            prev_x: 0.0,
            prev_y: 0.0,
        };
        game.colliders[enemy] = Some(Collider { radius: 0.1 });

        // Set up one laser overlapping it
        let laser = game.spawn_entity(EntityType::Laser);
        game.positions[laser] = Position {
            x: 0.0,
            y: 0.0,
            prev_x: 0.0,
            prev_y: 0.0,
        };
        game.colliders[laser] = Some(Collider { radius: 0.1 });

        let initial_score = game.score;

        game.update(); // Trigger collision

        // The entities should be destroyed (though slots may be reused for explosions)
        let has_enemy = game
            .active
            .iter()
            .zip(game.entity_types.iter())
            .any(|(&a, &t)| a && t == EntityType::Enemy);
        let has_laser = game
            .active
            .iter()
            .zip(game.entity_types.iter())
            .any(|(&a, &t)| a && t == EntityType::Laser);

        assert!(!has_enemy, "Enemy should be destroyed");
        assert!(!has_laser, "Laser should be destroyed");
        assert_eq!(game.score, initial_score + 100);
    }

    #[test]
    fn test_asteroid_collision() {
        let mut game = GameState::new();
        game.active.fill(false);

        // Spawn asteroid
        let ast_id = game.spawn_entity(EntityType::Asteroid);
        game.positions[ast_id] = Position {
            x: 0.0,
            y: 0.0,
            prev_x: 0.0,
            prev_y: 0.0,
        };
        game.asteroids[ast_id] = Some(AsteroidData {
            points: vec![(0.0, 0.0)],
            point_radius: 0.1,
            angle: 0.0,
            spin_rate: 0.0,
        });

        // Spawn laser
        let laser = game.spawn_entity(EntityType::Laser);
        game.positions[laser] = Position {
            x: 0.0,
            y: 0.0,
            prev_x: 0.0,
            prev_y: 0.0,
        };
        game.colliders[laser] = Some(Collider { radius: 0.1 });

        game.update(); // Trigger collision

        let has_laser = game
            .active
            .iter()
            .zip(game.entity_types.iter())
            .any(|(&a, &t)| a && t == EntityType::Laser);
        assert!(!has_laser, "Laser should be destroyed by asteroid");
    }
}

// https://www.youtube.com/watch?v=ZyvEOnP6240

use rand::Rng;
use sfml::{graphics::Color, system::Vector2f};

pub struct Entity {
    pos: Vector2f,
    vel: Vector2f,
    speed: f32,
    max_speed: f32,
    target_dir: Vector2f,
    color: Color,
    size: f32,
}

impl Entity {
    pub fn new(pos: Vector2f, speed: f32, max_speed: f32, target_dir: Vector2f) -> Self {
        let mut rng = rand::thread_rng();
        let min_rgba = (15, 0, 0, 10);
        let max_rgba = (255, 25, 185, 100);
        let size_range = (1.0, 6.0);

        return Self {
            pos: pos,
            vel: Vector2f::new(0.0, 0.0),
            speed: speed,
            max_speed: max_speed,
            target_dir: target_dir,
            color: Color::rgba(
                rng.gen_range(min_rgba.0..max_rgba.0),
                rng.gen_range(min_rgba.1..max_rgba.1),
                rng.gen_range(min_rgba.2..max_rgba.2),
                rng.gen_range(min_rgba.3..max_rgba.3),
            ),
            size: rng.gen_range(size_range.0..size_range.1),
        };
    }

    pub fn update(&mut self, bounds: (f32, f32, f32, f32)) {
        // Apply velocity calculations and limit to max speed
        self.vel += self.target_dir * self.speed;

        if self.vel.x >= self.max_speed {
            self.vel.x = self.max_speed;
        }
        if self.vel.y >= self.max_speed {
            self.vel.y = self.max_speed;
        }

        // Reverse velocity on collide with map bounds + random walk + pushoff to middle
        let arena_middle = Vector2f::new(bounds.2 / 2.0 - bounds.0, bounds.3 / 2.0 - bounds.1);
        if self.pos.x <= bounds.0 || self.pos.x + self.size >= bounds.2 {
            self.vel.x = -self.vel.x;
            self.set_random_target();
            self.push_towards_pos(arena_middle, 1.0);
        }
        if self.pos.y <= bounds.1 || self.pos.y + self.size >= bounds.3 {
            self.vel.y = -self.vel.y;
            self.set_random_target();
            self.push_towards_pos(arena_middle, 1.0);
        }

        // Update position
        self.pos += self.vel;
    }

    pub fn get_pos(&self) -> Vector2f {
        return self.pos;
    }

    pub fn get_color(&self) -> Color {
        return self.color;
    }

    pub fn get_size(&self) -> f32 {
        return self.size;
    }

    pub fn set_random_target(&mut self) {
        let mut rng = rand::thread_rng();
        self.target_dir = Vector2f::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0));
    }

    pub fn apply_force(&mut self, f_norm: Vector2f, strength: f32) {
        self.vel += f_norm * strength;
        self.target_dir = f_norm;
    }

    pub fn push_towards_pos(&mut self, pos: Vector2f, strength: f32) {
        let x_dist = self.pos.x - pos.x;
        let y_dist = self.pos.y - pos.y;
        let radians = f32::atan2(y_dist, x_dist);

        let relative_pos_to_vec = -Vector2f::new(radians.cos(), radians.sin());
        let rel_pos_to_vec_len = f32::sqrt(
            relative_pos_to_vec.x * relative_pos_to_vec.x
                + relative_pos_to_vec.y * relative_pos_to_vec.y,
        ); // BRO WTF WHY ISNT THIS IMPLEMENTED????
        let rel_pos_to_vec_norm = Vector2f::new(
            relative_pos_to_vec.x / rel_pos_to_vec_len,
            relative_pos_to_vec.y / rel_pos_to_vec_len,
        );

        self.apply_force(rel_pos_to_vec_norm, strength);
    }
}

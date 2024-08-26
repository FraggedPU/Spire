// https://www.youtube.com/watch?v=ZyvEOnP6240

use rand::Rng;
use sfml::{
    graphics::Color,
    system::Vector2f,
};

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
    pub fn new(pos: Vector2f, speed: f32, max_speed: f32) -> Self {
        let mut rng = rand::thread_rng();
        let min_rgba = (15, 0, 0, 10);
        let max_rgba = (255, 25, 185, 100);
        let size_range = (1.0, 6.0);

        return Self {
            pos: pos,
            vel: Vector2f::new(0.0, 0.0),
            speed: speed,
            max_speed: max_speed,
            target_dir: Vector2f::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)),
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

        // Reverse velocity on collide with map bounds + random walk
        if self.pos.x <= bounds.0 || self.pos.x + self.size >= bounds.2 {
            self.vel.x = -self.vel.x;
            self.set_random_target();
        }
        if self.pos.y <= bounds.1 || self.pos.y + self.size >= bounds.3 {
            self.vel.y = -self.vel.y;
            self.set_random_target();
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
}

use rand::Rng;
use sfml::{
    graphics::{CircleShape, RenderTarget, RenderTexture, RenderWindow, Shape, Transformable},
    system::Vector2f,
};

use crate::entity::Entity;

pub struct World {
    entities: Vec<Entity>,
    bounds: (f32, f32, f32, f32),
}

impl World {
    pub fn new(
        num_entities: u32,
        base_accel_range: (f32, f32),
        max_speed_range: (f32, f32),
        bounds: (f32, f32, f32, f32),
    ) -> Self {
        let mut rng = rand::thread_rng();
        let mut temp_entity_vec: Vec<Entity> = Vec::new();

        // Populate
        for _ in 0..num_entities {
            let rand_pos = Vector2f::new(
                rng.gen_range(bounds.0..bounds.2),
                rng.gen_range(bounds.1..bounds.3),
            );
            let rand_acceleration = rng.gen_range(base_accel_range.0..base_accel_range.1);
            let rand_max_speed = rng.gen_range(max_speed_range.0..max_speed_range.1);

            temp_entity_vec.push(Entity::new(rand_pos, rand_acceleration, rand_max_speed));
        }

        return Self {
            entities: temp_entity_vec,
            bounds: bounds,
        };
    }

    pub fn update(&mut self) {
        for entity in &mut self.entities {
            entity.update(self.bounds);
        }
    }

    pub fn draw(&self, target: &mut RenderTexture) {
        let mut shape: CircleShape = CircleShape::new(1.0, 5);

        for entity in &self.entities {
            shape.set_fill_color(entity.get_color());
            shape.set_position(entity.get_pos());
            shape.set_radius(entity.get_size());
            target.draw(&shape);
        }
    }

    pub fn set_entity_focus_point(&mut self, origin: Vector2f, inverse: bool) {
        // let angle_to_focus = atan2f(origin.y, origin.x) - atan2f(entity.get_pos().y, entity.get_pos().x);
        for entity in &mut self.entities {
            let x_dist = entity.get_pos().x - origin.x;
            let y_dist = entity.get_pos().y - origin.y;
            let radians = f32::atan2(y_dist, x_dist);

            let force_vec = -Vector2f::new(radians.cos(), radians.sin());
            let force_vec_len = f32::sqrt(force_vec.x * force_vec.x + force_vec.y * force_vec.y); // BRO WTF WHY ISNT THIS IMPLEMENTED????
            let force_vec_norm =
                Vector2f::new(force_vec.x / force_vec_len, force_vec.y / force_vec_len);

            if inverse {
                entity.apply_force(-force_vec_norm, 0.07);
            } else {
                entity.apply_force(force_vec_norm, 0.07);
            }
        }
    }

    pub fn map(n: f32, start1: f32, stop1: f32, start2: f32, stop2: f32) -> f32 {
        return ((n - start1) / (stop1 - start1)) * (stop2 - start2) + start2;
    }
}

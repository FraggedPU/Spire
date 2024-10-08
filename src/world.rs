use std::f32::consts;

use rand::Rng;
use sfml::{
    graphics::{
        CircleShape, Color, RectangleShape, RenderTarget, RenderTexture, RenderWindow, Shape,
        Transformable,
    },
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
            let pos = Vector2f::new(75.0, 75.0);
            let target_dir_norm = Vector2f::new(rng.gen_range(0.5..1.0), rng.gen_range(0.5..1.0));
            let rand_acceleration = rng.gen_range(base_accel_range.0..base_accel_range.1);
            let rand_max_speed = rng.gen_range(max_speed_range.0..max_speed_range.1);

            temp_entity_vec.push(Entity::new(
                pos,
                rand_acceleration,
                rand_max_speed,
                target_dir_norm,
            ));
        }

        return Self {
            entities: temp_entity_vec,
            bounds: bounds,
        };
    }

    pub fn update(&mut self, map_color: bool) {
        for entity in &mut self.entities {
            entity.update(self.bounds, map_color);
        }
    }

    pub fn draw(&self, target: &mut RenderTexture) {
        let mut shape: RectangleShape = RectangleShape::new();

        for entity in &self.entities {
            shape.set_fill_color(entity.get_color());
            shape.set_position(entity.get_pos());
            shape.set_size(Vector2f::new(entity.get_size(), entity.get_size()));
            shape.set_rotation(
                f32::atan2(
                    entity.get_target_dir_norm().x,
                    entity.get_target_dir_norm().y,
                ) * 180.0
                    / consts::PI,
            );
            target.draw(&shape);
        }
    }

    pub fn set_entity_focus_point(&mut self, origin: Vector2f, inverse: bool, focus_strength: f32) {
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
                entity.apply_force(-force_vec_norm, focus_strength);
            } else {
                entity.apply_force(force_vec_norm, focus_strength);
            }
        }
    }

    pub fn map(n: f32, start1: f32, stop1: f32, start2: f32, stop2: f32) -> f32 {
        return ((n - start1) / (stop1 - start1)) * (stop2 - start2) + start2;
    }
}

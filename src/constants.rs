use crate::vec2::Vec2;

pub const WINDOW_WIDTH: u32 = 800;
pub const WINDOW_HEIGHT: u32 = 600;
pub const rocket_velocity_multiplier: f64 = 0.5;
pub const rocket_accel_multiplier: f64 = 20.0;
pub const rocket_velocity_decay_rate: f64 = 0.9;
pub const rocket_start_position: Vec2 = Vec2{x: 0.0, y: WINDOW_HEIGHT as f64/2.0};
pub const rocket_radius: f32 = 40.0;
pub const rocket_point_count: u32 = 50;


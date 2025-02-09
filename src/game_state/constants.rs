use bevy::prelude::Color;

pub const BLOCK_SIZE: f32 = 1.0;
pub const GRID_WIDTH: u32 = 10;
pub const GRID_HEIGHT: u32 = 15;
pub const GRID_DEPTH: u32 = 8;
pub const FALL_TIME: f32 = 1.0;
pub const ROTATION_DEGREES: f32 = 90.0;
pub const CAMERA_HEIGHT: f32 = 20.0;
pub const CAMERA_RADIUS: f32 = 25.0;
pub const ROTATION_SPEED: f32 = 1.0;
pub const MOVE_SPEED: f32 = 5.0;
pub const EMPTY_CELL: i32 = 0;
pub const FILLED_CELL: i32 = 1;

pub const COLORS: [Color; 2] = [Color::srgb(0.3, 0.1, 0.5), Color::srgb(0.5, 0.2, 0.1)];

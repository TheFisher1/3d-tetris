use crate::game::game_elements::components::TetrominoType;
use bevy::math::Vec3;
use bevy::prelude::Color;

pub const BLOCK_SIZE: f32 = 1.0;
pub const GRID_WIDTH: i32 = 8;
pub const GRID_HEIGHT: i32 = 18;
pub const GRID_DEPTH: i32 = 8;
pub const FALL_TIME: f32 = 1.0;
pub const ROTATION_DEGREES: f32 = 90.0;
pub const CAMERA_HEIGHT: f32 = 15.0;
pub const CAMERA_RADIUS: f32 = 27.0;
pub const ROTATION_SPEED: f32 = 1.0;

pub const EMPTY_CELL: isize = -1;

pub const POINTS_DELTA_PER_ROW: f32 = 10.0;

pub static TYPES: &[TetrominoType] = &[
    TetrominoType::I,
    TetrominoType::L,
    TetrominoType::S,
    TetrominoType::O,
];

pub static TETROMINO_TYPE_I_INDEX: usize = 0;
pub static TETROMINO_TYPE_L_INDEX: usize = 1;
pub static TETROMINO_TYPE_S_INDEX: usize = 2;
pub static TETROMINO_TYPE_O_INDEX: usize = 3;

pub const COLORS: [Color; 5] = [
    Color::srgb(0.3, 0.1, 0.5),
    Color::srgb(0.5, 0.2, 0.1),
    Color::srgb(0.0, 0.0, 1.0),
    Color::srgb(0.0, 1.0, 0.0),
    Color::srgb(1.0, 0.0, 0.0),
];

pub static TETROMINO_OFFSETS: &[&[Vec3]] = &[
    // I
    &[
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        },
        // Vec3 {
        //     x: 2.0,
        //     y: 0.0,
        //     z: 0.0,
        // },
        // Vec3 {
        //     x: 3.0,
        //     y: 0.0,
        //     z: 0.0,
        // },
    ],
    &[
        // L
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        },
        Vec3 {
            x: 2.0,
            y: 0.0,
            z: 0.0,
        },
        Vec3 {
            x: 2.0,
            y: 0.0,
            z: 1.0,
        },
    ],
    // O
    &[
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        },
        Vec3 {
            x: 1.0,
            y: 1.0,
            z: 0.0,
        },
        Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
    ],
    // S
    &[
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        },
        Vec3 {
            x: 1.0,
            y: 1.0,
            z: 0.0,
        },
        Vec3 {
            x: 1.0,
            y: 2.0,
            z: 0.0,
        },
        Vec3 {
            x: 2.0,
            y: 2.0,
            z: 0.0,
        },
    ],
];

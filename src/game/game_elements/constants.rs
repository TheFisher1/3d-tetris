use bevy::math::{Vec3};
use bevy::prelude::{Color};

pub const BLOCK_SIZE: f32 = 1.0;
pub const GRID_WIDTH: u32 = 6;
pub const GRID_HEIGHT: u32 = 15;
pub const GRID_DEPTH: u32 = 6;
pub const FALL_TIME: f32 = 1.0;
pub const ROTATION_DEGREES: f32 = 90.0;
pub const CAMERA_HEIGHT: f32 = 20.0;
pub const CAMERA_RADIUS: f32 = 25.0;
pub const ROTATION_SPEED: f32 = 1.0;

pub const _EMPTY_CELL: isize = -1;

pub const COLORS: [Color; 5] = [
    Color::srgb(0.3, 0.1, 0.5),
    Color::srgb(0.5, 0.2, 0.1),
    Color::srgb(0.0, 0.0, 1.0),
    Color::srgb(0.0, 1.0, 0.0),
    Color::srgb(1.0, 0.0, 0.0),
];

pub static TETROMINO_OFFSETS: &[&[Vec3]] = &[
    &[
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }, // I
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
            x: 3.0,
            y: 0.0,
            z: 0.0,
        },
    ],
    &[
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }, // L
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
    &[
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }, // O
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
    &[
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }, // Z
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

use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(
                GRID_WIDTH as f32 / 2.0,
                CAMERA_HEIGHT,
                GRID_DEPTH as f32 * 1.5
            )
            .looking_at(Vec3::new(GRID_WIDTH as f32 / 2.0, 5.0, GRID_DEPTH as f32 / 2.0), Vec3::Y),
            ..default()
        },
        GameCamera { 
            angle: 0.0,
            height: CAMERA_HEIGHT,
        },
    ));

    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // Initial tetromino
    spawn_tetromino(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec3::new(
            GRID_WIDTH as f32 / 2.0,
            GRID_HEIGHT as f32 - 2.0,
            GRID_DEPTH as f32 / 2.0
        ),
    );
}

pub fn spawn_tetromino(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
) {
    // Adjust position to be relative to grid center
    let adjusted_position = Vec3::new(
        position.x - (GRID_WIDTH as f32 / 2.0),  // Center the x coordinate
        position.y,
        position.z - (GRID_DEPTH as f32 / 2.0),  // Center the z coordinate
    );

    let block_mesh = meshes.add(Mesh::from(shape::Cube { size: BLOCK_SIZE }));
    let block_material = materials.add(StandardMaterial {
        base_color: Color::rgb(0.0, 0.8, 0.8),
        ..default()
    });

    commands
        .spawn((
            TransformBundle::from(Transform::from_xyz(
                adjusted_position.x,
                adjusted_position.y,
                adjusted_position.z
            )),
            Tetromino,
            Falling {
                timer: Timer::from_seconds(FALL_TIME, TimerMode::Repeating),
            },
            MovementCooldown {
                timer: Timer::from_seconds(0.1, TimerMode::Once),
            },
        ))
        .with_children(|parent| {
            for i in 0..4 {
                parent.spawn(PbrBundle {
                    mesh: block_mesh.clone(),
                    material: block_material.clone(),
                    transform: Transform::from_xyz(i as f32 - 1.5, 0.0, 0.0),
                    ..default()
                });
            }
        });
} 
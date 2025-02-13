use crate::game::game_state::*;
use crate::game::systems::settings::node_settings::NodeSettings;
use crate::game::systems::spawn::button::spawn_button;
use bevy::prelude::*;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Projection::Perspective(PerspectiveProjection::default()),
        Transform::from_xyz(GRID_WIDTH as f32 / 2.0, CAMERA_HEIGHT, GRID_DEPTH as f32).looking_at(
            Vec3::new(GRID_WIDTH as f32 / 2.0, 5.0, GRID_DEPTH as f32 / 2.0),
            Vec3::Y,
        ),
        GlobalTransform::default(),
        GameCamera {
            angle: 0.0,
            height: CAMERA_HEIGHT,
        },
    ));

    commands.spawn((
        PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
        GlobalTransform::default(),
    ));

    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::End,
            justify_content: JustifyContent::End,
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        position_type: PositionType::Absolute,
                        bottom: Val::Px(35.0),
                        right: Val::Px(200.0),
                        ..default()
                    },
                    BorderColor(Color::BLACK),
                    BorderRadius::MAX,
                    Interaction::default(),
                    RotateButton::Left,
                ))
                .with_child((
                    Text::new("Left"),
                    TextFont {
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));

            spawn_button(
                parent,
                "Right",
                RotateButton::Right,
                NodeSettings::new(150.0, 65.0)
                    .border(UiRect::all(Val::Px(5.0)))
                    .position_type(PositionType::Absolute)
                    .bottom(35.0)
                    .right(35.0),
            );

            spawn_button(
                parent,
                "Up",
                RotateButton::Up,
                NodeSettings::new(150.0, 65.0)
                    .border(UiRect::all(Val::Px(5.0)))
                    .position_type(PositionType::Absolute)
                    .bottom(35.0)
                    .right(450.0),
            );
        });

    spawn_tetromino(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec3::new(
            GRID_WIDTH as f32,
            GRID_HEIGHT as f32 - 2.0,
            GRID_DEPTH as f32,
        ),
    );
}

pub fn spawn_tetromino(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
) {
    let adjusted_position = Vec3::new(
        position.x - (GRID_WIDTH as f32 / 2.0).round(),
        position.y,
        position.z - (GRID_DEPTH as f32 / 2.0).round(),
    );

    let block_mesh = meshes.add(Mesh::from(Cuboid::new(BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE)));
    let block_material = materials.add(StandardMaterial {
        base_color: COLORS[0].clone(),
        ..default()
    });

    commands
        .spawn((
            Transform::from_xyz(
                adjusted_position.x,
                adjusted_position.y,
                adjusted_position.z,
            ),
            Tetromino { id: 0 },
            Active,
            Falling {
                timer: Timer::from_seconds(FALL_TIME, TimerMode::Repeating),
            },
            MovementCooldown {
                timer: Timer::from_seconds(0.1, TimerMode::Once),
            },
        ))
        .with_children(|parent| {
            for i in 0..2 {
                parent.spawn(PbrBundle {
                    mesh: Mesh3d::from(block_mesh.clone()),
                    material: MeshMaterial3d::from(block_material.clone()),
                    transform: Transform::from_xyz(0.0, i as f32, 0.0),
                    ..default()
                });
            }
        });
}

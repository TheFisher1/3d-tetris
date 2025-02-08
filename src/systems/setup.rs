use bevy::prelude::*;
use crate::game_state::{Falling, GameCamera, MovementCooldown, RotateButton, Tetromino, BLOCK_SIZE, CAMERA_HEIGHT, COLORS, FALL_TIME, GRID_DEPTH, GRID_HEIGHT, GRID_WIDTH};

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Projection::Perspective(PerspectiveProjection::default()),
        Transform::from_xyz(
            GRID_WIDTH as f32 / 2.0,
            CAMERA_HEIGHT,
            GRID_DEPTH as f32 * 1.5,
        ).looking_at(
                Vec3::new(GRID_WIDTH as f32 / 2.0, 5.0, GRID_DEPTH as f32 / 2.0),
                Vec3::Y,
            ),
        GlobalTransform::default(),
        GameCamera {
            angle: 0.0,
            height: CAMERA_HEIGHT,
        },
    ));

    // Light
    commands.spawn((
        PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
        GlobalTransform::default(),
    ));

    // commands
    //     .spawn(Node {
    //         width: Val::Percent(100.0),
    //         height: Val::Percent(100.0),
    //         align_items: AlignItems::End,
    //         justify_content: JustifyContent::End,
    //         ..default()
    //     }
    //     )
    //     .with_children(|parent| {
    //         parent
    //             .spawn((
    //                 Button,
    //                 Node {
    //                     width: Val::Px(150.0),
    //                     height: Val::Px(65.0),
    //                     border: UiRect::all(Val::Px(5.0)),
    //                     justify_content: JustifyContent::Center,
    //                     align_items: AlignItems::Center,
    //                     position_type: PositionType::Absolute,
    //                     bottom: Val::Px(35.0),
    //                     right: Val::Px(35.0),
    //                     ..default()
    //                 },
    //                 BorderColor(Color::BLACK),
    //                 BorderRadius::MAX,
    //                 Interaction::default(),
    //                 RotateButton::Left
    //             ))
    //             .with_child((
    //                 Text::new("Left"),
    //                 TextFont {
    //                     font_size: 33.0,
    //                     ..default()
    //                 },
    //                 TextColor(Color::srgb(0.9, 0.9, 0.9))
    //             ));
    //     });
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::End,
            justify_content: JustifyContent::End,
            ..default()
        })
        .with_children(|parent| {
            // Left rotation button
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

            // Right rotation button

            spawn_button(parent, "Right", RotateButton::Right,
                         NodeSettings::new(150.0, 65.0)
                             .border(UiRect::all(Val::Px(5.0)))
                             .position_type(PositionType::Absolute)
                             .bottom(35.0)
                             .right(35.0)
            );

            spawn_button(parent, "Up", RotateButton::Up,
                         NodeSettings::new(150.0, 65.0)
                             .border(UiRect::all(Val::Px(5.0)))
                             .position_type(PositionType::Absolute)
                             .bottom(35.0)
                             .right(450.0)
            );

            // parent
            //     .spawn((
            //         Button,
            //         Node {
            //             width: Val::Px(150.0),
            //             height: Val::Px(65.0),
            //             border: UiRect::all(Val::Px(5.0)),
            //             justify_content: JustifyContent::Center,
            //             align_items: AlignItems::Center,
            //             position_type: PositionType::Absolute,
            //             bottom: Val::Px(35.0),
            //             right: Val::Px(35.0),
            //             ..default()
            //         },
            //         BorderColor(Color::BLACK),
            //         BorderRadius::MAX,
            //         Interaction::default(),
            //         RotateButton::Right,
            //     ))
            //     .with_child((
            //         Text::new("Right"),
            //         TextFont {
            //             font_size: 33.0,
            //             ..default()
            //         },
            //         TextColor(Color::srgb(0.9, 0.9, 0.9)),
            //     ));
        });


    // Initial tetromino
    spawn_tetromino(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec3::new(
            GRID_WIDTH as f32,
            GRID_HEIGHT as f32 - 2.0,
            GRID_DEPTH as f32
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
        position.x - (GRID_WIDTH as f32 / 2.0),
        position.y,
        position.z - (GRID_DEPTH as f32 / 2.0),
    );

    let block_mesh = meshes.add(Mesh::from(Cuboid::new(BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE)));// { size: BLOCK_SIZE }));
    let block_material = materials.add(StandardMaterial {
        base_color: COLORS[0].clone(),
        ..default()
    });

    commands
        .spawn((
            Transform::from_xyz(
                adjusted_position.x,
                adjusted_position.y,
                adjusted_position.z
            ),
            Tetromino { id: 0 },
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
                    mesh: Mesh3d::from(block_mesh.clone()),
                    material: MeshMaterial3d::from(block_material.clone()),
                    transform: Transform::from_xyz(i as f32, 0.0, 0.0),
                    ..default()
                });
            }
        });
}

#[derive(Default)]
pub struct NodeSettings {
    pub width: f32,
    pub height: f32,
    pub border: UiRect,
    pub align_items: AlignItems,
    pub justify_content: JustifyContent,
    pub position_type: PositionType,
    pub bottom: f32,
    pub right: f32,
}

impl NodeSettings {

    pub fn new(width: f32, height: f32) -> Self {
        NodeSettings {
            width, height,
            ..Default::default()
        }
    }

    pub fn border(mut self, border: UiRect) -> Self {
        self.border = border;
        self
    }

    pub fn align_items(mut self, align_items: AlignItems) -> Self {
        self.align_items = align_items;
        self
    }

    pub fn justify_content(mut self, justify_content: JustifyContent) -> Self {
        self.justify_content = justify_content;
        self
    }

    pub fn position_type(mut self, position_type: PositionType) -> Self {
        self.position_type = position_type;
        self
    }

    pub fn bottom(mut self, bottom: f32) -> Self {
        self.bottom = bottom;
        self
    }

    pub fn right(mut self, right: f32) -> Self {
        self.right = right;
        self
    }

    fn build(self) -> Node {
        Node {
            width: Val::Px(self.width),
            height: Val::Px(self.height),
            border: UiRect::all(Val::Px(5.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            position_type: PositionType::Absolute,
            bottom: Val::Px(self.bottom),
            right: Val::Px(self.right),
            ..default()
        }
    }
}

fn spawn_button(
    parent: &mut ChildBuilder,
    button_text: &str,
    rotate_button_action: RotateButton,
    settings: NodeSettings
) {
    parent
        .spawn((
            Button,
            settings.build(),
            BorderColor(Color::BLACK),
            BorderRadius::MAX,
            Interaction::default(),
            rotate_button_action,
        ))
        .with_child((
            Text::new(button_text),
            TextFont {
                font_size: 33.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
        ));
}

pub fn spawn_new_tetromino(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    falling_query: Query<&Tetromino, With<Falling>>,
) {
    let position =  Vec3::new(
        GRID_WIDTH as f32,
        GRID_HEIGHT as f32 - 2.0,
        GRID_DEPTH as f32
    );

    // Only spawn a new tetromino if there are no falling tetrominoes
    if falling_query.is_empty() {
        let adjusted_position = Vec3::new(
            position.x - (GRID_WIDTH as f32 / 2.0),
            position.y,
            position.z - (GRID_DEPTH as f32 / 2.0),
        );

        let block_mesh = meshes.add(Mesh::from(Cuboid::new(BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE)));// { size: BLOCK_SIZE }));
        let block_material = materials.add(StandardMaterial {
            base_color: COLORS[1].clone(),
            ..default()
        });

        commands
            .spawn((
                Transform::from_xyz(
                    adjusted_position.x,
                    adjusted_position.y,
                    adjusted_position.z
                ),
                Tetromino { id: 1 },
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
                        mesh: Mesh3d::from(block_mesh.clone()),
                        material: MeshMaterial3d::from(block_material.clone()),
                        transform: Transform::from_xyz(i as f32, 0.0, 0.0),
                        ..default()
                    });
                }
            });
    }

}
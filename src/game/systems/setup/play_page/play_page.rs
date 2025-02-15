use crate::game::game_elements::*;
use crate::game::systems::play_page::button::spawn_button;
use crate::game::systems::settings::node_settings::NodeSettings;
use bevy::prelude::*;

#[derive(Component)]
pub struct RotationControls;

pub fn setup(mut commands: Commands) {
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
        RotationControls,
        Camera {
            order: 1,
            ..Default::default()
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
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::End,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Row,
                bottom: Val::Px(10.0),
                column_gap: Val::Px(5.0),
                ..default()
            },
            RotationControls,
        ))
        .with_children(|parent| {
            spawn_button(
                parent,
                "Left",
                RotateButton::Left,
                NodeSettings::builder(Val::Px(150.0), Val::Px(65.0))
                    .border(UiRect::all(Val::Px(5.0)))
                    .justify_content(JustifyContent::Center)
                    .align_items(AlignItems::Center)
                    .build(),
            );

            spawn_button(
                parent,
                "Right",
                RotateButton::Right,
                NodeSettings::builder(Val::Px(150.0), Val::Px(65.0))
                    .border(UiRect::all(Val::Px(5.0)))
                    .justify_content(JustifyContent::Center)
                    .align_items(AlignItems::Center)
                    .build(),
            );

            spawn_button(
                parent,
                "Up",
                RotateButton::Up,
                NodeSettings::builder(Val::Px(150.0), Val::Px(65.0))
                    .align_items(AlignItems::Center)
                    .justify_content(JustifyContent::Center)
                    .border(UiRect::all(Val::Px(5.0)))
                    .build(),
            );

            spawn_button(
                parent,
                "Down",
                RotateButton::Down,
                NodeSettings::builder(Val::Px(150.0), Val::Px(65.0))
                    .border(UiRect::all(Val::Px(5.0)))
                    .justify_content(JustifyContent::Center)
                    .align_items(AlignItems::Center)
                    .build(),
            )
        });
}

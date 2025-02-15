use crate::game::state::game_info::GameState;
use crate::game::systems::settings::node_settings::NodeSettings;
use bevy::prelude::*;

#[derive(Component)]
pub enum NavigationControls {
    Start,
    Quit,
}

#[derive(Component)]
pub struct NavigationGroup;

pub fn setup_home_page(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands
        .spawn((
            NodeSettings::builder(Val::Percent(100.0), Val::Percent(100.0))
                .align_items(AlignItems::Center)
                .justify_content(JustifyContent::Center)
                .flex_direction(FlexDirection::Column)
                .row_gap(Val::Px(10.0))
                .build()
                .node(),
            NavigationGroup,
        ))
        .with_children(|parent| {
            let basic_button = NodeSettings::builder(Val::Px(150.0), Val::Px(65.0));
            parent
                .spawn((
                    Button,
                    basic_button
                        .clone()
                        .border(UiRect::all(Val::Px(5.0)))
                        .justify_content(JustifyContent::Center)
                        .align_items(AlignItems::Center)
                        .build()
                        .node(),
                    BorderColor(Color::BLACK),
                    BorderRadius::MAX,
                    Interaction::default(),
                    NavigationControls::Start,
                ))
                .with_child((
                    Text::new("Start"),
                    TextFont {
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));

            parent
                .spawn((
                    Button,
                    basic_button
                        .clone()
                        .border(UiRect::all(Val::Px(5.0)))
                        .justify_content(JustifyContent::Center)
                        .align_items(AlignItems::Center)
                        .padding(UiRect::all(Val::Px(10.0)))
                        .build()
                        .node(),
                    BorderColor(Color::BLACK),
                    BorderRadius::MAX,
                    Interaction::default(),
                    NavigationControls::Quit,
                ))
                .with_child((
                    Text::new("Quit"),
                    TextFont {
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));
        });
}

pub fn handle_start_button(
    mut interaction_query: Query<
        (&Interaction, &NavigationControls),
        (Changed<Interaction>, With<Button>),
    >,
    mut state: ResMut<NextState<GameState>>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, button) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            match button {
                NavigationControls::Start => state.set(GameState::Playing),
                NavigationControls::Quit => {
                    exit.send(AppExit::Success);
                }
            }
        }
    }
}

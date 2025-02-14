
use bevy::prelude::*;
use crate::game::state::game_state::GameState;

#[derive(Component)]
pub enum NavigationButton {
    Start,
    Quit
}

#[derive(Component)]
pub struct NavigationGroup;

pub fn setup_home_page(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands
        .spawn((Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        }, NavigationGroup))
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
                        ..default()
                    },
                    BorderColor(Color::BLACK),
                    BorderRadius::MAX,
                    Interaction::default(),
                    NavigationButton::Start,
                ))
                .with_child((
                    Text::new("Start"),
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
        (&Interaction, &NavigationButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut state: ResMut<NextState<GameState>>,
) {
    for (interaction,  _) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            state.set(GameState::Playing)
        }
    }
}
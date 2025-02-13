use crate::game::game_elements::RotateButton;
use crate::game::systems::settings::node_settings::NodeSettings;
use bevy::color::Color;
use bevy::prelude::*;
use bevy::prelude::{
    default, BorderColor, BorderRadius, Button, Interaction, Text, TextColor, TextFont,
};

pub fn spawn_button(
    parent: &mut ChildBuilder,
    button_text: &str,
    rotate_button_action: RotateButton,
    settings: NodeSettings,
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

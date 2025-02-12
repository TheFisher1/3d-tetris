mod game;

use crate::game::TetrisPlugin;
use bevy::app::App;
use bevy::DefaultPlugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TetrisPlugin)
        .run();
}

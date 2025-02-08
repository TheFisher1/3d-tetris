mod game_state;
mod plugin;
mod systems;

use bevy::app::App;
use bevy::DefaultPlugins;
use crate::plugin::TetrisPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TetrisPlugin)
        .run();
}
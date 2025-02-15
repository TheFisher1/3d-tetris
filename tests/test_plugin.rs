use bevy::app::App;
use bevy::{DefaultPlugins, MinimalPlugins};
use tetris::TetrisPlugin;

#[test]
fn test_plugin() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins).add_plugins(TetrisPlugin);
}

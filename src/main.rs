use crate::app::*;
use bevy::input::InputPlugin;
use bevy::prelude::*;
mod app;

fn main() {
    let mut app = create_app();
    let add_camera_fun = |mut commands: Commands| {
        commands.spawn(Camera2dBundle::default());
    };
    app.add_systems(Startup, add_camera_fun);

    assert!(!app.is_plugin_added::<InputPlugin>());
    app.add_plugins(DefaultPlugins);

    app.run();
}

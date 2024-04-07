use bevy::prelude::*;
use bevy::window::{Window, WindowPlugin, WindowResolution};
use board_plugin::components::Coordinate;
use board_plugin::BoardPlugin;

#[cfg(feature = "inspect")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::new(700., 800.),
            title: "Mine Sweeper".to_string(),
            ..default()
        }),
        ..default()
    }))
    .add_plugins(BoardPlugin);

    #[cfg(feature = "inspect")]
    app.add_plugins(WorldInspectorPlugin::new())
        .register_type::<Coordinate>();

    app.add_systems(Startup, setup_camera);
    app.run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

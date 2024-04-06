use bevy::prelude::*;
use bevy::window::{Window, WindowResolution, WindowPlugin};

#[cfg(feature = "inspect")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
#[cfg(feature = "inspect")]
use bevy::input::common_conditions;



fn main() {
    let mut app = App::new();
    // Window setup
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::new(700., 800.),
          title: "Mine Sweeper".to_string(),
          ..default()
        }),
        ..default()
      }));

      #[cfg(feature = "inspect")]
    app.add_plugins(WorldInspectorPlugin::default().run_if(common_conditions::input_toggle_active(true, KeyCode::Escape)));

    app.add_systems(Startup, setup_camera);
    app.run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(
        Camera2dBundle::default()
    );
}
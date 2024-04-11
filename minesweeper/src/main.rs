use bevy::prelude::*;
use bevy::window::{Window, WindowPlugin, WindowResolution};
use bevy::input::keyboard::KeyboardInput;
use bevy::log;
#[cfg(feature = "inspect")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use board_plugin::BoardPlugin;
use board_plugin::resources::BoardOptions;
use board_plugin::states::AppState;

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
    .insert_resource(BoardOptions {
        map_size: (20, 20),
        bomb_count: 40,
        tile_padding: 3.0,
        ..Default::default()
    })
    .add_plugins(BoardPlugin)
    .add_systems(Update, escape_handler.run_if(in_state(AppState::InGame)));

    #[cfg(feature = "inspect")]
    app.add_plugins(WorldInspectorPlugin::new());

    app.add_systems(Startup, setup_camera);
    app.run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn escape_handler(mut key_evr: EventReader<KeyboardInput>, mut next_state: ResMut<NextState<AppState>>) {
    for event in key_evr.read(){
        if let KeyCode::Escape = event.key_code{
            log::info!("clearing game");
            next_state.set(AppState::Out);
        }
    }
}
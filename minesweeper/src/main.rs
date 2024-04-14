use bevy::input::keyboard::KeyboardInput;
use bevy::log;
use bevy::prelude::*;
use bevy::window::{Window, WindowPlugin, WindowResolution};
#[cfg(feature = "inspect")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use board_plugin::resources::{BoardAssets, BoardOptions, ExitWindowTitle, SpriteMaterial};
use board_plugin::states::AppState;
use board_plugin::BoardPlugin;

use std::path::PathBuf;
use std::str::FromStr;

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
    .init_resource::<ExitWindowTitle>()
    .insert_resource(ExitWindowTitle {
        text: "MENU".into(),
    })
    .add_systems(Startup, (setup_camera, setup_board))
    .add_plugins(BoardPlugin)
    .add_systems(Update, escape_handler.run_if(in_state(AppState::InGame)));

    #[cfg(feature = "inspect")]
    app.add_plugins(WorldInspectorPlugin::new());

    app.run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_board(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(BoardOptions {
        map_size: (20, 20),
        bomb_count: 40,
        tile_padding: 1.,
        safe_start: true,
        ..Default::default()
    });
    let cur_path = PathBuf::from_str(env!("CARGO_MANIFEST_DIR")).unwrap();
    let asset_path = cur_path.join("assets");
    commands.insert_resource(BoardAssets {
        label: "Default".to_string(),
        board_material: SpriteMaterial {
            color: Color::WHITE,
            ..Default::default()
        },
        tile_material: SpriteMaterial {
            color: Color::GRAY,
            ..Default::default()
        },
        covered_tile_material: SpriteMaterial {
            color: Color::DARK_GRAY,
            ..Default::default()
        },
        bomb_counter_font: asset_server.load(asset_path.join("fonts").join("pixeled.ttf")),
        bomb_counter_colors: BoardAssets::default_colors(),
        flag_material: SpriteMaterial {
            texture: asset_server.load(asset_path.join("sprites").join("flag.png")),
            color: Color::WHITE,
        },
        bomb_material: SpriteMaterial {
            texture: asset_server.load(asset_path.join("sprites").join("bomb.png")),
            color: Color::WHITE,
        },
    });
    log::info!("Loaded assets!");
}

fn escape_handler(
    mut key_evr: EventReader<KeyboardInput>,
    mut exit_window_tile: ResMut<ExitWindowTitle>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for event in key_evr.read() {
        if let KeyCode::Escape = event.key_code {
            log::info!("clearing game");
            exit_window_tile.text = "MENU".into();
            next_state.set(AppState::Out);
        }
    }
}

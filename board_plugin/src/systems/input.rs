use crate::events::{TileMarkEvent, TileTriggerEvent};
use crate::Board;
use bevy::input::ButtonInput;
use bevy::log;
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, Window};

pub fn handle_mouse_input(
    windows: Query<&Window, With<PrimaryWindow>>,
    board: Res<Board>,
    input: Res<ButtonInput<MouseButton>>,
    mut tile_trigger_ewr: EventWriter<TileTriggerEvent>,
    mut tile_mark_ewr: EventWriter<TileMarkEvent>,
) {
    let window = windows
        .into_iter()
        .next()
        .expect("no primary window found!");

    if input.just_pressed(MouseButton::Left) {
        let position = window.cursor_position();
        if let Some(pos) = position {
            log::trace!("Mouse button pressed: left at {}", pos);
            let tile_coordinate = board.mouse_position(window, pos);
            if let Some(coordinate) = tile_coordinate {
                tile_trigger_ewr.send(TileTriggerEvent(coordinate));
            }
        }
    } else if input.just_pressed(MouseButton::Right) {
        let position = window.cursor_position();
        if let Some(pos) = position {
            log::trace!("Mouse button pressed: right at {}", pos);
            let tile_coordinate = board.mouse_position(window, pos);
            if let Some(coordinate) = tile_coordinate {
                tile_mark_ewr.send(TileMarkEvent(coordinate));
            }
        }
    }
}

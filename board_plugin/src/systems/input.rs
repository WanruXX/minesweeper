use crate::events::TileTriggerEvent;
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
    }
}

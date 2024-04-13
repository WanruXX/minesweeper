use crate::button_style::ExitWindowTitle;
use crate::events::{TileTriggerEvent};
use crate::{AppState, Board, Bomb, BombNeighbor, Coordinate, Uncover};
use bevy::log;
use bevy::prelude::*;

pub fn left_click_handler(
    mut commands: Commands,
    board: Res<Board>,
    mut tile_trigger_evr: EventReader<TileTriggerEvent>,
) {
    for trigger_event in tile_trigger_evr.read() {
        if let Some(entity) = board.tile_to_uncover(&trigger_event.0) {
            commands.entity(*entity).insert(Uncover);
        }
    }
}

pub fn uncover_tiles(
    mut commands: Commands,
    mut board: ResMut<Board>,
    mut exit_window_tile: ResMut<ExitWindowTitle>,
    children: Query<(Entity, &Parent), With<Uncover>>,
    parents: Query<(&Coordinate, Option<&Bomb>, Option<&BombNeighbor>)>,
    mut next_state: ResMut<NextState<AppState>>,
    // mut board_completed_event_wr: EventWriter<BoardCompletedEvent>,
    // mut bomb_explosion_event_wr: EventWriter<BombExplosionEvent>,
) {
    for (entity, parent) in children.iter() {
        commands.entity(entity).despawn_recursive();

        let (coord, bomb, bomb_counter) = match parents.get(parent.get()) {
            Ok(v) => v,
            Err(e) => {
                log::error!("{}", e);
                continue;
            }
        };

        match board.try_uncover_tile(coord) {
            None => log::debug!("Tried to uncover an already uncovered tile"),
            Some(e) => log::debug!("Uncovered tile {} (entity: {:?})", coord, e),
        }

        if board.is_completed() {
            log::info!("Board completed");
            exit_window_tile.text = "YOU WON!".into();
            next_state.set(AppState::Out);
        }

        if bomb.is_some() {
            log::info!("Boom !");
            exit_window_tile.text = "GAME OVER!".into();
            next_state.set(AppState::Out);
        } else if bomb_counter.is_none() {
            // We propagate the uncovering by adding the `Uncover` component to adjacent tiles
            // which will then be removed next frame
            for entity in board.adjacent_covered_tiles(*coord) {
                commands.entity(entity).insert(Uncover);
            }
        };
    }
}

pub fn clear_tiles(mut commands: Commands, mut board: ResMut<Board>) {
    for (_coord, entity) in board.covered_tiles.iter() {
        commands.entity(*entity).despawn_recursive();
    }
    board.covered_tiles.clear();
}

use crate::bounds::Bounds2;
use crate::components::Coordinate;
use crate::tile_map::TileMap;
use bevy::log;
use bevy::prelude::*;
use bevy::utils::HashMap;

#[derive(Debug, Resource)]
pub struct Board {
    pub tile_map: TileMap,
    pub bounds: Bounds2,
    pub tile_size: f32,
    pub covered_tiles: HashMap<Coordinate, Entity>,
    pub marked_tiles: Vec<Coordinate>,
    pub entity: Entity,
}

impl Board {
    pub fn mouse_position(&self, window: &Window, position: Vec2) -> Option<Coordinate> {
        let new_position = Vec2::new(
            position.x - window.width() / 2.,
            window.height() / 2. - position.y,
        );
        if !self.bounds.in_bounds(new_position) {
            return None;
        }
        let coordinate = new_position - self.bounds.position;
        Some(Coordinate {
            x: (coordinate.x / self.tile_size) as u16,
            y: (coordinate.y / self.tile_size) as u16,
        })
    }

    pub fn tile_to_uncover(&self, coord: &Coordinate) -> Option<&Entity> {
        if self.marked_tiles.contains(coord) {
            None
        } else {
            self.covered_tiles.get(coord)
        }
    }

    pub fn try_uncover_tile(&mut self, coord: &Coordinate) -> Option<Entity> {
        if self.marked_tiles.contains(coord) {
            self.unmark_tile(coord);
        }
        self.covered_tiles.remove(coord)
    }

    pub fn adjacent_covered_tiles(&self, coord: Coordinate) -> Vec<Entity> {
        self.tile_map
            .safe_square_at(coord)
            .filter_map(|c| self.covered_tiles.get(&c))
            .copied()
            .collect()
    }

    pub fn try_toggle_mark(&mut self, coord: &Coordinate) -> Option<(Entity, bool)> {
        let entity = *self.covered_tiles.get(coord)?;
        let mark = if self.marked_tiles.contains(coord) {
            self.unmark_tile(coord)?;
            false
        } else {
            self.marked_tiles.push(*coord);
            true
        };
        Some((entity, mark))
    }

    fn unmark_tile(&mut self, coords: &Coordinate) -> Option<Coordinate> {
        let pos = match self.marked_tiles.iter().position(|a| a == coords) {
            None => {
                log::error!("Failed to unmark tile at {}", coords);
                return None;
            }
            Some(p) => p,
        };
        Some(self.marked_tiles.remove(pos))
    }

    pub fn is_completed(&self) -> bool {
        self.tile_map.bomb_count() as usize == self.covered_tiles.len()
    }
}

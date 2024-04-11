use crate::bounds::Bounds2;
use crate::components::Coordinate;
use crate::tile_map::TileMap;
use bevy::prelude::*;
use bevy::utils::HashMap;

#[derive(Debug, Resource)]
pub struct Board {
    pub tile_map: TileMap,
    pub bounds: Bounds2,
    pub tile_size: f32,
    pub covered_tiles: HashMap<Coordinate, Entity>,
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
        self.covered_tiles.get(coord)
      }
  
    pub fn try_uncover_tile(&mut self, coord: &Coordinate) -> Option<Entity> {
        self.covered_tiles.remove(coord)
    }

    pub fn adjacent_covered_tiles(&self, coord: Coordinate) -> Vec<Entity> {
        self.tile_map
            .safe_square_at(coord)
            .filter_map(|c| self.covered_tiles.get(&c))
            .copied()
            .collect()
    }
}

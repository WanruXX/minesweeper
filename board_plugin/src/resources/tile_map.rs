use crate::components::Coordinate;
use crate::resources::tile::Tile;
use rand::{thread_rng, Rng};
use std::ops::{Deref, DerefMut};

pub struct TileMap {
    bomb_count: u16,
    height: u16,
    width: u16,
    map: Vec<Vec<Tile>>,
}
// Delta coordinates for all 8 square neighbors
const SQUARE_COORDINATES: [(i8, i8); 8] = [
    (-1, -1), // Bottom left
    (0, -1),  // Bottom
    (1, -1),  // Bottom right
    (-1, 0),  // Left
    (1, 0),   // Right
    (-1, 1),  // Top Left
    (0, 1),   // Top
    (1, 1),   // Top right
];

impl TileMap {
    pub fn create(width: u16, height: u16) -> Self {
        let map = (0..height)
            .into_iter()
            .map(|_| (0..width).into_iter().map(|_| Tile::Empty).collect())
            .collect();
        Self {
            bomb_count: 0,
            height,
            width,
            map,
        }
    }

    #[cfg(feature = "inspect")]
    pub fn console_output(&self) -> String {
        let mut buffer = format!(
            "Map ({}, {}) with {} bombs:\n",
            self.width, self.height, self.bomb_count
        );
        let line: String = (0..=(self.width + 1)).into_iter().map(|_| '-').collect();
        buffer = format!("{}{}\n", buffer, line);
        for line in self.iter().rev() {
            buffer = format!("{}|", buffer);
            for tile in line.iter() {
                buffer = format!("{}{}", buffer, tile.console_output());
            }
            buffer = format!("{}|\n", buffer);
        }
        format!("{}{}", buffer, line)
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn bomb_count(&self) -> u16 {
        self.bomb_count
    }

    pub fn safe_square_at(&self, coordinate: Coordinate) -> impl Iterator<Item = Coordinate> {
        SQUARE_COORDINATES
            .iter()
            .copied()
            .map(move |tuple| coordinate + tuple)
    }

    pub fn isbomb_at(&self, coordinate: Coordinate) -> bool {
        if coordinate.x >= self.width || coordinate.y >= self.height {
            return false;
        };
        self.map[coordinate.y as usize][coordinate.x as usize].is_bomb()
    }

    pub fn bomb_count_at(&self, coordinate: Coordinate) -> u8 {
        if self.isbomb_at(coordinate) {
            return 0;
        }
        let res = self
            .safe_square_at(coordinate)
            .filter(|coord| self.isbomb_at(*coord))
            .count();
        res as u8
    }

    pub fn set_bombs(&mut self, bomb_count: u16) {
        self.bomb_count = bomb_count;
        let mut ramained_bombs = bomb_count;
        let mut rng = thread_rng();
        while ramained_bombs > 0 {
            let (x, y) = (
                rng.gen_range(0..self.width) as usize,
                rng.gen_range(0..self.height) as usize,
            );
            if let Tile::Empty = self[y][x] {
                self[y][x] = Tile::Bomb;
                ramained_bombs -= 1;
            }
        }
        for y in 0..self.height {
            for x in 0..self.width {
                let coord = Coordinate { x, y };
                if self.isbomb_at(coord) {
                    continue;
                }
                let num = self.bomb_count_at(coord);
                if num == 0 {
                    continue;
                }
                let tile = &mut self[y as usize][x as usize];
                *tile = Tile::BombNeighbor(num);
            }
        }
    }
}

impl Deref for TileMap {
    type Target = Vec<Vec<Tile>>;
    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for TileMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

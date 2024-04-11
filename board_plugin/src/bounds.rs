use bevy::prelude::Vec2;

#[derive(Debug, Copy, Clone)]
pub struct Bounds2 {
    pub position: Vec2,
    pub size: Vec2,
}

impl Bounds2 {
    pub fn in_bounds(&self, coord: Vec2) -> bool {
        coord.x >= self.position.x
            && coord.y >= self.position.y
            && coord.x <= self.position.x + self.size.x
            && coord.y <= self.position.y + self.size.y
    }
}
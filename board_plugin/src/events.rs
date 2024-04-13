use bevy::ecs::event::Event;

use crate::components::Coordinate;

#[derive(Debug, Copy, Clone, Event)]
pub struct TileTriggerEvent(pub Coordinate);

#[derive(Debug, Copy, Clone, Event)]
pub struct GameCompletedEvent(pub bool); 

#[derive(Debug, Copy, Clone, Event)]
pub struct TileMarkEvent(pub Coordinate);
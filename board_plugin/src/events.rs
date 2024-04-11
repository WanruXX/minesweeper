use bevy::ecs::event::Event;

use crate::components::Coordinate;

#[derive(Debug, Copy, Clone, Event)]
pub struct TileTriggerEvent(pub Coordinate); 
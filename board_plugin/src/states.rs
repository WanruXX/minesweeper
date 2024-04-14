use bevy::ecs::schedule::States;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, States)]
pub enum AppState {
    InGame,
    Out,
}
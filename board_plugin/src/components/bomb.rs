use bevy::prelude::Component;

#[cfg(feature = "inspect")]
use bevy_inspector_egui::prelude::*;
#[cfg(feature = "inspect")]
use bevy::prelude::Reflect;

#[cfg_attr(feature = "inspect", derive(Reflect, InspectorOptions))]
#[cfg_attr(feature = "inspect", reflect(InspectorOptions))]
#[cfg_attr(feature = "inspect", inspector(validate = |ability| ability.current_charges <= ability.max_charges))]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct Bomb;
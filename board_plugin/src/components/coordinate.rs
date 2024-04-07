use bevy::prelude::Component;
use std::fmt::{self, Display, Formatter};
use std::ops::{Add, Sub};

#[cfg(feature = "inspect")]
use bevy::prelude::Reflect;
#[cfg(feature = "inspect")]
use bevy_inspector_egui::prelude::*;


#[cfg_attr(feature = "inspect", derive(Reflect, InspectorOptions))]
#[cfg_attr(feature = "inspect", reflect(InspectorOptions))]
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct Coordinate {
    pub x: u16,
    pub y: u16,
}

impl Add for Coordinate {
    type Output = Self;
    fn add(self, rh: Self) -> Self {
        Self {
            x: self.x + rh.x,
            y: self.y + rh.y,
        }
    }
}

impl Add<(i8, i8)> for Coordinate {
    type Output = Self;
    fn add(self, (x, y): (i8, i8)) -> Self {
        Self {
            x: ((self.x as i16) + x as i16) as u16,
            y: ((self.y as i16) + y as i16) as u16,
        }
    }
}

impl Sub for Coordinate {
    type Output = Self;
    fn sub(self, rh: Self) -> Self {
        Self {
            x: self.x.saturating_sub(rh.x),
            y: self.y.saturating_sub(rh.y),
        }
    }
}

impl Display for Coordinate {
    fn fmt(&self, f:&mut Formatter<'_>) ->fmt::Result{
        write!(f, "({},{})", self.x, self.y)
        // f.write_str("({},{})", self.x, self.y)
    }
}

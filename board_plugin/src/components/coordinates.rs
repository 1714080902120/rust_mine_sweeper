// coordinates.rs
use std::fmt::{self, Display, Formatter};
use std::ops::{Add, Sub};



// use my_macro::EasyInspector;

use bevy::prelude::Component;
#[cfg(feature = "debug")]
use bevy_inspector_egui::prelude::*;
#[cfg(feature = "debug")]
use bevy::prelude::Reflect;
#[cfg_attr(feature = "debug", derive(Reflect, InspectorOptions))]
#[cfg_attr(feature = "debug", reflect(InspectorOptions))]
#[cfg_attr(feature = "debug", inspector(validate = |ability| ability.current_charges <= ability.max_charges))]
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]

// #[derive(EasyInspector)]
pub struct Coordinates {
    #[cfg_attr(feature = "debug", inspector(min = 0, max = 50))]
    pub x: u16,
    #[cfg_attr(feature = "debug", inspector(min = 0, max = 50))]
    pub y: u16,
}

// We want to be able to make coordinates sums..
impl Add for Coordinates {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<(i8, i8)> for Coordinates {
    type Output = Self;

    fn add(self, (x, y): (i8, i8)) -> Self::Output {
        Self {
            x: ((self.x as i16) + x as i16) as u16,
            y: ((self.y as i16) + y as i16) as u16,
        }
    }
}

// ..and subtractions
impl Sub for Coordinates {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.saturating_sub(rhs.x),
            y: self.y.saturating_sub(rhs.y),
        }
    }
}

impl Display for Coordinates {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
} 
 
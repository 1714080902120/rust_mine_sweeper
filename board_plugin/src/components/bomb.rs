// bomb.rs
use bevy::prelude::Component;

/// Bomb component
#[cfg(feature = "debug")]
use bevy_inspector_egui::prelude::*;
#[cfg(feature = "debug")]
use bevy::prelude::Reflect;

#[cfg_attr(feature = "debug", derive(Reflect, InspectorOptions))]
#[cfg_attr(feature = "debug", reflect(InspectorOptions))]
#[cfg_attr(feature = "debug", inspector(validate = |ability| ability.current_charges <= ability.max_charges))]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct Bomb;
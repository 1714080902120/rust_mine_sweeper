use bevy::prelude::Component;

/// Bomb neighbor component
#[cfg(feature = "debug")]
use bevy_inspector_egui::prelude::*;
#[cfg(feature = "debug")]
use bevy::prelude::Reflect;

#[cfg_attr(feature = "debug", derive(Reflect, InspectorOptions))]
#[cfg_attr(feature = "debug", reflect(InspectorOptions))]
#[cfg_attr(feature = "debug", inspector(validate = |ability| ability.current_charges <= ability.max_charges))]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct BombNeighbor {
    /// Number of neighbor bombs
    #[cfg_attr(feature = "debug", inspector(min = 0, max = 8))]
    pub count: u8,
}
// systems/uncover.rs
use bevy::prelude::*;
use crate::{Board, Bomb, BombNeighbor, Coordinates, Uncover};
use crate::events::TileTriggerEvent;
use bevy::log;

pub fn trigger_event_handler(
    mut commands: Commands,
    board: Res<Board>,
    mut tile_trigger_evr: EventReader<TileTriggerEvent>,
) {
    for trigger_event in tile_trigger_evr.iter() {
        if let Some(entity) = board.tile_to_uncover(&trigger_event.0) {
            commands.entity(*entity).insert(Uncover);
        }
    }
}

pub fn uncover_tiles(
    mut commands: Commands,
    mut board: ResMut<Board>,
    children: Query<(Entity, &Parent), With<Uncover>>,
    parents: Query<(&Coordinates, Option<&Bomb>, Option<&BombNeighbor>)>,
) {
    for (entity, parent) in children.iter() {
        // we destroy the tile cover entity
        commands
            .entity(entity)
            .despawn_recursive();

            let (coords, bomb, bomb_counter) = match parents.get(parent.get()) {
                Ok(v) => v,
                Err(e) => {
                    log::error!("{}", e);
                    continue;
                }
            };
        
            // We remove the entity from the board covered tile map
            match board.try_uncover_tile(coords) {
                None => log::debug!("Tried to uncover an already uncovered tile"),
                Some(e) => log::debug!("Uncovered tile {} (entity: {:?})", coords, e),
            }
            if bomb.is_some() {
                log::info!("Boom !");
                // TODO: Add explosion event
            }
            // If the tile is empty..
            else if bomb_counter.is_none() {
                // .. We propagate the uncovering by adding the `Uncover` component to adjacent tiles
                // which will then be removed next frame
                for entity in board.adjacent_covered_tiles(*coords) {
                    commands.entity(entity).insert(Uncover);
                }
            };
    }
}
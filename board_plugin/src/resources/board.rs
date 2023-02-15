// board.rs
use crate::bounds::Bounds2;
use crate::{Coordinates, TileMap};
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy::log;

#[derive(Debug, Resource)]
pub struct Board {
    pub tile_map: TileMap,
    pub bounds: Bounds2,
    pub tile_size: f32,
    pub covered_tiles: HashMap<Coordinates, Entity>,
    pub entity: Entity,
    pub marked_tiles: Vec<Coordinates>,
    pub need_stop_listening_pressed: bool,
}

impl Board {
    /// We try to mark or unmark a tile, returning the entity and if the tile is marked
    pub fn try_toggle_mark(&mut self, coords: &Coordinates) -> Option<(Entity, bool)> {
        let entity = *self.covered_tiles.get(coords)?;
        let mark = if self.marked_tiles.contains(coords) {
            self.unmark_tile(coords)?;
            false
        } else {
            self.marked_tiles.push(*coords);
            true
        };
        Some((entity, mark))
    }
    /// Removes the `coords` from `marked_tiles`
    fn unmark_tile(&mut self, coords: &Coordinates) -> Option<Coordinates> {
        let pos = match self.marked_tiles.iter().position(|a| a == coords) {
            None => {
                log::error!("Failed to unmark tile at {}", coords);
                return None;
            }
            Some(p) => p,
        };
        Some(self.marked_tiles.remove(pos))
    }

    /// Is the board complete
    pub fn is_completed(&self) -> bool {
        println!("bomb: {}, tiles: {}", &self.tile_map.bomb_count(), &self.covered_tiles.len());
        self.tile_map.bomb_count() as usize == self.covered_tiles.len()
    }
    /// Translates a mouse position to board coordinates
    pub fn mouse_position(&self, window: &Window, position: Vec2) -> Option<Coordinates> {
        // Window to world space
        let window_size = Vec2::new(window.width(), window.height());
        let position = position - window_size / 2.;

        // Bounds check
        if !self.bounds.in_bounds(position) {
            return None;
        }
        // World space to board space
        let coordinates = position - self.bounds.position;
        Some(Coordinates {
            x: (coordinates.x / self.tile_size) as u16,
            y: (coordinates.y / self.tile_size) as u16,
        })
    }
    /// Retrieves a covered tile entity
    pub fn tile_to_uncover(&self, coords: &Coordinates) -> Option<&Entity> {
        if self.marked_tiles.contains(coords) {
            None
        } else {
            self.covered_tiles.get(coords)
        }
      }
  
      /// We try to uncover a tile, returning the entity
      pub fn try_uncover_tile(&mut self, coords: &Coordinates) -> Option<Entity> {
          if self.marked_tiles.contains(coords) {
            self.unmark_tile(coords);
          }
          self.covered_tiles.remove(coords)
      }
  
      /// We retrieve the adjacent covered tile entities of `coord`
      pub fn adjacent_covered_tiles(&self, coord: Coordinates) -> Vec<Entity> {
          self.tile_map
              .safe_square_at(coord)
              .filter_map(|c| self.covered_tiles.get(&c))
              .copied()
              .collect()
      }
      pub fn get_bound (&self) -> &Vec2 {
        &self.bounds.position
      }
      pub fn get_size (&self) -> Vec2 {
        self.bounds.size
      }
      pub fn get_middle_pos (&self) -> Vec2 {
        let position = self.get_bound();
        let size = self.get_size();
        Vec2::new(position.x + size.x, position.y + size.y)
      }
      pub fn set_need_stop_state (&mut self, state: bool) {
        self.need_stop_listening_pressed = state;
      }
}
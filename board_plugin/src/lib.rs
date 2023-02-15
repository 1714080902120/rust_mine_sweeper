// lib.rs
mod bounds;
pub mod components;
mod events;
pub mod resources;
mod systems;
use bevy::ecs::schedule::StateData;
use bevy::log;
use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::events::*;
use bevy::math::Vec3Swizzles;
use board::Board;
use bounds::Bounds2;
use components::*;
use resources::*;
use tile::*;
use tile_map::*;

use resources::BoardAssets;

pub struct BoardPlugin<T> {
    pub running_state: T,
}

impl<T: StateData> Plugin for BoardPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(self.running_state.clone()).with_system(Self::create_board),
        )
        // We handle input and trigger events only if the state is active
        .add_system_set(
            SystemSet::on_update(self.running_state.clone())
                .with_system(systems::input::input_handling)
                .with_system(systems::uncover::trigger_event_handler),
        )
        // We handle uncovering even if the state is inactive
        .add_system_set(
            SystemSet::on_in_stack_update(self.running_state.clone())
                .with_system(systems::uncover::uncover_tiles)
                .with_system(systems::mark::mark_tiles)
                .with_system(systems::fail::fail)
                .with_system(systems::completed::completed),
        )
        .add_system_set(
            SystemSet::on_exit(self.running_state.clone()).with_system(Self::cleanup_board),
        )
        .add_event::<TileTriggerEvent>()
        .add_event::<TileMarkEvent>()
        .add_event::<BombExplosionEvent>()
        .add_event::<BoardCompletedEvent>();
        #[cfg(feature = "debug")]
        {
            app.register_type::<Coordinates>()
                .register_type::<Bomb>()
                .register_type::<BombNeighbor>()
                .register_type::<Uncover>();
        }
        log::info!("Loaded Board Plugin");
    }
}

impl<T> BoardPlugin<T> {
    /// Computes a tile size that matches the window according to the tile map size
    fn adaptative_tile_size(
        window: Res<Windows>,
        (min, max): (f32, f32),      // Tile size constraints
        (width, height): (u16, u16), // Tile map dimensions
    ) -> f32 {
        let window = window.get_primary().expect("get window primary error");

        let max_width = window.width() / width as f32;
        let max_heigth = window.height() / height as f32;
        max_width.min(max_heigth).clamp(min, max)
    }
    fn cleanup_board(board: Res<Board>, mut commands: Commands) {
        commands.entity(board.entity).despawn_recursive();
        commands.remove_resource::<Board>();
    }
    /// System to generate the complete board
    pub fn create_board(
        mut commands: Commands,
        board_options: Option<Res<BoardOptions>>,
        window: Res<Windows>,
        board_assets: Res<BoardAssets>,
    ) {
        let options = match board_options {
            None => BoardOptions::default(), // If no options is set we use the default one
            Some(o) => o.clone(),
        };
        // Tilemap generation
        let mut tile_map = TileMap::empty(options.map_size.0, options.map_size.1);
        tile_map.set_bombs(options.bomb_count);
        #[cfg(feature = "debug")]
        // Tilemap debugging
        log::info!("{}", tile_map.console_output());

        let tile_size = match options.tile_size {
            TileSize::Fixed(v) => v,
            TileSize::Adaptive { min, max } => Self::adaptative_tile_size(
                window,
                (min, max),
                (tile_map.width(), tile_map.height()),
            ),
        };

        // We deduce the size of the complete board
        let board_size = Vec2::new(
            tile_map.width() as f32 * tile_size,
            tile_map.height() as f32 * tile_size,
        );
        log::info!("board size: {}", board_size);
        // We define the board anchor position (bottom left)
        let board_position = match options.position {
            BoardPosition::Centered { offset } => {
                Vec3::new(-(board_size.x / 2.), -(board_size.y / 2.), 0.) + offset
            }
            BoardPosition::Custom(p) => p,
        };

        let mut covered_tiles =
            HashMap::with_capacity((tile_map.width() * tile_map.height()).into());

        let mut safe_start = None;

        let board_entry = commands
            .spawn(SpatialBundle {
                visibility: Visibility::VISIBLE,
                transform: Transform::from_translation(board_position.into()),
                ..Default::default()
            })
            .insert(Name::new("Board"))
            .with_children(|parent| {
                // We spawn the board background sprite at the center of the board, since the sprite pivot is centered
                parent
                    .spawn(SpriteBundle {
                        sprite: Sprite {
                            color: board_assets.board_material.color,
                            custom_size: Some(board_size),
                            ..Default::default()
                        },
                        texture: board_assets.board_material.texture.clone(),
                        transform: Transform::from_xyz(board_size.x / 2., board_size.y / 2., 0.),
                        ..Default::default()
                    })
                    .insert(Name::new("Background"));

                Self::spawn_tiles(
                    parent,
                    &tile_map,
                    tile_size,
                    options.tile_padding,
                    &board_assets,
                    &mut covered_tiles,
                    &mut safe_start,
                );
            })
            .id();

        if options.safe_start {
            if let Some(entity) = safe_start {
                commands.entity(entity).insert(Uncover);
            }
        }

        commands.insert_resource(Board {
            tile_map,
            bounds: Bounds2 {
                position: board_position.xy(),
                size: board_size,
            },
            tile_size,
            covered_tiles,
            entity: board_entry,
            marked_tiles: Vec::new(),
            need_stop_listening_pressed: false,
        });
    }
    /// Generates the bomb counter text 2D Bundle for a given value
    fn bomb_count_text_bundle(count: u8, board_assets: &BoardAssets, size: f32) -> Text2dBundle {
        // We retrieve the text and the correct color
        let color = board_assets.bomb_counter_color(count);
        // We generate a text bundle
        Text2dBundle {
            text: Text {
                sections: vec![TextSection {
                    value: count.to_string(),
                    style: TextStyle {
                        color,
                        font: board_assets.bomb_counter_font.clone(),
                        font_size: size,
                    },
                }],
                alignment: TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                },
            },
            transform: Transform::from_xyz(0., 0., 1.),
            ..Default::default()
        }
    }
    fn spawn_tiles(
        parent: &mut ChildBuilder,
        tile_map: &TileMap,
        size: f32,
        padding: f32,
        board_assets: &BoardAssets,
        covered_tiles: &mut HashMap<Coordinates, Entity>,
        safe_start_enty: &mut Option<Entity>,
    ) {
        // Tiles
        for (y, line) in tile_map.iter().enumerate() {
            for (x, tile) in line.iter().enumerate() {
                let coordinates = Coordinates {
                    x: x as u16,
                    y: y as u16,
                };

                let tile_bundle = SpriteBundle {
                    sprite: Sprite {
                        color: board_assets.tile_material.color,
                        custom_size: Some(Vec2::splat(size - padding)),

                        ..Default::default()
                    },
                    transform: Transform::from_xyz(
                        (x as f32 * size) + (size / 2.),
                        (y as f32 * size) + (size / 2.),
                        1.,
                    ),
                    texture: board_assets.tile_material.texture.clone(),
                    ..Default::default()
                };

                let mut cmd = parent.spawn(tile_bundle);

                cmd.insert(Name::new(format!("Tile ({}, {})", x, y)))
                    .insert(coordinates);

                cmd.with_children(|parent| {
                    let entity = parent
                        .spawn(SpriteBundle {
                            sprite: Sprite {
                                custom_size: Some(Vec2::splat(size - padding)),
                                color: board_assets.covered_tile_material.color,
                                ..Default::default()
                            },
                            transform: Transform::from_xyz(0., 0., 2.),
                            texture: board_assets.covered_tile_material.texture.clone(),
                            ..Default::default()
                        })
                        .insert(Name::new("Tile Cover"))
                        .id();
                    covered_tiles.insert(coordinates, entity);

                    if safe_start_enty.is_none() && *tile == Tile::Empty {
                        *safe_start_enty = Some(entity);
                    }
                });

                match tile {
                    Tile::Bomb => {
                        cmd.insert(Bomb).with_children(|parent| {
                            parent.spawn(SpriteBundle {
                                sprite: Sprite {
                                    custom_size: Some(Vec2::splat(size - padding)),
                                    ..Default::default()
                                },
                                transform: Transform::from_xyz(0., 0., 1.),
                                texture: board_assets.bomb_material.texture.clone(),
                                ..Default::default()
                            });
                        });
                    }
                    Tile::BombNeighbor(count) => {
                        let bomb_neighbor = BombNeighbor { count: *count };
                        cmd.insert(bomb_neighbor).with_children(|parent| {
                            parent.spawn(Self::bomb_count_text_bundle(
                                *count,
                                board_assets,
                                size - padding,
                            ));
                        });
                    }
                    _ => (),
                };
            }
        }
    }
}

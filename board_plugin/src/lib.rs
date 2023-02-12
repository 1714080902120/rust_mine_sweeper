// lib.rs
mod bounds;
mod events;
pub mod components;
pub mod resources;
mod systems;
use bevy::log;
use bevy::prelude::*;
use bevy::utils::HashMap;

use bevy::math::Vec3Swizzles;
use board::Board;
use bounds::Bounds2;
use components::*;
use resources::*;
use tile::*;
use tile_map::*;
use crate::events::TileTriggerEvent;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::create_board)
            .add_system(systems::input::input_handling)
            .add_system(systems::uncover::trigger_event_handler)
            .add_system(systems::uncover::uncover_tiles)
            .add_event::<TileTriggerEvent>();
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

impl BoardPlugin {
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

    /// System to generate the complete board
    pub fn create_board(
        mut commands: Commands,
        board_options: Option<Res<BoardOptions>>,
        window: Res<Windows>,
        asset_server: Res<AssetServer>, // The AssetServer resource
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

        commands
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
                            color: Color::WHITE,
                            custom_size: Some(board_size),
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(board_size.x / 2., board_size.y / 2., 0.),
                        ..Default::default()
                    })
                    .insert(Name::new("Background"));

                let font = asset_server.load("fonts/pixeled.ttf");
                let bomb_image = asset_server.load("sprites/bomb.png");

                Self::spawn_tiles(
                    parent,
                    &tile_map,
                    tile_size,
                    options.tile_padding,
                    Color::GRAY,
                    bomb_image,
                    font,
                    Color::DARK_GRAY,
                    &mut covered_tiles,
                );
            });

        commands.insert_resource(Board {
            tile_map,
            bounds: Bounds2 {
                position: board_position.xy(),
                size: board_size,
            },
            tile_size,
            covered_tiles,
        });
    }
    /// Generates the bomb counter text 2D Bundle for a given value
    fn bomb_count_text_bundle(count: u8, font: Handle<Font>, size: f32) -> Text2dBundle {
        // We retrieve the text and the correct color
        let (text, color) = (
            count.to_string(),
            match count {
                1 => Color::WHITE,
                2 => Color::GREEN,
                3 => Color::YELLOW,
                4 => Color::ORANGE,
                _ => Color::PURPLE,
            },
        );
        // We generate a text bundle
        Text2dBundle {
            text: Text {
                sections: vec![TextSection {
                    value: text,
                    style: TextStyle {
                        color,
                        font,
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
        color: Color,
        bomb_image: Handle<Image>,
        font: Handle<Font>,
        covered_tile_color: Color,
        covered_tiles: &mut HashMap<Coordinates, Entity>,
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
                        color,
                        custom_size: Some(Vec2::splat(size - padding)),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(
                        (x as f32 * size) + (size / 2.),
                        (y as f32 * size) + (size / 2.),
                        1.,
                    ),
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
                                color: covered_tile_color,
                                ..Default::default()
                            },
                            transform: Transform::from_xyz(0., 0., 2.),
                            ..Default::default()
                        })
                        .insert(Name::new("Tile Cover"))
                        .id();
                    covered_tiles.insert(coordinates, entity);
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
                                texture: bomb_image.clone(),
                                ..Default::default()
                            });
                        });
                    }
                    Tile::BombNeighbor(count) => {
                        let bomb_neighbor = BombNeighbor { count: *count };
                        cmd.insert(bomb_neighbor).with_children(|parent| {
                            parent.spawn(Self::bomb_count_text_bundle(
                                *count,
                                font.clone(),
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

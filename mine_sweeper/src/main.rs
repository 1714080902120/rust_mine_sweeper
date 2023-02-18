use bevy::prelude::*;
use bevy::window::{WindowDescriptor, WindowPlugin};
use board_plugin::resources::{BoardAssets, SpriteMaterial};

#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use board_plugin::{resources::BoardOptions, BoardPlugin};

use bevy::log;

fn main() {
    let mut app = App::new();
    // Window setup
    app
        // Bevy default plugins
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Mine Sweeper!".to_string(),
                width: 700.,
                height: 800.,
                ..Default::default()
            },
            ..default()
        }));
    app.add_startup_system(setup_board);

    app.add_state(AppState::Out)
        .add_plugin(BoardPlugin {
            running_state: AppState::InGame,
        })
        .add_system(state_handler);

    #[cfg(feature = "debug")]
    // Debug hierarchy inspector
    app.add_plugin(WorldInspectorPlugin);

    // Startup system (cameras)
    app.add_startup_system(camera_setup);
    // Run the app
    app.run();
}

fn camera_setup(mut commands: Commands) {
    // 2D orthographic camera
    commands.spawn(Camera2dBundle::default());
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    InGame,
    Out,
}

fn state_handler(mut state: ResMut<State<AppState>>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::C) {
        log::debug!("clearing detected");
        if state.current() == &AppState::InGame {
            log::info!("clearing game");
            state.set(AppState::Out).unwrap();
        }
    }
    if keys.just_pressed(KeyCode::G) {
        log::debug!("loading detected");
        if state.current() == &AppState::Out {
            log::info!("loading game");
            state.set(AppState::InGame).unwrap();
        }
    }
}

fn setup_board(
    mut commands: Commands,
    mut state: ResMut<State<AppState>>,
    asset_server: Res<AssetServer>,
) {
    // Board plugin options
    commands.insert_resource(BoardOptions {
        map_size: (20, 20),
        bomb_count: 40,
        tile_padding: 1.,
        safe_start: false,
        ..Default::default()
    });
    // Board assets
    commands.insert_resource(BoardAssets {
        label: "Default".to_string(),
        board_material: SpriteMaterial {
            color: Color::WHITE,
            ..Default::default()
        },
        tile_material: SpriteMaterial {
            color: Color::DARK_GRAY,
            ..Default::default()
        },
        covered_tile_material: SpriteMaterial {
            color: Color::GRAY,
            ..Default::default()
        },
        bomb_counter_font: asset_server.load("/fonts/pixeled.ttf"),
        bomb_counter_colors: BoardAssets::default_colors(),
        flag_material: SpriteMaterial {
            texture: asset_server.load("/sprites/flag.png"),
            color: Color::WHITE,
        },
        bomb_material: SpriteMaterial {
            texture: asset_server.load("/sprites/bomb.png"),
            color: Color::WHITE,
        },
        fail_material: SpriteMaterial {
            color: Color::rgb(100., 255., 100.),
            ..Default::default()
        },
        fail_cover_board_material: SpriteMaterial {
            color: Color::Rgba {
                red: 10.,
                green: 10.,
                blue: 50.,
                alpha: 0.2,
            },
            ..Default::default()
        },
        game_over_font_size: 72.
    });
    // Plugin activation

    state.overwrite_replace(AppState::InGame).expect("666");
}

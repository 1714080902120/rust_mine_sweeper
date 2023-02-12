use bevy::prelude::*;
use bevy::window::{ WindowDescriptor, WindowPlugin };


#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use board_plugin::{ BoardPlugin, resources::BoardOptions };

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

    app.insert_resource(BoardOptions {
        map_size: (20, 20),
        bomb_count: 40,
        tile_padding: 3.0,
        ..Default::default()
    });

    app.add_plugin(BoardPlugin);

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
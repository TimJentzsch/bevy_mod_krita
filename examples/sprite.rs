//! Displays a single [`Sprite`], created from a Krita document.

use bevy::prelude::*;
use bevy_mod_krita::KritaPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            // Enable hot reloading
            watch_for_changes: true,
            ..default()
        }))
        // Add the Krita plugin to enable loading of `.kra` files
        .add_plugin(KritaPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: asset_server.load("krita/demo.kra"),
        ..default()
    });
}

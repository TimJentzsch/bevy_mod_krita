//! Krita documents have relatively large file sizes, making them unsuitable to distribute with the game release.
//! Asset pre-processing, introduced in Bevy 0.12, can help here.
//! The Krita document can be pre-processed into, e.g., basis universal to reduce the file size.
//! At development, you can still enjoy the benefits of hot reloading and fast prototyping.

use bevy::prelude::*;
use bevy_mod_krita::KritaPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            // Enable asset pre-processing
            mode: AssetMode::Processed,
            ..default()
        }))
        // Add the Krita plugin to enable loading of `.kra` files
        .add_plugins(KritaPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        // Load a Krita document as a texture
        texture: asset_server.load("krita/pre_processing.kra"),
        ..default()
    });
}

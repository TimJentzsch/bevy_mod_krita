//! Displays a single [`Sprite`], created from a Krita document.

use bevy::prelude::*;
use bevy_mod_krita::KritaPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Add the Krita plugin to enable loading of `.kra` files
        .add_plugins(KritaPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        // Load a Krita document as a texture
        texture: asset_server.load("krita/demo.kra"),
        ..default()
    });
}

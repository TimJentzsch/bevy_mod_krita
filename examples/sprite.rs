//! Displays a single [`Sprite`], created from a Krita document.

use std::time::Duration;

use bevy::prelude::*;
use bevy_asset::ChangeWatcher;
use bevy_mod_krita::KritaPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            // Enable hot reloading
            watch_for_changes: ChangeWatcher::with_delay(Duration::from_millis(200)),
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
        texture: asset_server.load("krita/demo.kra"),
        ..default()
    });
}

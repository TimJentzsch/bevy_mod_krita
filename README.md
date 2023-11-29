# bevy_mod_krita [![Crates.io version](https://img.shields.io/crates/v/bevy_mod_krita)](https://crates.io/crates/bevy_mod_krita) [![Crates.io license](https://img.shields.io/crates/l/bevy_mod_krita)](LICENSE-MIT)

Load Krita's `.kra` documents directly in Bevy for rapid prototyping or game jams.

Please keep in mind that `.kra` files are not optimized for size, so you should probably not use them for production bundles.

## Bevy Compatibility

| `bevy` version | `bevy_mod_krita` version |
| -------------- | ------------------------ |
| `0.11`         | `0.2.0`                  |
| `0.10`         | `0.1.0`                  |

## Installation

```cli
cargo add bevy_mod_krita
```

## Usage

Simply add the `KritaPlugin` to your app, enable hot reloading (optional) and load `.kra` files!

```rs
use bevy::prelude::*;
use bevy_mod_krita::KritaPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            // Enable hot reloading (optional)
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
        // Load a `.kra` file
        texture: asset_server.load("krita/demo.kra"),
        ..default()
    });
}
```

## License

This project is licensed under the terms of the [MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE) license at your choice.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

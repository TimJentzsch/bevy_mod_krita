use bevy::prelude::*;
use bevy_mod_krita::KritaPlugin;

#[derive(Component)]
struct Cube;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            // Enable hot reloading
            watch_for_changes: true,
            ..default()
        }))
        .add_plugin(KritaPlugin)
        .add_startup_system(setup)
        .add_system(rotate_cube)
        .run();
}

/// Spawn a textured cube, a light and a camera.
fn setup(
    mut commands: Commands,
    assert_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // cube
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(assert_server.load("krita/demo.kra")),
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Cube,
    ));

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 2000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(3.0, 5.0, 4.0),
        ..default()
    });

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

/// Rotate the cube to show more of the texture.
fn rotate_cube(mut query: Query<&mut Transform, With<Cube>>, time: Res<Time>) {
    let delta = time.delta_seconds();
    let rot_axis = Vec3::new(1. + delta % 1., 1. + delta % 3., 1. + delta % 2.);
    let rotation = Quat::from_axis_angle(rot_axis.normalize(), 1. * delta);

    for mut transform in query.iter_mut() {
        transform.rotate(rotation.clone())
    }
}

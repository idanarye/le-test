use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("icon.png"),
        transform: Transform::from_xyz(0.0, -150.0, 0.0),
        ..Default::default()
    });

    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("icon.png"),
        transform: Transform::from_xyz(0.0, 150.0, 0.0),
        ..Default::default()
    });
}

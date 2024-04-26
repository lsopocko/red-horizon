use bevy::{log, prelude::*};

use bevy_rapier3d::prelude::*;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_assets, setup_colliders));
    }
}

#[derive(Bundle)]
struct TerrainBundle {
    scene: SceneBundle,
}

fn setup_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    log::info!("Loading assets...");

    commands.spawn(TerrainBundle {
        scene: SceneBundle {
            scene: asset_server.load("Map.glb#Scene0"),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
    });
}

fn setup_colliders(mut commands: Commands) {
    // Ground
    commands
        .spawn(Collider::cuboid(10.0, 0.1, 20.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -0.1, -10.0)));

    // // Walls
    commands
        .spawn(Collider::cuboid(0.1, 10.0, 10.0))
        .insert(TransformBundle::from(Transform::from_xyz(-5.0, 10.0, -5.0)));

    // // Right wall
    commands
        .spawn(Collider::cuboid(0.1, 10.0, 10.0))
        .insert(TransformBundle::from(Transform::from_xyz(5.0, 10.0, -5.0)));

    // Back wall
    commands
        .spawn(Collider::cuboid(10.0, 10.0, 0.1))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 10.0, -15.0)));

    // Front wall
    commands
        .spawn(Collider::cuboid(10.0, 10.0, 0.1))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 10.0, 5.0)));

    // Launch pad
    commands
        .spawn(Collider::cylinder(0.2, 1.2))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)));
}

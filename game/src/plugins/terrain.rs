use bevy::{log, prelude::*};

use bevy_rapier3d::prelude::*;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_assets, setup_colliders));
        app.add_systems(Update, display_events);
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
            scene: asset_server.load("Planet3.glb#Scene0"),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
    });
}

fn setup_colliders(mut commands: Commands) {
    // Sphere planet
    commands
        .spawn(Collider::ball(25.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -25.0, 0.0)))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Name::new("Planet"));

    // Launch pad
    commands
        .spawn(Collider::cylinder(0.2, 1.2))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Name::new("Launch Pad"));
}

fn display_events(mut collision_events: EventReader<CollisionEvent>, _query: Query<&Name>) {
    for collision_event in collision_events.read() {
        if let CollisionEvent::Started(entity1, _entity2, _) = collision_event {
            if let Ok(name1) = _query.get(*entity1) {
                log::info!("Collision started with: {:?}", name1);
            }
        }

        if let CollisionEvent::Stopped(entity1, _entity2, _) = collision_event {
            if let Ok(name1) = _query.get(*entity1) {
                log::info!("Collision stopped with: {:?}", name1);
            }
        }
    }
}

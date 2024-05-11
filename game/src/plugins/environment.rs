use bevy::{
    log,
    pbr::{CascadeShadowConfigBuilder, NotShadowCaster, NotShadowReceiver},
    prelude::*,
};

use rand::Rng;
pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup));
    }
}
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let cascade_shadow_config = CascadeShadowConfigBuilder {
        first_cascade_far_bound: 10.3,
        maximum_distance: 30.0,
        ..default()
    }
    .build();

    // // Sun
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::rgb(0.98, 0.95, 0.82),
            illuminance: 3000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(30.0, 40.0, 5.0)
            .looking_at(Vec3::new(-0.15, -0.05, 0.25), Vec3::Y),
        cascade_shadow_config,
        ..default()
    });

    // Sky
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Sphere::new(40.0)),
            material: materials.add(StandardMaterial {
                cull_mode: None,
                unlit: true,
                base_color: Color::rgb(0.0, 0.0, 0.0),
                ..default()
            }),
            transform: Transform::from_scale(Vec3::splat(1.0)),
            ..default()
        },
        NotShadowCaster,
        NotShadowReceiver,
    ));

    // random stars
    for _ in 0..100 {
        let x = rand::thread_rng().gen_range(-20.0..20.0);
        let y = rand::thread_rng().gen_range(0.0..38.0);
        let z = rand::thread_rng().gen_range(-20.0..-10.0);
        let scale = rand::random::<f32>() * 0.05;

        commands.spawn(PbrBundle {
            mesh: meshes.add(Sphere::new(1.0)),
            material: materials.add(StandardMaterial {
                cull_mode: None,
                unlit: true,
                base_color: Color::rgb(1.0, 1.0, 1.0),
                emissive: Color::rgb(1.0, 1.0, 1.0),
                ..default()
            }),
            transform: Transform {
                translation: Vec3::new(x, y, z),
                rotation: Quat::from_rotation_y(rand::random::<f32>() * std::f32::consts::PI),
                scale: Vec3::splat(scale),
            },
            ..default()
        });
    }
}

use bevy::{
    pbr::{CascadeShadowConfigBuilder, NotShadowCaster, NotShadowReceiver},
    prelude::*,
};

use rand::Rng;
pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cascade_shadow_config = CascadeShadowConfigBuilder {
        first_cascade_far_bound: 5.3,
        maximum_distance: 30.0,
        ..default()
    }
    .build();

    // // Sun
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Srgba::rgb(0.98, 0.95, 0.82).into(),
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
            mesh: meshes.add(Sphere::new(30.0)),
            material: materials.add(StandardMaterial {
                cull_mode: None,
                unlit: true,
                ior: 1.0,
                perceptual_roughness: 1000.0,
                base_color: Srgba::rgb(0.0, 0.0, 0.0).into(),
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
        let x = rand::thread_rng().gen_range(-25.0..25.0);
        let y = rand::thread_rng().gen_range(2.0..37.0);
        let z = rand::thread_rng().gen_range(-15.0..-5.0);
        let scale = rand::random::<f32>() * 0.05;

        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Sphere::default().mesh().uv(3, 3)),
                material: materials.add(StandardMaterial {
                    unlit: true,
                    base_color: Srgba::rgb(1.0, 1.0, 1.0).into(),
                    emissive: Srgba::rgb(1.0, 1.0, 1.0).into(),
                    ..default()
                }),
                transform: Transform {
                    translation: Vec3::new(x, y, z),
                    rotation: Quat::from_rotation_y(rand::random::<f32>() * std::f32::consts::PI),
                    scale: Vec3::splat(scale),
                },
                ..default()
            },
            NotShadowCaster,
            NotShadowReceiver,
        ));
    }
}

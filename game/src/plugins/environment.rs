use bevy::{
    log,
    pbr::{CascadeShadowConfigBuilder, NotShadowCaster},
    prelude::*,
};

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
) {
    let cascade_shadow_config = CascadeShadowConfigBuilder {
        first_cascade_far_bound: 10.3,
        maximum_distance: 50.0,
        ..default()
    }
    .build();

    // Sun
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
            mesh: meshes.add(Cuboid::new(200.0, 200.0, 200.0)),
            material: materials.add(StandardMaterial {
                base_color: Color::rgba(0.0, 0.0, 0.0, 1.0),
                unlit: true,
                cull_mode: None,
                ..default()
            }),
            transform: Transform::from_scale(Vec3::splat(1.0)),
            ..default()
        },
        NotShadowCaster,
    ));
}

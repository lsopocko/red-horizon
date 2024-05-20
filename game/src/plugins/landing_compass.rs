use bevy::{pbr::NotShadowCaster, prelude::*};

#[derive(Component)]
pub struct LandingCompass;

pub struct LandingCompassPlugin;

impl Plugin for LandingCompassPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup));
        app.add_systems(Update, track_landing_system);
    }
}
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // span compass as triangle at bottom of screen}
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Torus::new(0.98, 1.0)),
            material: materials.add(StandardMaterial {
                base_color: Color::rgba(0.0, 1.0, 0.0, 1.0),
                unlit: true,
                cull_mode: None,
                ..default()
            }),
            transform: Transform {
                translation: Vec3::new(-0.005, 0.0, 0.8),
                // scale: Vec3::splat(0.8),
                // rotation: Quat::from_rotation_x(std::f32::consts::PI / 2.0),
                ..default()
            },
            ..default()
        },
        NotShadowCaster,
        LandingCompass,
    ));
}

fn track_landing_system(
    mut compass_query: Query<&mut Transform, (With<LandingCompass>, Without<Camera3d>)>,
    mut camera_query: Query<&Transform, (With<Camera3d>, Without<LandingCompass>)>,
) {
    for mut compass_transform in compass_query.iter_mut() {
        for camera_transform in camera_query.iter_mut() {
            compass_transform.translation.y = camera_transform.translation.y - 2.5;
        }
    }
}

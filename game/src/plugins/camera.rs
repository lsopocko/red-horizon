use bevy::prelude::*;

use super::{
    rocket::{Rocket, START_ALTITUDE},
    splash::GameState,
};

pub struct CameraPlugin;

const CAMERA_OFFSET: Vec3 = Vec3::new(0.0, 0.5, 12.0);

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup));
        app.add_systems(
            Update,
            (follow_rocket_system).run_if(in_state(GameState::Playing)),
        );
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(((
        Camera3dBundle {
            projection: PerspectiveProjection {
                // We must specify the FOV in radians.
                // Rust can convert degrees to radians for us.
                fov: 55.0_f32.to_radians(),
                far: 50.0,
                ..default()
            }
            .into(),
            transform: Transform::from_xyz(
                CAMERA_OFFSET.x,
                START_ALTITUDE + CAMERA_OFFSET.y,
                CAMERA_OFFSET.z,
            )
            .looking_at(Vec3::new(0.0, 1.5 + START_ALTITUDE, -3.0), Vec3::Y),
            ..default()
        },
        FogSettings {
            color: Color::rgba(0.89, 0.51, 0.27, 1.0),
            directional_light_color: Color::rgba(1.0, 0.85, 0.85, 0.5),
            directional_light_exponent: 30.0,
            falloff: FogFalloff::ExponentialSquared { density: 0.005 },
        },
    ),));
}

fn follow_rocket_system(
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    mut rocket_query: Query<&Transform, (With<Rocket>, Without<Camera3d>)>,
) {
    for mut camera_transform in camera_query.iter_mut() {
        for rocket_transform in rocket_query.iter_mut() {
            camera_transform.translation = rocket_transform.translation + CAMERA_OFFSET;
        }
    }
}

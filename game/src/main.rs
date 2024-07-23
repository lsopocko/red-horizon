// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::math::DMat3;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use plugins::splash::GameState;

use crate::plugins::camera::CameraPlugin;
use crate::plugins::environment::EnvironmentPlugin;
use crate::plugins::landing_compass::LandingCompassPlugin;
use crate::plugins::rocket::RocketPlugin;
use crate::plugins::splash::SplashPlugin;
use crate::plugins::telemetry::TelemetryPlugin;
use crate::plugins::terrain::TerrainPlugin;
use crate::plugins::weather::MarsWeather;
use crate::plugins::weather::WeatherPlugin;

mod plugins;

#[tokio::main(flavor = "multi_thread", worker_threads = 3)]
async fn main() {
    let _res = MarsWeather::get().await;

    App::new()
        .register_type::<DMat3>()
        // External plugins
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Red Horizon".to_string(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            RapierPhysicsPlugin::<NoUserData>::default(),
            // RapierDebugRenderPlugin::default(),
        ))
        .add_systems(Update, rapier_context_system)
        // Internal plugins
        .add_plugins(SplashPlugin)
        .add_plugins(EnvironmentPlugin)
        .add_plugins(TerrainPlugin)
        .add_plugins(RocketPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(LandingCompassPlugin)
        .add_plugins(TelemetryPlugin)
        .add_plugins(WeatherPlugin {
            weather: _res.unwrap(),
        })
        .insert_resource(RapierConfiguration {
            gravity: Vec3::new(0.0, -3.71, 0.0),
            ..RapierConfiguration::new(1.0)
        })
        .run();
}

fn rapier_context_system(
    mut rapier_config: ResMut<RapierConfiguration>,
    mut game_state: ResMut<State<GameState>>,
) {
    match game_state.get() {
        GameState::Playing => {
            rapier_config.physics_pipeline_active = true;
        }
        GameState::Paused => {
            rapier_config.physics_pipeline_active = false;
        }
    }
}

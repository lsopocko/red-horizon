use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::plugins::camera::CameraPlugin;
use crate::plugins::environment::EnvironmentPlugin;
use crate::plugins::rocket::RocketPlugin;
use crate::plugins::telemetry::TelemetryPlugin;
use crate::plugins::terrain::TerrainPlugin;
use crate::plugins::weather::MarsWeather;
use crate::plugins::weather::WeatherPlugin;

mod plugins;

#[tokio::main(flavor = "multi_thread", worker_threads = 3)]
async fn main() {
    let _res = MarsWeather::get().await;

    App::new()
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
        // Internal plugins
        .add_plugins(EnvironmentPlugin)
        .add_plugins(TerrainPlugin)
        .add_plugins(RocketPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(TelemetryPlugin)
        .add_plugins(WeatherPlugin {
            weather: _res.unwrap(),
        })
        .insert_resource(RapierConfiguration {
            gravity: Vec3::new(0.0, -3.71, 0.0),
            ..Default::default()
        })
        .run();
}

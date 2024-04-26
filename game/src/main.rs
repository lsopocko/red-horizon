use bevy::prelude::*;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;

use crate::plugins::camera::CameraPlugin;
use crate::plugins::environment::EnvironmentPlugin;
use crate::plugins::rocket::RocketPlugin;
use crate::plugins::telemetry::TelemetryPlugin;
use crate::plugins::terrain::TerrainPlugin;

mod plugins;

fn main() {
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
            WorldInspectorPlugin::new(),
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
        ))
        // Internal plugins
        .add_plugins(EnvironmentPlugin)
        .add_plugins(TerrainPlugin)
        .add_plugins(RocketPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(TelemetryPlugin)
        .run();
}

use bevy::{log, prelude::*};

use super::rocket::*;

pub struct TelemetryPlugin;

impl Plugin for TelemetryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (print_telemetry_system));
    }
}

fn print_telemetry_system(
    fuel_query: Query<&Fuel>,
    altitute_query: Query<&Altitute>,
    velocity_query: Query<&Velocity>,
    thrust_query: Query<&Thrust>,
    left_ecs_query: Query<&LeftEcs>,
    right_ecs_query: Query<&RightEcs>,
) {
    for fuel in fuel_query.iter() {
        log::info!("Fuel: {}", fuel.value);
    }

    for altitute in altitute_query.iter() {
        log::info!("Altitude: {}", altitute.value);
    }

    for thrust in thrust_query.iter() {
        log::info!("Thrust: {}", thrust.value);
    }

    for left_ecs in left_ecs_query.iter() {
        log::info!("Left ECS: {}", left_ecs.value);
    }

    for right_ecs in right_ecs_query.iter() {
        log::info!("Right ECS: {}", right_ecs.value);
    }

    for velocity in velocity_query.iter() {
        log::info!("Velocity: {:?}", velocity);
    }
}

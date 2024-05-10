use bevy::{log, prelude::*};
use serde::Serialize;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::sync::mpsc;

use super::{
    rocket::*,
    weather::{WindDirection, WindSpeed},
};
#[derive(Debug, Clone, Serialize)]
pub struct TelemetryData {
    pub fuel: f32,
    pub altitude: f32,
    pub velocity: Vec3,
    pub thrust: f32,
    pub left_ecs: f32,
    pub right_ecs: f32,
    pub wind_speed: f32,
    pub wind_direction: Vec3,
}

#[derive(Resource)]
pub struct TelemetryChannel {
    pub tx: mpsc::Sender<TelemetryData>,
}

impl TelemetryChannel {
    pub fn new() -> Self {
        let (tx, mut rx) = mpsc::channel::<TelemetryData>(32);
        tokio::spawn(async move {
            let listener = TcpListener::bind("127.0.0.1:8088").await.unwrap();
            let mut last_recieved_data = TelemetryData {
                fuel: 0.0,
                altitude: 0.0,
                velocity: Vec3::ZERO,
                thrust: 0.0,
                left_ecs: 0.0,
                right_ecs: 0.0,
                wind_speed: 0.0,
                wind_direction: Vec3::ZERO,
            };

            match listener.accept().await {
                Ok((mut _socket, addr)) => {
                    while let Some(telemetry) = rx.recv().await {
                        last_recieved_data = telemetry;

                        if _socket
                            .write_all(&bincode::serialize(&last_recieved_data).unwrap_or_default())
                            .await
                            .is_err()
                        {
                            break;
                        }
                    }
                }
                Err(e) => {
                    log::error!("Failed to accept connection: {:?}", e);
                }
            }
        });
        Self { tx }
    }

    pub fn send_telemetry_data(&self, data: TelemetryData) {
        let mut tx = self.tx.clone();
        match tx.try_send(data) {
            Ok(_) => {
                log::info!("Telemetry data sent");
            }
            Err(e) => {
                log::error!("Failed to send telemetry data: {:?}", e);
            }
        }
    }
}

pub struct TelemetryPlugin;

impl Plugin for TelemetryPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TelemetryChannel::new())
            .add_systems(Update, (broadcast_telemetry_system));
    }
}

fn broadcast_telemetry_system(
    rocket_telemetry_query: Query<(&Fuel, &Thrust, &LeftEcs, &RightEcs)>,
    telemetry_channel: Res<TelemetryChannel>,
) {
    let mut telemetry_data = TelemetryData {
        fuel: 0.0,
        altitude: 0.0,
        velocity: Vec3::ZERO,
        thrust: 0.0,
        left_ecs: 0.0,
        right_ecs: 0.0,
        wind_speed: 0.0,
        wind_direction: Vec3::ZERO,
    };

    for (fuel, thrust, left_ecs, right_ecs) in rocket_telemetry_query.iter() {
        telemetry_data.fuel = fuel.value;
        telemetry_data.thrust = thrust.value;
        telemetry_data.left_ecs = left_ecs.value;
        telemetry_data.right_ecs = right_ecs.value;
    }

    telemetry_channel.send_telemetry_data(telemetry_data);
}

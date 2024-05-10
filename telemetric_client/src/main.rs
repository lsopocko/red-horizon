use std::error::Error;
use std::mem;
use bincode;
use serde::Deserialize;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

use bevy::prelude::*;

#[derive(Debug, Clone, Deserialize)]
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:8088").await?;
    let mut buffer = [0; mem::size_of::<TelemetryData>()];

    loop {
        let n = stream.read(&mut buffer).await?;
        if n == 0 {
            break;
        }

        let telemetry: TelemetryData = bincode::deserialize(&buffer).unwrap();
        // clear screen
        print!("\x1B[2J\x1B[1;1H");
        // print telemetry data line by line
        println!("Fuel: {}", telemetry.fuel);
        println!("Altitude: {}", telemetry.altitude);
        println!("Velocity: {:?}", telemetry.velocity);
        println!("Thrust: {}", telemetry.thrust);
        println!("Left ECS: {}", telemetry.left_ecs);
        println!("Right ECS: {}", telemetry.right_ecs);
        println!("Wind Speed: {}", telemetry.wind_speed);
        println!("Wind Direction: {:?}", telemetry.wind_direction);
    }

    Ok(())
}

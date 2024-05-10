use bevy::{ecs::query, log, prelude::*};
use rand::distributions::weighted;
use serde_derive::{Deserialize, Serialize};

#[derive(Component, Default)]
pub struct WindDirection {
    pub value: Vec3,
}

#[derive(Component, Default)]
pub struct WindSpeed {
    pub value: f32,
}
#[derive(Bundle)]
struct WeatherBundle {
    wind_direction: WindDirection,
    wind_speed: WindSpeed,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Current {
    temp_c: f32,
    wind_kph: f32,
    wind_degree: f32,
}

#[derive(Resource)]
struct CurrentWeather {
    temp_c: f32,
    wind_kph: f32,
    wind_degree: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MarsWeather {
    current: Current,
}

impl MarsWeather {
    pub async fn get() -> Result<MarsWeather, reqwest::Error> {
        let key = "ff39f338f00742eab5924359240905";
        let query = format!("q={}", "48.8567,2.3508");
        let url = format!(
            "https://api.weatherapi.com/v1/current.json?key={}&{}",
            key, query
        );

        let response = reqwest::get(url).await;

        match response {
            Ok(response) => {
                let json = response.json::<MarsWeather>().await;
                match json {
                    Ok(json) => {
                        println!("Mars weather data: {:?}", json);
                        Ok(json)
                    }
                    Err(e) => {
                        println!("Error parsing Mars weather data: {:?}", e);
                        Err(e)
                    }
                }
            }
            Err(e) => {
                println!("Error fetching Mars weather data: {:?}", e);
                Err(e)
            }
        }
    }
}

pub struct WeatherPlugin {
    pub weather: MarsWeather,
}

impl Plugin for WeatherPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentWeather {
            temp_c: self.weather.current.temp_c,
            wind_kph: self.weather.current.wind_kph,
            wind_degree: self.weather.current.wind_degree,
        })
        .add_systems(Startup, (setup));
    }
}

fn convert_degrees_to_vec3(degrees: f32) -> Vec3 {
    let radians = degrees.to_radians();
    let x = radians.cos();
    let y = radians.sin();
    Vec3::new(x, y, 0.0)
}

fn setup(mut commands: Commands, mut current_weather: ResMut<CurrentWeather>) {
    let wind_direction = convert_degrees_to_vec3(current_weather.wind_degree);
    let wind_speed = current_weather.wind_kph / 100.0;
    log::info!("Wind direction: {:?}", wind_direction);
    log::info!("Wind speed: {:?}", wind_speed);
    commands.spawn(WeatherBundle {
        wind_direction: WindDirection {
            value: wind_direction,
        },
        wind_speed: WindSpeed { value: wind_speed },
    });
}

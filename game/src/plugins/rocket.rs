use bevy::{audio::PlaybackMode, prelude::*};

use bevy_rapier3d::prelude::*;
use rand::Rng;

use super::{
    splash::GameState,
    weather::{WindDirection, WindSpeed},
};

pub struct RocketPlugin;

#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3,
}

#[derive(Component)]
pub struct Thrust {
    pub value: f32,
}

#[derive(Component)]
pub struct LeftEcs {
    pub value: f32,
}

#[derive(Component)]
pub struct RightEcs {
    pub value: f32,
}

#[derive(Component)]
pub struct Fuel {
    pub value: f32,
}

#[derive(Component)]
pub struct Altitute {
    pub value: f32,
}

#[derive(Component)]
pub struct Rocket;

#[derive(Component)]
struct RocketSoundEffect;

#[derive(Component)]
pub struct RocketCollider;

#[derive(Component)]
struct Particle {
    position: Vec3,
    velocity: Vec3,
    lifetime: f32,
    scale: f32,
    rotation: Quat,
}

const MAX_THRUST: f32 = 6.5;
const MAX_ECS: f32 = 3.0;
pub const START_ALTITUDE: f32 = 26.75; // 36.0;

impl Plugin for RocketPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_assets, setup_collider_body));
        app.add_systems(
            Update,
            (
                rocket_physics_system,
                keyboard_control_system,
                applied_physics_forces_system,
                rocket_fuel_system,
                update_particle_system,
                particle_emitter_system,
                engine_sound_system,
            )
                .run_if(in_state(GameState::Playing)),
        );
    }
}

fn setup_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(SceneBundle {
            transform: Transform {
                translation: Vec3::new(0.0, START_ALTITUDE, 0.0),
                rotation: Quat::IDENTITY,
                scale: Vec3::splat(0.20),
            },
            scene: asset_server.load("Rocket.glb#Scene0"),
            ..default()
        })
        .insert(Thrust { value: 0.0 })
        .insert(Fuel { value: 1000.0 })
        .insert(Velocity {
            value: Vec3::new(0.0, 0.0, 0.0),
        })
        .insert(LeftEcs { value: 0.0 })
        .insert(RightEcs { value: 0.0 })
        .insert(Altitute { value: 0.0 })
        .insert(Rocket);

    commands.spawn((
        AudioBundle {
            source: asset_server.load("rocket-engine.ogg"),
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                paused: true,
                ..default()
            },
            ..default()
        },
        RocketSoundEffect,
    ));
}

fn setup_collider_body(mut commands: Commands) {
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::cuboid(0.20, 0.20, 0.20))
        .insert(ExternalForce {
            force: Vec3::new(0.0, 0.0, 0.0),
            torque: Vec3::new(0.0, 0.0, 0.0),
        })
        .insert(ColliderMassProperties::Density(20.0))
        .insert(Damping {
            linear_damping: 1.5,
            angular_damping: 1.0,
        })
        .insert(TransformBundle::from(Transform::from_xyz(
            0.0,
            START_ALTITUDE,
            0.0,
        )))
        .insert(RocketCollider);
}

fn engine_sound_system(
    music_controller: Query<&AudioSink, With<RocketSoundEffect>>,
    mut _thrust: Query<&mut Thrust, With<Rocket>>,
) {
    let thrust = _thrust.single_mut().value;
    if let Ok(sink) = music_controller.get_single() {
        sink.set_volume(thrust / MAX_THRUST);
        // sink.set_speed(1.0 + thrust / MAX_THRUST);
        if thrust == 0.0 {
            sink.pause();
        } else {
            sink.play();
        }
    }
}

fn particle_emitter_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut _rocket_transform: Query<&Transform, With<Rocket>>,
    mut _thrust: Query<&mut Thrust, With<Rocket>>,
) {
    let player_translation = _rocket_transform.single_mut().translation;
    let thrust = _thrust.single_mut().value;
    let is_thrusting = thrust > 0.0;
    let thrust_percentage = thrust / MAX_THRUST;
    let num_particles = (8.0 * thrust_percentage) as i32;

    if !is_thrusting {
        return;
    }

    for _ in 0..num_particles {
        let position = player_translation;
        let velocity = Vec3::new(
            rand::thread_rng().gen_range(-0.45..0.45),
            if is_thrusting {
                rand::thread_rng().gen_range(2.5..((thrust / 5.0) + 2.5)) * -1.0
            } else {
                0.5
            },
            rand::thread_rng().gen_range(-0.45..0.45),
        );
        let scale = rand::thread_rng().gen_range(0.01..0.1);
        // random rotation
        let rotation =
            Quat::from_rotation_y(rand::thread_rng().gen_range(0.0..std::f32::consts::PI));
        let lifetime = 2.5;
        let color = Color::srgba(1.0, 1.0, 1.0, 0.5);

        commands
            .spawn(PbrBundle {
                mesh: meshes.add(Sphere::default().mesh().uv(3, 3)),
                material: materials.add(StandardMaterial {
                    base_color: color,
                    cull_mode: None,
                    ..default()
                }),
                transform: Transform {
                    translation: position,
                    scale: Vec3::splat(scale),
                    ..default()
                },

                ..default()
            })
            .insert(Particle {
                position,
                velocity,
                lifetime,
                scale,
                rotation,
            });
    }
}

fn update_particle_system(
    mut commands: Commands,
    time: Res<Time>,
    mut particles: Query<(Entity, &mut Particle)>,
    mut transforms: Query<&mut Transform>,
) {
    for (entity, mut particle) in particles.iter_mut() {
        let mut velocity = particle.velocity;

        if particle.position.y < 0.35 {
            velocity.y = 0.0;
        }

        particle.position += velocity * time.delta_seconds();
        particle.lifetime -= time.delta_seconds();
        particle.scale += time.delta_seconds() * 0.1;
        transforms.get_mut(entity).unwrap().translation = particle.position;
        transforms.get_mut(entity).unwrap().scale = Vec3::splat(particle.scale);
        transforms.get_mut(entity).unwrap().rotation = particle.rotation;

        if particle.lifetime <= 0.0 {
            commands.entity(entity).despawn()
        }
    }
}

fn rocket_fuel_system(
    mut fuel: Query<&mut Fuel, With<Rocket>>,
    mut _engines: Query<(&mut Thrust, &mut LeftEcs, &mut RightEcs), With<Rocket>>,
) {
    for mut fuel in fuel.iter_mut() {
        for (thrust, left_ecs, right_ecs) in _engines.iter_mut() {
            let total_ecs = left_ecs.value + right_ecs.value;
            let total_thrust = thrust.value + total_ecs;
            fuel.value = (fuel.value - total_thrust * 0.1).max(0.0);
        }
    }
}

fn rocket_physics_system(
    mut rocket: Query<&mut Transform, With<Rocket>>,
    mut collider: Query<&mut Transform, (With<RocketCollider>, Without<Rocket>)>,
    mut _altitude: Query<&mut Altitute, With<Rocket>>,
    mut _velocity: Query<&mut Velocity, With<Rocket>>,
) {
    for mut transform in rocket.iter_mut() {
        for body in collider.iter_mut() {
            _velocity.single_mut().value =
                ((body.translation - transform.translation) * 1000.0).round() / 10.0;
            transform.translation = body.translation;
            transform.rotation = body.rotation;
            _altitude.single_mut().value = body.translation.y;
        }
    }
}

fn keyboard_control_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut _engines: Query<(&mut Thrust, &mut LeftEcs, &mut RightEcs), With<Rocket>>,
) {
    for (mut thrust, mut left_ecs, mut right_ecs) in _engines.iter_mut() {
        if keyboard_input.pressed(KeyCode::Space) {
            thrust.value = MAX_THRUST.min(thrust.value + 2.0 * time.delta_seconds());
        } else {
            thrust.value = 0.0_f32.max(thrust.value - 3.0 * time.delta_seconds());
        }

        if keyboard_input.pressed(KeyCode::KeyA) {
            left_ecs.value = (left_ecs.value + 0.1 * time.delta_seconds()).min(MAX_ECS);
        } else {
            left_ecs.value = 0.0;
        }

        if keyboard_input.pressed(KeyCode::KeyD) {
            right_ecs.value = (right_ecs.value + 0.1 * time.delta_seconds()).min(MAX_ECS);
        } else {
            right_ecs.value = 0.0;
        }
    }
}

fn applied_physics_forces_system(
    mut ext_forces: Query<&mut ExternalForce, With<RocketCollider>>,
    mut _thrust: Query<&mut Thrust, With<Rocket>>,
    mut _left_ecs: Query<&mut LeftEcs, With<Rocket>>,
    mut _right_ecs: Query<&mut RightEcs, With<Rocket>>,
    mut _rocket_transform: Query<&Transform, With<Rocket>>,
    mut _weather: Query<(&mut WindDirection, &mut WindSpeed)>,
) {
    const LOCAL_UP: Vec3 = Vec3::Y;

    for mut ext_force in ext_forces.iter_mut() {
        let rotation = _rocket_transform.single_mut().rotation;
        let thrust_direction = rotation.mul_vec3(LOCAL_UP);
        let right_ecs = rotation.mul_vec3(-Vec3::Z) * _right_ecs.single_mut().value;
        let left_ecs = rotation.mul_vec3(Vec3::Z) * _left_ecs.single_mut().value;
        //tilt from left and right ecs
        let tilt = right_ecs + left_ecs;
        ext_force.torque = tilt * 0.1;

        for (wind_direction, wind_speed) in _weather.iter_mut() {
            let wind = wind_direction.value * wind_speed.value;
            ext_force.force = thrust_direction * _thrust.single_mut().value + wind;
        }
    }
}

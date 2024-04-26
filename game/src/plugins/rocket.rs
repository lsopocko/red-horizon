use bevy::{prelude::*, ui::update};

use bevy_rapier3d::prelude::*;
use rand::Rng;

pub struct RocketPlugin;

#[derive(Component, Debug)]
pub struct Velocity(Vec3);

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
pub struct RocketCollider;

#[derive(Component)]
struct Particle {
    position: Vec3,
    velocity: Vec3,
    lifetime: f32,
    color: Color,
    scale: f32,
    rotation: Quat,
}

const MAX_THRUST: f32 = 5.4;
const MAX_ECS: f32 = 1.0;

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
            ),
        );
    }
}

fn setup_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(SceneBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                rotation: Quat::IDENTITY,
                scale: Vec3::splat(0.15),
            },
            scene: asset_server.load("Rocket.glb#Scene0"),
            ..default()
        })
        .insert(Thrust { value: 0.0 })
        .insert(Fuel { value: 1000.0 })
        .insert(Velocity(Vec3::ZERO))
        .insert(LeftEcs { value: 0.0 })
        .insert(RightEcs { value: 0.0 })
        .insert(Altitute { value: 0.0 })
        .insert(Rocket);
}

fn setup_collider_body(mut commands: Commands) {
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::cuboid(0.15, 0.15, 0.15))
        .insert(ExternalForce {
            force: Vec3::new(0.0, 0.0, 0.0),
            torque: Vec3::new(0.0, 0.0, 0.0),
        })
        .insert(ColliderMassProperties::Density(20.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.3, 0.0)))
        .insert(RocketCollider);
}

fn particle_emitter_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut _rocket_transform: Query<&Transform, With<Rocket>>,
    mut _thrust: Query<&mut Thrust, With<Rocket>>,
) {
    let num_particles = 10;

    let playerTranslation = _rocket_transform.single_mut().translation;
    let thrust = _thrust.single_mut().value;
    let is_thrusting = thrust > 0.0;

    if !is_thrusting {
        return;
    }

    for _ in 0..num_particles {
        let position = playerTranslation;
        let velocity = Vec3::new(
            rand::thread_rng().gen_range(-0.2..0.2),
            if is_thrusting {
                rand::thread_rng().gen_range(0.0..(thrust * 0.001)) * -1.0
            } else {
                0.0
            },
            rand::thread_rng().gen_range(-0.2..0.2),
        );
        let scale = rand::thread_rng().gen_range(0.01..0.1);
        let rotation = Quat::IDENTITY;
        let lifetime = 2.0;
        let color = Color::rgba(1.0, 1.0, 1.0, 0.5);

        commands
            .spawn(PbrBundle {
                mesh: meshes.add(Sphere::default().mesh().uv(5, 5)),
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
                color,
            });
    }
}

fn update_particle_system(
    mut commands: Commands,
    time: Res<Time>,
    mut particles: Query<(Entity, &mut Particle)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut transforms: Query<&mut Transform>,
    handles: Query<(&Handle<StandardMaterial>)>,
) {
    for (entity, mut particle) in particles.iter_mut() {
        let velocity = particle.velocity;
        particle.position += velocity * time.delta_seconds();
        particle.lifetime -= time.delta_seconds();
        particle.scale += time.delta_seconds() * 0.1;
        particle.rotation *= Quat::from_rotation_y(0.1);
        transforms.get_mut(entity).unwrap().translation = particle.position;
        transforms.get_mut(entity).unwrap().scale = Vec3::splat(particle.scale);
        transforms.get_mut(entity).unwrap().rotation = particle.rotation;
        // materials
        //     .get_mut(handles.get(entity).unwrap())
        //     .unwrap()
        //     .base_color = Color::rgba(1.0, 1.0, 1.0, particle.lifetime / 4.0);

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
        for mut body in collider.iter_mut() {
            transform.translation = body.translation;
            transform.rotation = body.rotation;
            _altitude.single_mut().value = transform.translation.y;
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
            thrust.value = (thrust.value - 1.0 * time.delta_seconds()).max(0.0);
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
) {
    const LOCAL_UP: Vec3 = Vec3::Y;

    for mut ext_force in ext_forces.iter_mut() {
        let rotation = _rocket_transform.single_mut().rotation;
        let thrust_direction = rotation.mul_vec3(LOCAL_UP);
        let right_ecs = rotation.mul_vec3(Vec3::X) * _right_ecs.single_mut().value;
        let left_ecs = rotation.mul_vec3(-Vec3::X) * _left_ecs.single_mut().value;

        for thrust in _thrust.iter() {
            ext_force.force = thrust_direction * thrust.value + right_ecs + left_ecs;
        }
    }
}

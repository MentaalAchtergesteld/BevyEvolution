use bevy::prelude::*;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                update_velocity,
                damp_velocity,
                clamp_velocity,
                update_position,
                update_rotation
            ));
    }
}

#[derive(Component, Default)]
pub struct Acceleration(pub Vec2);

#[derive(Component, Default)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct VelocityDamping(pub f32);

#[derive(Component)]
pub struct MaxVelocity(pub f32);

fn update_velocity(
    mut query: Query<(&mut Velocity, &mut Acceleration)>,
    time: Res<Time>
) {
    for (mut velocity, mut acceleration) in &mut query {
        if acceleration.0.is_nan() {
            acceleration.0 = Vec2::ZERO;
        }

        velocity.0 += acceleration.0 * time.delta_secs();

        if velocity.0.length_squared() < 1e-5 {
            velocity.0 = Vec2::ZERO;
        }

        acceleration.0 = Vec2::ZERO;
    }
}

fn damp_velocity(
    mut query: Query<(&mut Velocity, &VelocityDamping)>,
    time: Res<Time>
) {
    for (mut velocity, damping) in &mut query {
        let damping_factor = (1.0 - damping.0 * time.delta_secs()).max(0.0);
        velocity.0 *= damping_factor;
    }
}

fn clamp_velocity(
    mut query: Query<(&mut Velocity, &MaxVelocity)>
) {
    for (mut velocity, max_velocity) in &mut query {
        velocity.0 = velocity.0.clamp_length_max(max_velocity.0);
    }
}

fn update_position(
    mut query: Query<(&mut Transform, &Velocity)>,
    time: Res<Time>
) {
    for (mut transform, velocity) in &mut query {
        transform.translation += velocity.0.extend(0.) * time.delta_secs();
    }
}

fn update_rotation(
    mut query: Query<(&mut Transform, &Velocity)>,
) {
    for (mut transform, velocity) in &mut query {
        if velocity.0.length_squared() > 0. {
            transform.rotation = Quat::from_rotation_z(velocity.0.to_angle());
        }
    }
}
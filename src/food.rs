use std::f32::consts::TAU;

use bevy::{color::palettes::css::GREEN, prelude::*, sprite::Material2d, window::PrimaryWindow};
use rand::Rng;

use crate::{movement::{Velocity, VelocityDamping}, GameRng};

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_initial_food);
    }
}

#[derive(Component)]
pub struct Food {
    pub saturation: f32,
}

#[derive(Bundle)]
struct FoodBundle<T: Material2d> {
    food: Food,
    transform: Transform,
    velocity: Velocity,
    velocity_damping: VelocityDamping,

    mesh: Mesh2d,
    material: MeshMaterial2d<T>
}

fn spawn_initial_food(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut rng: ResMut<GameRng>
) {
    let window = window_query.single();
    let window_size = window.size();

    let food_count = 4;

    for _ in 0..food_count {
        let transform = Transform::from_translation(Vec2::new(
            rng.0.random_range(-window_size.x*0.5..window_size.x*0.5),
            rng.0.random_range(-window_size.y*0.5..window_size.y*0.5),
        ).extend(0.));

        let initial_force_length = rng.0.random_range(5.0..80.0);
        let initial_force_direction = rng.0.random_range(0.0..TAU);

        let velocity = Velocity(Vec2::from_angle(initial_force_direction) * initial_force_length);

        let velocity_damping = VelocityDamping(1.0);

        let saturation = rng.0.random_range(5.0..8.0);

        let mesh = Mesh2d(meshes.add(Circle::new(saturation)));
        let material = MeshMaterial2d(materials.add(ColorMaterial::from_color(GREEN)));

        commands.spawn(FoodBundle {
            food: Food {
                saturation
            },
            transform,
            velocity,
            velocity_damping,
            mesh,
            material
        });
    }
}
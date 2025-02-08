use std::f32::consts::TAU;

use bevy::{color::palettes::css::GREEN, prelude::*, sprite::Material2d, window::PrimaryWindow};
use rand::Rng;

use crate::{interaction_forces::InteractionGroup, movement::{Acceleration, Velocity, VelocityDamping}, GameRng};

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_initial_food)
            .add_systems(Update, (
                update_food_radius,
                count_food_neighbours,
                food_splitting,
                apply_saturation_change,
            ));
    }
}

#[derive(Component)]
pub struct Food {
    pub max_neighbours_for_split: i32,
    pub min_split_saturation: f32,
    pub split_chance: f64
}

#[derive(Component)]
pub struct Saturation {
    pub saturation: f32,
    pub max_saturation: f32,
    pub saturation_change: f32
}

impl Saturation {
    fn modify(&mut self, amount: f32) {
        self.saturation += amount;

        self.saturation = self.saturation.min(self.max_saturation).max(0.0);
    }
}

#[derive(Component)]
pub struct Neighbourhood {
    radius: f32,
    count: i32,
}

impl Neighbourhood {
    fn new(radius: f32) -> Self {
        Self {
            radius,
            count: 0
        }
    }
}

#[derive(Bundle)]
struct FoodBundle<T: Material2d> {
    food: Food,
    saturation: Saturation,
    neighbourhood: Neighbourhood,
    transform: Transform,
    velocity: Velocity,
    acceleration: Acceleration,
    velocity_damping: VelocityDamping,

    interaction_group: InteractionGroup,

    mesh: Mesh2d,
    material: MeshMaterial2d<T>
}

fn spawn_food(
    position: Vec2,
    max_saturation: f32,
    saturation: f32,
    saturation_change: f32,
    max_neighbours_for_split: i32,
    min_split_saturation: f32,
    split_chance: f64,
    neighbourhood_radius: f32,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    rng: &mut impl Rng
) -> Entity {
    let transform = Transform::from_translation(position.extend(0.1))
        .with_scale(Vec2::splat(saturation).extend(0.));

    let initial_force_length = rng.random_range(5.0..80.0);
    let initial_force_direction = rng.random_range(0.0..TAU);

    let velocity = Velocity(Vec2::from_angle(initial_force_direction) * initial_force_length);
    let acceleration = Acceleration::default();

    let velocity_damping = VelocityDamping(1.0);

    let interaction_group = InteractionGroup(0);

    let mesh = Mesh2d(meshes.add(Circle::new(1.0)));
    let material = MeshMaterial2d(materials.add(ColorMaterial::from_color(GREEN)));

    commands.spawn(FoodBundle {
        food: Food {
            max_neighbours_for_split,
            min_split_saturation,
            split_chance
        },
        saturation: Saturation {
            saturation,
            max_saturation,
            saturation_change
        },
        neighbourhood: Neighbourhood::new(neighbourhood_radius),
        transform,
        velocity,
        acceleration,
        velocity_damping,
        interaction_group,
        mesh,
        material
    }).id()
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
        let position = Vec2::new(
            rng.0.random_range(-window_size.x*0.5..window_size.x*0.5),
            rng.0.random_range(-window_size.y*0.5..window_size.y*0.5),
        );

        let max_saturation = rng.0.random_range(5.0..10.0);
        let start_saturation = max_saturation * 0.5;
        let saturation_change = 0.5;

        let max_neighbours_for_split = 8;
        let min_split_saturation = max_saturation * 0.75;
        let split_chance = 0.01;

        let neighbourhood_radius = 64.;

        spawn_food(
            position,
            max_saturation,
            start_saturation,
            saturation_change,
            max_neighbours_for_split,
            min_split_saturation,
            split_chance,
            neighbourhood_radius,
            &mut commands,
            &mut meshes,
            &mut materials,
            &mut rng.0
        );
    }
}

fn update_food_radius(
    mut query: Query<(&mut Transform, &Saturation), With<Food>>,
) {
    for (mut transform, saturation) in &mut query {
        transform.scale = Vec2::splat(saturation.saturation).extend(0.);
    }
}

fn count_food_neighbours(
    mut foods: Query<(&mut Neighbourhood, &Transform, Entity), With<Food>>,
    other_foods: Query<(&Transform, Entity), With<Food>>
) {
    for (mut neighbourhood, transform_a, entity_a) in &mut foods {
        neighbourhood.count = 0;
        
        for (transform_b, entity_b) in &other_foods {
            if entity_a.index() == entity_b.index() {
                continue;
            };

            if transform_a.translation.distance(transform_b.translation) < neighbourhood.radius {
                neighbourhood.count += 1;
            }
        }
    } 
}

fn food_splitting(
    mut query: Query<(&mut Saturation, &Neighbourhood, &Food, &Transform)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut rng: ResMut<GameRng>
) {
    for (mut saturation, neighbourhood, food, transform) in &mut query {
        if neighbourhood.count > food.max_neighbours_for_split || saturation.saturation < food.min_split_saturation {
            continue;
        }

        if rng.0.random_bool(food.split_chance) {
            let max_saturation = saturation.max_saturation * rng.0.random_range(0.9..1.1);

            if max_saturation < 1. {
                continue;
            }

            let new_saturation = saturation.saturation * 0.5;
            let saturation_change = saturation.saturation_change;
            let max_neighbours_for_split = food.max_neighbours_for_split;
            let min_split_saturation = food.min_split_saturation;
            let split_chance = food.split_chance;

            spawn_food(
                transform.translation.xy(),
                max_saturation,
                new_saturation,
                saturation_change,
                max_neighbours_for_split,
                min_split_saturation,
                split_chance,
                neighbourhood.radius,
                &mut commands,
                &mut meshes,
                &mut materials,
                &mut rng.0
            );
            saturation.saturation *= 0.5;
        }
    }
}

fn apply_saturation_change(
    mut query: Query<&mut Saturation>,
    time: Res<Time>
) {
    for mut saturation in &mut query {
        let change = saturation.saturation_change * time.delta_secs();
        saturation.modify(change);
    }
}
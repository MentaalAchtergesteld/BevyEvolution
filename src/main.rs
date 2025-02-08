use std::collections::HashMap;

use age::AgePlugin;
use bevy::{app::{App, Startup}, asset::Assets, core_pipeline::core_2d::Camera2d, ecs::{query::With, system::{Commands, Query, ResMut, Resource}}, math::Vec2, render::mesh::Mesh, sprite::ColorMaterial, window::{PrimaryWindow, Window}, DefaultPlugins};
use food::{spawn_food, FoodPlugin};
use interaction_forces::{ForceParams, InteractionForcesPlugin, InteractionRules};
use mortality::MortalityPlugin;
use movement::MovementPlugin;
use rand::{rngs::StdRng, Rng, SeedableRng};
use saturation::SaturationPlugin;
use wrapping::{WrappingPlugin, WrappingRect};

mod wrapping;
mod movement;
mod saturation;
mod age;
mod mortality;
mod interaction_forces;
mod food;

#[derive(Resource)]
struct GameRng(StdRng);

impl GameRng {
    fn new(seed: u64) -> Self {
        Self(StdRng::seed_from_u64(seed))
    }
}

fn main() {
    App::new()
        .insert_resource(GameRng::new(420))
        .add_plugins(DefaultPlugins)
        .add_plugins((
            WrappingPlugin,
            MovementPlugin,
            SaturationPlugin,
            AgePlugin,
            MortalityPlugin,
            InteractionForcesPlugin,
            FoodPlugin,
        ))
        .add_systems(Startup, (
            create_wrapping_rect,
            setup_interaction_rules,
            spawn_initial_food,
            spawn_camera,
        ))
        .run();
}

fn create_wrapping_rect(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.single();

    let half_width = window.width() * 0.5;
    let half_height = window.height() * 0.5;
    
    let x = -half_width;
    let y = -half_height;

    let width = window.width();
    let height = window.height();

    commands.insert_resource(WrappingRect {x, y, width, height});
}

fn setup_interaction_rules(
    mut commands: Commands
) {
    let mut rules = HashMap::new();
    rules.insert((0, 0), ForceParams {
        attract_strength: 8.0,
        repulse_strength: 64.0,
        min_distance: 16.0,
        max_distance: 128.0
    });

    commands.insert_resource(InteractionRules(rules));
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

    let food_count = 32;

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
        let split_chance = 0.001;

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

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}
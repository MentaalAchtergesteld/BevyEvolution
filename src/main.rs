use age::AgePlugin;
use bevy::{app::{App, Startup}, core_pipeline::core_2d::Camera2d, ecs::{query::With, system::{Commands, Query, Resource}}, window::{PrimaryWindow, Window}, DefaultPlugins};
use food::FoodPlugin;
use interaction_forces::InteractionForcesPlugin;
use mortality::MortalityPlugin;
use movement::MovementPlugin;
use rand::{rngs::StdRng, SeedableRng};
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
        .add_systems(Startup, (spawn_camera, create_wrapping_rect))
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

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}
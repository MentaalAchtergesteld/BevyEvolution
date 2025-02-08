use bevy::{app::{App, Startup}, core_pipeline::core_2d::Camera2d, ecs::system::{Commands, Resource}, DefaultPlugins};
use food::FoodPlugin;
use interaction_forces::InteractionForcesPlugin;
use movement::MovementPlugin;
use rand::{rngs::StdRng, SeedableRng};
use saturation::SaturationPlugin;

mod movement;
mod saturation;
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
            MovementPlugin,
            SaturationPlugin,
            InteractionForcesPlugin,
            FoodPlugin,
        ))
        .add_systems(Startup, spawn_camera)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}
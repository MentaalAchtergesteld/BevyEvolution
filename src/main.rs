use bevy::{app::App, ecs::system::Resource, DefaultPlugins};
use food::FoodPlugin;
use movement::MovementPlugin;
use rand::{rngs::StdRng, SeedableRng};

mod movement;
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
            FoodPlugin,
        ))
        .run();
}

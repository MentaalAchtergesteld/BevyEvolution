use bevy::prelude::*;

pub struct SaturationPlugin;

impl Plugin for SaturationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, update_saturation);
    }
}

#[derive(Component)]
pub struct Saturation {
    pub max: f32,
    pub current: f32,
}

#[derive(Component)]
pub struct SaturationChange(pub f32);

fn update_saturation(
    mut query: Query<(&mut Saturation, &SaturationChange)>,
    time: Res<Time>
) {
    for (mut saturation, change) in &mut query {
        saturation.current += change.0 * time.delta_secs();

        if saturation.current < 0.             { saturation.current = 0.             };
        if saturation.current > saturation.max { saturation.current = saturation.max };
    }
}
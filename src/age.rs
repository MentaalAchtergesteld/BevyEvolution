use bevy::prelude::*;

pub struct AgePlugin;

impl Plugin for AgePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, update_age);
    }
}

#[derive(Component)]
pub struct Age {
    pub current: f32,
    pub change: f32,
}

#[derive(Component)]
pub struct MaxAge(pub f32);

fn update_age(
    mut query: Query<(&mut Age, Option<&MaxAge>)>,
    time: Res<Time>
) {
    for (mut age, max_age) in &mut query {
        age.current += age.change * time.delta_secs();

        if let Some(max_age) = max_age {
            age.current = age.current.min(max_age.0);
        }
    }
}
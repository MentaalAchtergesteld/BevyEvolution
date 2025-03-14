use std::collections::HashMap;

use bevy::prelude::*;

use crate::{movement::Acceleration, wrapping::{wrapping_delta, WrappingRect}};

pub struct InteractionForcesPlugin;

impl Plugin for InteractionForcesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, apply_interaction_forces);
    }
}

#[derive(Resource)]
pub struct InteractionRules(pub HashMap<(u8, u8), ForceParams>);

pub struct ForceParams {
    pub attract_strength: f32,
    pub repulse_strength: f32,
    pub min_distance: f32,
    pub max_distance: f32,
}

#[derive(Component)]
pub struct InteractionGroup(pub u8);

fn apply_interaction_forces(
    mut query: Query<(&mut Acceleration, &InteractionGroup, &Transform, Entity)>,
    rules: Res<InteractionRules>,
    wrapping_rect: Option<Res<WrappingRect>>
) {
    let mut food = query.iter_combinations_mut();

    while let Some([
        (mut acceleration_a, group_a, transform_a, entity_a),
        (_, group_b, transform_b, entity_b)
    ]) = food.fetch_next() {
        if entity_a.index() == entity_b.index() { continue; }

        let force_params = if let Some(force_params) = rules.0.get(&(group_a.0, group_b.0)) {
            force_params
        } else {
            continue;
        };

        let delta = if let Some(rect) = &wrapping_rect {
            wrapping_delta(transform_a.translation.xy(), transform_b.translation.xy(), rect)
        } else {
            (transform_b.translation - transform_a.translation).xy()
        };
        
        let distance = delta.length();

        if distance < force_params.min_distance {
            let repulsion_force = delta.normalize_or_zero() * -force_params.repulse_strength;
            acceleration_a.0 +=  repulsion_force * distance / force_params.min_distance;
        } else if distance < force_params.max_distance {
            let attraction_force = delta.normalize_or_zero() * force_params.attract_strength;
            acceleration_a.0 += attraction_force;
        }
    }
}
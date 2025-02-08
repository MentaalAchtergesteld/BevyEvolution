use bevy::prelude::*;

use crate::{age::{Age, MaxAge}, saturation::Saturation};

pub struct MortalityPlugin;

impl Plugin for MortalityPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                check_starvation,
                check_old_age,
                remove_died,
            ));
    }
}

#[derive(Component)]
pub struct CanDie;

#[derive(Component)]
pub struct Died;

#[derive(Component)]
pub struct Starved;

#[derive(Component)]
pub struct OldAge;

fn check_starvation(
    mut commands: Commands,
    query: Query<(Entity, &Saturation), Without<Died>>
) {
    for (entity, saturation) in &query {
        if saturation.current <= 0. {
            commands.entity(entity).insert((Starved, Died));
        }
    }
}

fn check_old_age(
    mut commands: Commands,
    query: Query<(Entity, &Age, &MaxAge), Without<Died>>
) {
    for (entity, age, max_age) in &query {
        if age.current > max_age.0 {
            commands.entity(entity).insert((OldAge, Died));
        }
    }
}

fn remove_died(
    mut commands: Commands,
    query: Query<Entity, (With<Died>, With<CanDie>)>
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
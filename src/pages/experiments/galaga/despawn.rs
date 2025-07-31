use bevy::prelude::*;

use crate::pages::experiments::galaga::{
    health::Health, schedule::InGameSet, spaceship::Spaceship, state::GameState,
};

const DESPAWN_DISTANCE: f32 = 500.0;

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (despawn_far_away_entities, despawn_dead_entities).in_set(InGameSet::DespawnEntities),
        )
        .add_systems(OnEnter(GameState::GameOver), despawn_all_entities);
    }
}

fn despawn_far_away_entities(
    mut commands: Commands,
    query: Query<(Entity, &Transform), Without<Spaceship>>,
    spaceship_query: Query<(Entity, &Transform), With<Spaceship>>,
) {
    let Ok((_, spaceship_position)) = spaceship_query.single() else {
        return;
    };

    for (entity, transform) in query.iter() {
        let distance = transform
            .translation
            .distance(spaceship_position.translation);

        // Entity is far away from the camera's viewport (i.e. origin).
        if distance > DESPAWN_DISTANCE {
            commands.entity(entity).despawn();
        }
    }
}

fn despawn_dead_entities(mut commands: Commands, query: Query<(Entity, &Health)>) {
    for (entity, health) in query.iter() {
        if health.value <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn despawn_all_entities(mut commands: Commands, query: Query<Entity, With<Health>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

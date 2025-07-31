use bevy::prelude::*;

use crate::pages::experiments::galaga::{collision_detection::Collider, schedule::InGameSet};

#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3,
}

impl Velocity {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}

#[derive(Component, Debug)]
pub struct Acceleration {
    pub value: Vec3,
}

impl Acceleration {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}

#[derive(Component, Debug)]
pub struct PitchAcceleration {
    pub value: f32,
}

impl PitchAcceleration {
    pub fn new(value: f32) -> Self {
        Self { value }
    }
}

#[derive(Component, Debug)]
pub struct RollAcceleration {
    pub value: f32,
}

impl RollAcceleration {
    pub fn new(value: f32) -> Self {
        Self { value }
    }
}

#[derive(Bundle)]
pub struct MovingObjectBundle {
    pub velocity: Velocity,
    pub acceleration: Acceleration,
    pub model: (SceneRoot, Transform),
    pub collider: Collider,
    pub pitch_acceleration: PitchAcceleration,
    pub roll_acceleration: RollAcceleration,
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_position, update_velocity)
                .chain()
                .in_set(InGameSet::EntityUpdates),
        );
    }
}

fn update_velocity(mut query: Query<(&Acceleration, &mut Velocity)>, time: Res<Time>) {
    for (acceleration, mut velocity) in query.iter_mut() {
        velocity.value += acceleration.value * time.delta_secs();
    }
}

fn update_position(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.value * time.delta_secs();
    }
}

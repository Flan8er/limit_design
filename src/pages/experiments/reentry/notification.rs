use bevy::prelude::*;

#[derive(Event, Clone)]
pub struct CapsuleState {
    pub altitude: f32,
    pub velocity: f32,
}

impl Default for CapsuleState {
    fn default() -> Self {
        Self {
            altitude: 0.,
            velocity: 0.,
        }
    }
}

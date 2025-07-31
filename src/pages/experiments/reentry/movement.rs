use bevy::prelude::*;
use std::f32::consts::TAU;

#[derive(Component)]
pub struct Rotatable {
    pub speed: f32,
    pub axis: String,
}

pub fn rotate_object(mut objects: Query<(&mut Transform, &Rotatable)>, timer: Res<Time>) {
    for (mut transform, object) in &mut objects {
        match object.axis.as_str() {
            "x" => {
                transform.rotate_local_x(object.speed * TAU * timer.delta_secs());
            }
            "y" => {
                transform.rotate_local_y(object.speed * TAU * timer.delta_secs());
            }
            "z" => {
                transform.rotate_local_z(object.speed * TAU * timer.delta_secs());
            }
            _ => continue,
        }
    }
}

use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;

use crate::pages::experiments::threat_prediction::components::simulation_params::{
    EARTH_RADIUS, WORLD_SCALE,
};

pub fn spawn_camera(mut commands: Commands) {
    let initial = Vec3::new(1000.0, 600.0, 1000.0);
    let radius = initial.length();
    let yaw = initial.x.atan2(initial.z);
    let pitch = (initial.y / radius).asin();

    commands.spawn((
        Camera3d::default(),
        PanOrbitCamera {
            // Set focal point (what the camera should look at)
            focus: Vec3::new(0.0, 2.0, 0.0),
            // Set the starting position, relative to focus (overrides camera's transform).
            radius: Some(radius),
            yaw: Some(yaw),
            pitch: Some(pitch),
            // zoom_upper_limit: Some(25.0),
            zoom_lower_limit: (EARTH_RADIUS * WORLD_SCALE) as f32,
            ..default()
        },
    ));

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 300.,
        ..default()
    });

    commands.insert_resource(ClearColor(Color::NONE));
}

use bevy::prelude::*;

use crate::pages::experiments::reentry::{
    capsule::{Capsule, INITIAL_POSITION},
    earth::EARTH_DIAMETER,
};
// use bevy_panorbit_camera::PanOrbitCamera;

const CAMERA_DISTANCE: f32 = 600.;
const CAMERA_FOV: f32 = 90.; // [deg]

#[derive(Component)]
pub struct Camera;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera,
        Camera3d::default(),
        Projection::Perspective(PerspectiveProjection {
            fov: (CAMERA_FOV * std::f32::consts::PI) / 180.0, // [rad]
            near: 0.1,
            far: EARTH_DIAMETER,
            aspect_ratio: 1.77778,
        }),
        Transform::from_xyz(
            INITIAL_POSITION.x + CAMERA_DISTANCE,
            INITIAL_POSITION.y,
            INITIAL_POSITION.z,
        )
        .looking_at(INITIAL_POSITION, Vec3::Y),
        GlobalTransform::default(),
        // PanOrbitCamera::default(),
        Name::new("Main Camera"),
    ));
}

pub fn update_camera_focus(
    updated_state: ResMut<Capsule>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    let capsule_position = updated_state.position;

    // Get the single camera entity from the query.
    let Ok(mut camera_transform) = camera_query.single_mut() else {
        return;
    };

    // Calculate the direction the camera should be offset in - using velocity as the direction.
    let capsule_forward = updated_state.velocity.normalize();
    // Adjusting the camera's position to be behind the capsule.
    let camera_offset = capsule_forward * (-CAMERA_DISTANCE);
    // Update the camera's position.
    let updated_camera_position = capsule_position + camera_offset;

    // Compute a perpendicular "up" vector by normalizing to the object's position
    let up_direction = (capsule_position).normalize();

    // Set the camera's transform (position and orientation).
    *camera_transform = Transform::from_translation(updated_camera_position)
        .looking_at(capsule_position, up_direction);
}

use bevy::prelude::*;

use crate::pages::experiments::robot::simulation::plugins::debug::dev_tools::ShowDevTools;

/// The `ShowAxes` component is attached to an entity to get the `draw_axes` system to
/// display axes according to its Transform component.
#[derive(Component)]
pub struct ShowAxes;

const AXIS_LENGTH: f32 = 1.0;

// This system draws the axes based on the object's transform.
pub fn draw_object_coordinates(
    mut gizmos: Gizmos,
    query: Query<&Transform, With<ShowAxes>>,
    show_dev_tools: Res<ShowDevTools>,
) {
    if !show_dev_tools.0 {
        return;
    }

    for &transform in &query {
        gizmos.axes(transform, AXIS_LENGTH);
    }
}

pub fn draw_world_coordinates(mut gizmos: Gizmos, show_dev_tools: Res<ShowDevTools>) {
    if !show_dev_tools.0 {
        return;
    }

    gizmos.axes(Transform::IDENTITY, AXIS_LENGTH);
}

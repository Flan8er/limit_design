use bevy::prelude::*;

use crate::pages::experiments::robot::simulation::plugins::setup::robot_spawner::{
    JointAngles, Robot, RobotJoint,
};

#[derive(Resource, Default)]
/// A way to correlate of the relative position of the robot to world space.
struct PrevAngles {
    j1: f32,
    j2: f32,
    j3: f32,
    j4: f32,
    j5: f32,
    j6: f32,
}

pub struct RotationPlugin;
impl Plugin for RotationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PrevAngles>()
            .add_systems(Update, update_joint_angles);
    }
}

fn update_joint_angles(
    mut events: EventReader<JointAngles>,
    mut prev: ResMut<PrevAngles>,
    robot_children: Query<&Children, With<Robot>>,
    joint_enum: Query<&RobotJoint>,
    mut xform_q: Query<&mut Transform>,
) {
    for evt in events.read() {
        let children = robot_children
            .single()
            .iter()
            .cloned()
            .collect::<Vec<Entity>>();

        // map each into (chain_index, Entity), skipping Base entirely
        let mut list: Vec<(usize, Entity)> = children
            .into_iter()
            .filter_map(|ent| {
                joint_enum.get(ent).ok().and_then(|j| {
                    // define the physical chain order:
                    let idx = match j {
                        RobotJoint::J1 => 0,
                        RobotJoint::J2 => 1,
                        RobotJoint::J3 => 2,
                        RobotJoint::J4 => 3,
                        RobotJoint::J5 => 4,
                        RobotJoint::J6 => 5,
                        RobotJoint::Base => return None,
                    };
                    Some((idx, ent))
                })
            })
            .collect();

        // sort so list[0] is J1, [1] is J2, …, [5] is J6
        list.sort_by_key(|(chain_i, _)| *chain_i);

        // compute deltas (in radians)
        let deltas = [
            (evt.j1 - prev.j1).to_radians(),
            (evt.j2 - prev.j2).to_radians(),
            (evt.j3 - prev.j3).to_radians(),
            (evt.j4 - prev.j4).to_radians(),
            (evt.j5 - prev.j5).to_radians(),
            (evt.j6 - prev.j6).to_radians(),
        ];

        // rotate each joint i by deltas[i] on itself + downstream
        for (i, &(_idx, ent_i)) in list.iter().enumerate() {
            if i == deltas.len() {
                break;
            }

            let angle_delta = deltas[i];
            if angle_delta.abs() < std::f32::EPSILON {
                continue;
            }

            // get pivot from joint i
            let (pivot, pivot_rot) = {
                if let Ok(tf) = xform_q.get_mut(ent_i) {
                    (tf.translation, tf.rotation)
                } else {
                    continue;
                }
            };
            // pick local axis
            let axis = match i {
                0 => Vec3::Y,
                1 => Vec3::X,
                2 => Vec3::X,
                3 => Vec3::Z,
                4 => Vec3::X,
                5 => Vec3::Z,
                _ => continue,
            };

            // spin i..end
            for &(_down_i, ent_j) in list.iter().skip(i) {
                if let Ok(mut tf) = xform_q.get_mut(ent_j) {
                    rotate_around_pivot(&mut tf, pivot, pivot_rot, axis, angle_delta);
                }
            }
        }

        // update stored values
        prev.j1 = evt.j1;
        prev.j2 = evt.j2;
        prev.j3 = evt.j3;
        prev.j4 = evt.j4;
        prev.j5 = evt.j5;
        prev.j6 = evt.j6;
    }
}

/// Rotate `transform` around the point `pivot`, by `angle_radians`,
/// about the pivot’s local `axis` (in its own rotated space).
fn rotate_around_pivot(
    transform: &mut Transform,
    pivot: Vec3,
    pivot_rotation: Quat, // pivot’s world‐space orientation
    local_axis: Vec3,     // e.g. Vec3::X, Vec3::Y, Vec3::Z
    angle_radians: f32,
) {
    // Turn the pivot’s local axis into world‐space
    let world_axis = (pivot_rotation * local_axis).normalize();

    // Build the spin quaternion
    let spin = Quat::from_axis_angle(world_axis, angle_radians);

    // Orbit: translate into pivot‐space, spin, translate back
    let rel_pos = transform.translation - pivot;
    transform.translation = spin * rel_pos + pivot;

    // Also rotate the object itself so it “faces” correctly
    transform.rotation = spin * transform.rotation;
}

use std::f32::consts::PI;

use bevy::prelude::*;

use crate::pages::experiments::node_tree::canvas::core::node::Node;
const MAIN_LEN: f32 = 500.0;
const BRANCH_ANGLE: i16 = 35;
// const BRANCH_ANGLE: i16 = 45;

pub fn construct_node_tree(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    node_tree: Res<Node<String>>,
) {
    // Check base cases
    // Total end nodes in tree <= 2
    //      Those end nodes should form a straight line / a single point
    let num_end_nodes = total_end_nodes(node_tree.children.clone(), 0);

    let node_shape = meshes.add(Circle::new(25.0));
    let node_color = materials.add(Color::hsl(188.0, 0.19, 0.61));
    let path_color = materials.add(Color::srgb(1.0, 1.0, 1.0));
    // let path_shape

    // let num_end_nodes = 2;

    match num_end_nodes {
        0 => (),
        1 => {
            commands.spawn((
                Mesh2d(node_shape),
                MeshMaterial2d(node_color),
                Transform::from_xyz(0.0, 0.0, 0.0),
            ));
        }
        2 => {
            commands.spawn((
                Mesh2d(node_shape.clone()),
                MeshMaterial2d(node_color.clone()),
                Transform::from_xyz(0.0, -MAIN_LEN / 2., 0.0),
            ));

            commands.spawn((
                Mesh2d(node_shape),
                MeshMaterial2d(node_color.clone()),
                Transform::from_xyz(0.0, MAIN_LEN / 2., 0.0),
            ));

            commands.spawn((
                Mesh2d(meshes.add(Rectangle::new(25.0, MAIN_LEN))),
                MeshMaterial2d(path_color),
                Transform::from_xyz(0.0, 0.0, 0.0),
            ));
        }
        _ => {
            commands.spawn((
                Mesh2d(node_shape.clone()),
                MeshMaterial2d(node_color.clone()),
                Transform::from_xyz(0.0, -MAIN_LEN / 2., 0.0),
            ));

            navigate_node_tree(
                Vec::new(),
                node_tree.children.clone(),
                &mut commands,
                &node_shape,
                &node_color,
                &path_color,
            );
        }
    }
}

fn navigate_node_tree(
    node_history: Vec<(usize, usize)>,
    children: Vec<Node<String>>,
    commands: &mut Commands,
    node_shape: &Handle<Mesh>,
    node_color: &Handle<ColorMaterial>,
    path_color: &Handle<ColorMaterial>,
) {
    if children.is_empty() {
        // Spawn the node at the end of the branch
        let mut node_history = node_history.clone();
        node_history.push((0, 1));

        let mut collection = Vec::new();
        let mut start_point = Vec2::new(0.0, -MAIN_LEN / 2.0);
        for breakpoint in node_history.clone() {
            collection.push(breakpoint);

            let breakpoint_pos = calculate_node_position(collection.clone());

            let dir = breakpoint_pos - start_point;
            let len = dir.length();
            let ang = dir.y.atan2(dir.x);
            let mid = (start_point + breakpoint_pos) * 0.5;
            commands.spawn((
                Sprite {
                    custom_size: Some(Vec2::new(len, 25.0)),
                    color: Color::WHITE,
                    ..Default::default()
                },
                Transform {
                    translation: mid.extend(-1.0),        // position at midpoint
                    rotation: Quat::from_rotation_z(ang), // rotate to point from start→end
                    ..Default::default()
                },
            ));

            commands.spawn((
                Mesh2d(node_shape.clone()),
                MeshMaterial2d(path_color.clone()),
                Transform::from_translation(start_point.extend(-1.0)).with_scale(Vec3::splat(0.5)),
            ));
            commands.spawn((
                Mesh2d(node_shape.clone()),
                MeshMaterial2d(path_color.clone()),
                Transform::from_translation(breakpoint_pos.extend(-1.0))
                    .with_scale(Vec3::splat(0.5)),
            ));

            start_point = breakpoint_pos;
        }

        let node_pos = calculate_node_position(node_history.clone()).extend(0.0);

        commands.spawn((
            Mesh2d(node_shape.clone()),
            MeshMaterial2d(node_color.clone()),
            Transform::from_xyz(node_pos.x, node_pos.y, node_pos.z),
        ));

        return;
    }

    for (index, child) in children.iter().enumerate() {
        let mut local_node_history = node_history.clone();
        local_node_history.push((index, children.len()));

        navigate_node_tree(
            local_node_history,
            child.children.clone(),
            commands,
            node_shape,
            node_color,
            path_color,
        );
    }
}

/// Vec<(u32, u32)> -> Vec<(index of child in branch, total children in this branch)>
pub fn calculate_node_position(child_history: Vec<(usize, usize)>) -> Vec2 {
    let mut cumulative_position = Vec2::new(0.0, -MAIN_LEN / 2.);

    let mut parent_angle = 90;
    for (index, branch) in child_history.iter().enumerate() {
        // Distance up the branch this node branches off
        let branch_point = (branch.0 + 1) as f32 * branch_spacing(index, branch.1);

        let delta_x = (parent_angle as f32 * PI / 180.).cos() * branch_point;
        let delta_y = (parent_angle as f32 * PI / 180.).sin() * branch_point;

        cumulative_position += Vec2::new(delta_x, delta_y);

        // Update the parent angle for next iteration
        let branch_angle = absolute_branch_angle(parent_angle, branch.0);
        parent_angle = branch_angle;
    }

    cumulative_position
}

fn absolute_branch_angle(parent_angle: i16, child_index: usize) -> i16 {
    if child_index % 2 == 0 {
        parent_angle - BRANCH_ANGLE
    } else {
        parent_angle + BRANCH_ANGLE
    }
}

fn branch_length(depth: usize) -> f32 {
    MAIN_LEN / (depth + 1) as f32
}

fn branch_spacing(depth: usize, children: usize) -> f32 {
    branch_length(depth) / children as f32
}

fn total_end_nodes(children: Vec<Node<String>>, mut collected_end_nodes: usize) -> usize {
    if children.is_empty() {
        return collected_end_nodes + 1;
    }

    for child in children {
        collected_end_nodes = total_end_nodes(child.children, collected_end_nodes);
    }

    return collected_end_nodes;
}

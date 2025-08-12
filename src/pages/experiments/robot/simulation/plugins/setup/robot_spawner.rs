use bevy::{
    asset::LoadState,
    gltf::{Gltf, GltfMesh, GltfNode},
    prelude::*,
};
use serde::{Deserialize, Serialize};
use web_sys::wasm_bindgen::JsValue;

use crate::pages::experiments::robot::simulation::plugins::{
    debug::axis::ShowAxes, setup::asset_loader::SceneAssets,
};

#[derive(Serialize, Deserialize, Clone, PartialEq, Event)]
pub struct JointAngles {
    pub j1: f32,
    pub j2: f32,
    pub j3: f32,
    pub j4: f32,
    pub j5: f32,
    pub j6: f32,
}

#[derive(Component)]
pub struct Robot;

#[derive(Component, Copy, Clone)]
pub enum RobotJoint {
    J1 = 0,
    Base = 1,
    J6 = 2,
    J2 = 3,
    J4 = 4,
    J3 = 5,
    J5 = 6,
}
impl RobotJoint {
    pub fn from_index(index: usize) -> Self {
        match index {
            0 => Self::J1,
            1 => Self::Base,
            2 => Self::J6,
            3 => Self::J2,
            4 => Self::J4,
            5 => Self::J3,
            6 => Self::J5,
            // Instead of dealing with options, just set any other value to be the base (soft panic).
            _ => Self::Base,
        }
    }
}

#[derive(Resource, Default)]
pub struct RobotSpawned(pub bool);

pub fn spawn_robot(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    scene_assets: Res<SceneAssets>,
    gltfs: Res<Assets<Gltf>>,
    mesh_assets: Res<Assets<GltfMesh>>,
    node_assets: Res<Assets<GltfNode>>,
    mut spawned: ResMut<RobotSpawned>,
) {
    if spawned.0 {
        return;
    }

    match asset_server
        .get_load_state(&scene_assets.robot)
        .unwrap_or(LoadState::NotLoaded)
    {
        LoadState::Loaded => {}
        _ => return,
    }

    let gltf = match gltfs.get(&scene_assets.robot) {
        Some(g) => g,
        None => return,
    };

    commands
        .spawn((
            Robot,
            Transform::default(),
            GlobalTransform::default(),
            Visibility::default(),
        ))
        .with_children(|parent| {
            for (i, node_handle) in gltf.nodes.iter().enumerate() {
                if let Some(gltf_node) = node_assets.get(node_handle) {
                    if let Some(mesh_handle) = &gltf_node.mesh {
                        if let Some(gltf_mesh) = mesh_assets.get(mesh_handle) {
                            // 1) spawn one “joint” entity, with its local node transform:
                            parent
                                .spawn((
                                    gltf_node.transform, // local-to-Robot position/orientation
                                    GlobalTransform::default(),
                                    ShowAxes, // if you still want axes on the joint
                                    RobotJoint::from_index(i), // ← only this entity has the enum tag
                                    Visibility::default(),
                                ))
                                .with_children(|joint_parent| {
                                    // 2) for each primitive in that mesh, spawn a child mesh
                                    for prim in gltf_mesh.primitives.iter() {
                                        if let Some(material) = &prim.material {
                                            joint_parent.spawn((
                                                Mesh3d(prim.mesh.clone()),
                                                MeshMaterial3d(material.clone()),
                                                // No extra Transform here—the child meshes
                                                // inherit the joint’s transform.
                                            ));
                                        } else {
                                            web_sys::console::log_1(&JsValue::from_str(
                                                "Warning: primitive without material, skipping",
                                            ));
                                        }
                                    }
                                });
                        }
                    }
                }
            }
        });

    spawned.0 = true;
}

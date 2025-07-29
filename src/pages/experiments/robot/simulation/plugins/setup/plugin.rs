use bevy::prelude::*;

use crate::pages::experiments::robot::simulation::plugins::setup::{
    asset_loader::{load_assets, SceneAssets},
    camera::spawn_camera,
    floor::draw_floor,
    robot_spawner::{spawn_robot, RobotSpawned},
};

pub struct SetupPlugin;
impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>()
            .init_resource::<RobotSpawned>()
            .add_systems(
                Startup,
                (load_assets, draw_floor, spawn_camera).chain(), //spawn_robot
            )
            .add_systems(Update, spawn_robot);
    }
}

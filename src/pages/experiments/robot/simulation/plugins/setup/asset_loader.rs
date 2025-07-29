use bevy::{gltf::Gltf, prelude::*};

#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub robot: Handle<Gltf>,
}

pub fn load_assets(mut scene_assets: ResMut<SceneAssets>, asset_server: Res<AssetServer>) {
    scene_assets.robot = asset_server.load("FanucArm.glb");
}

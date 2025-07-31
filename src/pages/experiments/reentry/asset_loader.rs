use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub capsule: Handle<Scene>,
    pub earth: Handle<Scene>,
}

pub fn load_assets(mut scene_assets: ResMut<SceneAssets>, asset_server: Res<AssetServer>) {
    *scene_assets = SceneAssets {
        capsule: asset_server.load("Bullet_Scaled.glb#Scene0"),
        earth: asset_server.load("Earth_Low_Res_Scaled.glb#Scene0"),
    }
}

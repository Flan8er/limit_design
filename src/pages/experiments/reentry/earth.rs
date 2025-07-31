use bevy::prelude::*;

use crate::pages::experiments::reentry::asset_loader::SceneAssets;

pub const EARTH_DIAMETER: f32 = 12756274.; // [m]

pub fn spawn_earth(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((
        SceneRoot(scene_assets.earth.clone()),
        // Rotatable { speed: -0.005, axis: "y".to_string() },
        Transform {
            rotation: Quat::from_axis_angle(
                Vec3::new(0., 1., 0.),
                (-45. * std::f32::consts::PI) / 180.0,
            ),
            ..Default::default()
        },
        GlobalTransform::default(),
        Name::new("Earth"),
    ));
}

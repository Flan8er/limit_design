use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_third_person_camera::*;

use crate::pages::experiments::galaga::{
    asset_loader::AssetLoaderPlugin, asteroids::AsteroidPlugin, camera::CameraPlugin,
    collision_detection::CollisionDetectionPlugin, despawn::DespawnPlugin,
    movement::MovementPlugin, schedule::SchedulePlugin, spaceship::SpaceshipPlugin,
    state::StatePlugin,
};

pub fn render_game() -> App {
    let mut app = App::new();
    app
        // Bevy built-ins.
        .insert_resource(ClearColor(Color::srgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 750.0,
            ..default()
        })
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        canvas: Some("#bevy_canvas".into()),
                        transparent: true,
                        decorations: false,
                        fit_canvas_to_parent: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                }),
        )
        .add_plugins(ThirdPersonCameraPlugin)
        // User configured plugins.
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(SpaceshipPlugin)
        .add_plugins(AsteroidPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(CollisionDetectionPlugin)
        .add_plugins(DespawnPlugin)
        .add_plugins(SchedulePlugin)
        .add_plugins(StatePlugin);
    // .add_plugins(DebugPlugin)
    app
}

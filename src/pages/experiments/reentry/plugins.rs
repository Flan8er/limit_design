use bevy::prelude::*;

use crate::pages::experiments::reentry::{
    asset_loader::{load_assets, SceneAssets},
    camera::{spawn_camera, update_camera_focus},
    capsule::{
        spawn_capsule, update_capsule_position, Capsule, INITIAL_POSITION, INITIAL_VELOCITY,
    },
    earth::spawn_earth,
    movement::rotate_object,
};

pub struct AppPlugins;

impl Plugin for AppPlugins {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>()
            .insert_resource(Capsule::new(INITIAL_POSITION, INITIAL_VELOCITY))
            .add_systems(
                Startup,
                (load_assets, spawn_camera, spawn_earth, spawn_capsule).chain(),
            )
            .add_systems(
                PreUpdate,
                (update_capsule_position, update_camera_focus, rotate_object).chain(),
            )
            .add_systems(
                PreUpdate,
                ApplyDeferred
                    .after(update_capsule_position)
                    .before(update_camera_focus),
            );
    }
}

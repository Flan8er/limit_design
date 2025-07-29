use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_panorbit_camera::PanOrbitCameraPlugin;
use leptos_bevy_canvas::prelude::*;

use crate::pages::experiments::robot::simulation::plugins::{
    debug::{dev_tools::ShowDevTools, plugin::DebugPlugin},
    rotation::plugin::RotationPlugin,
    setup::{plugin::SetupPlugin, robot_spawner::JointAngles},
};

pub fn init_bevy_app(
    angle_receiver: BevyEventReceiver<JointAngles>,
    dev_tool_receiver: BevyEventReceiver<ShowDevTools>,
) -> App {
    let mut app = App::new();
    app.add_plugins(
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
    .insert_resource(ClearColor(Color::NONE))
    .import_event_from_leptos(angle_receiver)
    .import_event_from_leptos(dev_tool_receiver)
    .add_plugins(PanOrbitCameraPlugin)
    .add_plugins(DebugPlugin)
    .add_plugins(SetupPlugin)
    .add_plugins(RotationPlugin);

    app
}

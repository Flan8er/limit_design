use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_panorbit_camera::PanOrbitCameraPlugin;
use leptos_bevy_canvas::prelude::*;

use crate::pages::experiments::reentry::{notification::CapsuleState, plugins::AppPlugins};

pub fn run_reentry(event_sender: BevyEventSender<CapsuleState>) -> App {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::srgb(0., 0., 0.)))
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
        .export_event_to_leptos(event_sender)
        .insert_resource(ClearColor(Color::NONE))
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(AppPlugins);
    app
}

use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_panorbit_camera::PanOrbitCameraPlugin;
use bevy_polyline::prelude::*;
use leptos_bevy_canvas::prelude::*;

use crate::pages::experiments::threat_prediction::components::{
    asset_loader::AssetLoaderPlugin,
    camera::spawn_camera,
    communication::update_simulation_parameters,
    render::{clear_midcourse, draw_midcourse},
    setup::setup,
    simulation_params::SimulationParameters,
};

#[derive(Event, Clone)]
pub struct OrbitalNotification;

pub fn init_bevy_app(
    param_receiver: BevyEventReceiver<SimulationParameters>,
    event_sender: BevyEventSender<OrbitalNotification>,
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
    .add_plugins(AssetLoaderPlugin)
    .add_systems(Startup, (spawn_camera, setup).chain())
    .add_systems(
        Update,
        (
            update_simulation_parameters,
            clear_midcourse,
            draw_midcourse,
        )
            .chain(),
    )
    .add_systems(
        Update,
        ApplyDeferred
            .after(update_simulation_parameters)
            .after(clear_midcourse),
    )
    .add_plugins(PanOrbitCameraPlugin)
    .add_plugins(PolylinePlugin)
    .import_event_from_leptos(param_receiver)
    .export_event_to_leptos(event_sender);

    app
}

use bevy::prelude::*;
use bevy_light_2d::prelude::*;
use leptos_bevy_canvas::prelude::*;

use crate::pages::experiments::node_tree::{
    canvas::{
        core::node::Node,
        plugins::{lightning::plugin::LightningPlugin, setup::plugin::SetupPlugin},
    },
    page::NodeTransmission,
};

pub fn init_bevy_app(
    initial_node_tree: Node<String>,
    message_receiver: BevyEventReceiver<NodeTransmission>,
) -> App {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas: Some("#bevy_canvas".into()),
                transparent: true,
                decorations: false,
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }),
        Light2dPlugin,
    ))
    .import_event_from_leptos(message_receiver)
    .insert_resource(ClearColor(Color::NONE))
    .insert_resource(initial_node_tree)
    .add_plugins(SetupPlugin)
    .add_plugins(LightningPlugin);

    app
}

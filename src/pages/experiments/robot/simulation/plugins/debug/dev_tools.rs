use bevy::prelude::*;
use bevy_dev_tools::fps_overlay::FpsOverlayConfig;

#[derive(Resource, Default, Event, Clone)]
pub struct ShowDevTools(pub bool);

pub fn update_dev_tool_state(
    mut events: EventReader<ShowDevTools>,
    mut show_dev_tools: ResMut<ShowDevTools>,
    mut overlay: ResMut<FpsOverlayConfig>,
) {
    for evt in events.read() {
        overlay.enabled = evt.0;
        *show_dev_tools = evt.clone();
    }
}

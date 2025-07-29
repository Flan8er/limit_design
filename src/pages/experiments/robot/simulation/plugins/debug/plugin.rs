use bevy::{prelude::*, text::FontSmoothing};
use bevy_dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};

use crate::pages::experiments::robot::simulation::plugins::debug::{
    axis::{draw_object_coordinates, draw_world_coordinates},
    dev_tools::{update_dev_tool_state, ShowDevTools},
};

pub struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ShowDevTools>()
            .add_plugins(FpsOverlayPlugin {
                config: FpsOverlayConfig {
                    text_config: TextFont {
                        font_size: 25.0,
                        font: default(),
                        font_smoothing: FontSmoothing::default(),
                        ..default()
                    },
                    text_color: Color::WHITE,
                    enabled: false,
                },
            })
            .add_systems(
                Update,
                (
                    update_dev_tool_state,
                    draw_world_coordinates,
                    draw_object_coordinates,
                )
                    .chain(),
            );
    }
}

use bevy::prelude::*;
use bevy_light_2d::prelude::*;

use crate::pages::experiments::node_tree::canvas::plugins::setup::node_tree::construct_node_tree;

pub struct SetupPlugin;
impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_ui, construct_node_tree).chain());
    }
}

fn setup_ui(mut commands: Commands) {
    commands.spawn((
        Camera2d::default(),
        Projection::Orthographic({
            let mut something = OrthographicProjection::default_2d();
            something.scale = 2.0;
            something
        }),
        Transform {
            translation: Vec3::new(0., 200., 0.),
            ..default()
        },
        Light2d {
            ambient_light: AmbientLight2d {
                brightness: 0.1,
                ..default()
            },
        },
    ));
}

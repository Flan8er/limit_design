use bevy::prelude::*;

pub fn draw_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(100., 100.))),
        MeshMaterial3d(materials.add(Color::srgb(0.05, 0.05, 0.05))),
        Transform::from_xyz(0., 0., 0.),
    ));
}

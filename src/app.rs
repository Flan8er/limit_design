use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use leptos::prelude::*;
use leptos_bevy_canvas::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <main class="w-screen h-screen flex items-center justify-center overflow-hidden">
            <BevyCanvas
                init=move || {
                    init_bevy_app()
                }
            />
        </main>
    }
}

fn init_bevy_app() -> App {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            canvas: Some("#bevy_canvas".into()),
            transparent: true,
            fit_canvas_to_parent: true,
            ..default()
        }),
        ..default()
    }))
    .add_systems(Startup, spawn_cube)
    .add_systems(Startup, setup_ui)
    .add_plugins(PanOrbitCameraPlugin);
    app
}

fn setup_ui(mut commands: Commands) {
    // Spawn a camera targeting the primary window
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 1.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        PanOrbitCamera::default(),
    ));

    // Add a light to illuminate the scene
    commands.spawn((PointLight::default(), Transform::from_xyz(4.0, 8.0, 4.0)));
}

fn spawn_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.0, 0.0),
            ..default()
        })),
        Transform::from_xyz(0., 0., 0.),
    ));
}

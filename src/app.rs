use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use leptos::prelude::*;
use leptos_bevy_canvas::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <main class="w-screen h-screen flex items-center justify-center overflow-hidden">
            <div class="w-full h-full">
                <BevyCanvas
                    init=move || {
                        init_bevy_app()
                    }
                />
            </div>
        </main>
    }
}

#[derive(Resource)]
struct SineWaveTime {
    time: f32,
}

#[derive(Component)]
struct SineWave {
    offset: f32,     // horizontal position (x)
    amplitude: f32,  // height of oscillation
    wavelength: f32, // peak-to-peak distance
    speed: f32,      // how fast each particle bobs up/down
}

#[derive(Component)]
struct Direction(Vec3);

fn init_bevy_app() -> App {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            canvas: Some("#bevy_canvas".into()),
            transparent: true,
            decorations: false,
            fit_canvas_to_parent: true,
            ..default()
        }),
        ..default()
    }))
    .insert_resource(ClearColor(Color::NONE))
    .insert_resource(SineWaveTime { time: 0.0 })
    .add_systems(Startup, (setup_ui, spawn_particle))
    .add_systems(Update, (update_sine_time, animate_sine_wave))
    .add_plugins(PanOrbitCameraPlugin);
    app
}

fn setup_ui(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0., 100.0).looking_at(Vec3::ZERO, Vec3::Y),
        PanOrbitCamera::default(),
    ));

    commands.spawn((PointLight::default(), Transform::from_xyz(4.0, 8.0, 4.0)));
}

fn spawn_particle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let amplitude = 12.0;
    let speed = 1.;
    let spacing = 1.;
    let wavelength = 100.0;
    let count = 200;

    for i in 0..count {
        let x = i as f32 * spacing - (count as f32 * spacing / 2.0);
        commands.spawn((
            Mesh3d(meshes.add(Sphere::default())),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.8, 0.0, 0.0),
                ..default()
            })),
            SineWave {
                offset: x,
                amplitude,
                wavelength,
                speed,
            },
            Direction(Vec3::Y), // Direction the sine wave should animate in
        ));
    }
}

fn update_sine_time(time: Res<Time>, mut sine_wave: ResMut<SineWaveTime>) {
    sine_wave.time += time.delta_secs();
}

fn animate_sine_wave(
    sine_wave: Res<SineWaveTime>,
    mut query: Query<(&SineWave, &Direction, &mut Transform)>,
) {
    let t = sine_wave.time;
    for (wave, dir, mut transform) in &mut query {
        let k = std::f32::consts::TAU / wave.wavelength; // 2π / λ
        let y = wave.amplitude * (k * wave.offset + wave.speed * t).sin();
        transform.translation = wave.offset * Vec3::X + y * dir.0;
    }
}

use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use leptos::prelude::*;
use leptos_bevy_canvas::prelude::*;
use leptos_icons::Icon;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <main class="w-dvw h-dvh relative max-w-[1580px] mx-auto">
            // Header
            <div class="glass h-[50px] lg:h-[75px] border-b-[1px] absolute md:sticky top-0 w-full flex items-center z-10 pl-12">
                <div class="w-full h-full flex items-center relative">
                    // Name plate
                    <div class="flex items-center gap-2 absolute left-0 top-1/2 -translate-y-1/2">
                        <div class="bg-accent w-[10px] h-[10px]" />
                        <h3 class="uppercase">"Casey Vaughn"</h3>
                    </div>

                    <div class="max-lg:hidden rounded-full px-6 py-4 absolute left-1/2 top-1/2 -translate-y-1/2 -translate-x-1/2 border flex items-center justify-center gap-8">
                        <h3>About Me</h3>
                        <h3>Experiments</h3> // on hover bubble thats animated to kinda bounce
                        <h3>Skills</h3>
                    </div>
                    // put socials in this one
                    // have another one for navigation
                    // idk maybe flip-flop those two
                    <div class="lg:hidden flex items-center gap-2 absolute right-0 top-1/2 -translate-y-1/2">
                        <button>
                            <Icon icon=icondata::CgMenu width="24px" height="24px"/>
                        </button>
                    </div>
                </div>
            </div>

            // Content window
            <div class="w-full flex h-max relative">
                // Margin display
                <MainMargin class="h-screen sticky left-0 top-[50px] lg:top-[75px]"/>

                // Main content
                <div class="w-full h-[12000px] ">
                </div>

                // Margin display
                <MainMargin class="h-screen sticky right-0 top-[50px] lg:top-[75px]"/>
            </div>
        </main>
    }
}

#[component]
pub fn MainMargin(#[prop(optional)] class: &'static str) -> impl IntoView {
    view! {
        <div class=format!("w-[40px] max-md:hidden text-border border-x border-x-current bg-size-[10px_10px] bg-fixed bg-[repeating-linear-gradient(315deg,currentColor_0px,currentColor_1px,transparent_0px,transparent_10px)] {}", class)/>
    }
}

#[derive(Component)]
struct Particle {
    position: Vec3,
}

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
    .add_systems(Startup, (setup_ui, spawn_particles))
    .add_systems(Update, animate_sine_wave)
    .add_plugins(PanOrbitCameraPlugin);
    app
}

fn setup_ui(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, -125., 25.0).looking_at(Vec3::ZERO, Vec3::Y),
        PanOrbitCamera::default(),
    ));

    commands.spawn((PointLight::default(), Transform::from_xyz(4.0, 8.0, 4.0)));

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 2000.,
    });
}

/// Calculates the unit radius for evenly distribued points inside a circle
fn radius(index: u32, total_points: u32, boundary_points: u32) -> f32 {
    if index > total_points - boundary_points {
        1.0
    } else {
        (index as f32 - 0.5).sqrt()
            / ((total_points as f32 - boundary_points as f32 + 1.0) / 2.0).sqrt()
    }
}

fn spawn_particles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let total_points: u32 = 5_000;
    let distribution: u32 = 1;
    let scale = 80.0;

    let boundary_points = (distribution as f32 * (total_points as f32).sqrt()) as u32;
    let phi = ((5.0_f32).sqrt() + 1.0) / 2.0;
    let golden_angle = std::f32::consts::TAU * (1.0 - 1.0 / phi);

    let mesh = meshes.add(Sphere::default());
    let material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.208, 0.612, 1.),
        ..default()
    });

    for i in 0..total_points {
        let r = radius(i, total_points, boundary_points) * scale;
        let theta = i as f32 * golden_angle;

        let pos = Vec3::new(r * theta.cos(), r * theta.sin(), 0.0);

        commands.spawn((
            Mesh3d(mesh.clone()),
            MeshMaterial3d(material.clone()),
            Transform::from_translation(pos).with_scale(Vec3::splat(0.5)),
            Particle { position: pos },
        ));
    }
}

fn animate_sine_wave(time: Res<Time>, mut query: Query<(&Particle, &mut Transform)>) {
    let t = time.elapsed_secs();

    let amplitude = 2.0; // wave height
    let wavelength = 30.0; // peak-to-peak distance
    let omega = 0.5; // wave propagation speed

    let k = std::f32::consts::TAU / wavelength; // spatial frequency
    for (particle, mut transform) in &mut query {
        let x = particle.position.x;
        let y = particle.position.y;
        let r = (x * x + y * y).sqrt();

        let phase = k * r + omega * t;
        let z = amplitude * phase.sin();

        transform.translation = Vec3::new(x, y, z);
    }
}

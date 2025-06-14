use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use leptos::{
    ev,
    prelude::{Update, *},
};
use leptos_bevy_canvas::prelude::*;
use leptos_icons::Icon;
use leptos_meta::provide_meta_context;
use leptos_router::{
    components::{Outlet, ParentRoute, Route, Router, Routes},
    hooks::use_navigate,
    path, MatchNestedRoutes,
};
use web_sys::{wasm_bindgen::JsCast, MouseEvent};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router>
            <Routes fallback=|| "Route not found...">
                <MainPageRoutes/>
            </Routes>
        </Router>
    }
}

#[component(transparent)]
pub fn MainPageRoutes() -> impl MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=path!("") view=MainPageContainer>
            <Route path=path!("/") view=|| view!{<div>main page</div>} />
            <Route path=path!("/aboutme") view=|| view!{<div>about me</div>} />
            <Route path=path!("/experiments") view=|| view!{<div>experiments</div>} />
            <Route path=path!("/catalog") view=|| view!{<div>catalog</div>} />
            <Route path=path!("/skills") view=|| view!{<div>skills</div>} />
        </ParentRoute>
    }
    .into_inner()
}

#[component]
pub fn MainPageContainer() -> impl IntoView {
    let navigate = use_navigate();
    let navigate_home = move |_| navigate("/", Default::default());

    view! {
        <main class="w-dvw h-dvh relative max-w-[1580px] mx-auto bg-background">
            // Header
            <div class="glass h-[50px] md:h-[75px] border-b-[1px] absolute md:sticky top-0 w-full flex items-center z-10 relative">
                <div class="w-full h-full flex items-center">
                    // Name plate
                    <div
                        class="flex items-center gap-2 absolute left-8 md:left-12 top-1/2 -translate-y-1/2 cursor-default"
                        on:click=navigate_home
                    >
                        <div class="bg-accent w-[10px] h-[10px]" />
                        <h3 class="uppercase text-fluid-h3">"Casey Vaughn"</h3>
                    </div>

                    <div class="max-md:hidden rounded-full px-6 py-4 absolute left-1/2 top-1/2 -translate-y-1/2 -translate-x-1/2 border flex items-center justify-center gap-12">
                        <NavItem name="About Me" absolute_route="/aboutme"/>
                        <NavItem name="Experiments" absolute_route="/experiments"/>
                        <NavItem name="Catalog" absolute_route="/catalog"/>
                        <NavItem name="Skills" absolute_route="/skills"/>
                    </div>

                    <div class="md:hidden flex items-center gap-2 absolute right-0 top-1/2 -translate-y-1/2 pr-[8px]">
                        <Menu/>
                        // <button class="rounded-md">
                        //     <Icon icon=icondata::CgMenu width="24px" height="24px"/>
                        // </button>
                    </div>
                </div>
            </div>

            // Content window
            <div class="w-full flex h-max relative">
                // Margin display
                <MainMargin class="left-0"/>

                // Main content
                <div class="w-full h-[12000px] ">
                <Outlet/>
                    <div class="w-full h-[75px] bg-red-600"/>
                </div>

                // Margin display
                <MainMargin class="right-0"/>
            </div>
        </main>
    }
}

#[component]
fn Menu() -> impl IntoView {
    // let on_select = move |_| {};
    let open = RwSignal::new(false);
    let menu_ref: NodeRef<leptos::html::Div> = NodeRef::new();

    // Close menu on outside click
    let _ = window_event_listener(ev::click, move |ev: MouseEvent| {
        if let Some(target) = ev.target() {
            if let Some(menu_el) = menu_ref.get() {
                if let Ok(target_node) = target.dyn_into::<web_sys::Node>() {
                    if !menu_el.contains(Some(&target_node)) {
                        open.set(false);
                    }
                }
            }
        }
    });
    let _ = window_event_listener(ev::touchstart, move |ev: web_sys::TouchEvent| {
        if let Some(target) = ev.target() {
            if let Some(menu_el) = menu_ref.get() {
                if let Ok(target_node) = target.dyn_into::<web_sys::Node>() {
                    if !menu_el.contains(Some(&target_node)) {
                        open.set(false);
                    }
                }
            }
        }
    });

    view! {
        <div class="relative" node_ref=menu_ref>
            <button
                class="rounded-md"
                on:click=move |_| open.update(|v| *v = !*v)
            >
                <Icon icon=icondata::CgMenu width="24px" height="24px"/>
            </button>

            {move || open.get().then(|| view! {
                <div class="absolute right-0 mt-2 w-48 bg-white shadow-lg rounded-md border z-50">
                    <button
                        class="flex items-center w-full px-4 py-2 hover:bg-gray-100 rounded-md"
                        on:click=move |_| {
                            // on_select.call("facebook".to_string());
                            open.set(false);
                        }
                    >
                        <Icon icon=icondata::AiFacebookOutlined width="20px" height="20px"/>
                        "Facebook"
                    </button>

                    <button
                        class="flex items-center w-full px-4 py-2 text-gray-400 cursor-not-allowed rounded-md"
                        disabled
                    >
                        <Icon icon=icondata::AiTwitterOutlined width="20px" height="20px"/>
                        "Twitter"
                    </button>
                </div>
            })}
        </div>
    }
}

#[component]
fn NavItem(name: &'static str, absolute_route: &'static str) -> impl IntoView {
    let navigate = use_navigate();

    let base_style = "\
        position: absolute;\
        top: -14px;\
        bottom: -14px;\
        left: -22px;\
        right: -22px;\
        background: rgba(255, 255, 255, 0.2);\
        border: 1px solid rgba(255, 255, 255, 0.5);\
        border-radius: 9999px;\
        z-index: 0;\
        transition: opacity 0.25s ease-in-out;\
    ";

    let item_style = RwSignal::new(format!("{base_style} opacity: 0;"));

    let navigate_to = move |_| navigate(absolute_route, Default::default());

    view! {
        <h3
            class="relative cursor-default"
        >
            <span
                style=move || item_style.get()
                on:mouseover=move |_| {
                    item_style.set(format!("{base_style} opacity: 1;"));
                }
                on:mouseleave=move |_| {
                    item_style.set(format!("{base_style} opacity: 0;"));
                }
                on:click=navigate_to
            ></span>
            {name}
        </h3>
    }
}

#[component]
fn MainMargin(#[prop(optional)] class: &'static str) -> impl IntoView {
    view! {
        <div class=format!("w-[40px] max-md:hidden text-border border-x border-x-current bg-size-[10px_10px] bg-fixed bg-[repeating-linear-gradient(315deg,currentColor_0px,currentColor_1px,transparent_0px,transparent_10px)] h-screen sticky top-[75px] {}", class)/>
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

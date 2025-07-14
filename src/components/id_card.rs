use leptos::{html::Div, prelude::*};
use leptos_use::{use_window_size, UseWindowSizeReturn};
use leptos_verlet::prelude::*;
use web_sys::wasm_bindgen::JsCast;

#[component]
pub fn SpawnIdCard() -> impl IntoView {
    let simulation_container = NodeRef::<Div>::new();
    let spawn_request = expect_context::<SpawnSender>();

    const RECT_WIDTH: f32 = 0.75;
    const RECT_HEIGHT: f32 = 1.;

    const POINT_SIZE: f32 = 0.015;
    const STICK_SIZE: f32 = 0.015;

    // Mesh & Materials
    let point_mesh = MeshType::Sphere;
    let stick_mesh = MeshType::Cuboid;
    let rope_material = MaterialType::Color([0., 0., 0., 1.0]);
    let badge_material = MaterialType::Color([1.0, 1.0, 1.0, 0.0]);

    // Define rope connections
    let rope_1 = Vec3::new(0.0, 3.5, 0.0);
    let rope_2 = Vec3::new(0.0, 3.25, 0.0);
    let rope_3 = Vec3::new(0.0, 3., 0.0);
    let rope_4 = Vec3::new(0.0, 2.75, 0.0);
    let rope_5 = Vec3::new(0.0, 2.5, 0.0);
    let rope_6 = Vec3::new(0.0, 2.25, 0.0);

    // Badge properties
    let half_w = RECT_WIDTH / 2.0;
    let bottom_y = 1.0;
    let top_y = bottom_y + RECT_HEIGHT;
    let center_y = top_y + 0.05;

    // Define badge connections
    let top_left = Vec3::new(-half_w, top_y, 0.0);
    let top_right = Vec3::new(half_w, top_y, 0.0);
    let top_center = Vec3::new(0.0, center_y, 0.0);
    let bottom_left = Vec3::new(-half_w, top_y - RECT_HEIGHT, 0.0);
    let bottom_right = Vec3::new(half_w, top_y - RECT_HEIGHT, 0.0);

    // Spawning a custom mesh when requested
    let centroid: Vec3 = (top_right + bottom_left + bottom_right) / 3.;

    model_loader_with_options(
        "/static/id_badge.glb",
        "id_badge.glb",
        0,
        Some(Vec3::new(0., centroid.y * 3. / 2. + 0.05, 0.)),
        Some(Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2)),
        Some(0.6),
    );

    let make_rope =
        |pos: Vec3, vel: Vec3, conns: Vec<Vec3>, fixed: bool, attachment: Option<String>| {
            let n = conns.len();
            let custom_material = match rope_material.clone() {
                MaterialType::Color(mut color) => {
                    color[3] = 1.0;
                    MaterialType::Color(color)
                }
            };
            SpawnNode {
                point: Point::new(pos, vel, fixed),
                connection: Some(conns),
                point_material: custom_material.clone(),
                connection_material: Some(vec![rope_material.clone(); n]),
                point_mesh: point_mesh.clone(),
                connection_mesh: Some(vec![stick_mesh.clone(); n]),
                point_size: POINT_SIZE,
                connection_size: Some(vec![STICK_SIZE; n]),
                attachment,
                point_scale: Vec3::new(1., 1., 1.),
                connection_scale: Some(vec![Vec3::new(1., 1., 1.); n]),
            }
        };
    let make_badge = |pos: Vec3,
                      vel: Vec3,
                      conns: Vec<Vec3>,
                      fixed: bool,
                      extra: f32,
                      attachment: Option<String>,
                      external_forces: Vec3| {
        let n = conns.len();
        SpawnNode {
            point: Point::new_with_options(pos, vel, fixed, external_forces),
            connection: Some(conns),
            point_material: badge_material.clone(),
            connection_material: Some(vec![badge_material.clone(); n]),
            point_mesh: point_mesh.clone(),
            connection_mesh: Some(vec![stick_mesh.clone(); n]),
            point_size: POINT_SIZE + extra,
            connection_size: Some(vec![STICK_SIZE + extra; n]),
            attachment,
            connection_scale: Some(vec![Vec3::new(1., 1., 1.); n]),
            ..default()
        }
    };

    let mesh_network = vec![
        make_rope(rope_1, rope_1, vec![rope_2], true, None),
        make_rope(rope_2, rope_2, vec![rope_1, rope_3], false, None),
        make_rope(rope_3, rope_3, vec![rope_2, rope_4], false, None),
        make_rope(rope_4, rope_4, vec![rope_3, rope_5], false, None),
        make_rope(rope_5, rope_5, vec![rope_4, rope_6], false, None),
        make_rope(rope_6, rope_6, vec![rope_5, top_center], false, None),
        make_badge(
            top_center,
            top_center,
            vec![top_left, top_right, rope_6],
            false,
            0.0,
            None,
            Vec3::ZERO,
        ),
        make_badge(
            top_right,
            top_right,
            vec![top_center, bottom_right, bottom_left, top_left],
            false,
            0.0,
            Some(String::from("id_badge.glb")),
            Vec3::new(0.1, 0., 0.),
        ),
        make_badge(
            top_left,
            top_left,
            vec![top_center, bottom_left, bottom_right, top_right],
            false,
            0.0,
            None,
            Vec3::new(-0.1, 0., 0.),
        ),
        make_badge(
            bottom_left,
            bottom_left,
            vec![top_right, top_left, bottom_right],
            false,
            0.0,
            Some(String::from("id_badge.glb")),
            Vec3::new(-0.1, 0., 0.),
        ),
        make_badge(
            bottom_right,
            bottom_right,
            vec![top_right, top_left, bottom_left],
            false,
            0.0,
            Some(String::from("id_badge.glb")),
            Vec3::new(0.1, 0., 0.),
        ),
    ];

    let _ = spawn_request.send(SpawnRequest::new(mesh_network.clone()));

    view! {
        <div
            node_ref=simulation_container
            class="w-full h-full relative"
        >
            <VerletCanvas parent_element=simulation_container/>

            <MouseMonitor/>
        </div>
    }
}

#[component]
pub fn MouseMonitor() -> impl IntoView {
    let event_sender = expect_context::<ModificationEventSender>();
    let UseWindowSizeReturn { width, height } = use_window_size();

    let left_click_action = {
        let sender = event_sender.clone();
        move |x: f64, y: f64| {
            let container_width = width.get_untracked();
            let container_height = height.get_untracked();
            let _ = sender.send(ModifyEventType::Left(RelativeWindowPosition {
                event_x: x as f32,
                event_y: y as f32,
                container_h: container_height as f32,
                container_w: container_width as f32,
            }));
        }
    };

    let move_action = {
        let sender = event_sender.clone();
        move |x: f64, y: f64| {
            let container_width = width.get_untracked();
            let container_height = height.get_untracked();
            let _ = sender.send(ModifyEventType::Move(RelativeWindowPosition {
                event_x: x as f32,
                event_y: y as f32,
                container_h: container_height as f32,
                container_w: container_width as f32,
            }));
        }
    };
    let release_action = {
        let sender = event_sender.clone();
        move |x: f64, y: f64| {
            let container_width = width.get_untracked();
            let container_height = height.get_untracked();
            let _ = sender.send(ModifyEventType::Release(RelativeWindowPosition {
                event_x: x as f32,
                event_y: y as f32,
                container_h: container_height as f32,
                container_w: container_width as f32,
            }));
        }
    };

    view! {
        <div
            class="absolute inset-0 z-[10] cursor-default"
            on:mousedown=move |ev| {
                ev.prevent_default();
                if let Some((x, y)) = target_mouse_position(&ev) {
                    match ev.button() {
                        0 => left_click_action(x, y),
                        _ => return
                    }
                }
            }
            on:mouseup={
                let release_action = release_action.clone();
                move |ev| {
                ev.prevent_default();
                if let Some((x, y)) = target_mouse_position(&ev) {
                    release_action(x, y);
                }
            }}
            on:contextmenu=move |ev| {
                ev.prevent_default();
            }
            on:mousemove=move |ev| {
                ev.prevent_default();
                if let Some((x, y)) = target_mouse_position(&ev) {
                    move_action(x, y);
                }
            }
            on:mouseleave={
                let release_action = release_action.clone();
                move |ev| {
                ev.prevent_default();
                if let Some((x, y)) = target_mouse_position(&ev) {
                    release_action(x, y);
                }
            }}
        ></div>
    }
}

fn target_mouse_position(ev: &web_sys::MouseEvent) -> Option<(f64, f64)> {
    let x: f64;
    let y: f64;

    if let Some(target) = ev
        .target()
        .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
    {
        let rect = target.get_bounding_client_rect();
        x = ev.client_x() as f64 - rect.left();
        y = ev.client_y() as f64 - rect.top();
    } else {
        return None;
    }

    Some((x, y))
}

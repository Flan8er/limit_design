use bevy::{color::palettes::css::RED, prelude::*};
use bevy_light_2d::light::PointLight2d;

use crate::pages::experiments::node_tree::{
    canvas::{core::node::Node, plugins::setup::node_tree::calculate_node_position},
    page::NodeTransmission,
};

#[derive(Component)]
struct PathMover {
    path: Vec<Vec3>, // absolute world‐space waypoints
    speed: f32,      // units per second
    seg_idx: usize,  // current segment index
    seg_pos: f32,    // distance traveled along this segment
}

/// Recursively search `node` for `target`; if found, return the sequence of
/// `(child_index, sibling_count)` from _root_ down to that node.
fn find_path(node: &Node<String>, target: &str) -> Option<Vec<(usize, usize)>> {
    if node.value == target {
        return Some(vec![]);
    }
    let count = node.children.len();
    for (i, child) in node.children.iter().enumerate() {
        if let Some(mut sub) = find_path(child, target) {
            sub.insert(0, (i, count));
            return Some(sub);
        }
    }
    None
}

/// Turn two “child_history” paths into a single Vec<Vec2> of waypoints.
/// Walk _up_ from `from_hist` back to the LCA, then _down_ into `to_hist`.
fn build_node_path(from_hist: Vec<(usize, usize)>, to_hist: Vec<(usize, usize)>) -> Vec<Vec2> {
    // 1) prefix‐positions from root→from (0..=len)
    let prefixes_from: Vec<Vec2> = (0..=from_hist.len())
        .map(|d| calculate_node_position(from_hist[..d].to_vec()))
        .collect();
    // 2) same for root→to
    let prefixes_to: Vec<Vec2> = (0..=to_hist.len())
        .map(|d| calculate_node_position(to_hist[..d].to_vec()))
        .collect();
    // 3) find LCA depth: first index where the histories differ
    let max_common = from_hist.len().min(to_hist.len());
    let mut lca = 0;
    while lca < max_common && from_hist[lca] == to_hist[lca] {
        lca += 1;
    }
    // 4) assemble the full path:
    //    - from prefixes_from[len] down to prefixes_from[lca]
    //    - then prefixes_to[lca+1..]
    let mut path = Vec::new();
    for idx in (lca..prefixes_from.len()).rev() {
        path.push(prefixes_from[idx]);
    }
    for idx in (lca + 1)..prefixes_to.len() {
        path.push(prefixes_to[idx]);
    }
    path
}

pub struct LightningPlugin;
impl Plugin for LightningPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_lightning, update_path_movers).chain());
    }
}

/// On a NodeTransmission event, compute the path and spawn a light + mover.
fn spawn_lightning(
    mut evr: EventReader<NodeTransmission>,
    mut commands: Commands,
    node_tree: Res<Node<String>>,
) {
    for ev in evr.read() {
        // 1) find the raw histories
        let mut from_hist = match find_path(&node_tree, &ev.from) {
            Some(h) => h,
            None => continue,
        };
        let mut to_hist = match find_path(&node_tree, &ev.to) {
            Some(h) => h,
            None => continue,
        };

        // 2) if it’s a leaf, tack on the dummy (0,1) step
        if let Some(leaf) = get_node_by_hist(&node_tree, &from_hist) {
            if leaf.children.is_empty() {
                from_hist.push((0, 1));
            }
        }
        if let Some(leaf) = get_node_by_hist(&node_tree, &to_hist) {
            if leaf.children.is_empty() {
                to_hist.push((0, 1));
            }
        }

        // 3) build your 2D polyline & spawn the mover as before
        let pts2 = build_node_path(from_hist, to_hist);
        let pts3: Vec<Vec3> = pts2.into_iter().map(|p| p.extend(0.0)).collect();
        let clean = simplify_path(pts3);
        if clean.len() < 2 {
            continue;
        }
        // 4. spawn the light at the first waypoint + our PathMover
        commands.spawn((
            PointLight2d {
                color: Color::Srgba(RED),
                radius: 48.0,
                intensity: 1.0,
                falloff: 14.0,
                ..default()
            },
            Transform::from_translation(clean[0]),
            PathMover {
                path: clean,
                speed: 1250.0, // m/s
                seg_idx: 0,
                seg_pos: 0.0,
            },
        ));
    }
}

/// Each frame, advance every PathMover along its segment(s).
fn update_path_movers(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut PathMover)>,
) {
    let dt = time.delta_secs();
    for (ent, mut tr, mut mover) in query.iter_mut() {
        let mut remainder = mover.speed * dt;
        // consume `remainder` over one or more segments
        while remainder > 0.0 {
            let idx = mover.seg_idx;
            // if we’re at or past the last point, despawn
            if idx + 1 >= mover.path.len() {
                commands.entity(ent).despawn();
                break;
            }
            let start = mover.path[idx];
            let end = mover.path[idx + 1];
            let seg = end - start;
            let seg_len = seg.length();
            let to_go = seg_len - mover.seg_pos;
            if remainder < to_go {
                // advance partway along this segment
                mover.seg_pos += remainder;
                let dir = seg / seg_len; // normalize
                tr.translation = start + dir * mover.seg_pos;
                remainder = 0.0;
            } else {
                // finish this segment, move to next
                remainder -= to_go;
                mover.seg_idx += 1;
                mover.seg_pos = 0.0;
                tr.translation = end;
            }
        }
    }
}

fn get_node_by_hist<'a>(
    mut node: &'a Node<String>,
    hist: &[(usize, usize)],
) -> Option<&'a Node<String>> {
    for (i, _) in hist {
        node = node.children.get(*i)?;
    }
    Some(node)
}

/// Remove any immediate A→B→A loops from a Vec of Vec3 waypoints.
fn simplify_path(path: Vec<Vec3>) -> Vec<Vec3> {
    if path.len() < 3 {
        return path;
    }
    let mut out = Vec::with_capacity(path.len());
    out.push(path[0]);

    for i in 1..path.len() - 1 {
        let prev = out.last().unwrap();
        let curr = path[i];
        let next = path[i + 1];

        // directions, avoiding zero‑length segments
        let v1 = (curr - *prev).try_normalize().unwrap_or_default();
        let v2 = (next - curr).try_normalize().unwrap_or_default();

        // dot≈-1 means a U‑turn (180°). tweak the threshold as needed.
        if v1.dot(v2) < -0.98 {
            // skip `curr` → collapsing A→B→C into A→C
            continue;
        }

        out.push(curr);
    }

    // always keep the final point
    out.push(*path.last().unwrap());
    out
}

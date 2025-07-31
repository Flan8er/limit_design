use leptos::{html::Div, prelude::*};
use web_sys::{wasm_bindgen::JsCast, Element};

/// Calculates animation progress (0.0–1.0) based on scroll position and thresholds.
/// Returns `None` if the element cannot be resolved.
pub fn compute_scroll_animation_progress(
    el: &NodeRef<Div>,
    parent_scroll: f64,
    start_threshold: f64,
    end_threshold: f64,
) -> Option<f64> {
    let el_node = el.get()?;
    let el: Element = el_node.dyn_into().ok()?;

    let el_offset_top = el.get_bounding_client_rect().top() + parent_scroll;
    let el_height = el.scroll_height() as f64;
    let viewport_height = window()
        .inner_height()
        .ok()
        .and_then(|v| v.as_f64())
        .unwrap_or(1.0);

    let distance_from_top = el_offset_top - parent_scroll;
    let visible_ratio = ((viewport_height - distance_from_top) / el_height).clamp(0.0, 1.0);
    let anim_progress =
        ((visible_ratio - start_threshold) / (end_threshold - start_threshold)).clamp(0.0, 1.0);

    Some(anim_progress)
}

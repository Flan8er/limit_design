use leptos::prelude::*;

use leptos::{html::Div, prelude::*};
use leptos_use::use_window_scroll;
use strum::IntoEnumIterator;

use crate::pages::about::{
    components::timeline::Timeline as TimelineEnum, pages::timeline::page::Timeline,
};

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <ScrollProvider>
            // <Timeline/>

            {
                TimelineEnum::iter()
                    .map(|section| section.render())
                    .collect_view()
            }
        </ScrollProvider>
    }
}

#[component]
pub fn ScrollProvider(children: Children) -> impl IntoView {
    let active_timestep = RwSignal::new(TimelineEnum::default());
    provide_context(active_timestep.read_only());

    let el = NodeRef::<Div>::new();

    let timeline_variants = TimelineEnum::iter().collect::<Vec<_>>();
    let num_sections = timeline_variants.len();

    let (_, y) = use_window_scroll();

    provide_context(y);

    Effect::new(move |_| {
        let scroll_top = y.get();
        let viewport_height = window()
            .inner_height()
            .ok()
            .and_then(|v| v.as_f64())
            .unwrap_or(1.0);

        let page_index = (((scroll_top + (viewport_height / 3.0)) / viewport_height) as usize)
            .min(num_sections - 1);
        active_timestep.set(timeline_variants[page_index]);
    });

    view! {
        <div
            node_ref=el
            class="w-full h-full overflow-y-auto overflow-x-hidden flex flex-col"
        >
            {children()}
        </div>
    }
}

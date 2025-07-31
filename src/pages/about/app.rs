use leptos::{html::Div, prelude::*};
use leptos_use::{use_scroll, UseScrollReturn};
use strum::IntoEnumIterator;

use crate::pages::about::{components::timeline::Timeline, pages::timeline::page::Timeline};

#[component]
pub fn PortfolioFrame() -> impl IntoView {
    view! {
        <main
            class="w-screen h-screen"
        >
            <ScrollProvider>
                <Timeline/>

                {
                    Timeline::iter()
                        .map(|section| section.render())
                        .collect_view()
                }
            </ScrollProvider>
        </main>
    }
}

#[component]
pub fn ScrollProvider(children: Children) -> impl IntoView {
    let active_timestep = RwSignal::new(Timeline::default());
    provide_context(active_timestep.read_only());

    let el = NodeRef::<Div>::new();

    let timeline_variants = Timeline::iter().collect::<Vec<_>>();
    let num_sections = timeline_variants.len();

    #[allow(unused)]
    let UseScrollReturn {
        x,
        y,
        set_x,
        set_y,
        is_scrolling,
        arrived_state,
        directions,
        ..
    } = use_scroll(el);

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
            class="w-full h-full overflow-scroll overflow-x-hidden"
        >
            {children()}
        </div>
    }
}

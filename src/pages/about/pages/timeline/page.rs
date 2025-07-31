use leptos::prelude::*;
use leptos_use::{use_window_size, UseWindowSizeReturn};
use strum::IntoEnumIterator;

use crate::pages::about::components::timeline::Timeline;

#[component]
pub fn Timeline() -> impl IntoView {
    let sections: Vec<Timeline> = Timeline::iter().collect();
    let num_sections = sections.len();

    let active_timestep = expect_context::<ReadSignal<Timeline>>();

    #[allow(unused)]
    let UseWindowSizeReturn { width, height } = use_window_size();

    view! {
        <div class="fixed top-0 left-0 h-screen w-20 flex flex-col items-center justify-center z-[100] max-md:hidden">
            <div class="w-1 h-[33.333%] bg-white/20 backdrop-blur-sm rounded-full relative shadow-md border border-white/30">
                { move || {
                    let dot_height_px = 16.0;
                    let timeline_height_px = height.get() as f32 / 3.0;

                    sections.iter().enumerate().map(|(i, section)| {
                        let is_active = active_timestep.get() == section.clone();

                        let last_dot_top_percent: f32 = (((timeline_height_px - dot_height_px) / timeline_height_px) * 100.0) as f32;
                        let position = (i as f32) / ((num_sections - 1) as f32) * last_dot_top_percent;

                        view! {
                            <div
                                class="absolute left-1/2 -translate-x-1/2"
                                style=format!("top: {:.4}%;", position)
                            >
                                <div
                                    class=move || {
                                        let base_class = "w-4 h-4 rounded-full border-2 transition-all duration-500 ease-out";

                                        if is_active {
                                            format!("{base_class} bg-white border-white scale-125 shadow-lg")
                                        } else {
                                            format!("{base_class} bg-transparent border-white scale-100 shadow-none")
                                        }

                                    }
                                >
                                </div>
                            </div>

                            <div
                                class="absolute left-full ml-4 text-white text-xs whitespace-nowrap text-center"
                                style=format!("top: {:.4}%;", position) //  transform: translateY(-50%);
                            >
                                {section.into_string()}
                            </div>
                        }

                    }).collect_view()
                }}
            </div>
        </div>
    }
}

use leptos::{html::Div, prelude::*};

use crate::pages::about::components::{page::Page, scroll::compute_scroll_animation_progress};

#[component]
pub fn AboutMe() -> impl IntoView {
    let parent_scroll = expect_context::<Signal<f64>>();
    let el = NodeRef::<Div>::new();
    let left_style = RwSignal::new(String::new());
    let right_style = RwSignal::new(String::new());

    Effect::new(move |_| {
        let scroll_y = parent_scroll.get();
        let Some(progress) = compute_scroll_animation_progress(&el, scroll_y, 0.2, 0.6) else {
            return;
        };

        let translate_left = (1.0 - progress) * -100.0;
        let translate_right = (1.0 - progress) * 100.0;
        let opacity = progress;

        left_style.set(format!(
            "transform: translateX({}%) scale(1); opacity: {}; transition: all 0.4s ease-out;",
            translate_left, opacity
        ));
        right_style.set(format!(
            "transform: translateX({}%) scale(1); opacity: {}; transition: all 0.4s ease-out;",
            translate_right, opacity
        ));
    });

    view! {
        <Page>
            <div
                node_ref=el
                class="w-full h-full flex items-center justify-center relative bg-gradient-to-b from-purple-600/10 to-[rgba(183,45,177,0.1)] py-6"
            >
                <div class="max-w-6xl w-full px-8 grid lg:grid-cols-2 gap-12 items-center z-10">
                    <div class="text-primary-text-muted" style=move || left_style.get()>
                        <h2 class="text-5xl font-bold mb-8 bg-gradient-to-r from-purple-400 to-pink-400 bg-clip-text text-transparent">
                            "About Me"
                        </h2>

                        <p class="text-lg leading-relaxed mb-6">
                            "I’m a full-stack developer who cares a lot about making things that are both functional and thoughtfully designed. I’ve been at this for about 4 years now — building clean interfaces, shipping real products, and fine-tuning every pixel and line of code along the way."
                        </p>

                        <p class="text-lg leading-relaxed">
                            "I got into this by just being curious. I’ve always wanted to understand how these beautiful apps I was seeing were made, and that’s driven me to push the bounds of what I knew how to make, blending design and engineering to get there. For me, it’s not just about these apps solving problems — it’s about building something that feels good to use."
                        </p>

                    </div>

                    <div class="flex justify-center items-center" style=move || right_style.get()>
                        <div class="relative w-80 h-80 rounded-full p-5 ">
                            <div class="absolute inset-0 rounded-full bg-gradient-to-br from-purple-500 to-pink-500 opacity-20 z-0"/>

                            <img
                                src="static/CaseyVaughnUT-148.jpg"
                                alt=""
                                class="w-full h-full object-cover rounded-full relative z-10"
                            />
                        </div>
                    </div>
                </div>
            </div>
        </Page>
    }
}

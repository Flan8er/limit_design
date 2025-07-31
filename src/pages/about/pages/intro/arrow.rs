use leptos::prelude::*;
use leptos_icons::Icon;
use leptos_use::use_interval_fn;

use crate::pages::about::components::translation::{
    translate_object, TranslationConfig, TranslationState,
};

#[component]
pub fn AnimatedChevron() -> impl IntoView {
    // Chevron animation
    let chevron_translation = RwSignal::new(TranslationState {
        position: 0.0,
        velocity: 0.0,
        moving_up: true,
    });
    let chevron_translation_config = TranslationConfig {
        target_speed: 0.5,
        easing_factor: 0.1,
        max_translation: 10.0,
        threshold: 0.1,
    };
    use_interval_fn(
        move || {
            chevron_translation.update(|s| {
                translate_object(s, &chevron_translation_config);
            });
        },
        16,
    );

    view! {
        <div
            class="flex justify-center absolute bottom-8"
            style=move || {
                format!("transform: translateY({:.2}px)", chevron_translation.get().position)
            }
        >
            <Icon icon=icondata::ChChevronDown width="32px" height="32px" />
        </div>
    }
}

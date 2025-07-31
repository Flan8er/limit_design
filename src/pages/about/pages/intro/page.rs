use leptos::prelude::*;

use crate::pages::about::{
    components::page::Page,
    pages::intro::{arrow::AnimatedChevron, socials::SocialIcons},
};

#[component]
pub fn Intro() -> impl IntoView {
    view! {
        <Page>
            <div
                class="w-full h-full flex flex-col items-center justify-start relative pt-12"
            >
                // <BevyWaveform/>

                <h1 class="text-6xl md:text-8xl font-bold">"Casey Vaughn"</h1>

                <p class="text-xl md:text-2xl text-gray-300 mt-8">"Full Stack Developer & UI/UX Designer"</p>

                <SocialIcons/>

                <AnimatedChevron/>
            </div>
        </Page>
    }
}

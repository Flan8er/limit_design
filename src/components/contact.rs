use leptos::prelude::*;

use crate::components::ui::AnimatedLinkButton;

#[component]
pub fn ContactSection() -> impl IntoView {
    view! {
        <div class="w-full py-12 flex flex-col items-center justify-center bg-tertiary-background gap-6 text-secondary-text relative z-[0]">
            <div class="absolute inset-0 grid-background bg-grid-20 border-none z-[-1]"/>

            <h2>"Get in touch"</h2>
            <h3 class="mx-4 md:mx-0 md:w-[60%] md:max-w-[60%] lg:w-[30%] lg:max-w-[30%] text-center font-normal">"If you have a project idea that you want to bring to life, I can deliver a tailor-made, AI free, application suited perfectly to your needs."</h3>

            <AnimatedLinkButton link="mailto:casey.vaughn9@aol.com" title="Get in touch" class="mr-auto"/>
        </div>
    }
}

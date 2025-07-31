use leptos::prelude::*;

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <div class="w-full h-full max-h-full flex">
            <iframe class="m-0 p-0 w-full h-full z-[0]" src="/portfolio-frame"/>
        </div>
    }
}

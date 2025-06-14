use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="w-full flex flex-col p-4 md:flex-row gap-4 ">
            <div class="w-full h-[75px] bg-red-600"/>
            <div class="w-full h-[75px] bg-red-600"/>
        </div>
    }
}

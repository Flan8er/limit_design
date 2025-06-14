use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="w-full flex flex-col p-4 md:flex-row gap-4 min-h-[12000px] h-[12000px] items-end justify-end">
            <div class="w-full h-[75px] bg-red-600 "/>
            <div class="w-full h-[75px] bg-red-600"/>
        </div>
    }
}

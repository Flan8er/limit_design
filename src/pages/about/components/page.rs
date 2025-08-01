use leptos::prelude::*;

#[component]
pub fn Page(children: Children) -> impl IntoView {
    view! {
        <div class="h-fit min-h-[calc(100vh-50px)] md:min-h-[calc(100vh-75px)] w-full flex">
            {children()}
        </div>
    }
}

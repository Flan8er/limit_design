use leptos::prelude::*;

#[component]
pub fn Page(children: Children) -> impl IntoView {
    view! {
        <div class="h-screen w-screen">
            {children()}
        </div>
    }
}

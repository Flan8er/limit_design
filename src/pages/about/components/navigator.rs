use leptos::prelude::*;

#[component]
pub fn Navigator(index: RwSignal<u8>, length: usize) -> impl IntoView {
    view! {
        <div class="flex gap-4 px-6 py-4 rounded-full backdrop-blur-md bg-white/5 border border-white/10 shadow-md">
            <For
                each=move || 0..length
                key=|&i| i
                children=move |i| {

                    view! {
                        <button
                            class=move || "h-[12px] rounded-full bg-white/70 transition-bounce p-0"
                            style=move || format!(
                                "width: {}px;",
                                if index.get() == i as u8 { 40 } else { 12 }
                            )
                            on:click=move |_| index.set(i as u8)
                        />
                    }
                }
            />
        </div>
    }
}

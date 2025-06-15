use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="w-full flex flex-col p-4 gap-4 items-end justify-end">
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4 w-full">
                <div class="w-full h-full flex flex-col items-center justify-center relative">
                    <div class="absolute inset-0 grid-background bg-grid-20"/>
                    <span>hi</span>
                    <span>hi</span>
                    <span>hi</span>
                    <span>hi</span>
                    <span>hi</span>
                    <span>hi</span>
                </div>

                <div class="w-full h-[1000px] min-h-[1000px]  flex flex-col">
                    <span>there</span>
                    <span>there</span>
                    <span>there</span>
                    <span>there</span>
                    <span>there</span>
                    <span>there</span>
                    <span>there</span>
                    <span>there</span>
                    <span>there</span>
                    <span>there</span>
                    <span>there</span>
                </div>
            </div>
        </div>
    }
}

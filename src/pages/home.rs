use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="w-full flex flex-col p-4 gap-4 items-end justify-end">
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4 w-full">
                <GridBoxContainer>
                    <div class="w-full h-full flex flex-col items-center justify-center">
                        <span>hi</span>
                        <span>hi</span>
                        <span>hi</span>
                        <span>hi</span>
                        <span>hi</span>
                        <span>hi</span>
                    </div>
                </GridBoxContainer>
                <div class="w-full h-full bg-red-600 flex flex-col">
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

#[component]
pub fn GridBoxContainer(children: Children) -> impl IntoView {
    view! {
        <div
            class="w-full bg-grid-lines bg-grid-20 pointer-events-none border [background-position:-2px_-2px]"
            style="
            /* mask out the grid from opaque → transparent */
            mask-image: linear-gradient(
                to bottom,
                rgba(255,0,0,1) 0%,
                rgba(0,0,0,0) 100%
            );
            mask-size: 100% 100%;
            mask-repeat: no-repeat;
            -webkit-mask-image: linear-gradient(
                to bottom,
                rgba(255,0,0,1) 0%,
                rgba(0,0,0,0) 100%
            );
            -webkit-mask-size: 100% 100%;
            -webkit-mask-repeat: no-repeat;
            "
        >
            {children()}
        </div>
    }
}

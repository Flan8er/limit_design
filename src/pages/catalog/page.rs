use crate::components::{
    contact::ContactSection,
    icons::{BevyIcon, BlenderIcon, LeptosIcon},
};
use leptos::prelude::*;
use web_sys::{wasm_bindgen::JsCast, HtmlElement};

#[component]
pub fn CatalogHome() -> impl IntoView {
    let six_axis_icons = vec![
        view! {<LeptosIcon/>}.into_any(),
        view! {<BlenderIcon/>}.into_any(),
        view! {<BevyIcon/>}.into_any(),
    ];
    let node_tree_icons = vec![
        view! {<LeptosIcon/>}.into_any(),
        view! {<BevyIcon/>}.into_any(),
    ];
    let verlet_icons = vec![
        view! {<LeptosIcon/>}.into_any(),
        view! {<BevyIcon/>}.into_any(),
    ];
    let waveform_icons = vec![
        view! {<LeptosIcon/>}.into_any(),
        view! {<BevyIcon/>}.into_any(),
    ];

    view! {
        <div class="w-full h-full flex flex-col">
            <div class="flex items-center gap-x-3.5 py-8 px-4 w-full relative z-[0] h-fit md:h-[250px]">
                <div class="absolute inset-0 grid-background bg-grid-20 border-none z-[-1]"/>

                <div class="h-1.5 w-1.5 bg-accent"></div>
                <h1 class="leading-none font-medium text-primary-text text-[24px] md:text-[32px]">
                    "Catalog"
                </h1>
            </div>

            <div
                id="glow-cards"
                class="grid gap-x-4 gap-y-8 pb-8 px-4 grid-cols-1 lg:grid-cols-2"
                on:mousemove=move |ev| {
                    let cards = document().get_elements_by_class_name("glow-card");

                    for i in 0..cards.length() {
                        if let Some(node) = cards.item(i) {
                            if let Ok(element) = node.dyn_into::<HtmlElement>() {
                                let rect = element.get_bounding_client_rect();
                                let x = ev.client_x() as f64 - rect.left();
                                let y = ev.client_y() as f64 - rect.top();

                                element
                                    .style()
                                    .set_property("--mouse-x", &format!("{x}px"))
                                    .ok();
                                element
                                    .style()
                                    .set_property("--mouse-y", &format!("{y}px"))
                                    .ok();
                            }
                        }
                    }
                }
            >
                <CatalogCard title="Reveal" image="/static/RobotSimulationPreview.png" icons=six_axis_icons/>
                <CatalogCard title="LimitFab" image="/static/NodeTreePreview.png" icons=node_tree_icons/>
                <CatalogCard title="Meteorite" image="/static/VerletPreview.png" icons=verlet_icons/>
                <CatalogCard title="Labelize" image="/static/WaveformPreview.png" icons=waveform_icons/>
            </div>

            <div class="flex-grow"/>

            <ContactSection/>
        </div>
    }
}

#[component]
pub fn CatalogCard(title: &'static str, image: &'static str, icons: Vec<AnyView>) -> impl IntoView {
    view! {
        <div
            class="w-full h-full rounded-lg glow-card"
        >
            <div class="glow-card-border"/>

            <div class="flex flex-col glow-card-content">
                <div class="flex flex-col w-full h-full p-3 gap-4">
                    <h2 class="font-light">{title}</h2>
                    <div class="flex gap-2">
                        {icons.into_iter().map(|icon| view! {
                            <div class="w-[20px] h-[20px]">
                                {icon}
                            </div>
                        }).collect_view()}
                    </div>
                </div>

                <img
                    class="aspect-video w-full object-cover"
                    loading="lazy"
                    aria-hidden="true"
                    src=image
                    alt=title
                    width="2970"
                    height="1678"
                />

                <div
                    class="w-full rounded-md p-4 gap-2 group-hover:gap-4 flex justify-center items-center cursor-default"
                >
                    <h3 class="text-primary-text-muted font-light">"Discover this experiment"</h3>
                </div>
            </div>
        </div>
    }
}

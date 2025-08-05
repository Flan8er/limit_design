use crate::components::{
    contact::ContactSection,
    icons::{BevyIcon, BlenderIcon, LeptosIcon},
};
use leptos::prelude::*;
use leptos_icons::Icon;
use web_sys::{wasm_bindgen::JsCast, HtmlElement};

#[component]
pub fn CatalogHome() -> impl IntoView {
    let reveal_icons = vec![
        view! {<Icon icon=icondata::SiTauri width="100%" height="100%" style="fill: #696969;"/>}
            .into_any(),
        view! {<Icon icon=icondata::BiReact width="100%" height="100%" style="fill: #58C4DC;"/>}
            .into_any(),
        view! {<Icon icon=icondata::FaRustBrands width="100%" height="100%" style="fill: #696969;"/>}
            .into_any(),
    ];
    let limitfab_icons = vec![
        view! {<Icon icon=icondata::SiNextdotjs width="100%" height="100%"/>}.into_any(),
        view! {<Icon icon=icondata::BiTailwindCss width="100%" height="100%" style="fill: #00BCFF;"/>}.into_any(),
        view! {<BlenderIcon/>}.into_any()
    ];
    let meteorite_icons = vec![
        view! {<LeptosIcon/>}.into_any(),
        view! {<BevyIcon/>}.into_any(),
        view! {<BlenderIcon/>}.into_any(),
    ];
    let labelize_icons = vec![
        view! {<Icon icon=icondata::SiNextdotjs width="100%" height="100%"/>}.into_any(),
        view! {<Icon icon=icondata::BiTailwindCss width="100%" height="100%" style="fill: #00BCFF;"/>}.into_any(),
    ];

    let reveal_description = "No-code editor for Reveal.js applications, useable as a standalone presentation client or to inject presentations into web apps.";
    let limitfab_description =
        "Designing and building custom, high-performance carbon fiber steering wheels.";
    let meteorite_description =
        "3D printing software for aerospace hardware and spaceflight-grade re-entry capsules.";
    let labelize_description = "Tool for building layouts, text, barcodes, images, and shapes to be printed on a label maker.";

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
                <CatalogCard title="Reveal" image="/static/RevealPreview.png" icons=reveal_icons description=reveal_description/>
                <CatalogCard title="LimitFab" image="/static/LimitFabPreview.png" icons=limitfab_icons description=limitfab_description/>
                <CatalogCard title="Meteorite" image="/static/MeteoritePreview.png" icons=meteorite_icons description=meteorite_description/>
                <CatalogCard title="Labelize" image="/static/LabelizePreview.png" icons=labelize_icons description=labelize_description/>
            </div>

            <div class="flex-grow"/>

            <ContactSection/>
        </div>
    }
}

#[component]
pub fn CatalogCard(
    title: &'static str,
    image: &'static str,
    icons: Vec<AnyView>,
    description: &'static str,
) -> impl IntoView {
    view! {
        <div
            class="w-full h-full rounded-lg glow-card"
        >
            <div class="glow-card-border"/>

            <div class="flex flex-col glow-card-content">
                <div class="flex flex-col w-full p-4 gap-4">
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

                // <div class="flex-grow"/>

                <div
                    class="w-full h-full rounded-md p-4 gap-2 flex justify-center items-center text-center"
                >
                    <h3 class="text-primary-text-muted font-light">{description}</h3>
                </div>
            </div>
        </div>
    }
}

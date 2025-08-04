use crate::{
    components::{
        contact::ContactSection,
        icons::{BevyIcon, BlenderIcon, LeptosIcon},
    },
    pages::experiments::home::page::ExperimentCard,
};
use leptos::prelude::*;

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

            <div class="grid gap-x-4 gap-y-8 pb-8 px-4 grid-cols-1 lg:grid-cols-2">
                <ExperimentCard title="Reveal" route="/catalog/reveal" image="/static/RobotSimulationPreview.png" icons=six_axis_icons/>
                <ExperimentCard title="LimitFab" route="/catalog/limit-fab" image="/static/NodeTreePreview.png" icons=node_tree_icons/>
                <ExperimentCard title="Meteorite" route="/catalog/meteorite" image="/static/VerletPreview.png" icons=verlet_icons/>
                <ExperimentCard title="Labelize" route="/catalog/labelize" image="/static/WaveformPreview.png" icons=waveform_icons/>
            </div>

            <div class="flex-grow"/>

            <ContactSection/>
        </div>
    }
}

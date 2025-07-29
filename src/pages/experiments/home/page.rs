use leptos::prelude::*;
use leptos_icons::Icon;
use leptos_router::hooks::use_navigate;

use crate::components::icons::{BevyIcon, BlenderIcon, LeptosIcon};

#[component]
pub fn ExperimentsHome() -> impl IntoView {
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
    let reentry_icons = vec![
        view! {<LeptosIcon/>}.into_any(),
        view! {<BevyIcon/>}.into_any(),
    ];
    let galaga_icons = vec![
        view! {<LeptosIcon/>}.into_any(),
        view! {<BevyIcon/>}.into_any(),
    ];

    view! {
        <div class="w-full h-full flex flex-col">
            <div class="flex items-center gap-x-3.5 text-[24px] md:text-[32px] py-8 px-4 w-full relative z-[0] h-fit md:h-[250px]">
                <div class="absolute inset-0 grid-background bg-grid-20 border-none z-[-1]"/>

                <div class="h-1.5 w-1.5 bg-accent"></div>
                <h1 class="leading-none font-medium text-primary-text">
                    "Experiments"
                </h1>
            </div>

            <div class="grid gap-x-4 gap-y-8 pb-8 px-4 grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
                <ExperimentCard title="6-Axis Robot Arm" route="robot-simulation" image="/static/RobotSimulationPreview.png" icons=six_axis_icons/>
                <ExperimentCard title="MQTT Node Tree Transmission" route="node-tree" image="/static/NodeTreePreview.png" icons=node_tree_icons/>
                <ExperimentCard title="Verlet Simulation Crate" route="verlet-simulation" image="/static/VerletPreview.png" icons=verlet_icons/>
                <ExperimentCard title="Waveform Animation" route="waveform" image="/static/WaveformPreview.png" icons=waveform_icons/>
                <ExperimentCard title="Reentry Capsule Simulation" route="reentry-simulation" image="/static/ReentryPreview.png" icons=reentry_icons/>
                <ExperimentCard title="Galaga" route="galaga" image="/static/GalagaPreview.png" icons=galaga_icons/>
            </div>
        </div>
    }
}

#[component]
pub fn ExperimentCard(
    title: &'static str,
    route: &'static str,
    image: &'static str,
    icons: Vec<AnyView>,
) -> impl IntoView {
    let navigate = use_navigate();
    let icon_style = RwSignal::new(String::from(
        "transition-duration: 300ms; transform: rotate(-45deg); color: hsl(var(--primary-text-muted));",
    ));

    view! {
        <div
            class="flex flex-col w-full h-full border hover:border-primary-text-muted duration-300 rounded-lg group"
            on:mouseenter=move |_| icon_style.set(String::from("transition-duration: 300ms; transform: rotate(0deg); color: hsl(var(--primary-text-muted));"))
            on:mouseleave=move |_| icon_style.set(String::from("transition-duration: 300ms; transform: rotate(-45deg); color: hsl(var(--primary-text-muted));"))
            on:click=move |_| navigate(&format!("/experiments/{}", route), Default::default())
        >
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
                class="w-full rounded-md group-hover:scale-[1.05] duration-300 p-4 gap-2 group-hover:gap-4 flex justify-center items-center cursor-default"
            >
                <h3 class="text-primary-text-muted font-light">"Discover this experiment"</h3>
                <Icon icon=icondata::VsArrowRight width="22px" height="22px"
                    style=icon_style
                />
            </div>
        </div>
    }
}

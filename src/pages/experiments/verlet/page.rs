use leptos::{html::Div, prelude::*};

use leptos_verlet::prelude::*;

use crate::pages::experiments::{
    page::ExpandedView,
    verlet::components::{
        control_pane::ControlPane, element_pane::ElementPane, info::InfoModal,
        mouse_monitor::MouseMonitor,
    },
};

#[component]
pub fn VerletExperiment() -> impl IntoView {
    view! {
        <ExpandedView title="Verlet Simulation Crate" description="A custom crate published to crates.io - an engine that allows the addition of interactive Verlet simulations into any leptos app.">
            <iframe class="m-0 p-0 w-full h-full z-[0]" src="/verlet-frame"/>
        </ExpandedView>
    }
}

#[component]
pub fn VerletFrame() -> impl IntoView {
    let simulation_container = NodeRef::<Div>::new();

    let active_modifier: RwSignal<ModificationTarget> = RwSignal::new(ModificationTarget::None);

    view! {
        <VerletConfigProvider
            simulation_settings=SimulationSettings{
                jerk_damping: 0.5,
                simulation_bounds: SimulationBounds::new(true, true, true),
                ..default()
            }
        />

        <div class="w-full h-full flex items-center justify-center overflow-hidden relative">
            <ElementPane active_modifier/>
            <InfoModal active_modifier/>

            <div
                node_ref=simulation_container
                class="w-full h-full relative"
            >
                <VerletCanvas parent_element=simulation_container/>

                <MouseMonitor active_modifier/>
            </div>

            <ControlPane active_modifier/>
        </div>
    }
}

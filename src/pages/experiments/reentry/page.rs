use leptos::prelude::*;
use leptos_bevy_canvas::prelude::{event_b2l, BevyCanvas};

use crate::pages::experiments::{
    page::ExpandedView,
    reentry::{notification::CapsuleState, system::run_reentry},
};

#[component]
pub fn ReentryExperiment() -> impl IntoView {
    view! {
        <ExpandedView title="Reentry Capsule Simulation" description="A real-time simulation of SpaceX's V1 Starship reentry path.">
            <iframe class="m-0 p-0 w-full h-full z-[0]" src="/reentry-frame"/>
        </ExpandedView>
    }
}

#[component]
pub fn ReentryFrame() -> impl IntoView {
    let (current_state, bevy_transmitter) = event_b2l::<CapsuleState>();
    let current_velocity = Signal::derive({
        let current_state = current_state.clone();
        move || current_state.get().unwrap_or_default().velocity
    });
    let current_altitude = Signal::derive({
        let current_state = current_state.clone();
        move || current_state.get().unwrap_or_default().altitude
    });

    view! {
        <div class="w-full h-full relative">
            <div class="absolute top-4 left-4 flex flex-col text-primary-text">
                <h3 class="font-mono">{move || format!("Velocity: {:.0} m/s", current_velocity.get())}</h3>
                <h3 class="font-mono">{move || format!("Altitude: {:.0} m", current_altitude.get())}</h3>
            </div>

            <BevyCanvas
                init=move || {
                    run_reentry(bevy_transmitter)
                }
            />
        </div>
    }
}

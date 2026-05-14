use leptos::prelude::*;
use leptos_bevy_canvas::prelude::*;
use leptos_use::{watch_debounced_with_options, WatchDebouncedOptions};
use thaw::SpinButton;
use thaw_utils::Model;

use crate::pages::experiments::{
    page::ExpandedView,
    threat_prediction::{
        components::simulation_params::SimulationParameters,
        threat_prediction::{init_bevy_app, OrbitalNotification},
    },
};

#[component]
pub fn ThreatPredictionFrame() -> impl IntoView {
    let simulation_parameters = RwSignal::new(SimulationParameters::default());

    let (param_sender, bevy_param_receiver) = event_l2b::<SimulationParameters>();
    let (error_receiver, bevy_error_sender) = event_b2l::<OrbitalNotification>();

    let _ = watch_debounced_with_options(
        move || simulation_parameters.get(),
        move |p, _, _| {
            param_sender.send(p.clone()).ok();
        },
        1000.0,
        WatchDebouncedOptions::default().max_wait(Some(5000.0)),
    );

    Effect::new({
        let receiver = error_receiver.clone();
        move |_| {
            receiver.get();
            simulation_parameters.set(SimulationParameters::default());
        }
    });

    let inclination_model: Model<u16> = (
        Signal::derive(move || simulation_parameters.get().inclination),
        SignalSetter::map(move |updated: u16| {
            let mut params = simulation_parameters.get_untracked();
            params.inclination = updated;
            simulation_parameters.set(params);
        }),
    )
        .into();
    let altitude_model: Model<u32> = (
        Signal::derive(move || simulation_parameters.get().separation_altitude),
        SignalSetter::map(move |updated: u32| {
            let mut params = simulation_parameters.get_untracked();
            params.separation_altitude = updated;
            simulation_parameters.set(params);
        }),
    )
        .into();
    let velocity_model: Model<u32> = (
        Signal::derive(move || simulation_parameters.get().separation_velocity),
        SignalSetter::map(move |updated: u32| {
            let mut params = simulation_parameters.get_untracked();
            params.separation_velocity = updated;
            simulation_parameters.set(params);
        }),
    )
        .into();
    let heading_model: Model<f64> = (
        Signal::derive(move || simulation_parameters.get().heading),
        SignalSetter::map(move |updated: f64| {
            let mut params = simulation_parameters.get_untracked();
            params.heading = updated;
            simulation_parameters.set(params);
        }),
    )
        .into();
    let latitude_model: Model<f32> = (
        Signal::derive(move || simulation_parameters.get().launch_latitude),
        SignalSetter::map(move |updated: f32| {
            let mut params = simulation_parameters.get_untracked();
            params.launch_latitude = updated;
            simulation_parameters.set(params);
        }),
    )
        .into();
    let longitude_model: Model<f32> = (
        Signal::derive(move || simulation_parameters.get().launch_logitude),
        SignalSetter::map(move |updated: f32| {
            let mut params = simulation_parameters.get_untracked();
            params.launch_logitude = updated;
            simulation_parameters.set(params);
        }),
    )
        .into();

    view! {
        <div class="w-full h-full relative">
            <div class="absolute top-2 p-1 border flex items-center justify-between rounded-md w-full glass z-[20]">
                <div class="flex items-center gap-2 mx-1">
                    <p>"Inclination:"</p>
                    <SpinButton<u16> value=inclination_model step_page=10 min=0 max=360/>
                </div>
                <div class="flex items-center gap-2 mx-1">
                    <p>"Altitude:"</p>
                    <SpinButton<u32> value=altitude_model step_page=10_000 min=120_000 max=300_000/>
                </div>
                <div class="flex items-center gap-2 mx-1">
                    <p>"Velocity:"</p>
                    <SpinButton<u32> value=velocity_model step_page=500 min=100 max=8_000/>
                </div>
                <div class="flex items-center gap-2 mx-1">
                    <p>"Heading:"</p>
                    <SpinButton<f64> value=heading_model step_page=10. min=5. max=85./>
                </div>
                <div class="flex items-center gap-2 mx-1">
                    <p>"Latitude:"</p>
                    <SpinButton<f32> value=latitude_model step_page=10. min=-90. max=90./>
                </div>
                <div class="flex items-center gap-2 mx-1">
                    <p>"Longitude:"</p>
                    <SpinButton<f32> value=longitude_model step_page=20. min=-180. max=180./>
                </div>
            </div>

            <BevyCanvas
                init=move || {
                    init_bevy_app(bevy_param_receiver, bevy_error_sender)
                }
            />
        </div>
    }
}
// Idealized shows the trajectory as if no outside forces were at play and the Earth was stationary.
#[component]
pub fn ThreatPredictionExperiment() -> impl IntoView {
    view! {
        <ExpandedView
            title="Threat Prediction"
            description="Real-time ballistic trajectory prediction pipeline modeled after missile defense applications. Fuses an Euler integrator (modeling payload weight, atmospheric drag, and altitude-dependent atmospheric density and temperature) with a Keplerian orbit solver accounting for Coriolis effects. Reconstructs full threat trajectories from just 3 points within 500 meters and re-couples to the Euler simulation at Kármán line for atmospheric reentry phase modeling.\n\nThe 'Idealized' plot shows the trajectory as if no outside forces were at play and the Earth were stationary, and the 'Actual' plot shows these forces."
        >
            <div class="flex w-full h-full relative z-[0]">
                <div class="z-[10] absolute bottom-4 right-4 gap-4 flex flex-col items-end text-primary-text">
                    <div class="flex items-center gap-4">
                        <div class="h-[15px] w-[15px] rounded-full bg-red-600"></div>
                        <p class="text-xl text-primary-text font-mono">"Launch"</p>
                    </div>
                    <div class="flex items-center gap-4">
                        <div class="h-[0px] w-[30px] border-[1px] border-red-600"></div>
                        <p class="text-xl text-primary-text font-mono">"Idealized"</p>
                    </div>

                    <div class="flex items-center gap-4">
                        <div class="h-[0px] w-[30px] border-[1px] border-white"></div>
                        <p class="text-xl text-primary-text font-mono">"Actual"</p>
                    </div>
                </div>

                <iframe class="m-0 p-0 w-full h-full z-[0]" src="/threat-prediction-frame"/>
            </div>
        </ExpandedView>
    }
}

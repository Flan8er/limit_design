use codee::string::JsonSerdeCodec;
use leptos::prelude::*;
use leptos_bevy_canvas::prelude::*;
use leptos_use::{use_broadcast_channel, UseBroadcastChannelReturn};
use thaw::Slider;
use thaw_utils::Model;

use crate::pages::experiments::{
    page::ExpandedView,
    waveform::waveform::{init_bevy_app, Waveform, WaveformModification},
};

#[component]
pub fn WaveformFrame() -> impl IntoView {
    let UseBroadcastChannelReturn { message, .. } = use_broadcast_channel::<
        WaveformModification,
        JsonSerdeCodec,
    >("waveform-modification-channel");

    let (modification_sender, modification_receiver) = event_l2b::<WaveformModification>();

    Effect::new(move |_| {
        if let Some(message) = message.get() {
            modification_sender.send(message).ok();
        };
    });

    view! {
        <BevyCanvas
            init=move || {
                init_bevy_app(modification_receiver)
            }
        />
    }
}

#[component]
pub fn WaveformExperiment() -> impl IntoView {
    let waveform_state = RwSignal::new(Waveform::default());
    let UseBroadcastChannelReturn { post, .. } = use_broadcast_channel::<
        WaveformModification,
        JsonSerdeCodec,
    >("waveform-modification-channel");

    Effect::new(move |_| {
        let updated_waveform_request = waveform_state.get();

        post(&WaveformModification {
            amplitude: updated_waveform_request.amplitude,
            wavelength: updated_waveform_request.wavelength,
            omega: updated_waveform_request.omega,
        });
    });

    let amplitude_model: Model<f64> = (
        Signal::derive(move || waveform_state.get().amplitude as f64),
        SignalSetter::map(move |updated: f64| {
            let mut new_waveform = waveform_state.get_untracked();
            new_waveform.amplitude = updated as f32;
            waveform_state.set(new_waveform);
        }),
    )
        .into();
    let wavelength_model: Model<f64> = (
        Signal::derive(move || waveform_state.get().wavelength as f64),
        SignalSetter::map(move |updated: f64| {
            let mut new_waveform = waveform_state.get_untracked();
            new_waveform.wavelength = updated as f32;
            waveform_state.set(new_waveform);
        }),
    )
        .into();
    let omega_model: Model<f64> = (
        Signal::derive(move || waveform_state.get().omega as f64),
        SignalSetter::map(move |updated: f64| {
            let mut new_waveform = waveform_state.get_untracked();
            new_waveform.omega = updated as f32;
            waveform_state.set(new_waveform);
        }),
    )
        .into();

    view! {
        <ExpandedView title="Waveform Animation">
            <div class="flex w-full h-full relative z-[0]">
                <iframe class="m-0 p-0 w-full h-full z-[0]" src="/waveform-frame"/>

                <div class="h-[200px] absolute right-4 top-4 flex flex-col items-start justify-start z-[100]">
                    <p class="text-xl text-primary-text font-mono">"Amplitude"</p>
                    <Slider value=amplitude_model min=0. max=20./>

                    <p class="text-xl text-primary-text font-mono">"Wavelength"</p>
                    <Slider value=wavelength_model min=1. max=100./>

                    <p class="text-xl text-primary-text font-mono">"Omega"</p>
                    <Slider value=omega_model min=0. max=10./>
                </div>
            </div>
        </ExpandedView>
    }
}

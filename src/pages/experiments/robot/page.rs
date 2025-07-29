use codee::string::{FromToStringCodec, JsonSerdeCodec};
use leptos::prelude::*;
use leptos_bevy_canvas::prelude::*;
use leptos_use::{use_broadcast_channel, UseBroadcastChannelReturn};
use thaw::{Checkbox, Slider};

use crate::pages::experiments::{
    page::ExpandedView,
    robot::simulation::{
        core::init::init_bevy_app,
        plugins::{debug::dev_tools::ShowDevTools, setup::robot_spawner::JointAngles},
    },
};

#[component]
pub fn RobotSimulation() -> impl IntoView {
    match window().document() {
        Some(window) => match window.body() {
            Some(body) => {
                let _ = body.style().set_property("background", "none transparent");
            }
            None => (),
        },
        None => (),
    };

    let UseBroadcastChannelReturn { message, .. } =
        use_broadcast_channel::<JointAngles, JsonSerdeCodec>("joint-angle-channel");
    let UseBroadcastChannelReturn {
        message: show_dev_tools,
        ..
    } = use_broadcast_channel::<bool, FromToStringCodec>("show-dev-tools");

    let (angle_sender, bevy_angle_receiver) = event_l2b::<JointAngles>();
    let (dev_sender, bevy_dev_receiver) = event_l2b::<ShowDevTools>();

    Effect::new(move |_| {
        if let Some(message) = message.get() {
            angle_sender.send(message).ok();
        }
    });
    Effect::new(move |_| {
        if let Some(message) = show_dev_tools.get() {
            dev_sender.send(ShowDevTools(message)).ok();
        }
    });

    view! {
        <BevyCanvas
            init=move || {
                init_bevy_app(bevy_angle_receiver, bevy_dev_receiver)
            }
        />
    }
}

#[component]
pub fn RobotExperiment() -> impl IntoView {
    let j1 = RwSignal::new(0.0);
    let j2 = RwSignal::new(0.0);
    let j3 = RwSignal::new(0.0);
    let j4 = RwSignal::new(0.0);
    let j5 = RwSignal::new(0.0);
    let j6 = RwSignal::new(0.0);
    let dev_tool_input = RwSignal::new(false);

    let UseBroadcastChannelReturn { post, .. } =
        use_broadcast_channel::<JointAngles, JsonSerdeCodec>("joint-angle-channel");
    let UseBroadcastChannelReturn {
        post: show_dev_tools,
        ..
    } = use_broadcast_channel::<bool, FromToStringCodec>("show-dev-tools");

    Effect::new(move |_| {
        post(&JointAngles {
            j1: j1.get() as f32,
            j2: j2.get() as f32,
            j3: j3.get() as f32,
            j4: j4.get() as f32,
            j5: j5.get() as f32,
            j6: j6.get() as f32,
        })
    });
    Effect::new(move |_| show_dev_tools(&dev_tool_input.get()));

    view! {
        <ExpandedView>
            <div class="w-full h-full relative z-[0]">
                <div class="z-[10] absolute top-4 right-4 gap-4 flex flex-col items-end text-primary-text">
                    <div class="flex items-center gap-4">
                        <h3 class="font-mono">{move || j1.get()}</h3>
                        <Slider value=j1 max=359.9/>
                        <h2 class="font-mono">"J1"</h2>
                    </div>
                    <div class="flex items-center gap-4">
                        <h3 class="font-mono">{move || j2.get()}</h3>
                        <Slider value=j2 min=-180.0 max=180.0/>
                        <h2 class="font-mono">"J2"</h2>
                    </div>
                    <div class="flex items-center gap-4">
                        <h3 class="font-mono">{move || j3.get()}</h3>
                        <Slider value=j3 max=359.9/>
                        <h2 class="font-mono">"J3"</h2>
                    </div>
                    <div class="flex items-center gap-4">
                        <h3 class="font-mono">{move || j4.get()}</h3>
                        <Slider value=j4 max=359.9/>
                        <h2 class="font-mono">"J4"</h2>
                    </div>
                    <div class="flex items-center gap-4">
                        <h3 class="font-mono">{move || j5.get()}</h3>
                        <Slider value=j5 max=359.9/>
                        <h2 class="font-mono">"J5"</h2>
                    </div>
                    <div class="flex items-center gap-4">
                        <h3 class="font-mono">{move || j6.get()}</h3>
                        <Slider value=j6 max=359.9/>
                        <h2 class="">"J6"</h2>
                    </div>
                    <div class="flex items-center gap-4">
                        <h3 class="font-mono">"Show dev tools"</h3>
                        <Checkbox checked=dev_tool_input/>
                    </div>
                </div>

                <iframe class="m-0 p-0 w-full h-full z-[0]" src="/robot-simulation-frame"/>
            </div>
        </ExpandedView>
    }
}

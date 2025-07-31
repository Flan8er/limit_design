use leptos::prelude::*;
use thaw::{ConfigProvider, Theme};

pub mod components;
pub mod pages;

mod app;
use app::*;

use crate::{
    components::id_card::BadgeSimulation,
    pages::{
        about::app::PortfolioFrame,
        experiments::{
            galaga::page::GalagaFrame, node_tree::canvas::page::TreeRouter,
            reentry::page::ReentryFrame, robot::page::RobotFrame, verlet::page::VerletFrame,
            waveform::page::WaveformFrame,
        },
    },
};

pub fn main() {
    let theme = RwSignal::new(Theme::dark());
    console_error_panic_hook::set_once();

    let path = web_sys::window()
        .and_then(|w| w.location().pathname().ok())
        .unwrap_or_default();

    if path.contains("badge_simulation") {
        mount_to_body(move || {
            view! {
                <BadgeSimulation/>
            }
        })
    } else if path.contains("robot-simulation-frame") {
        mount_to_body(move || {
            view! {
                <RobotFrame/>
            }
        })
    } else if path.contains("node-tree-frame") {
        mount_to_body(move || {
            view! {
                <TreeRouter/>
            }
        })
    } else if path.contains("waveform-frame") {
        mount_to_body(move || {
            view! {
                <WaveformFrame/>
            }
        })
    } else if path.contains("verlet-frame") {
        mount_to_body(move || {
            view! {
                <VerletFrame/>
            }
        })
    } else if path.contains("reentry-frame") {
        mount_to_body(move || {
            view! {
                <ReentryFrame/>
            }
        })
    } else if path.contains("galaga-frame") {
        mount_to_body(move || {
            view! {
                <GalagaFrame/>
            }
        })
    } else if path.contains("portfolio-frame") {
        mount_to_body(move || {
            view! {
                <PortfolioFrame/>
            }
        })
    } else {
        mount_to_body(move || {
            view! {
                <ConfigProvider theme class="bg-background">
                    <App/>
                </ConfigProvider>
            }
        })
    };
}

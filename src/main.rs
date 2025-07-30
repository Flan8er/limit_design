use leptos::prelude::*;
use thaw::{ConfigProvider, Theme};

pub mod components;
pub mod pages;

mod app;
use app::*;
mod badge;
use badge::*;

use crate::pages::experiments::{
    node_tree::canvas::page::TreeRouter, robot::page::RobotSimulation,
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
                <RobotSimulation/>
            }
        })
    } else if path.contains("node-tree-frame") {
        mount_to_body(move || {
            view! {
                <TreeRouter/>
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

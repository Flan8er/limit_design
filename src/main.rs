use leptos::prelude::*;
mod app;
mod badge;
pub mod components;
pub mod pages;

use app::*;
use badge::*;
use thaw::{ConfigProvider, Theme};

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

use leptos::prelude::*;
mod app;
pub mod components;
pub mod pages;

use app::*;
use thaw::{ConfigProvider, Theme};

pub fn main() {
    let theme = RwSignal::new(Theme::dark());
    console_error_panic_hook::set_once();

    mount_to_body(move || {
        view! {
            <ConfigProvider theme>
                <App/>
            </ConfigProvider>
        }
    });
}

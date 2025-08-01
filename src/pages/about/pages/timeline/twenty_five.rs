use leptos::prelude::*;
use leptos_use::use_interval_fn;

use crate::pages::about::components::page::Page;

#[component]
pub fn TwentyTwentyFive() -> impl IntoView {
    let photos = [
        "Meteorite1.png",
        "Meteorite2.png",
        "Meteorite3.png",
        "Meteorite4.png",
        "Meteorite5.png",
        "Meteorite6.png",
        "Meteorite7.png",
    ]
    .map(String::from);

    let active_photo: RwSignal<u8> = RwSignal::new(0);

    use_interval_fn(
        {
            let photo_count = photos.len() as u8;
            move || {
                active_photo.update(|c| {
                    *c = (*c + 1) % photo_count;
                });
            }
        },
        12000,
    );

    view! {
        <Page>
            <div class="w-full h-full flex items-center justify-center relative py-6">
                <div
                    class="absolute inset-0 bg-center bg-no-repeat bg-cover opacity-[15%]"
                    style=move || {
                        let index = active_photo.get() as usize;
                        format!("background-image: url(\"static/{}\");", photos[index])
                    }
                />
                <div class="max-w-6xl w-full h-full px-8 flex items-center justify-center z-10">
                    <div>
                        <h2 class="text-5xl font-bold mb-8 leading-tight bg-gradient-to-r from-purple-400 to-pink-400 bg-clip-text text-transparent">
                            "Rust all the way down"
                        </h2>

                        <p class="text-lg text-primary-text-muted leading-relaxed mb-6">
                            "The next contract I received was from a space startup whose goal was to 3D print solid rocket motors and reentry capsules.
                            Up until this point, I haven’t said much about my previous full-time role at the KCNSC (Kansas City National Security Campus),
                            where I worked in a group developing high-precision 5-axis DIW 3D printers and the motion control/slicer software that powered them.
                            We built a program capable of coordinated motion along complex contoured parts.
                            This new contract had a similar goal — but with the benefit of prior experience: build it better."
                        </p>

                        <p class="text-lg text-primary-text-muted leading-relaxed">
                            "Acting as the SME, I led their internal team to develop a real-time monitoring and control system for a 6-axis robot-arm-based MAD 3D printer.
                            The software brought together a range of past experiences: Reveal, built in "

                            <a
                                href="https://v2.tauri.app"
                                target="_blank"
                                rel="noopener noreferrer"
                                class="text-blue-500"
                            >"Tauri"</a>

                            ", was originally a desperate attempt to escape "

                            <a
                                href="https://www.electronjs.org"
                                target="_blank"
                                rel="noopener noreferrer"
                                class="text-blue-500"
                            >"Electron"</a>

                            " — that process taught me Rust. Indie game development through Rust introduced me to "

                            <a
                                href="https://bevy.org"
                                target="_blank"
                                rel="noopener noreferrer"
                                class="text-blue-500"
                            >"Bevy"</a>

                            ", an ECS-driven game engine.
                            My prior work at KCNSC gave me a foundation in motion control systems.
                            This new project leveraged all of them. Bevy powered the backend — its entity-driven logic mapped well to real-world interactions —
                            while the front-end was built with yet another Rust library: "

                            <a
                                href="https://leptos.dev"
                                target="_blank"
                                rel="noopener noreferrer"
                                class="text-blue-500"
                            >"Leptos"</a>

                            "."
                        </p>
                    </div>
                </div>
            </div>
        </Page>
    }
}

use leptos::prelude::*;

use crate::pages::about::components::{navigator::Navigator, page::Page};

#[component]
pub fn TwentyTwentyThree() -> impl IntoView {
    let photos: [String; 7] = [
        String::from("Reveal1.png"),
        String::from("Reveal2.png"),
        String::from("Reveal3.png"),
        String::from("Reveal4.png"),
        String::from("Reveal5.png"),
        String::from("Reveal6.png"),
        String::from("Reveal7.png"),
    ];

    let active_photo: RwSignal<u8> = RwSignal::new(0);

    view! {
        <Page>
            <div
                class="w-full h-full flex items-center justify-center relative bg-gradient-to-b from-[rgba(201,42,148,0.125)] to-[rgba(183,46,176,0.15)] py-6"
            >
                <div class="max-w-6xl w-full px-8 grid lg:grid-cols-2 lg:gap-12 items-center z-10">
                    <div>
                        <h2 class="text-5xl font-bold mb-2 lg:mb-8 leading-tight bg-gradient-to-r from-purple-400 to-pink-400 bg-clip-text text-transparent">
                            "The Big \"Reveal\""
                        </h2>

                        <p class="text-lg text-primary-text-muted leading-relaxed lg:mb-6">
                            "After five iterations across just as many languages and frameworks, I became eager to work on something other than a workout tracking app.
                            At the time, I was working full-time as a mechanical engineer, constantly sniffing out any software opportunities that crossed my path.
                            Still, I felt I could do more to pave the way toward becoming a full-time software developer.
                            So, after hours, I came home to build my own software company."
                        </p>

                        <p class="text-lg text-primary-text-muted leading-relaxed lg:mb-6">
                            "Looking back, I had such naïve ambition — stay tuned, as that’s a recurring theme — but after several months,
                            I had created something I could hang my hat on."
                        </p>

                        <p class="text-lg text-primary-text-muted leading-relaxed">
                            "My goal was to create a "

                            <a
                                href="https://setapp.com"
                                target="_blank"
                                rel="noopener noreferrer"
                                class="text-blue-500"
                            >"Setapp"</a>

                            "-esque company focused on no-code builders for popular NPM packages. The idea was simple: not everyone wants to learn how to use the
                            latest-and-greatest library, but diving into those challenges was exactly what energized me.
                            I had my sights set on "

                            <a
                                href="https://d3js.org"
                                target="_blank"
                                rel="noopener noreferrer"
                                class="text-blue-500"
                            >"D3"</a>

                            ", "

                            <a
                                href="https://nodered.org"
                                target="_blank"
                                rel="noopener noreferrer"
                                class="text-blue-500"
                            >"Node-RED"</a>

                            ", and "

                            <a
                                href="https://threejs.org"
                                target="_blank"
                                rel="noopener noreferrer"
                                class="text-blue-500"
                            >"three.js"</a>

                            ", but I started by building a no-code editor for "

                            <a
                                href="https://revealjs.com"
                                target="_blank"
                                rel="noopener noreferrer"
                                class="text-blue-500"
                            >"Reveal.js"</a>

                            ". Taking stylistic influence from Apple's approach towards UI/UX design — built on Rust using the Tauri framework with a React front-end: Reveal."
                        </p>
                    </div>

                    <div class="w-full md:h-[500px] flex flex-col items-center justify-between pt-4 md:pt-0">
                        <img
                            src={
                                let photos = photos.clone();
                                move || format!("static/{}", photos[active_photo.get() as usize])
                            }
                            alt=""
                            class="w-full h-full object-contain transition-all duration-300"
                        />

                        <div class="z-10 mdmt-auto">
                            <Navigator index=active_photo length=photos.len() />
                        </div>
                    </div>
                </div>
            </div>
        </Page>
    }
}

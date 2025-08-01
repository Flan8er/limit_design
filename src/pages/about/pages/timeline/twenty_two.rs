use leptos::prelude::*;

use crate::pages::about::components::page::Page;

#[component]
pub fn TwentyTwentyTwo() -> impl IntoView {
    view! {
        <Page>
            <div
                class="w-full h-full flex items-center justify-center relative bg-gradient-to-b from-[rgba(219,39,119,0.1)] to-[rgba(201,42,148,0.125)] py-6"
            >
                <div class="max-w-6xl w-full px-8 grid lg:grid-cols-2 gap-12 items-center z-10">
                    <div>
                        <h2 class="text-5xl font-bold mb-8 leading-tight bg-gradient-to-r from-purple-400 to-pink-400 bg-clip-text text-transparent">
                            "Journey Begins"
                        </h2>

                        <p class="text-lg text-primary-text-muted leading-relaxed mb-6">
                            "My original path lead me to getting my degree in mechanical engineering — this was my passion since sophomore year of high school.
                            A short internship duing my junior year of college quickly changed that, however.
                            My mentor stressed the importance, as a Mechainical Engineer, the ability to design machines and then, on top of that,
                            to have the ability to develop the software that runs them. And that single conversation is where this journey began."
                        </p>

                        <p class="text-lg text-primary-text-muted leading-relaxed">
                            "Getting started in app development I did what any sane person would do — go to Python.
                            My senior year of college consisted of using it for coursework and the occasional spontaneous inspiration to make a workout tracking app.
                            The resulting app, while it wasn't the most beautiful, to me it was perfect. It was something tangible that I made from nothing.
                            It was enough to say \"This is what I wan't to do.\""
                        </p>

                    </div>

                    <div class="flex justify-center items-center" >
                        <img
                            src="static/WOTrack.png"
                            alt=""
                            class="w-full h-full object-cover relative z-10"
                        />
                    </div>
                </div>
            </div>
        </Page>
    }
}

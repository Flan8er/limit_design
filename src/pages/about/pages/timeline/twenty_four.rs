use leptos::prelude::*;

use crate::pages::about::components::page::Page;

#[component]
pub fn TwentyTwentyFour() -> impl IntoView {
    let photos: [String; 4] = [
        String::from("Labelize1.png"),
        String::from("Labelize2.png"),
        String::from("Labelize3.png"),
        String::from("LimitFab.png"),
    ];

    view! {
        <Page>
            <div
                class="w-full h-full flex items-center justify-center relative bg-gradient-to-b from-[rgba(183,46,176,0.15)] to-[rgba(165,49,205,0.175)] py-6"
            >
                <div class="max-w-6xl w-full h-full px-8 grid lg:grid-rows-2 items-center z-10">
                    <div>
                        <h2 class="text-5xl font-bold mb-8 leading-tight bg-gradient-to-r from-purple-400 to-pink-400 bg-clip-text text-transparent">
                            "Off to the Races"
                        </h2>

                        <p class="text-lg text-primary-text-muted leading-relaxed mb-6">
                            "After gaining confidence from successfully developing such an intricate app,
                            I finally had the opportunity to fully venture into freelance software development —
                            this marked the beginning of a goal I had worked so hard to achieve.
                            During the early stages of building a client base, I was still working full-time as a mechanical engineer.
                            But not long after, with a few contracts under my belt, I was able to take off like never before!"
                        </p>

                        <p class="text-lg text-primary-text-muted leading-relaxed">
                            "The contracts themselves were relatively tame, but in volume, they added up to more work than I had initially expected:
                            a few indie game proof-of-concepts, web apps, and internal company tools.
                            I was finally able to operate wide open — exploring every avenue and scratching every itch.
                            I even returned to my roots as a mechanical engineer on one project, developing a full carbon fiber steering wheel."
                        </p>
                    </div>

                    <div class="overflow-hidden py-[20px] animation-container">
                      <div class="flex whitespace-nowrap w-max">
                        <div class="inline-block animate-slide">
                            <For
                                each={
                                    let photos = photos.clone();
                                    move || photos.clone()
                                }
                                key=|photo| format!("{}", photo)
                                children=move |photo| {
                                    let photo = format!("static/{}", photo);
                                    view! {
                                        <img
                                            src=photo
                                            alt=""
                                            class="h-[200px] md:h-[300px] lg:h-[400px] mx-10 inline-block"
                                        />
                                    }
                                }
                            />
                        </div>

                        <div class="inline-block animate-slide">
                            <For
                                each=move || photos.clone()
                                key=|photo| format!("{}", photo)
                                children=move |photo| {
                                    let photo = format!("static/{}", photo);
                                    view! {
                                        <img
                                            src=photo
                                            alt=""
                                            class="h-[200px] md:h-[300px] lg:h-[400px] mx-10 inline-block"
                                        />
                                    }
                                }
                            />
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </Page>
    }
}

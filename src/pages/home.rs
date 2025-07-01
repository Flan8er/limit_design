use leptos::prelude::*;
use leptos_icons::Icon;
use leptos_router::hooks::use_navigate;
use leptos_verlet::prelude::*;

use crate::components::id_card::SpawnIdCard;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="w-full flex flex-col gap-4">
            <div class="grid grid-cols-1 md:grid-cols-[4fr_3fr] gap-4 w-full p-4">
                <div class="w-full h-full relative p-4 flex md:flex-col">
                    <div class="absolute inset-0 grid-background bg-grid-20 z-[-1]"/>

                    <section>
                        <h1>
                            <span class="text-accent leading-[1.2] text-[calc(clamp(40px,4vw,72px))]">"Fullstack web developer "</span>
                            <span class="leading-[1.2] text-primary-text text-[calc(clamp(40px,4vw,72px))]">"and designer"</span>
                        </h1>
                        <h2 class="text-primary-text-muted leading-[1.2]">
                            "Perusing my passion for combining design with functionality."
                        </h2>
                    </section>

                    <div class="flex md:mt-6 md:w-full md:h-full md:flex-col md:justify-between">
                        <div class="flex max-md:hidden items-center gap-4">
                            <IconContainer link="https://react.dev">
                                <Icon icon=icondata::BiReact width="100%" height="100%" style="fill: #58C4DC;"/>
                            </IconContainer>
                            <IconContainer link="https://nextjs.org">
                                <Icon icon=icondata::SiNextdotjs width="100%" height="100%"/>
                            </IconContainer>
                            <IconContainer link="https://www.typescriptlang.org">
                                <Icon icon=icondata::BiTypescript width="100%" height="100%" style="fill: #3178C6;"/>
                            </IconContainer>
                            <IconContainer link="https://www.rust-lang.org">
                                <Icon icon=icondata::FaRustBrands width="100%" height="100%" style="fill: black;"/>
                            </IconContainer>
                            <IconContainer link="https://tailwindcss.com">
                                <Icon icon=icondata::BiTailwindCss width="100%" height="100%" style="fill: #00BCFF;"/>
                            </IconContainer>
                            <IconContainer link="https://leptos.dev">
                                <LeptosIcon/>
                            </IconContainer>
                        </div>
                        // have one section for quick references to skill/technologies
                        // have another section for contact stuff
                        <div class="flex flex-col md:flex-row gap-4 md:items-center">
                            <IconContainer link="https://github.com/Flan8er">
                                <Icon icon=icondata::AiGithubFilled width="100%" height="100%"/>
                            </IconContainer>
                            <IconContainer link="https://linkedin.com/in/casey-vaughn-1a8ba72b2">
                                <Icon icon=icondata::BiLinkedin width="100%" height="100%" style="fill: #0966C2;"/>
                            </IconContainer>
                            <IconContainer link="mailto:info@example.com">
                                <Icon icon=icondata::BiAtRegular width="100%" height="100%"/>
                            </IconContainer>

                            <ServiceButton/>
                        </div>
                    </div>
                </div>

                <div class="w-full aspect-[3/4]">
                    <VerletConfigProvider
                        simulation_settings=SimulationSettings{
                            jerk_damping: 0.5,
                            simulation_bounds: SimulationBounds::new(false, false, false),
                            ..default()
                        }
                    />

                    <SpawnIdCard/>
                </div>
            </div>
            <div class="w-full h-[400px] bg-secondary-background"></div> // tertiary-background
            <div class="w-full h-[400px] bg-tertiary-background"></div>
        </div>
    }
}

#[component]
fn ServiceButton() -> impl IntoView {
    let navigate = use_navigate();
    let icon_style = RwSignal::new(String::from(
        "transition-duration: 300ms; transform: rotate(-45deg);",
    ));

    view! {
        <div
            class="max-md:hidden ml-auto bg-accent w-[175px] rounded-md hover:scale-[1.05] duration-300 p-4 hover:px-2 flex justify-between items-center cursor-default"
            on:mouseenter=move |_| icon_style.set(String::from("transition-duration: 300ms; transform: rotate(0deg);"))
            on:mouseleave=move |_| icon_style.set(String::from("transition-duration: 300ms; transform: rotate(-45deg);"))
            on:click=move |_| navigate("/catalog", Default::default())
        >
            <h3 class="text-secondary-text font-medium">"View services"</h3>
            <Icon icon=icondata::HiArrowSmallRightOutlineLg width="24px" height="24px"
                style=icon_style
            />
        </div>
    }
}

#[component]
fn IconContainer(children: Children, link: &'static str) -> impl IntoView {
    view! {
        <a
            href=link
            target="_blank"
            rel="noopener noreferrer"
            class="bg-card rounded-sm h-[36px] w-[36px] flex items-center justify-center p-[6px] hover:bg-card-active hover:scale-[0.85] duration-300 cursor-default"
        >
            {children()}
        </a>
    }
}

#[component]
fn LeptosIcon() -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            xml:space="preserve"
            id="Layer_1"
            x="0"
            y="0"
            style="width: 100%; height: 100%;"
            version="1.1"
            viewBox="0 0 116 116"
        >
            <style id="style2"></style>
            <path
                id="path6"
                d="M81.9 15c3.7 0 6.9-2.1 8.5-5.1-5.3-3.6-11.3-6.3-17.6-8-.4 1.1-.6 2.2-.6 3.4 0 5.3 4.4 9.7 9.7 9.7z"
                class="st1"
                fill="#EF3B38"
            />
            <circle
                id="circle10"
                cx="58"
                cy="58"
                r="22.4"
                class="st1"
                fill="#EF3B38"
            />
            <path
                id="path12"
                d="M78.3 21c1.2.3 2.4.4 3.7.4 5.7 0 10.8-3 13.6-7.6-1.6-1.4-3.3-2.7-5.1-3.9-1.7 3-4.9 5.1-8.5 5.1-5.4 0-9.7-4.4-9.7-9.7 0-1.2.2-2.3.6-3.4-2.1-.5-4.2-1-6.3-1.3-.4 1.5-.7 3-.7 4.7 0 4.3 1.7 8.2 4.4 11-3.3 3.9-5.9 8.4-7.9 13.2-1.4-.2-2.9-.3-4.4-.3-15.9 0-28.8 12.9-28.8 28.8 0 13.2 8.9 24.3 21 27.7-3.9 9.9-11.5 18-21 22.6 2.3 1.3 4.6 2.5 7.1 3.4 9.4-5.7 16.7-14.5 20.4-25h1.4c15.9 0 28.8-12.9 28.8-28.8 0-12.2-7.6-22.6-18.2-26.8 1.7-4.1 4-7.9 6.9-11.2.8.5 1.7.8 2.7 1.1zm2.1 37c0 12.4-10.1 22.4-22.4 22.4-12.4 0-22.4-10.1-22.4-22.4 0-12.4 10.1-22.4 22.4-22.4 12.4 0 22.4 10 22.4 22.4z"
                style="fill:#fff"
            />
        </svg>
    }
}

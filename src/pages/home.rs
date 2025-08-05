use leptos::prelude::*;
use leptos_icons::Icon;

use crate::{
    components::{
        contact::ContactSection,
        icons::LeptosIcon,
        ui::{AnimatedNavButton, IconContainer},
    },
    pages::skills::{
        JavaScriptSkillCard, LeptosSkillCard, PythonSkillCard, ReactSkillCard, RustSkillCard,
        SkillCardContainer, SkillSectionHeader, TypeScriptSkillCard,
    },
};

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="w-full flex flex-col">
            <div class="grid grid-cols-1 md:grid-cols-[4fr_3fr] gap-4 w-full p-4">
                <div class="w-full h-full relative p-4 flex md:flex-col">
                    <div class="absolute inset-0 grid-background bg-grid-20 z-[-1]"/>

                    <section>
                        <h1>
                            <span class="text-accent leading-[1.2] text-[calc(clamp(40px,4vw,72px))]">"Full-stack web developer "</span>
                            <span class="leading-[1.2] text-primary-text text-[calc(clamp(40px,4vw,72px))]">"and designer"</span>
                        </h1>
                        <h2 class="text-primary-text-muted leading-[1.2]">
                            "Perusing the beauty of combining design with functionality."
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
                            <IconContainer link="mailto:casey.vaughn9@aol.com">
                                <Icon icon=icondata::BiAtRegular width="100%" height="100%"/>
                            </IconContainer>

                            <AnimatedNavButton route="catalog" title="View services" class="max-md:hidden"/>
                        </div>
                    </div>
                </div>

                <div class="w-full aspect-[3/4]">
                    <iframe class="m-0 p-0 w-full h-full bg-transparent" src="/badge_simulation"/>
                </div>
            </div>

            <div class="w-full bg-secondary-background py-8 px-4 grid grid-cols-1 lg:grid-cols-[2fr_4fr] gap-4">
                <div class="flex flex-col h-full justify-between gap-4">
                    <SkillSectionHeader title="Skills" count=14/>

                    <h3 class="text-secondary-text">"Learning new technologies can bring an incredible improvemnt over traditional approaches to app development. I strive to learn as many approaches as possible to be able to deliver the best technology for your use case."</h3>
                </div>

                <div class="flex flex-col h-full w-full items-start gap-4">
                    <SkillCardContainer class="h-full">
                        <LeptosSkillCard/>
                        <JavaScriptSkillCard/>
                        <TypeScriptSkillCard/>
                        <ReactSkillCard/>
                        <RustSkillCard/>
                        <PythonSkillCard/>
                    </SkillCardContainer>

                    <AnimatedNavButton route="skills" title="See all skills"/>
                </div>
            </div>

            <ContactSection/>
        </div>
    }
}

use leptos::prelude::*;
use leptos_icons::Icon;
use thaw::Divider;

use crate::components::{
    contact::ContactSection,
    icons::{BevyIcon, BlenderIcon, LeptosIcon, PythonIcon},
    ui::{AnimatedNavButton, SkillCard},
};

#[component]
pub fn SkillsPage() -> impl IntoView {
    view! {
        <div class="w-full h-full flex flex-col justify-between">
            <div class="w-full py-8 px-4 grid grid-cols-1 lg:grid-cols-[2fr_4fr] gap-4 bg-tertiary-background">
                <SkillSectionHeader title="Front-end" count=5 class="h-fit"/>

                <SkillCardContainer class="h-fit">
                    <LeptosSkillCard/>
                    <JavaScriptSkillCard/>
                    <TypeScriptSkillCard/>
                    <ReactSkillCard/>
                    <TailwindSkillCard/>
                </SkillCardContainer>

                <Divider class="lg:col-span-2"/>

                <SkillSectionHeader title="Systems" count=3 class="h-fit"/>

                <SkillCardContainer class="h-fit">
                    <PythonSkillCard/>
                    <RustSkillCard/>
                    <SwiftSkillCard/>
                </SkillCardContainer>

                <Divider class="lg:col-span-2"/>

                <SkillSectionHeader title="Full-stack" count=3 class="h-fit"/>

                <SkillCardContainer class="h-fit">
                    <NextSkillCard/>
                    <ElectronSkillCard/>
                    <TauriSkillCard/>
                </SkillCardContainer>

                <Divider class="lg:col-span-2"/>

                <SkillSectionHeader title="Graphics" count=3 class="h-fit"/>

                <SkillCardContainer class="h-fit">
                    <BevySkillCard/>
                    <ThreeSkillCard/>
                    <BlenderSkillCard/>
                </SkillCardContainer>
            </div>

            <div class="flex-grow bg-tertiary-background"/>

            <ContactSection/>
        </div>
    }
}

#[component]
pub fn SkillSectionHeader(
    title: &'static str,
    count: i16,
    #[prop(optional)] class: &'static str,
) -> impl IntoView {
    view! {
        <div class=format!("relative inline-flex items-center gap-x-3.5 pr-1 w-fit {}", class)>
            <div class="h-1.5 w-1.5 bg-secondary-text"></div>

            <h1 class="leading-none font-medium text-secondary-text text-[24px] md:text-[32px]">
                {title}
            </h1>

            <div class="absolute right-0 top-[-0.25em] translate-x-full text-[0.5em] text-secondary-text">
                {format!("({})", count)}
            </div>
        </div>
    }
}

#[component]
pub fn SkillCardContainer(
    #[prop(optional)] class: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=format!("{} w-full grid grid-cols-1 lg:grid-cols-3 text-secondary-text gap-4", class)>
            {children()}
        </div>
    }
}

#[component]
pub fn LeptosSkillCard() -> impl IntoView {
    view! {
        <SkillCard title="Leptos" description="Leptos harnesses Rust's strong type safety, speed, and concurrency to deliver highly performant and reliable applications.">
            <LeptosIcon/>
        </SkillCard>
    }
}

#[component]
pub fn JavaScriptSkillCard() -> impl IntoView {
    view! {
        <SkillCard title="JavaScript" description="Core technology of the World Wide Web, alongside HTML and CSS. Ninety-nine percent of websites use JavaScript on the client side for webpage behavior.">
            <Icon icon=icondata::TbBrandJavascript width="100%" height="100%" style="fill: #FEDA3D;"/>
        </SkillCard>
    }
}

#[component]
pub fn TypeScriptSkillCard() -> impl IntoView {
    view! {
        <SkillCard title="TypeScript" description="TypeScript is a strongly typed programming language that builds on JavaScript, delivering better tooling at any scale.">
            <Icon icon=icondata::BiTypescript width="100%" height="100%" style="fill: #3178C6;"/>
        </SkillCard>
    }
}

#[component]
pub fn ReactSkillCard() -> impl IntoView {
    view! {
        <SkillCard title="React" description="JavaScript library for building user interfaces, centered around reusable components and a virtual DOM for efficient rendering and state management.">
            <Icon icon=icondata::BiReact width="100%" height="100%" style="fill: #58C4DC;"/>
        </SkillCard>
    }
}

#[component]
pub fn NextSkillCard() -> impl IntoView {
    view! {
        <SkillCard title="Next.js" description="React-based web framework that enables server-side rendering, static site generation, and full-stack capabilities for building fast, scalable web applications.">
            <Icon icon=icondata::SiNextdotjs width="100%" height="100%"/>
        </SkillCard>
    }
}

#[component]
pub fn PythonSkillCard() -> impl IntoView {
    view! {
        <SkillCard title="Python" description="A high-level, versatile programming language known for its readability, simplicity, and wide range of applications from web development to data science and automation.">
            <PythonIcon/>
        </SkillCard>
    }
}

#[component]
pub fn RustSkillCard() -> impl IntoView {
    view! {
        <SkillCard title="Rust" description="System level programming language focused on performance, reliability, and memory safety, empowering developers to build fast and secure software.">
            <Icon icon=icondata::FaRustBrands width="100%" height="100%" style="fill: black;"/>
        </SkillCard>
    }
}

#[component]
pub fn SwiftSkillCard() -> impl IntoView {
    view! {
        <SkillCard title="Swift" description="Developed by Apple for building apps across iOS, macOS, and beyond, combining performance with modern syntax and safety.">
            <Icon icon=icondata::MdiLanguageSwift width="100%" height="100%" style="fill: #F05137;"/>
        </SkillCard>
    }
}

#[component]
pub fn ElectronSkillCard() -> impl IntoView {
    view! {
        <SkillCard title="Electron" description="Framework that enables developers to build cross-platform desktop applications using web technologies like HTML, CSS, and JavaScript by combining Chromium and Node.js.">
            <Icon icon=icondata::IoLogoElectron width="100%" height="100%" style="fill: #9FEAF9;"/>
        </SkillCard>
    }
}

#[component]
pub fn TauriSkillCard() -> impl IntoView {
    view! {
        <SkillCard title="Tauri" description="Lightweight, secure framework for building fast and native-feeling desktop applications using web technologies for the frontend and Rust for the backend.">
            <Icon icon=icondata::SiTauri width="100%" height="100%" style="fill: black;"/>
        </SkillCard>
    }
}

#[component]
pub fn TailwindSkillCard() -> impl IntoView {
    view! {
        <SkillCard title="Tailwind CSS" description="A utility-first CSS framework that enables rapid UI development by providing low-level, composable classes directly in your markup.">
            <Icon icon=icondata::BiTailwindCss width="100%" height="100%" style="fill: #00BCFF;"/>
        </SkillCard>
    }
}

#[component]
pub fn BevySkillCard() -> impl IntoView {
    view! {
        <SkillCard title="Bevy" description="Modern, data-driven game engine built in Rust, designed for high performance, parallelism, and developer ergonomics using an ECS (Entity Component System) architecture.">
            <BevyIcon/>
        </SkillCard>
    }
}

#[component]
pub fn ThreeSkillCard() -> impl IntoView {
    view! {
        <SkillCard title="Three.js" description="JavaScript library that enables developers to build rich, interactive 3D experiences in the browser using high-level tools built on top of WebGL.">
            <Icon icon=icondata::SiThreedotjs width="100%" height="100%" style="fill: black;"/>
        </SkillCard>
    }
}

#[component]
pub fn BlenderSkillCard() -> impl IntoView {
    view! {
        <SkillCard title="Blender" description="An open-source 3D creation suite that supports modeling, animation, simulation, rendering, and video editing in a single unified environment.">
            <BlenderIcon/>
        </SkillCard>
    }
}

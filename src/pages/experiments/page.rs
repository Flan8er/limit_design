use leptos::prelude::*;
use leptos_router::{
    components::{Outlet, ParentRoute, Route},
    path, MatchNestedRoutes,
};

use crate::{
    components::contact::ContactSection,
    pages::experiments::{
        galaga::page::GalagaExperiment, home::page::ExperimentsHome,
        node_tree::page::NodeTreeExperiment, reentry::page::ReentryExperiment,
        robot::page::RobotExperiment, verlet::page::VerletExperiment,
        waveform::page::WaveformExperiment,
    },
};

#[component(transparent)]
pub fn ExperimentRoutes() -> impl MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=path!("/experiments") view=ExperimentsPage>
            <Route path=path!("/") view=|| ExperimentsHome />
            <Route path=path!("/node-tree") view=|| NodeTreeExperiment />
            <Route path=path!("/robot-simulation") view=|| RobotExperiment />
            <Route path=path!("/verlet-simulation") view=|| VerletExperiment />
            <Route path=path!("/waveform") view=|| WaveformExperiment />
            <Route path=path!("/reentry-simulation") view=|| ReentryExperiment />
            <Route path=path!("/galaga") view=|| GalagaExperiment />
        </ParentRoute>
    }
    .into_inner()
}

#[component]
pub fn ExperimentsPage() -> impl IntoView {
    view! {
        <div class="w-full h-full flex">
            <Outlet/>
        </div>
    }
}

#[component]
pub fn ExpandedView(
    title: &'static str,
    children: Children,
    #[prop(optional)] description: &'static str,
    #[prop(optional)] description_class: &'static str,
) -> impl IntoView {
    view! {
        <div class="flex flex-col w-full h-full z-[0] items-center">
            <div class="absolute inset-0 grid-background bg-grid-20 border-none z-[-1]"/>

            <div class="p-8 w-full lg:w-[90%]">
                <div class="flex flex-col items-start justify-center py-8 w-full z-[0] h-fit md:h-[250px] gap-4">
                    <div class="flex items-center gap-x-3.5">
                        <div class="h-1.5 w-1.5 bg-accent"></div>

                        <h1 class="leading-none font-medium text-primary-text text-[24px] md:text-[32px]">
                            {title}
                        </h1>
                    </div>

                    <h3 class=format!("text-primary-text-muted {}", description_class)>{description}</h3>

                    <h3 class="text-primary-text-muted visible md:invisible">"For the best experience, view on desktop."</h3>
                </div>

                <div class="aspect-[16/9] w-full rounded object-cover">
                    {children()}
                </div>
            </div>

            <div class="flex-grow"/>

            <ContactSection/>
        </div>
    }
}

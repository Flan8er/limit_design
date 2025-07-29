use leptos::prelude::*;
use leptos_router::{
    components::{Outlet, ParentRoute, Route},
    path, MatchNestedRoutes,
};

use crate::pages::experiments::home::page::ExperimentsHome;

#[component(transparent)]
pub fn ExperimentRoutes() -> impl MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=path!("/experiments") view=ExperimentsPage>
            <Route path=path!("/") view=|| ExperimentsHome />
            <Route path=path!("/node-tree") view=|| view!{<div>node tree</div>} />
            <Route path=path!("/robot-simulation") view=|| view!{<div>robot simulation</div>} />
            <Route path=path!("/verlet-simulation") view=|| view!{<div>verlet simulation</div>} />
            <Route path=path!("/waveform") view=|| view!{<div>waveform</div>} />
            <Route path=path!("/reentry-simulation") view=|| view!{<div>reentry simulation</div>} />
            <Route path=path!("/galaga") view=|| view!{<div>galaga</div>} />
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
pub fn ExpandedView(children: Children) -> impl IntoView {
    view! {
        <div class="flex flex-col w-full h-full z-[0] items-center">
            <div class="absolute inset-0 grid-background bg-grid-20 border-none z-[-1]"/>

            <div class="p-8 w-full lg:w-[90%]">
                <div class="aspect-[16/9] w-full rounded bg-black object-cover">
                    {children()}
                </div>
            </div>
        </div>
    }
}

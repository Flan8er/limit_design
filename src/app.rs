use leptos::prelude::{Update, *};
use leptos_icons::Icon;
use leptos_meta::provide_meta_context;
use leptos_router::{
    components::{Outlet, ParentRoute, Route, Router, Routes},
    hooks::use_navigate,
    path, MatchNestedRoutes,
};
use thaw::{DrawerPosition, OverlayDrawer};

use crate::pages::{experiments::page::ExperimentRoutes, home::Home, skills::SkillsPage};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router>
            <Routes fallback=|| "Route not found...">
                <MainPageRoutes/>
            </Routes>
        </Router>
    }
}

#[component(transparent)]
pub fn MainPageRoutes() -> impl MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=path!("") view=MainPage>
            <Route path=path!("/") view=|| Home />
            <Route path=path!("/about") view=|| view!{<div>About Me</div>} />
            <ExperimentRoutes/>
            <Route path=path!("/catalog") view=|| view!{<div>Catalog</div>} />
            <Route path=path!("/skills") view=|| SkillsPage />
        </ParentRoute>
    }
    .into_inner()
}

#[component]
pub fn MainPage() -> impl IntoView {
    let navigate = use_navigate();
    let navigate_home = move |_| navigate("/", Default::default());

    let nav_menu = vec![
        ("About", "/about"),
        ("Experiments", "/experiments"),
        ("Catalog", "/catalog"),
        ("Skills", "/skills"),
    ];

    view! {
        <main class="w-full h-full relative max-w-[1580px] mx-auto bg-background">
            // Header
            <div class="glass h-[50px] md:h-[75px] border-b-[1px] absolute md:sticky top-0 w-full flex items-center z-10 relative">
                <div class="w-full h-full flex items-center">
                    // Name plate
                    <div
                        class="flex items-center gap-2 absolute left-8 md:left-12 top-1/2 -translate-y-1/2 cursor-grab"
                        on:click=navigate_home
                    >
                        <div class="bg-accent w-[10px] h-[10px]" />
                        <h3 class="uppercase">"Casey Vaughn"</h3>
                    </div>

                    <div class="max-md:hidden rounded-full px-6 py-3 absolute left-1/2 top-1/2 -translate-y-1/2 -translate-x-1/2 border flex items-center justify-center gap-12">
                        <For
                            each={
                                let nav_menu = nav_menu.clone();
                                move || nav_menu.clone()
                            }
                            key=|menu_set| format!("{}-{}", menu_set.0, menu_set.1)
                            children=move |menu_set| {
                                view! {
                                    <NavItem name=menu_set.0 absolute_route=menu_set.1/>
                                }
                            }
                        />
                    </div>

                    <div class="md:hidden flex items-center gap-2 absolute right-0 top-1/2 -translate-y-1/2 pr-[8px]">
                        <Menu menu_items=nav_menu/>
                    </div>
                </div>
            </div>



            // Content window
            <div class="w-full h-full flex">
                // Margin display
                <MainMargin1 class="left-0"/>

                // Main content
                <div class="w-full relative z-0">
                    <AnimatedStripes/>

                    <div class="w-full h-full z-[10] relative">
                        <Outlet/>
                    </div>
                </div>

                // Margin display
                <MainMargin1 class="right-0"/>
            </div>

        </main>
    }
}

#[component]
fn Menu(menu_items: Vec<(&'static str, &'static str)>) -> impl IntoView {
    let open = RwSignal::new(false);

    let on_select = {
        let navigate = use_navigate();

        Callback::new(move |absolute_route: String| {
            navigate(&absolute_route, Default::default());
        })
    };

    view! {
        <div>
            <button
                class="rounded-md"
                on:click=move |_| open.update(|v| *v = !*v)
            >
                <Icon icon=icondata::CgMenu width="24px" height="24px"/>
            </button>

            <OverlayDrawer open position=DrawerPosition::Right class="w-screen h-screen bg-white/0 backdrop-blur-[5px]">
                <button
                    class="mt-[4px] ml-auto mr-[8px]"
                    on:click=move |_| open.update(|v| *v = !*v)
                >
                    <Icon icon=icondata::CgClose width="24px" height="24px"/>
                </button>

                <div class="w-full h-full flex flex-col items-center justify-center gap-4 text-primary-text-muted">
                    <button
                        class="hover:text-primary-text"
                        on:click=move |_| {
                            on_select.run(String::from("/"));
                            open.set(false);
                        }
                    >
                        <h2>"Home"</h2>
                    </button>
                    {menu_items
                        .iter()
                        .map(|(label, route)| {
                            let label = *label;
                            let route = (*route).to_string();
                            let on_select = on_select.clone();
                            view! {
                                <button
                                    class="hover:text-primary-text"
                                    on:click=move |_| {
                                        on_select.run(route.clone());
                                        open.set(false);
                                    }
                                >
                                    <h2>{label}</h2>
                                </button>
                            }
                        })
                        .collect_view()
                    }
                </div>
            </OverlayDrawer>
        </div>
    }
}

#[component]
fn NavItem(name: &'static str, absolute_route: &'static str) -> impl IntoView {
    let navigate = use_navigate();

    let base_style = "\
        position: absolute;\
        top: -11px;\
        bottom: -11px;\
        left: -22px;\
        right: -22px;\
        background: rgba(255, 255, 255, 0.2);\
        border: 1px solid rgba(255, 255, 255, 0.5);\
        border-radius: 9999px;\
        z-index: 0;\
        transition: opacity 0.25s ease-in-out;\
    ";

    let item_style = RwSignal::new(format!("{base_style} opacity: 0;"));

    let navigate_to = move |_| navigate(absolute_route, Default::default());

    view! {
        <h3
            class="relative cursor-grab whitespace-nowrap"
        >
            <span
                style=move || item_style.get()
                on:mouseover=move |_| {
                    item_style.set(format!("{base_style} opacity: 1;"));
                }
                on:mouseleave=move |_| {
                    item_style.set(format!("{base_style} opacity: 0;"));
                }
                on:click=navigate_to
            ></span>
            {name}
        </h3>
    }
}

#[component]
fn MainMargin1(#[prop(optional)] class: &'static str) -> impl IntoView {
    view! {
        <div class=format!("w-[40px] max-md:hidden text-border border-x border-x-current bg-size-[10px_10px] bg-fixed bg-[repeating-linear-gradient(315deg,currentColor_0px,currentColor_1px,transparent_0px,transparent_10px)] h-[calc(100vh-75px)] sticky top-[75px] {}", class)/>
        <div class=format!("w-[0px] bg-fixed h-[calc(100vh-50px)] md:h-[calc(100vh-75px)] sticky top-[50px] md:top-[75px] {}", class)/>
    }
}
#[component]
fn MainMargin2(#[prop(optional)] class: &'static str) -> impl IntoView {
    view! {
        <div class=format!("w-[40px] max-md:hidden text-border border-x border-x-current margin-background-2 h-[calc(100vh-75px)] sticky top-[75px] {}", class)/>
    }
}

#[component]
fn AnimatedStripes() -> impl IntoView {
    view! {
        <div class=" absolute inset-0 overflow-hidden pointer-events-none z-0">
            <span class="shooting-star"></span>
            <span class="shooting-star"></span>
            <span class="shooting-star"></span>
            <span class="shooting-star"></span>
            <span class="shooting-star"></span>
            <span class="shooting-star"></span>
            <span class="shooting-star"></span>
            <span class="shooting-star"></span>
            <span class="shooting-star"></span>
            <span class="shooting-star"></span>
        </div>
    }
}

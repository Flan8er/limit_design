use leptos::{
    ev,
    prelude::{Update, *},
};
use leptos_icons::Icon;
use leptos_meta::provide_meta_context;
use leptos_router::{
    components::{Outlet, ParentRoute, Route, Router, Routes},
    hooks::use_navigate,
    path, MatchNestedRoutes,
};
use web_sys::{wasm_bindgen::JsCast, MouseEvent};

use crate::pages::home::Home;

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
            <Route path=path!("/aboutme") view=|| view!{<div>About Me</div>} />
            <Route path=path!("/experiments") view=|| view!{<div>Experiments</div>} />
            <Route path=path!("/catalog") view=|| view!{<div>Catalog</div>} />
            <Route path=path!("/skills") view=|| view!{<div>Skills</div>} />
        </ParentRoute>
    }
    .into_inner()
}

#[component]
pub fn MainPage() -> impl IntoView {
    let navigate = use_navigate();
    let navigate_home = move |_| navigate("/", Default::default());

    let nav_menu = vec![
        ("About Me", "/aboutme"),
        ("Experiments", "/experiments"),
        ("Catalog", "/catalog"),
        ("Skills", "/skills"),
    ];

    view! {
        <main class="w-dvw h-dvh relative max-w-[1580px] mx-auto bg-background">
            // Header
            <div class="glass h-[50px] md:h-[75px] border-b-[1px] absolute md:sticky top-0 w-full flex items-center z-10 relative">
                <div class="w-full h-full flex items-center">
                    // Name plate
                    <div
                        class="flex items-center gap-2 absolute left-8 md:left-12 top-1/2 -translate-y-1/2 cursor-default"
                        on:click=navigate_home
                    >
                        <div class="bg-accent w-[10px] h-[10px]" />
                        <h3 class="uppercase text-fluid-h3">"Casey Vaughn"</h3>
                    </div>

                    <div class="max-md:hidden rounded-full px-6 py-4 absolute left-1/2 top-1/2 -translate-y-1/2 -translate-x-1/2 border flex items-center justify-center gap-12">
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
            <div class="w-full flex h-max relative">
                // Margin display
                <MainMargin class="left-0"/>

                // Main content
                <div class="w-full h-[12000px] ">
                    <Outlet/>
                </div>

                // Margin display
                <MainMargin class="right-0"/>
            </div>
        </main>
    }
}

#[component]
fn Menu(menu_items: Vec<(&'static str, &'static str)>) -> impl IntoView {
    let open = RwSignal::new(false);
    let menu_ref: NodeRef<leptos::html::Div> = NodeRef::new();

    // Close menu on outside click
    let _ = window_event_listener(ev::click, move |ev: MouseEvent| {
        if let Some(target) = ev.target() {
            if let Some(menu_el) = menu_ref.get() {
                if let Ok(target_node) = target.dyn_into::<web_sys::Node>() {
                    if !menu_el.contains(Some(&target_node)) {
                        open.set(false);
                    }
                }
            }
        }
    });
    let _ = window_event_listener(ev::touchstart, move |ev: web_sys::TouchEvent| {
        if let Some(target) = ev.target() {
            if let Some(menu_el) = menu_ref.get() {
                if let Ok(target_node) = target.dyn_into::<web_sys::Node>() {
                    if !menu_el.contains(Some(&target_node)) {
                        open.set(false);
                    }
                }
            }
        }
    });

    let on_select = {
        let navigate = use_navigate();

        Callback::new(move |absolute_route: String| {
            navigate(&absolute_route, Default::default());
        })
    };

    view! {
        <div class="relative" node_ref=menu_ref>
            <button
                class="rounded-md"
                on:click=move |_| open.update(|v| *v = !*v)
            >
                <Icon icon=icondata::CgMenu width="24px" height="24px"/>
            </button>

            {move || open.get().then(|| view! {
                <div class="absolute right-0 mt-2 w-48 bg-tertiary-background shadow-lg rounded-md z-50">
                    {menu_items
                        .iter()
                        .map(|(label, route)| {
                            let label = *label;
                            let route = (*route).to_string();
                            let on_select = on_select.clone();
                            view! {
                                <button
                                    class="flex items-center w-full px-4 py-2 hover:bg-secondary-background rounded-md text-secondary-text"
                                    on:click=move |_| {
                                        on_select.run(route.clone());
                                        open.set(false);
                                    }
                                >
                                    {label}
                                </button>
                            }
                        })
                        .collect_view()
                    }
                </div>
            })}
        </div>
    }
}

#[component]
fn NavItem(name: &'static str, absolute_route: &'static str) -> impl IntoView {
    let navigate = use_navigate();

    let base_style = "\
        position: absolute;\
        top: -14px;\
        bottom: -14px;\
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
            class="relative cursor-default"
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
fn MainMargin(#[prop(optional)] class: &'static str) -> impl IntoView {
    view! {
        <div class=format!("w-[40px] max-md:hidden text-border border-x border-x-current bg-size-[10px_10px] bg-fixed bg-[repeating-linear-gradient(315deg,currentColor_0px,currentColor_1px,transparent_0px,transparent_10px)] h-screen sticky top-[75px] {}", class)/>
    }
}

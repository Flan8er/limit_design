use leptos::prelude::*;
use leptos_icons::Icon;
use leptos_router::hooks::use_navigate;

#[component]
pub fn SkillCard(
    title: &'static str,
    children: Children,
    // #[prop(default = "black")] fill: &'static str,
    description: &'static str,
) -> impl IntoView {
    view! {
        <div class="w-full h-full flex flex-col p-2 gap-2 border-l justify-between min-w-fit">
            <div class="flex items-center gap-2">
                <div class="h-[25px] w-[25px]">
                    {children()}
                    // <Icon icon width="100%" height="100%" style=format!("fill: {};", fill)/>
                </div>
                <h2>{title}</h2>
            </div>
            <div class="w-full">
                <p>{description}</p>
            </div>
        </div>
    }
}

#[component]
pub fn AnimatedNavButton(
    route: &'static str,
    title: &'static str,
    #[prop(optional)] class: &'static str,
) -> impl IntoView {
    let navigate = use_navigate();
    let icon_style = RwSignal::new(String::from(
        "transition-duration: 300ms; transform: rotate(-45deg); color: hsl(var(--secondary-text));",
    ));

    view! {
        <div
            class=format!("{} ml-auto bg-accent w-[175px] rounded-md hover:scale-[1.05] duration-300 p-4 hover:px-2 flex justify-between items-center cursor-default", class)
            on:mouseenter=move |_| icon_style.set(String::from("transition-duration: 300ms; transform: rotate(0deg); color: hsl(var(--secondary-text));"))
            on:mouseleave=move |_| icon_style.set(String::from("transition-duration: 300ms; transform: rotate(-45deg); color: hsl(var(--secondary-text));"))
            on:click=move |_| navigate(&format!("/{}", route), Default::default())
        >
            <h3 class="text-secondary-text font-medium">{title}</h3>
            <Icon icon=icondata::VsArrowRight width="22px" height="22px"
                style=icon_style
            />
        </div>
    }
}
#[component]
pub fn AnimatedLinkButton(
    link: &'static str,
    title: &'static str,
    #[prop(optional)] class: &'static str,
) -> impl IntoView {
    let icon_style = RwSignal::new(String::from(
        "transition-duration: 300ms; transform: rotate(-45deg); color: hsl(var(--secondary-text));",
    ));

    view! {
        <a
            class=format!("{} ml-auto bg-accent w-[175px] rounded-md hover:scale-[1.05] duration-300 p-4 hover:px-2 flex justify-between items-center cursor-default", class)
            on:mouseenter=move |_| icon_style.set(String::from("transition-duration: 300ms; transform: rotate(0deg); color: hsl(var(--secondary-text));"))
            on:mouseleave=move |_| icon_style.set(String::from("transition-duration: 300ms; transform: rotate(-45deg); color: hsl(var(--secondary-text));"))
            href=link
            target="_blank"
            rel="noopener noreferrer"
        >
            <h3 class="text-secondary-text font-medium">{title}</h3>
            <Icon icon=icondata::VsArrowRight width="22px" height="22px"
                style=icon_style
            />
        </a>
    }
}

#[component]
pub fn IconContainer(children: Children, link: &'static str) -> impl IntoView {
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

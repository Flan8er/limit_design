use leptos::prelude::*;
use leptos_icons::Icon;
use leptos_verlet::prelude::*;

use crate::pages::experiments::verlet::components::id_card::SpawnIdCard;

#[component]
pub fn ElementPane(active_modifier: RwSignal<ModificationTarget>) -> impl IntoView {
    let target_sender = expect_context::<ModificationTargetSender>();

    let modification_point = {
        let sender = target_sender.clone();
        move |_| {
            sender.send(ModificationTarget::Point).ok();
            active_modifier.set(ModificationTarget::Point)
        }
    };
    let modification_line = {
        let sender = target_sender.clone();
        move |_| {
            sender.send(ModificationTarget::Line).ok();
            active_modifier.set(ModificationTarget::Line)
        }
    };
    let modification_lock = {
        let sender = target_sender.clone();
        move |_| {
            sender.send(ModificationTarget::Lock).ok();
            active_modifier.set(ModificationTarget::Lock)
        }
    };
    let modification_cut = {
        let sender = target_sender.clone();
        move |_| {
            sender.send(ModificationTarget::Cut).ok();
            active_modifier.set(ModificationTarget::Cut)
        }
    };
    let modification_none = {
        let sender = target_sender.clone();
        move |_| {
            sender.send(ModificationTarget::None).ok();
            active_modifier.set(ModificationTarget::None)
        }
    };

    let spawn_square = {
        let sender = target_sender.clone();
        move |_| {
            sender.send(ModificationTarget::SpawnSquare).ok();
            active_modifier.set(ModificationTarget::SpawnSquare)
        }
    };
    let spawn_rope = {
        let sender = target_sender.clone();
        move |_| {
            sender.send(ModificationTarget::SpawnRope).ok();
            active_modifier.set(ModificationTarget::SpawnRope)
        }
    };
    let spawn_cloth = {
        let sender = target_sender.clone();
        move |_| {
            sender.send(ModificationTarget::SpawnCloth).ok();
            active_modifier.set(ModificationTarget::SpawnCloth)
        }
    };
    let spawn_cube = {
        let sender = target_sender.clone();
        move |_| {
            sender.send(ModificationTarget::SpawnCube).ok();
            active_modifier.set(ModificationTarget::SpawnCube)
        }
    };
    let point_info = {
        let sender = target_sender.clone();
        move |_| {
            sender.send(ModificationTarget::PointInfo).ok();
            active_modifier.set(ModificationTarget::PointInfo)
        }
    };

    view! {
        <div class="absolute top-2 left-[1/2] -translate-x-[1/2] p-1 border flex items-center justify-between rounded-md w-[calc(50dvw)] glass z-[20]">
            <div class="flex gap-4">
                <ElementPaneItem icon=icondata::CgEditBlackPoint on_click=modification_point selected_item=active_modifier this_item=Some(ModificationTarget::Point) tooltip="Add a point"/>
                <ElementPaneItem icon=icondata::BsLink45deg on_click=modification_line selected_item=active_modifier this_item=Some(ModificationTarget::Line) tooltip="Connect two point by clicking them"/>
                <ElementPaneItem icon=icondata::LuSquareSlash on_click=spawn_square selected_item=active_modifier this_item=Some(ModificationTarget::SpawnSquare) tooltip="Add a 2D square"/>
                <ElementPaneItem icon=icondata::TbCube on_click=spawn_cube selected_item=active_modifier this_item=Some(ModificationTarget::SpawnCube) tooltip="Add a 3D cube"/>
                <ElementPaneItem icon=icondata::MdiTransitConnection on_click=spawn_rope selected_item=active_modifier this_item=Some(ModificationTarget::SpawnRope) tooltip="Add a rope"/>
                <ElementPaneItem icon=icondata::CgMenuGridO on_click=spawn_cloth selected_item=active_modifier this_item=Some(ModificationTarget::SpawnCloth) tooltip="Add a cloth"/>
                <SpawnIdCard active_modifier/>
            </div>

            <div class="flex gap-4">
                <ElementPaneItem icon=icondata::AiInfoCircleOutlined on_click=point_info selected_item=active_modifier this_item=Some(ModificationTarget::PointInfo) tooltip="Modify a point's velocity"/>
                <ElementPaneItem icon=icondata::BiLockOpenAltRegular on_click=modification_lock selected_item=active_modifier this_item=Some(ModificationTarget::Lock) tooltip="Click a point so lock it in space"/>
                <ElementPaneItem icon=icondata::IoCut on_click=modification_cut selected_item=active_modifier this_item=Some(ModificationTarget::Cut) tooltip="Click and drag to cut connections between points"/>
                <ElementPaneItem icon=icondata::LuGrab on_click=modification_none selected_item=active_modifier this_item=Some(ModificationTarget::None) tooltip="Grab points and move them"/>
            </div>
        </div>
    }
}

#[component]
pub fn ElementPaneItem<F>(
    icon: icondata::Icon,
    on_click: F,
    selected_item: RwSignal<ModificationTarget>,
    this_item: Option<ModificationTarget>,
    tooltip: impl Into<String>,
) -> impl IntoView
where
    F: Fn(web_sys::MouseEvent) + Clone + 'static,
{
    let tooltip: String = tooltip.into();

    view! {
        <div
            class=move || {
                format!("w-[35px] h-[35px] rounded-md hover:bg-card-active p-[4px] cursor-grab duration-200 relative inline-block group {}",
                    if let Some(this_item) = this_item.clone() {
                        if selected_item.get() == this_item {
                            "bg-card-active"
                        } else {
                            "bg-none"
                        }
                    } else {
                        "bg-none"
                    }
                )
            }
            on:click=on_click
        >
            <span class="invisible group-hover:visible absolute z-10 w-[130px] bg-card-active text-primary-text text-center rounded-md p-[5px] top-full left-1/2 -translate-x-1/2">{tooltip}</span>
            <Icon icon width="100%" height="100%"/>
        </div>
    }
}

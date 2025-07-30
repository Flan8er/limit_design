use bevy::prelude::Event;
use codee::string::JsonSerdeCodec;
use leptos::{prelude::*, reactive::wrappers::write::SignalSetter};
use leptos_use::{use_broadcast_channel, UseBroadcastChannelReturn};
use serde::{Deserialize, Serialize};
use thaw::{NavCategory, NavCategoryItem, NavDrawer, NavItem};
use thaw_utils::OptionModel;

use crate::pages::experiments::{
    node_tree::canvas::core::node::{generate_random_tree, Node},
    page::ExpandedView,
};

#[derive(Serialize, Deserialize, Clone, PartialEq, Event)]
pub struct NodeTransmission {
    pub from: String,
    pub to: String,
}

#[component]
pub fn NodeTreeExperiment() -> impl IntoView {
    let node_tree = generate_random_tree(3, 3, 3);
    let json = serde_json::to_string(&node_tree).unwrap_or_default();
    let href = format!("/node-tree-frame?tree={}", json);

    view! {
        <ExpandedView title="MQTT Node Tree Transmission" description="The node trees are randomly generated on page load - refresh for a new tree. Select a sender (from) and a reciever (to) and visualize the data transmission.">
            <div class="w-full h-full relative mx-auto bg-background flex ">
                <div class="max-w-[250px] min-w-[250px] pl-4 py-4">
                    <NavTree node_tree=node_tree.clone()/>
                </div>

                <div class="w-full h-full">
                    <iframe class="m-0 p-0 w-full h-full bg-transparent" src=href/>
                </div>
            </div>
        </ExpandedView>
    }
}

#[component]
pub fn NavTree(node_tree: Node<String>) -> impl IntoView {
    let UseBroadcastChannelReturn { post, .. } =
        use_broadcast_channel::<NodeTransmission, JsonSerdeCodec>("node-transmission");

    let message_transmission: RwSignal<(Option<String>, Option<String>)> =
        RwSignal::new((None, None)); // (from, to)

    Effect::new(move |_| {
        let updated_message = message_transmission.get();

        if updated_message.0.is_none() || updated_message.1.is_none() {
            return;
        }

        leptos::logging::log!(
            "From: {} - To: {}",
            updated_message.0.clone().unwrap(),
            updated_message.1.clone().unwrap()
        );
        post(&NodeTransmission {
            from: updated_message.0.unwrap(),
            to: updated_message.1.unwrap(),
        });

        message_transmission.set((None, None))
    });

    let selected_value: RwSignal<Option<String>> = RwSignal::new(None);
    Effect::new(move |_| {
        let selected_value = selected_value.get();
        if let Some(selected_value) = selected_value {
            if selected_value.contains("to") {
                message_transmission.update(|c| c.1 = Some(selected_value.replace("to-", "")));
            } else {
                message_transmission.update(|c| c.0 = Some(selected_value.replace("from-", "")));
            }
        };
    });

    let selected_catagory: OptionModel<String> = (
        Signal::derive(move || String::new()),
        SignalSetter::map(move |_selected_value: String| {}),
    )
        .into();

    let from_children = node_tree.children.clone();
    let to_children = node_tree.children;

    view! {
        <NavDrawer multiple=true class="bg-background border-white border-[1px] py-3 h-fit max-h-full w-full" selected_value selected_category_value=selected_catagory>
            <NavCategory value="from">
                <NavCategoryItem slot>
                    <h3 class="text-[18px] font-mono">"From"</h3>
                </NavCategoryItem>
                {
                    from_children.into_iter().map(move |child| render_node(child, 1, "from".to_string())).collect::<Vec<_>>()
                }
            </NavCategory>
            <NavCategory value="to">
                <NavCategoryItem slot>
                    <h3 class="text-[18px] font-mono">"To"</h3>
                </NavCategoryItem>
                {
                    to_children.into_iter().map(move |child| render_node(child, 1, "to".to_string())).collect::<Vec<_>>()
                }
            </NavCategory>
        </NavDrawer>
    }
}

fn render_node(node: Node<String>, depth: usize, side: String) -> impl IntoView {
    if node.children.is_empty() && depth == 0 {
        view! {
            <NavItem value=format!("{}-{}", side, node.value)>
                <h3 style=format!("padding-left: {}px;", depth * 10) class="font-mono">
                    { node.value }
                </h3>
            </NavItem>
        }
        .into_any()
    } else if node.children.is_empty() {
        view! {
            <NavItem value=format!("{}-{}", side, node.value)>
                <h3 style=format!("padding-left: {}px;", depth * 10) class="font-mono">
                    { node.value }
                </h3>
            </NavItem>
        }
        .into_any()
    } else {
        view! {
            <NavCategory value=format!("{}-{}", side, node.value)>
                <NavCategoryItem slot>
                    <h3 style=format!("padding-left: {}px;", depth * 10) class="font-mono">
                        { node.value }
                    </h3>
                </NavCategoryItem>

                {
                    node.children
                        .into_iter()
                        .map(|child| render_node(child, depth + 1, side.clone()))
                        .collect::<Vec<_>>()
                }
            </NavCategory>
        }
        .into_any()
    }
}

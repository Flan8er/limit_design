use codee::string::JsonSerdeCodec;
use leptos::prelude::*;
use leptos_bevy_canvas::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    hooks::use_query_map,
    path,
};
use leptos_use::{use_broadcast_channel, UseBroadcastChannelReturn};

use crate::pages::experiments::node_tree::{
    canvas::core::{init::init_bevy_app, node::Node},
    page::NodeTransmission,
};

#[component]
pub fn TreeRouter() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| "404 - Page not found.">
                <Route path=path!("/node-tree-frame") view=NodeTree />
            </Routes>
        </Router>
    }
}
#[component]
pub fn NodeTree() -> impl IntoView {
    let query = use_query_map();
    let tree = Memo::new(move |_| {
        query
            .read()
            .get("tree")
            .and_then(|j| serde_json::from_str::<Node<String>>(&j).ok())
    });

    let (event_sender, bevy_event_receiver) = event_l2b::<NodeTransmission>();
    let UseBroadcastChannelReturn { message, .. } =
        use_broadcast_channel::<NodeTransmission, JsonSerdeCodec>("node-transmission");

    Effect::new(move |_| {
        if let Some(message) = message.get() {
            let _ = event_sender.send(message);
        }
    });

    view! {
        <BevyCanvas
            init=move || {
                init_bevy_app(tree.get_untracked().unwrap_or_default(), bevy_event_receiver)
            }
        />
    }
}

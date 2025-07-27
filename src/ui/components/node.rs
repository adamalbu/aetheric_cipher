use dioxus::{logger::tracing, prelude::*};

use crate::game::GameManager;

#[component]
pub fn NodeElement(id: String, game_state: Signal<GameManager>) -> Element {
    let state = game_state.read();
    let node = state.nodes.get(&id);

    match node {
        Some(node) => rsx! {
            div {
                class: "flex flex-row",
                "{node.name}: "
                "{node.get_flux_per_second()} flux/s"
                button { onclick: move |_| {
                    game_state.write()
                        .upgrade_producer(&id)
                        .unwrap_or_else(|err| {tracing::info!("{err}")});
                }, "{node.get_cost()} Flux" }
            }
        },
        None => rsx! { "Node \"{id}\" not found" },
    }
}

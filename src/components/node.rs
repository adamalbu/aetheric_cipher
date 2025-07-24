use dioxus::{logger::tracing, prelude::*};

use crate::game_info::GameState;

#[component]
pub fn Node(id: String, game_state: Signal<GameState>) -> Element {
    let state = game_state.read();
    let producer = state.producers.get(&id);

    match producer {
        Some(producer) => rsx! {
            div {
                class: "flex flex-row",
                "{producer.name}: "
                "{producer.flux_per_second} flux/s"
                button { onclick: move |_| {
                    game_state.write()
                        .upgrade_producer(&id)
                        .unwrap_or_else(|err| {tracing::info!("{err}")});
                }, "{producer.get_upgrade_cost()} Flux" }
            }
        },
        None => rsx! { "Node \"{id}\" not found" },
    }
}

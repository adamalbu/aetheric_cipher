use dioxus::prelude::*;

use crate::game_info::GameState;

#[component]
pub fn Conduit(id: String, game_state: Signal<GameState>) -> Element {
    let state = game_state.read();
    let producer = state.producers.get(&id);

    match producer {
        Some(producer) => rsx! {
            button { "Conduit: {producer:?} " }
        },
        None => rsx! { "Conduit \"{id}\" not found" },
    }
}

use dioxus::prelude::*;

use crate::game::GameManager;

#[component]
pub fn NexusInfo(id: String, game_state: Signal<GameManager>) -> Element {
    let state = game_state.read();
    let nexus = &state.nexus;

    rsx! {
        div {
            class: "flex flex-row",
            "Nexus Info: "
            "\tLevel {nexus.level}"
            "\tMultiplier {nexus.multiplier}"
            "\t{nexus.get_flux_per_second()} flux/s"
            button { onclick: move |_| {
                let _ = game_state.write().upgrade_nexus();
            }, "{nexus.get_cost()} Flux" }
        }
    }
}

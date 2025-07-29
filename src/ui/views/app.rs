use crate::game::GameManager;
use crate::ui::components::NexusInfo;

use dioxus::prelude::*;
use std::time::Duration;
use wasmtimer::{std::Instant, tokio::sleep};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const UI_UPDATE_RATE: u64 = 100;

#[component]
pub fn App() -> Element {
    let mut game_state = use_signal(|| GameManager::new_default());

    let mut dt = use_signal(|| Duration::new(0, 0));

    use_future(move || async move {
        loop {
            let start_time = Instant::now();

            sleep(Duration::from_millis(UI_UPDATE_RATE)).await;

            dt.set(start_time.elapsed());
            game_state.write().tick(dt());
        }
    });

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        pre { "Game manager: {game_state:#?} " }
        span { "Elapsed {dt:?} " }
        div {
            class: "flex flex-row",
            span { "Flux: {game_state.read().flux:.1} "}
        }
        NexusInfo { id: "node1", game_state }
    }
}

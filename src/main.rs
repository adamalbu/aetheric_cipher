use dioxus::prelude::*;
use game_info::{GameState, Producer};
use std::time::Duration;
use wasmtimer::{std::Instant, tokio::sleep};

use components::Node;

mod components;
mod game_info;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

const UI_UPDATE_RATE: u64 = 100;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut game_state = use_signal(|| GameState::new());

    use_effect(move || {
        let conduit = Producer::new(
            "Node 1".into(),
            2.0, // TODO: Change to 0 when added unlockable producers
            Box::new(|current_flux| current_flux * 2.0), // TODO: Balance
        );
        let mut state = game_state.write();
        state.producers.insert("node1".into(), conduit);
    });

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

        pre { "Game state: {game_state:#?} " }
        span { "Elapsed {dt:?} " }
        div {
            class: "flex flex-row",
            span { "Flux: {game_state.read().flux:.1} "}
        }
        Node { id: "node1", game_state }
    }
}

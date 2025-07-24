use dioxus::prelude::*;
use game_info::{GameState, Producer};
use std::time::Duration;
use wasmtimer::{std::Instant, tokio::sleep};

use components::Conduit;

mod components;
mod game_info;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut game_state = use_signal(|| GameState::new());

    use_effect(move || {
        let conduit = Producer::new(1.0);
        let mut state = game_state.write();
        state.producers.insert("conduit1".into(), conduit);
    });

    let mut dt = use_signal(|| Duration::new(0, 0));

    use_future(move || async move {
        loop {
            let start_time = Instant::now();

            sleep(Duration::from_millis(1000)).await;

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
            span { "Nodes: {game_state.read().nodes:.1} "}
        }
        Conduit { id: "conduit1", game_state }
    }
}

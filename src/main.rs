use dioxus::prelude::*;
use std::time::Duration;
use wasmtimer::std::Instant;
use wasmtimer::tokio::sleep;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

struct GameState {
    nodes: i64,
}

impl GameState {
    fn new() -> Self {
        Self { nodes: 5 }
    }
}

#[component]
fn App() -> Element {
    let mut game_state = use_signal(|| GameState::new());
    let mut count = use_signal(|| 0);
    let mut dt = use_signal(|| Duration::new(0, 0));

    use_future(move || async move {
        loop {
            count += 1;
            let start_time = Instant::now();
            sleep(Duration::from_secs(1)).await;
            dt.set(start_time.elapsed());
        }
    });

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        span { "Count: {count}" }
        span { "Elapsed {dt:?} " }
        div {
            class: "flex flex-row",
            button { onclick: move |_| game_state.write().nodes -= 1, "-" }
            span { "Nodes: {game_state.read().nodes} "}
            button { onclick: move |_| game_state.write().nodes += 1, "+" }
        }
    }
}

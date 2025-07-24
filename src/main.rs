use dioxus::prelude::*;
use std::time::Duration;
use wasmtimer::std::Instant;
use wasmtimer::tokio::sleep;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(app);
}

#[derive(Debug)]
struct Producer {
    nodes_per_second: f64,
}

impl Producer {
    fn new(nodes_per_second: f64) -> Self {
        Self { nodes_per_second }
    }
}

#[derive(Debug)]
struct GameState {
    nodes: f64,
    producers: Vec<Producer>,
}

impl GameState {
    fn new() -> Self {
        Self {
            nodes: 0.0,
            producers: Vec::new(),
        }
    }

    fn tick(&mut self, dt: Duration) {
        let dt_secs = dt.as_millis() as f64 / 1000.0;

        for producer in &self.producers {
            self.nodes += producer.nodes_per_second * dt_secs;
        }
    }
}

#[component]
fn app() -> Element {
    let mut game_state = use_signal(|| GameState::new());

    use_effect(move || {
        let conduit = Producer::new(1.0);
        game_state.write().producers.push(conduit);
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
    }
}

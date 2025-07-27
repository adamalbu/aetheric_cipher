mod game;
mod ui;

use ui::views::App;

fn main() {
    dioxus::launch(App);
}

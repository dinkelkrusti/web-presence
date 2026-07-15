use dioxus::prelude::*;

fn main() {
    launch(app);
}

#[component]
fn app() -> Element {
    rsx! {"Event Planning"}
}

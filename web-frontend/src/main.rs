use dioxus::prelude::*;
use dioxus_primitives::navbar::Navbar;

fn main() {
    launch(app);
}

#[component]
fn app() -> Element {
    rsx! {
        HeaderComponent {}
        EventComponent { name: "Recital", date: "19.06.2026"}
        FooterComponent {}
    }
}

#[component]
fn HeaderComponent() -> Element {
    rsx!(
        Navbar {
            aria_label: "Eventually"
        }
    )
}

#[component]
fn EventComponent(name: String, date: String) -> Element {
    rsx! {
        "{date}: {name}"
    }
}

#[component]
fn FooterComponent() -> Element {
    rsx!(
        div { "about" }
    )
}

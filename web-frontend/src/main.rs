use dioxus::prelude::*;

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
        h2 { "Event Planning"}
    )
}

#[component]
fn EventComponent(name: String, date: String) -> Element {
    rsx! { "{date}: {name}"}
}

#[component]
fn FooterComponent() -> Element {
    rsx!(
        div { "about" }
    )
}

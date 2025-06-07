use dioxus::prelude::*;

#[component]
pub fn NotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Not Found" }
        pre { color: "red", "log:\nattempted to navigate to: {route:?}" }
    }
}
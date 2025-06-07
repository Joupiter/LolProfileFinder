use dioxus::prelude::*;

use crate::app::Route;
use crate::models::region::Region;

#[component]
fn RegionSelector() -> Element {
    let selected_region = use_signal(|| Region::EUW);

    rsx! {
        select {
            class: "px-3 py-2 border rounded",
            value: selected_region.read().get_region_name(),
            // onchange: move |event| {
            //     if let Some(region) = Region::from_str(&event) {
            //         selected_region.set(region);
            //     }
            // },
            for region in Region::get_all().iter() {
                option {
                    value: region.to_string(),
                    selected: *selected_region.read() == *region,
                    "{region.get_region_name()}"
                }
            }
        }
    }
}

#[component]
pub fn Search() -> Element {
    let navigator = use_navigator();

    rsx! {
        div {
            class: "flex flex-col items-center justify-center min-h-screen bg-gray-50",
            h1 { class: "text-3xl font-bold text-gray-800 mb-6", "LoL Finder" }
            div {
                class: "flex space-x-2",
                RegionSelector {}
                input { class: "px-3 py-2 border rounded", placeholder: "Summoner" }
                span { class: "text-xl", "#" }
                input { class: "px-3 py-2 border rounded", placeholder: "Tag" }
                button {
                    class: "px-4 py-2 bg-blue-600 text-white rounded",
                    onclick: move |_| {
                        match navigator.push(Route::Profile { name: "Hide on bush".to_string(), tag: "KR1".to_string() }) {
                            None => (),
                            Some(_) => ()
                        }
                    },
                    "→"
                }
            }
        }
    }
}
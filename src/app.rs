use dioxus::prelude::*;

use crate::routes::{
    search::Search,
    profile::Profile,
    error::NotFound
};

pub const MAIN_CSS: Asset = asset!("/assets/main.css");
pub const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

pub const RANK_ICON_FOLDER: Asset = asset!("/assets/rank");

#[derive(Routable, Debug, Clone, PartialEq)]
pub enum Route {
    #[route("/")]
    Search {},

    #[route("/profile/:name/:tag")]
    Profile { name: String, tag: String },

    #[route("/:..route")]
    NotFound { route: Vec<String> },
}

#[component]
pub fn App() -> Element {
    rsx! {
        link { rel: "stylesheet", href: MAIN_CSS }
        link { rel: "stylesheet", href: TAILWIND_CSS }
        
        Router::<Route> {}
    }
}
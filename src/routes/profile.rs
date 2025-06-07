use dioxus::prelude::*;

use crate::models::{
    account::ProfileAccount,
    rank::Rank,
    region::Region,
};

#[derive(Clone, PartialEq, Props, Debug)]
pub struct ProfileProps {
    name: String,
    tag: String,
}

#[component]
pub fn Profile(props: ProfileProps) -> Element {
    let account = ProfileAccount {
        name: props.name,
        tag: props.tag,
        region: Region::KR,
        rank: Rank::CHALLENGER,
        level: 69,
    };

    rsx! {
        Account { account: account }
    }
}

#[component]
pub fn Account(account: ProfileAccount) -> Element {
    rsx! {
        GoBackButton { "Retour" }
        div {
            style: "padding: 20px; font-family: sans-serif;",
            h1 { "Profile" }
            div {
                style: "display: flex; align-items: center; gap: 16px;",
                
                img {
                    src: "{account.rank.get_icon_path()}",
                    width: "64px",
                    height: "64px",
                }
                
                div {
                    h2 { "{account.name}#{account.tag}" }
                    p {
                        style: "color: {account.region.get_color()};",
                        "{account.region.get_region_name()}"
                    }
                    p {
                        style: "color: {account.rank.get_color()};",
                        "{account.rank}"
                    }
                    p { "Level: {account.level}" }
                }
            }
        }
    }
}
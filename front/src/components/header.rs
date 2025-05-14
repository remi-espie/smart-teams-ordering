#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
fn Header() -> Element {
    rsx! {
        header {
            h1 {
                "Dioxus"
            }
        }
    }
}
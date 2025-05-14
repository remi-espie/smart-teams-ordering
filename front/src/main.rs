#![allow(non_snake_case)]

mod local_storage;
mod components;
mod routes;

use crate::routes::Route;
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

static CSS_BULMA: Asset = asset!("/assets/bulma.min.css");
static CSS_MAIN: Asset = asset!("/assets/main.css");
static LOGO: Asset = asset!("/assets/logo.png");


fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

fn App() -> Element {
    rsx! {
        document::Stylesheet { href: CSS_BULMA },
        document::Stylesheet { href: CSS_MAIN },
        document::Link { rel: "icon", href: LOGO },
        document::Title { "Smart Teams Ordering" },
        document::Meta { name: "viewport", content: "width=device-width, initial-scale=1" },
        document::Meta { name: "description", content: "Create teams on different subjects from users' preferences!" },
        document::Meta { name: "author", content: "Rémi Espié" },
        Router::<Route> {}
    }
}
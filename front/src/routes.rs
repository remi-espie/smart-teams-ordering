use dioxus::prelude::*;

use crate::components::teams::Teams;
use crate::components::home::Home;

#[derive(Clone, Routable, Debug, PartialEq)]
pub(crate) enum Route {
    #[route("/")]
    Home {},
    #[route("/teams/:uuid")]
    Teams { uuid: String },
}

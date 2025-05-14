use dioxus::prelude::*;
use crate::local_storage::use_persistent;
use crate::routes::Route;

static LOGO: Asset = asset!("/assets/logo.png");

#[component]
pub(crate) fn Home() -> Element {
    let nav = navigator();

    let mut team = use_persistent("team_name", || "".to_string());
    let mut team_code = use_signal(|| "".to_string());

    rsx! {
    div {
        class: "container has-text-centered",
        img {
            src: LOGO,
            alt: "Smart Teams Ordering logo",
            class: "is-256x256",
        }
        h1 {
            class: "title",
            "Smart Teams Ordering"
        }
        p {
            class: "subtitle",
            "Create teams on different subjects from users' preferences!"
        }
        div {
            class: "columns",
            div {
                class: "column",
                h2 {
                    class: "title is-4",
                    "Create a team"
                }
                form {
                    onsubmit: move |_| {
                        nav.push(Route::Teams { uuid: team.get() });
                    },
                    class: "box field is-flex is-grouped",
                    input {
                        class: "input",
                        placeholder: "Team name",
                        oninput: move |e| team.set(e.value()), value: team.get() 
                        }
                    button {
                        class: "button is-primary",
                        disabled: team.get().is_empty(),
                        "Create"
                    }
                    }
                }
            div {
                class: "column",
                h2 {
                    class: "title is-4",
                    "Join a team"
                }
                form {
                    class: "box field is-flex is-grouped",
                    input {
                        class: "input",
                        placeholder: "Team code",
                        oninput: move |e| team_code.set(e.value()), value: team_code() }
                    button {
                        class: "button is-primary",
                        disabled: team_code().is_empty(),
                        "Join"
                    }
                    }
                }
            }
        }
    }
}

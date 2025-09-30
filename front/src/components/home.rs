use dioxus::prelude::*;
use uuid::Uuid;
use crate::local_storage::use_persistent;
use crate::routes::Route;

static LOGO: Asset = asset!("/assets/logo.png");

#[component]
pub(crate) fn Home() -> Element {
    let nav = navigator();

    let mut team = use_signal(|| "".to_string());
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
                            let uuid = Uuid::new_v4();
                            let mut name = use_persistent(format!("name_{}", uuid), || "".to_string());
                            name.set(team().to_string());
                        nav.push(Route::Teams { uuid } );
                    },
                    class: "box field is-flex is-grouped",
                    input {
                        class: "input",
                        placeholder: "Team name",
                        oninput: move |e| team.set(e.value()), value: team()
                        }
                    button {
                        class: "button is-primary",
                        disabled: team().is_empty(),
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
                        onsubmit: move |_| {
                        nav.push(Route::Teams { uuid: Uuid::parse_str(team_code().as_str()).unwrap() } );
                    },
                    input {
                        class: "input",
                        placeholder: "Team code",
                        oninput: move |e| team_code.set(e.value()), value: team_code() }
                    button {
                        class: "button is-primary",
                        disabled: Uuid::parse_str(team_code().as_str()).is_err(),
                        "Join"
                    }
                    }
                }
            }
        }
    }
}

#![allow(non_snake_case)]

use crate::local_storage::use_persistent;
use dioxus::prelude::*;
use dioxus_logger::tracing::info;

#[derive(PartialEq, Props, Clone)]
pub(crate) struct TeamProps {
    uuid: String,
}

pub(crate) fn Teams(props: TeamProps) -> Element {
    let users = use_persistent(
        format!("users_{}", props.uuid),
        || vec!["User 1".to_string(), "User 2".to_string()]
    );
    let teams = use_persistent(
        format!("teams_{}", props.uuid),
        || vec!["Team 1".to_string(), "Team 2".to_string()]
    );
    let mut teams_size = use_persistent(
        format!("teams_size_{}", props.uuid),
        || vec![1, 1]
    );
    let mut preferences = use_persistent(
        format!("preferences_{}", props.uuid),
        || vec![vec![0; 2]; 2]
    );
    let is_valid = use_signal(|| false);
    let is_valid_message = use_signal(|| "Invalid dataset".to_string());

    // Helper: Validate preferences
    let mut validate_preferences = {
        let users = users.to_owned();
        let teams = teams.to_owned();
        let preferences = preferences.to_owned();
        let teams_size = teams_size.to_owned();
        let mut is_valid = is_valid.to_owned();
        let mut is_valid_message = is_valid_message.to_owned();
        move || {
            let users = users.get();
            let teams = teams.get();
            let preferences = preferences.get();
            for i in 0..users.len() {
                for j in 0..teams.len() {
                    // Check if preference is in range
                    if preferences[i][j] < 1 || preferences[i][j] > teams.len() {
                        is_valid.set(false);
                        is_valid_message
                            .set(format!("{} has no preference for {}", users[i], teams[j]));
                        return;
                    }
                    // Check if preference is unique
                    for k in 0..teams.len() {
                        if preferences[i][j] == preferences[i][k] && j != k {
                            is_valid.set(false);
                            is_valid_message.set(format!(
                                "{} has duplicate preferences for {} and {}",
                                users[i], teams[j], teams[k]
                            ));
                            return;
                        }
                    }
                }
            }
            for i in 0..teams.len() {
                // Check if team size is valid
                if teams_size.get()[i] < 1 || teams_size.get()[i] > users.len() {
                    is_valid.set(false);
                    is_valid_message.set(format!("{} has invalid team size", teams[i]));
                    return;
                }
            }
            let total_size: usize = teams_size.get().iter().sum();
            if total_size < users.len() {
                is_valid.set(false);
                is_valid_message.set(format!(
                    "Total team size ({}) is less than number of users ({})",
                    total_size,
                    users.len()
                ));
                return;
            }
            is_valid.set(true);
            is_valid_message.set("Preferences are valid!".to_string());
        }
    };

    // Add user
    let mut add_user = {
        let mut users = users.to_owned();
        let mut preferences = preferences.to_owned();
        let teams = teams.to_owned();
        let mut validate_preferences = validate_preferences;
        move || {
            let mut usr = users.get();
            let len = usr.len();
            usr.push(format!("User {}", len + 1));
            users.set(usr);

            let mut pref = preferences.get();
            pref.push(vec![0; teams.get().len()]);
            preferences.set(pref);

            validate_preferences();
        }
    };

    // Add team
    let mut add_team = {
        let mut teams = teams.to_owned();
        let mut teams_size = teams_size.to_owned();
        let mut preferences = preferences.to_owned();
        let mut validate_preferences = validate_preferences;
        move || {
            let mut tms = teams.get();
            let len = tms.len();
            tms.push(format!("Team {}", len + 1));
            teams.set(tms);

            let mut tms_size = teams_size.get();
            tms_size.push(1);
            teams_size.set(tms_size);

            let mut pref = preferences.get();
            for p in pref.iter_mut() {
                p.push(0);
            }
            preferences.set(pref);

            validate_preferences();
        }
    };

    // Rename user/team
    let mut rename_user = {
        let mut users = users.to_owned();
        let mut validate_preferences = validate_preferences;
        move |idx: usize, new_user: String| {
            let mut usr = users.get();
            if let Some(user) = usr.get_mut(idx) {
                *user = new_user;
            }
            users.set(usr);

            validate_preferences();
        }
    };
    let mut rename_team = {
        let mut teams = teams.to_owned();
        let mut validate_preferences = validate_preferences;
        move |idx: usize, new_team: String| {
            let mut tms = teams.get();
            if let Some(team) = tms.get_mut(idx) {
                *team = new_team;
            }
            teams.set(tms);

            validate_preferences();
        }
    };

    // Remove user/team
    let mut remove_user = {
        let mut users = users.to_owned();
        let mut preferences = preferences.to_owned();
        let mut validate_preferences = validate_preferences;
        move |idx: usize| {
            let mut usr = users.get();
            usr.remove(idx);
            users.set(usr);

            let mut pref = preferences.get();
            pref.remove(idx);
            preferences.set(pref);

            validate_preferences();
        }
    };
    let mut remove_team = {
        let mut teams = teams.to_owned();
        let mut teams_size = teams_size.to_owned();
        let mut preferences = preferences.to_owned();
        let mut validate_preferences = validate_preferences;
        move |idx: usize| {
            let mut tms = teams.get();
            tms.remove(idx);
            teams.set(tms);

            let mut tms_size = teams_size.get();
            tms_size.remove(idx);
            teams_size.set(tms_size);

            let mut pref = preferences.get();
            for p in pref.iter_mut() {
                p.remove(idx);
            }
            preferences.set(pref);

            validate_preferences();
        }
    };

    // Gale-Shapley algorithm
    let gale_shapley_results = use_signal(|| vec![vec![]]);

    let mut gale_shapley_loading = use_signal(|| false);
    let mut show_result_modal = use_signal(|| false);

    let mut gale_shapley = {
        let users = users.to_owned();
        let teams = teams.to_owned();
        let teams_size = teams_size.to_owned();
        let preferences = preferences.to_owned();
        let mut gale_shapley_results = gale_shapley_results.to_owned();

        move || {
            gale_shapley_loading.set(true);
            let users = users.get();
            let teams = teams.get();
            let teams_size = teams_size.get();
            let preferences = preferences.get();

            let n_users = users.len();
            let n_teams = teams.len();

            // Build user preference lists (sorted by preference)
            let user_prefs: Vec<Vec<usize>> = preferences
                .iter()
                .map(|prefs| {
                    let mut team_indices: Vec<usize> = (0..n_teams).collect();
                    team_indices.sort_by_key(|&t| prefs[t]);
                    team_indices
                })
                .collect();

            // For each team, store the indices of matched users
            let mut team_matches: Vec<Vec<usize>> = vec![Vec::new(); n_teams];

            // For each user, track which team to propose to next
            let mut next_proposal = vec![0; n_users];

            let mut free_users: Vec<usize> = (0..n_users).collect();

            while let Some(&user) = free_users.first() {
                // Propose to the next team on the user's list
                let team = user_prefs[user][next_proposal[user]];
                next_proposal[user] += 1;

                // Add user to the team's proposals
                team_matches[team].push(user);

                // If team is over capacity, reject the least preferred
                if team_matches[team].len() > teams_size[team] {
                    // Sort team members by their preference for the team
                    team_matches[team].sort_by_key(|&u| preferences[u][team]);
                    // Remove the least preferred (last in sorted order)
                    let rejected = team_matches[team].pop().unwrap();
                    if rejected != user {
                        // If the rejected user is not the current proposer, keep current user as matched
                        free_users[0] = rejected;
                    } else {
                        // Current user was rejected, so they remain free
                        continue;
                    }
                } else {
                    // User is matched, remove from free_users
                    free_users.remove(0);
                }
            }

            // Store the results in the signal
            gale_shapley_results.set(
                team_matches
                    .iter()
                    .map(|members| members.iter().map(|&u| users[u].clone()).collect())
                    .collect(),
            );

            gale_shapley_loading.set(false);
            show_result_modal.set(true);
        }
    };

    use_effect(move || {
        validate_preferences();
    });

    // Render
    rsx! {
        div { class: "container is-fluid",
            h1 { class: "title has-text-centered py-5", {props.uuid} }
            div { class: "mb-5 has-text-centered",
                p { "Define your teams and users, then set each user's preferences for the teams. Once everything is set, click on 'Sort teams' to see the optimal assignment based on the Gale-Shapley algorithm." }
                p { "You can add or remove users and teams using the '➕' and '🗑️' buttons respectively. Make sure that each user has unique preferences for the teams and that team sizes are appropriate." }
                p { "There should be at least as many total team slots as users." }
            },
            div { class: "table-container mb-6",
                table { class: "table is-striped is-hoverable",
                    thead {
                        tr {
                            th { class: "has-text-centered is-vcentered", "/" }
                            {teams.get().iter().enumerate().map(|(idx, team)| rsx! {
                                th {
                                    div { class: "field is-flex is-grouped",
                                        button {
                                            class: "button is-danger is-small",
                                            onclick: move |_| remove_team(idx),
                                            "🗑️"
                                        }
                                        input {
                                            class: "input is-static",
                                            placeholder: "Team name",
                                            oninput: move |e| rename_team(idx, e.value()),
                                            value: team.as_str()
                                        }
                                    }
                                }
                            })}
                            th {
                                button {
                                    class: "button is-primary is-small",
                                    onclick: move |_| add_team(),
                                    "➕"
                                }
                            }
                        }
                    }
                    tbody {
                        tr {
                            th { class: "has-text-centered is-vcentered", "Team size" }
                            {teams.get().iter().enumerate().map(|(team_idx, _)| rsx! {
                                th {
                                    input {
                                        class: "input is-static",
                                        r#type: "number",
                                        placeholder: "0",
                                        min: "1",
                                        max: users.get().len().to_string(),
                                        value: teams_size.get()[team_idx].to_string(),
                                        oninput: move |e| {
                                            if let Ok(num) = e.value().parse::<usize>() {
                                                let mut tms_size = teams_size.get();
                                                tms_size[team_idx] = num;
                                                teams_size.set(tms_size);

                                                validate_preferences();
                                            } else {
                                                info!("Invalid input: {}", e.value());
                                            }
                                        }
                                    }
                                }
                            })}
                        }
                        {users.get().iter().enumerate().map(|(idx, user)| rsx! {
                            tr {
                                td {
                                    div { class: "field is-flex is-grouped",
                                        button {
                                            class: "button is-danger is-small",
                                            onclick: move |_| remove_user(idx),
                                            "🗑️"
                                        }
                                        input {
                                            class: "input is-static",
                                            placeholder: "User name",
                                            oninput: move |e| rename_user(idx, e.value()),
                                            value: user.as_str()
                                        }
                                    }
                                }
                                {teams.get().iter().enumerate().map(|(team_idx, _)| rsx! {
                                    td {
                                        input {
                                            class: "input is-static",
                                            r#type: "number",
                                            placeholder: "0",
                                            min: "1",
                                            max: teams.get().len().to_string(),
                                            value: preferences.get()[idx][team_idx].to_string(),
                                            oninput: move |e| {
                                                if let Ok(num) = e.value().parse::<usize>() {
                                                    let mut pref = preferences.get();
                                                    pref[idx][team_idx] = num;
                                                    preferences.set(pref);

                                                    validate_preferences();
                                                } else {
                                                    info!("Invalid input: {}", e.value());
                                                }
                                            }
                                        }
                                    }
                                })}
                            }
                        })}
                    }
                    tfoot {
                        td {
                            button {
                                class: "button is-primary is-small",
                                onclick: move |_| add_user(),
                                "➕"
                            }
                        }
                    }
                }
            }
            div {
                class: if is_valid() {"notification is-success mb-5"} else {"notification is-danger mb-5"},
                div { class: "is-flex is-flex-direction-row is-justify-content-center",
                    p { class: "is-size-4 mr-5", {is_valid_message()} }
                    button {
                        disabled: !is_valid(),
                        class: if gale_shapley_loading() {"button is-loading is-link"} else {"button is-link"},
                        onclick: move |_| gale_shapley(),
                        "Sort teams"
                    }
                }
            }
            if show_result_modal() {
                div { class: "modal is-active",
                    div {
                        class: "modal-background",
                        onclick: move |_| show_result_modal.set(false),
                    }
                    div { class: "modal-content",
                        div { class: "box",
                            h2 { class: "title is-4", "Team ordering results" }
                            table { class: "table is-striped is-hoverable is-fullwidth",
                                thead {
                                    tr {
                                        {teams.get().iter().map(|team| rsx! {
                                            th { {team.as_str()} }
                                        })}
                                    }
                                }
                                tbody {
                                    {
                                        // Transpose the result matrix
                                        let max_team_size = gale_shapley_results().iter().map(|team| team.len()).max().unwrap_or(0);
                                        (0..max_team_size).map(|i| rsx! {
                                            tr {
                                                {gale_shapley_results().iter().map(|team| rsx! {
                                                    td {
                                                        if let Some(user) = team.get(i) {
                                                            {user.as_str()}
                                                        }
                                                    }
                                                })}
                                            }
                                        })
                                    }
                                }
                            }
                        }
                    }
                    button {
                        onclick: move |_| show_result_modal.set(false),
                        class: "modal-close is-large",
                        aria_label: "close"
                    }
                }
            }
        }
    }
}

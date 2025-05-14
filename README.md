# Smart Teams Ordering

Create teams on different subjects from users' preferences!
<img src="front/assets/logo.png" alt="Smart Teams Ordering logo" width="256" height="256">

## Overview

This web application allows users to create teams based on their preferences. One can add teams and set a maximum size for each, add users, and set their preferences for each team.

Using the [Gale-Shapley](https://en.wikipedia.org/wiki/Gale%E2%80%93Shapley_algorithm) (stable matching) algorithm, the application will then create the best possible teams based on the users' preferences.

## Development

### Frontend

The frontend was developed in Rust using [Dioxus](https://dioxuslabs.com/).

- Run the following command in the `front` folder to start the Dioxus dev server:
```bash
cd front
dx serve --hot-reload
```
- Open the browser to http://localhost:8080

### Backend

The Backend is a (non-existent) work-in-progress, with the goal to be able to share a team with other users, edit the teams together, and temporarily save the teams in a database.
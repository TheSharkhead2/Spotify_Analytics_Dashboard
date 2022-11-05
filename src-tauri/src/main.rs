#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::State;
use spotifyrs::Spotify;

mod analysis;
use analysis::{get_recent_song_data_feature};

mod playlist_info;
use playlist_info::{get_all_playlist_tiles};

// constants for the app
const SCOPE: &str = "user-read-private user-read-recently-played playlist-read-private playlist-read-collaborative";
const LOCALHOST_PORT: &str = "8888";
const CLIENT_CREDS_PATH: &str = "../.client_creds";

// authenticate a new user
#[tauri::command]
fn authorization_codeflow(state: State<'_, Spotify>) {
    state.inner().authenticate(String::from(LOCALHOST_PORT), String::from(SCOPE)).unwrap(); // authorize api
    state.inner().save_to_file(CLIENT_CREDS_PATH).unwrap(); // save access token to file
}

// check to see if the user is already authenticated
#[tauri::command]
fn authenticated(state: State<'_, Spotify>) -> bool {
    state.inner().is_authenticated()
}

fn main() {
    let state = match Spotify::new_from_file(CLIENT_CREDS_PATH) {
        Ok(spotify) => {
            spotify.save_to_file(CLIENT_CREDS_PATH).unwrap(); // save to file once authenticated
            spotify
        },
        Err(_) => Spotify::new(),
    };

    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![authorization_codeflow, authenticated, get_recent_song_data_feature, get_all_playlist_tiles])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


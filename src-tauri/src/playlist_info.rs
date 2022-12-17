use spotifyrs::{Spotify, Playlist, SpotifyError, Track, FeatureTrack}; 
use tauri::State;

use crate::analysis::get_song_features;

/// Get information for producing playlist "tiles" for all user's playlists. 
/// Tiles are essentially a small preview of the playlist. 
/// 
/// Returns a vector of tuples containing the playlist name, description, and image url
/// 
#[tauri::command]
pub fn get_all_playlist_tiles(state: State<'_, Spotify>) -> Vec<(String, String, String, String)> {
    let mut playlists: Vec<Playlist> = Vec::new(); // vector of playlists

    let mut offset: i32 = 0; // offset for getting playlists 
    let mut total: i32 = 51; // total number of playlists user has. Start at 51 to enter while loop at least once
    
    while offset+50 < total {
        let user_playlists = state.inner().get_current_users_playlists(Some(50), Some(offset)); // get chunk of playlists 

        match user_playlists {
            Ok(mut user_playlists) => {
                total = user_playlists.total; // update total number of playlists user has
                offset += 50; // update offset for getting playlists

                playlists.append(&mut user_playlists.items); // add chunk of playlists to vector of playlists
            },
            Err(e) => {println!("Error: {:?}", e); break}, // do nothing on error (don't add to playlists vector, simply allow empty vector to be returned)
        }
    }

    let trimmed_playlists: Vec<(String, String, String, String)> = playlists.iter().map(|playlist| {
        (String::from(&playlist.name), match &playlist.description {Some(description) => String::from(description), None => String::new()}, String::from(&playlist.images[0].url), String::from(&playlist.id)) // tuple of playlist name, description, image url, and id 
    }).collect();

    trimmed_playlists
}

/// Get all tracks for specified playlist (could need to convert to dynamically grabbing for massive playlists) 
/// and get track features
/// 
fn get_playlist_tracks(state: State<'_, Spotify>, playlist_id: String) -> () {
    let mut tracks: Vec<(Track, FeatureTrack)> = Vec::new(); // vector of tracks and features 

    let mut track_objects: Vec<Track> = Vec::new(); // vector of just track objects
    let mut track_ids: Vec<String> = Vec::new(); // vector of just track ids
    let mut offset: i32 = 0; // offset for getting tracks
    let mut total: i32 = 101; // total number of tracks in playlist. Start at 51 to enter while loop at least once
    while offset+100 < total {
        let playlist_tracks = state.inner().get_playlist_tracks(&playlist_id, None, Some(100), Some(offset)); // get chunk of tracks

        match playlist_tracks {
            Ok(playlist_tracks) => {
                total = playlist_tracks.total; // update total number of tracks 
                offset += 100; // update offset for getting tracks 

                for track in &playlist_tracks.items {
                    track_ids.push(format!("{}", &track.track.id)); // strip track ids out 
                }

            },
            Err(e) => {},
        }

    }

}

/// Get information on single playlist for detailed view 
/// 
#[tauri::command]
pub fn get_playlist(state: State<'_, Spotify>, playlist_id: String) -> () {
    let playlist: Result<Playlist, SpotifyError> = state.inner().get_playlist(&playlist_id, None);

    // get playlist cover image and spotify url
    let (playlist_image, spotify_url) = match playlist {
        Ok(playlist) => {
            (String::from(&playlist.images[0].url), String::from(&playlist.spotify_url)) // update image url
        },
        Err(e) => {println!("Error: {:?}", e); (String::new(), String::new())}, // do nothing on error (set image url to empty string)
    };

}
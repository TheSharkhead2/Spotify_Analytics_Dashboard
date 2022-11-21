use spotifyrs::{Spotify, Playlist}; 
use tauri::State;

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
            Err(e) => {println!("Error: {:?}", e)}, // do nothing on error (don't add to playlists vector, simply allow empty vector to be returned)
        }
    }

    let trimmed_playlists: Vec<(String, String, String, String)> = playlists.iter().map(|playlist| {
        (String::from(&playlist.name), match &playlist.description {Some(description) => String::from(description), None => String::new()}, String::from(&playlist.images[0].url), String::from(&playlist.id)) // tuple of playlist name, description, image url, and id 
    }).collect();

    trimmed_playlists
}
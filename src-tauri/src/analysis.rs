use spotifyrs::{Spotify, SpotifyError, FeatureTrack}; 
use tauri::State;
use average::Mean;

fn get_recent_song_ids(spotify: &Spotify) -> Result<Vec<String>, SpotifyError> {
    let tracks = spotify.get_recently_played_tracks(None, None, Some(50))?; // grab recently played songs 

    let ids: Vec<String> = tracks.items.iter().map(|track| format!("{}", track.track.id)).collect(); // strip ids out of tracks

    return Ok(ids); // return ids as vector of strings
}

/// Get audio features for a set of songs 
/// 
pub fn get_song_features(spotify: &Spotify, ids: Vec<&str>) -> Result<Vec<Vec<f64>>, SpotifyError> {
    let mut audio_features: Vec<FeatureTrack> = Vec::new(); // blank vector to account for possible >100 track ids 
    
    let mut offset: i32 = 0; // offset for getting audio features 
    let mut total: usize = ids.len(); // total number of tracks in playlist 
    
    while total > 0 { // loop until all tracks are analyzed 
        let slice_size = if total > 100 {100} else {total}; // get size of slice to analyze
        let mut audio_ids_chunk: Vec<&str> = vec![""; slice_size]; // blank vec for chunk of track ids (of correct size for copying)
        audio_ids_chunk.copy_from_slice(&ids[(offset as usize)..(slice_size as usize)]); // copy chunk of track ids to audio_ids_chunk

        let audio_features_chunk = spotify.get_tracks_audio_features(audio_ids_chunk); // analyze chunk of ids 

        match audio_features_chunk { 
            Ok(mut audio_features_chunk) => {
                total -= audio_features_chunk.len(); // update total number of tracks 
                offset += audio_features_chunk.len() as i32; // update offset for getting audio features 

                audio_features.append(&mut audio_features_chunk); // add chunk of audio features to vector of audio features 
            },
            Err(e) => {println!("Error: {:?}", e); break}, // do nothing on error (don't add to audio_features vector, simply allow empty vector to be returned)
        }
    }

    // turn audio feature structs into vectors of floats for usability
    let data = audio_features
        .iter().map( // map audio features to a vector of vectors of floats
            |features| vec![
                features.acousticness, 
                features.danceability, 
                features.energy, 
                features.instrumentalness, 
                features.liveness, 
                features.speechiness, 
                features.valence
                    ]
        ).collect(); // strip data out of audio features
    
    return Ok(data)
}

/// Get audio features for recently played songs
/// 
fn get_recent_song_data(spotify: &Spotify) -> Result<Vec<Vec<f64>>, SpotifyError> {
    let ids = get_recent_song_ids(spotify)?; // get ids of recently played songs 
    let ids_as_references: Vec<&str> = ids.iter().map(|id| &**id).collect(); // convert from Vec<String> to Vec<&str>

    return get_song_features(spotify, ids_as_references) // get audio features of recently played songs
}

#[tauri::command]
pub fn get_recent_song_data_feature(spotify: State<'_, Spotify>, feature: &str) -> (Vec<f64>, f64) {
    let data = get_recent_song_data(&spotify.inner()).unwrap(); // get data from spotify about recent songs 

    // get specific audio feature
    let mut feature_data: Vec<f64> = match feature {
        "acousticness" => data.iter().map(|song| song[0]).collect(),
        "danceability" => data.iter().map(|song| song[1]).collect(),
        "energy" => data.iter().map(|song| song[2]).collect(),
        "instrumentalness" => data.iter().map(|song| song[3]).collect(),
        "liveness" => data.iter().map(|song| song[4]).collect(),
        "speechiness" => data.iter().map(|song| song[5]).collect(),
        "valence" => data.iter().map(|song| song[6]).collect(),
        _ => Vec::new(),
    };

    feature_data.reverse(); // reverse data so that it is in chronological order 

    let average_value: Mean = feature_data.iter().collect(); // get average value

    return (feature_data, average_value.mean())
}
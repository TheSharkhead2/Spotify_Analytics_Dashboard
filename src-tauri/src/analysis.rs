use spotifyrs::{Spotify, SpotifyError}; 
use tauri::State;
use average::Mean;

fn get_recent_song_ids(spotify: &Spotify) -> Result<Vec<String>, SpotifyError> {
    let tracks = spotify.get_recently_played_tracks(None, None, Some(50))?; // grab recently played songs 

    let ids: Vec<String> = tracks.items.iter().map(|track| format!("{}", track.track.id)).collect(); // strip ids out of tracks

    return Ok(ids)
}

fn get_recent_song_data(spotify: &Spotify) -> Result<Vec<Vec<f64>>, SpotifyError> {
    let ids = get_recent_song_ids(spotify)?; // get ids of recently played songs 

    let audio_features = spotify.get_tracks_audio_features(ids.iter().map(|s| s.as_ref()).collect())?; // get audio features of recently played songs

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
# Spotify Analytics Dashboard

This app is currently in a very early stage of development and doesn't have a proper name yet, hence the very generic name. The goal of this project is to create an application that allows you to gain insight into listening trends over time. Traditionally, most Spotify "data dashboards" give you cumulative data, for example: "recently the songs you listened to had an average valence of 0.4." However, this app aims to show you what kinds of trends exist in your listening from song to song: did the valence in your songs increase as you listened during the day or did it decrease? The goal is for this application to be the ultimate tool for people who like order, with a coming emphasis on playlists as well. 

## Technology 

This app has a backend built with [Tauri](https://tauri.app/), which allows the backend to be written in Rust. The frontend is built with [Svelte](https://svelte.dev/) and graphing functionality through [D3.js](https://d3js.org/). Additionally, integration with the [Spotify API](https://developer.spotify.com/documentation/web-api/) is supplied by my own [Spotify API Wrapper](https://crates.io/crates/spotifyrs), which is being developed alongside this application. 

## Roadmap 
- Descriptions of recent listening history graphs, including: descriptions of the various Spotify track features and what they mean, insights into trends, and on hover information on each datapoint.
- Ability to look at "net change" over time as an attribute/feature in the recent listening history graph. This would represent, in a sense, the derivative of the path traced by the listening history in the 7 feature dimensions and 1 time dimension (8 dimensional space representing the listening history, which just isn't feasible for humans to visualize alone).
- Ability to graph multiple features over time, rather than just one at a time. 
- Playlist analytics 
    - Same analytics as recently listening history, however with position in the playlist as the "time" variable. Essentially, how ordered is the playlist? 
    - Cumulative data on features and statistics. Average valence, speechiness, etc. Artist and genre makeup. 
- Ability to reorder playlist according to preferences. 
    - Ordering based on valence over time, acousticness over time, etc. 
    - Ability to order over a combination of the above, going from high to low or low to high. 
    - Ordering based on "net change" from previous song. 
    - Randomized ordering (shuffling basically), though based on maximizing the net difference between each song and the ones around it. 
        - Ideally this would allow for shuffling a playlist like this straight into the queue as shuffling into a seperate playlist then playing that playlist without shuffle every time is inconvenient
- Insights into groupings and patterns in entire music library (clustering?)
    - Better recommendations from this data. Possibly a subset of this app will continue my previous work predicting likelihood of enjoying a song: [Spotify Recommendations](https://github.com/TheSharkhead2/SpotifyRecommendations).
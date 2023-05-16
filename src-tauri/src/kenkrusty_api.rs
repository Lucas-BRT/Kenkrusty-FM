

#[allow(dead_code)]
pub mod controls {

    use reqwest::Client;
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    #[derive(Debug, Deserialize, Serialize)]
    struct Sound {
        id: String,
        url: String,
        title: String,
        #[serde(default)]
        loop_sound: bool,
        #[serde(default)]
        volume: f32,
        #[serde(default)]
        fade_in: u32,
        #[serde(default)]
        fade_out: u32,
    }
    
    #[derive(Debug, Deserialize, Serialize)]
    struct Soundboard {
        id: String,
        sounds: Vec<String>,
        background: String,
        title: String,
    }
    
    #[derive(Debug, Deserialize, Serialize)]
    pub struct SoundboardResponse {
        soundboards: Vec<Soundboard>,
        sounds: Vec<Sound>,
    }
    
    #[derive(Debug, Deserialize, Serialize)]
    struct Playlist {
        id: String,
        tracks: Vec<String>,
        background: String,
        title: String,
    }
    
    #[derive(Debug, Deserialize, Serialize)]
    struct Track {
        id: String,
        url: String,
        title: String,
    }
    
    #[derive(Debug, Deserialize, Serialize)]
    pub struct PlaylistResponse {
        playlists: Vec<Playlist>,
        tracks: Vec<Track>,
    }
    
    pub async fn get_soundboards(url: &str, client: &reqwest::Client) -> Result<SoundboardResponse, reqwest::Error> {
        let soundboard_url = "/v1/soundboard";
        let url = format!("{}{}", url, soundboard_url);
    
        let response = client.get(url).send().await.unwrap();
    
        response.json().await
    }
    
    pub async fn play_soundboard(url: &str, client: & reqwest::Client, id: &str) -> Result<(), reqwest::Error> {
        let soundboard_url_play = "/v1/soundboard/play";
        let url = format!("{}{}", url, soundboard_url_play);
        let json_id = json!({ "id": id , "volume" : 0.5});
    
    
        client
            .put(url)
            .header("Content-Type", "application/json")
            .json(&json_id)
            .send().await.unwrap();
    
        Ok(())
    }
    
    pub async fn play_playlist(url: &str, client: &Client, id: &str) -> Result<(), reqwest::Error> {
        let playlist_url_play = "/v1/playlist/play";
        let url = format!("{}{}", url, playlist_url_play);
        let json_id = json!({ "id": id });
    
        client
            .put(url)
            .header("Content-Type", "application/json")
            .json(&json_id)
            .send().await.unwrap();
    
        Ok(())
    }
    
    pub async fn get_playlists(url: &str, client: &Client) -> Result<PlaylistResponse, reqwest::Error> {
        let playlist_url = "/v1/playlist";
        let url = format!("{}{}", url, playlist_url);
    
        let response = client.get(url).send().await.unwrap();
    
        response.json().await
    }
    
    pub async fn playback_play(url: &str, client: &Client) -> Result<(), reqwest::Error> {
        let play_url = "/v1/playlist/playback/play";
        let url = format!("{}{}", url, play_url);
    
        client.put(url).send().await.unwrap();
    
        Ok(())
    }
    
    pub async fn playback_pause(url: &str, client: &Client) -> Result<(), reqwest::Error> {
        let pause_url = "/v1/playlist/playback/pause";
        let url = format!("{}{}", url, pause_url);
    
        client.put(url).send().await.unwrap();
    
        Ok(())
    }
    
    pub async fn playback_next(url: &str, client: &Client) -> Result<(), reqwest::Error> {
        let next_url = "/v1/playlist/playback/next";
        let url = format!("{}{}", url, next_url);
    
        client.post(url).send().await.unwrap();
    
        Ok(())
    }
    
    pub async fn playback_previous(url: &str, client: &Client) -> Result<(), reqwest::Error> {
        let previous_url = "/v1/playlist/playback/previous";
        let url = format!("{}{}", url, previous_url);
    
        client.post(url).send().await.unwrap();
    
        Ok(())
    }
    
    pub async fn playback_mute(url: &str, client: &Client, mute: bool) -> Result<(), reqwest::Error> {
        let play_url = "/v1/playlist/playback/mute";
        let url = format!("{}{}", url, play_url);
        let json_mute = json!({ "mute": mute });
    
        client
            .put(url)
            .header("Content-Type", "application/json")
            .json(&json_mute)
            .send().await.unwrap();
    
        Ok(())
    }
    
    pub async fn playback_volume(url: &str, client: &Client, mut volume: f32) -> Result<(), reqwest::Error> {
        if volume < 0.0 {
            volume = 0.0;
        } else if volume > 1.0 {
            volume = 1.0
        } else {
            volume = volume
        };
    
        let volume_url = "/v1/playlist/playback/volume";
        let url = format!("{}{}", url, volume_url);
        let json_volume = json!({ "volume": volume });
        
        client
            .put(&url)
            .header("Content-Type", "application/json")
            .json(&json_volume)
            .send().await.unwrap();
    
        Ok(())
    }
    
    pub async fn playback_shuffle(url: &str, client: &Client, shuffle: bool) -> Result<(), reqwest::Error> {
        let play_url = "/v1/playlist/playback/shuffle";
        let url = format!("{}{}", url, play_url);
        let json_shuffle = json!({ "shuffle": shuffle });
    
        client
            .put(url)
            .header("Content-Type", "application/json")
            .json(&json_shuffle)
            .send().await.unwrap();
    
        Ok(())
    }
    
    pub enum Repeat {
        Track,
        Playlist,
        Off,
    }
    
    pub async fn playback_repeat(url: &str, client: &Client, repeat: Repeat) -> Result<(), reqwest::Error> {
        let play_url = "/v1/playlist/playback/repeat";
        let url = format!("{}{}", url, play_url);
        let json_repeat = json!({ "repeat": match repeat {
            Repeat::Track => "track",
            Repeat::Playlist => "playlist",
            Repeat::Off => "off"
        } });
    
        client
            .put(url)
            .header("Content-Type", "application/json")
            .json(&json_repeat)
            .send().await.unwrap();
    
        Ok(())
    }
}





use reqwest::{blocking::Client, Response};
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
struct SoundboardResponse {
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
struct PlaylistResponse {
    playlists: Vec<Playlist>,
    tracks: Vec<Track>,
}

fn get_soundboards(url: &str, client: &Client) -> Result<SoundboardResponse, reqwest::Error> {
    let soundboard_url = "/v1/soundboard";
    let url = format!("{}{}", url, soundboard_url);

    let response = client.get(url).send()?;

    response.json()
}

fn play_soundboard(url: &str, client: &Client, id: &str) -> Result<(), reqwest::Error> {
    let soundboard_url_play = "/v1/soundboard/play";
    let url = format!("{}{}", url, soundboard_url_play);
    let json_id = json!({ "id": id , "volume" : 0.5});

    let response = client
        .put(url)
        .header("Content-Type", "application/json")
        .json(&json_id)
        .send()?;

    Ok(())
}

fn play_playlist(url: &str, client: &Client, id: &str) -> Result<(), reqwest::Error> {
    let playlist_url_play = "/v1/playlist/play";
    let url = format!("{}{}", url, playlist_url_play);
    let json_id = json!({ "id": id });

    let response = client
        .put(url)
        .header("Content-Type", "application/json")
        .json(&json_id)
        .send()?;

    Ok(())
}

fn get_playlists(url: &str, client: &Client) -> Result<PlaylistResponse, reqwest::Error> {
    let playlist_url = "/v1/playlist";
    let url = format!("{}{}", url, playlist_url);

    let response = client.get(url).send()?;

    response.json()
}

fn playback_play(url: &str, client: &Client) -> Result<(), reqwest::Error> {
    let play_url = "/v1/playlist/playback/play";
    let url = format!("{}{}", url, play_url);

    let Response = client.put(url).send()?;

    Ok(())
}

fn playback_pause(url: &str, client: &Client) -> Result<(), reqwest::Error> {
    let pause_url = "/v1/playlist/playback/pause";
    let url = format!("{}{}", url, pause_url);

    let Response = client.put(url).send()?;

    Ok(())
}

fn playback_next(url: &str, client: &Client) -> Result<(), reqwest::Error> {
    let next_url = "/v1/playlist/playback/next";
    let url = format!("{}{}", url, next_url);

    let Response = client.post(url).send()?;

    Ok(())
}

fn playback_previous(url: &str, client: &Client) -> Result<(), reqwest::Error> {
    let previous_url = "/v1/playlist/playback/previous";
    let url = format!("{}{}", url, previous_url);

    let Response = client.post(url).send()?;

    Ok(())
}

fn playback_mute(url: &str, client: &Client, mute: bool) -> Result<(), reqwest::Error> {
    let play_url = "/v1/playlist/playback/mute";
    let url = format!("{}{}", url, play_url);
    let json_mute = json!({ "mute": mute });

    let Response = client
        .put(url)
        .header("Content-Type", "application/json")
        .json(&json_mute)
        .send()?;

    Ok(())
}

fn main() {
    let base_url = "http://127.0.0.1:3333";
    let client = Client::new();

    let sounboard_test = "0461e8c7-c71b-4081-baef-edc8372a4086";
    let playlist_test = "3d97633e-ae77-4b85-b134-80cb67854137";
}

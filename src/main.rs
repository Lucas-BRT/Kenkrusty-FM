use reqwest::blocking::Client;
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

///

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
    let json_id = json!({ "id": id });

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

fn main() {
    let base_url = "http://127.0.0.1:3333";
    let client = Client::new();

    let test_id = "3d97633e-ae77-4b85-b134-80cb67854137";

    play_playlist(base_url, &client, test_id).unwrap();
}

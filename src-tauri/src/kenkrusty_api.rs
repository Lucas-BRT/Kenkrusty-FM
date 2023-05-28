use reqwest::Client;
use rocket::response;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
enum SoundType {
    Track,
    Sound,
}

#[derive(Debug, Serialize, Deserialize)]
enum Repeat {
    #[serde(rename = "track")]
    Track,
    #[serde(rename = "playlist")]
    Playlist,
    #[serde(rename = "off")]
    Off,
}

/* ------------------------------------------------------------------------------------------ */

#[derive(Debug, Deserialize, Serialize)]
struct Sound {
    url: String,
    title: String,
    id: String,
    #[serde(rename = "loop")]
    repeat: bool,
    volume: f32,
    #[serde(rename = "fadeIn")]
    fade_in: u32,
    #[serde(rename = "fadeOut")]
    fade_out: u32,
    #[serde(default)]
    progress: Option<f32>,
    #[serde(default)]
    duration: Option<f32>,
    #[serde(default)]
    sound_type: Option<SoundType>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Soundboard {
    id: String,
    background: String,
    title: String,
    sounds: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct SoundboardResponse {
    soundboards: Vec<Soundboard>,
    sounds: Vec<Sound>,
}

/* ------------------------------------------------------------------------------------------ */

#[derive(Debug, Deserialize, Serialize)]
struct Playlist {
    #[serde(default)]
    id: String,
    #[serde(default)]
    background: String,
    #[serde(default)]
    title: String,
    #[serde(default)]
    tracks: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Track {
    url: String,
    title: String,
    id: String,
    #[serde(default)]
    sound_type: Option<SoundType>,
    #[serde(default)]
    progress: Option<u32>,
    #[serde(default)]
    duration: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize)]
struct PlaylistResponse {
    playlists: Vec<Playlist>,
    tracks: Vec<Track>,
}

/* ------------------------------------------------------------------------------------------ */

#[derive(Debug, Deserialize, Serialize)]
struct PlaylistPlaybackResponse {
    playing: bool,
    volume: f32,
    muted: bool,
    repeat: Repeat,
    #[serde(default)]
    track: Option<Track>,
    #[serde(default)]
    playlist: Option<Playlist>,
}

#[derive(Debug, Deserialize, Serialize)]
struct SoundboardPlaybackResponse {
    sounds: Vec<Sound>,
}

/* ------------------------------------------------------------------------------------------ */

struct Controller {
    ip: String,
    port: String,
    client: Client,
    url: String,
}

impl Controller {
    pub fn new(ip: String, port: String) -> Controller {
        let client = Client::new();
        let url = format!("http://{ip}:{port}");
        Controller {
            ip: ip,
            port: port,
            client: client,
            url: url,
        }
    }

    pub async fn get_sounds(&self) -> Result<SoundboardResponse, reqwest::Error> {
        let sounds_path = "v1/soundboard";
        let url = format!("{}/{}", self.url, sounds_path);

        println!("{}",url);

        let sounds = self
            .client
            .get(url)
            .send()
            .await?
            .json::<SoundboardResponse>()
            .await?;

        Ok(sounds)
    }

    pub async fn get_tracks(&self) -> Result<PlaylistResponse, reqwest::Error> {
        let sounds_path = "v1/playlist";
        let url = format!("{}/{}", self.url, sounds_path);

        let tracks = self
            .client
            .get(url)
            .send()
            .await?
            .json::<PlaylistResponse>()
            .await?;

        Ok(tracks)
    }
}

struct MediaBoard {
    sounds: Vec<Sound>,
    tracks: Vec<Track>,
    controller: Controller,
}

impl MediaBoard {
    pub async fn new(ip: String, port: String) -> MediaBoard {
        let controller = Controller::new(ip, port);

        let sounds = controller.get_sounds().await.unwrap().sounds;
        let tracks = controller.get_tracks().await.unwrap().tracks;

        MediaBoard {
            sounds: sounds,
            tracks: tracks,
            controller: controller,
        }
    }
}

pub async fn teste() {

    let ip = "127.0.0.1".to_string();
    let port = "3333".to_string();

    let kenkrusty = MediaBoard::new(ip, port).await;

    let response = kenkrusty.sounds;


}

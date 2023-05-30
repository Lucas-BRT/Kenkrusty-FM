use std::thread::sleep;

use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize)]
enum SoundType {
    Track,
    Sound,
}

#[derive(Debug, Serialize, Deserialize,Clone)]
pub enum Repeat {
    #[serde(rename = "track")]
    Track,
    #[serde(rename = "playlist")]
    Playlist,
    #[serde(rename = "off")]
    Off,
}


pub fn repeat_default() -> Repeat{
    Repeat::Off
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

impl Sound {
    async fn play(&self, controller: &Controller) -> Result<(), reqwest::Error> {
        let client = controller.get_client();
        let url = format!("{}/soundboard/play", controller.url);

        let data = json!( {"id": self.id});

        client
            .put(url)
            .header("Content-Type", "application/json")
            .json(&data)
            .send()
            .await?;

        Ok(())
    }

    async fn stop(&self, controller: &Controller) -> Result<(), reqwest::Error> {
        let client = controller.get_client();
        let url = format!("{}/soundboard/stop", controller.url);

        let data = json!( {"id": self.id});

        client
            .put(url)
            .header("Content-Type", "application/json")
            .json(&data)
            .send()
            .await?;

        Ok(())
    }
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

impl Track {
    async fn play(&self, controller: &Controller) -> Result<(), reqwest::Error> {
        let client = controller.get_client();
        let url = format!("{}/playlist/play", controller.url);

        println!("{}", url);


        let data = json!( {"id": self.id});

        client
            .put(url)
            .header("Content-Type", "application/json")
            .json(&data)
            .send()
            .await?;

        Ok(())
    }
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
    shuffle: bool,
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

#[derive(Debug, Deserialize, Serialize,Clone)]
struct Command {
    #[serde(default)]
    mute: bool,
    #[serde(default)]
    shuffle: bool,
    #[serde(default = "repeat_default")]
    repeat: Repeat,

}

impl Command {
    fn new() -> Command {

        Command {
            mute: false,
            shuffle: false,
            repeat: Repeat::Off
        }


    }


    async fn play_playback(&self,controller: &Controller) -> Result<(),reqwest::Error> {
            let client = controller.get_client();
            let url = format!("{}/playlist/play", controller.url);
    
            client
                .put(url)
                .send()
                .await?;
    
            Ok(())
    }




}

#[derive(Clone)]
struct Controller {
    ip: String,
    port: String,
    v: String,
    client: Client,
    url: String,
    command: Command,
}

impl Controller {
    pub fn new(ip: String, port: String) -> Controller {
        let client = Client::new();
        let v = "v1".to_string();
        let url = format!("http://{ip}:{port}/{v}");

        Controller {
            ip: ip,
            port: port,
            v: v,
            client: client,
            url: url,
            command: Command::new()
        }
    }

    pub async fn get_sounds(&self) -> Result<SoundboardResponse, reqwest::Error> {
        let sounds_path = "soundboard";
        let url = format!("{}/{}", self.url, sounds_path);

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
        let tracks_path = "playlist";
        let url = format!("{}/{}", self.url, tracks_path);


        let tracks = self
            .client
            .get(url)
            .send()
            .await?
            .json::<PlaylistResponse>()
            .await?;

        Ok(tracks)
    }

    pub fn get_client(&self) -> Client {
        self.client.to_owned()
    }

    pub fn get_url(&self) -> String {
        self.url.to_owned()
    }

    async fn get_playback(&self) -> Result<PlaylistPlaybackResponse,reqwest::Error>  {
        let client = self.get_client();
        let url = format!("{}/playlist/playback", self.get_url());

        let response = client
            .get(url)
            .send()
            .await?
            .json::<PlaylistPlaybackResponse>()
            .await?;

        Ok( response )
    }

    async fn play_playback(&self) -> Result<StatusCode,reqwest::Error> {
        let client = self.get_client();
        let url = format!("{}/playlist/playback/play", self.get_url());

        let response = client
            .get(url)
            .send()
            .await?
            .status();

        Ok(response)
    }

    async fn pause_playback(&self) -> Result<StatusCode,reqwest::Error> {
        let client = self.get_client();
        let url = format!("{}/playlist/playback/pause", self.get_url());

        let response = client
            .get(url)
            .send()
            .await?
            .status();

        Ok(response)
    }

    async fn next_playback(&self) -> Result<StatusCode,reqwest::Error> {
        let client = self.get_client();
        let url = format!("{}/playlist/playback/next", self.get_url());

        let response = client
            .get(url)
            .send()
            .await?
            .status();

        Ok(response)
    }

    async fn previous_playback(&self) -> Result<StatusCode,reqwest::Error> {
        let client = self.get_client();
        let url = format!("{}/playlist/playback/previous", self.get_url());

        let response = client
            .get(url)
            .send()
            .await?
            .status();

        Ok(response)
    }





}

pub async fn is_kenku_remote_avaliable(ip: &String, port: &String) -> bool {
    let kenku_url = format!("http://{ip}:{port}");

    match reqwest::get(kenku_url).await {
        Ok(_) => true,
        Err(_) => false,
    }

    //    let playlist_online = reqwest::get( format!("{}/playlist",&kenku_url) ).await.unwrap().status() == StatusCode::OK;

    //    soundboard_online && playlist_online
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

    fn get_controller(&self) -> Controller {
        self.controller.to_owned()
    }
}

pub async fn teste() {
    let ip = "127.0.0.1".to_string();
    let port = "3333".to_string();

    if is_kenku_remote_avaliable(&ip, &port).await {
        let kenkrusty = MediaBoard::new(ip, port).await;

        println!("{:#?}", kenkrusty.tracks[0]);

        let battle1 = &kenkrusty.tracks[0];

        battle1.play(&kenkrusty.controller).await.unwrap();





        





        /* 
        let coruja = &kenkrusty.sounds[1];
        coruja.play(&kenkrusty.controller).await.unwrap();
        sleep(Duration::from_secs(5));
        coruja.stop(&kenkrusty.controller).await.unwrap();
        */






    }


}

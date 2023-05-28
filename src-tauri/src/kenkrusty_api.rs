use crate::kenku_remote_api;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

const DEFAULT_ADDRESS: &str = "127.0.0.1";
const DEFAULT_PORT: &str = "3333";

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum SoundType {
    Soundboard,
    Playlist,
}

trait SoundPlayer {
    async fn play(
        &self,
        controller: controller::Controller,
        id: String,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let sound_type = match self.get_soundtype() {
            SoundType::Playlist => "playlist",
            SoundType::Soundboard => "soundboard",
        };

        let json_id = json!({ "id" : id });

        let play_url = format!(
            "http://{}:{}/{}/{}",
            controller.address, controller.port, controller.path_v, sound_type
        );

        controller
            .sender
            .put(play_url)
            .header("Content-Type", "application/json")
            .json(&json_id)
            .send()
            .await
    }
    async fn get_duration(&self) -> Option<u32>;
    async fn get_progress(&self) -> Option<u32>;
    fn get_soundtype(&self) -> SoundType;
}

mod response {

    mod soundboard {
        pub struct Soundboard {
            pub id: String,
            pub sounds: Vec<Sound>,
            pub background: String,
            pub title: String,
        }

        pub struct Sound {
            pub id: String,
            pub url: String,
            pub title: String,
            pub repeat: bool,
            pub volume: f32,
            pub fade_in: u32,
            pub fade_out: u32,
        }

        pub struct SoundboardResponse {
            pub soundboards: Vec<Soundboard>,
            pub sounds: Vec<Sound>,
        }

        /* ----------------------------------------------------------------------------------- */

        pub struct SoundPlayback {
            pub id: String,
            pub url: String,
            pub title: String,
            pub repeat: bool,
            pub volume: f32,
            pub fade_in: u32,
            pub fade_out: u32,
            pub progress: f32,
            pub durarion: u32,
        }

        pub struct SoundPlaybackResponse {
            pub sounds: Vec<SoundPlayback>,
        }
    }

    mod playlist {
        use crate::kenkrusty_api::playlist;

        pub struct Playlist {
            pub id: String,
            pub tracks: Vec<Track>,
            pub background: String,
            pub title: String,
        }

        pub struct Track {
            pub id: String,
            pub url: String,
            pub title: String,
        }

        pub struct PlaylistResponse {
            pub playlists: Vec<Playlist>,
            pub tracks: Vec<Track>,
        }
        /* ----------------------------------------------------------------------------------- */

        pub struct TrackPlayback {
            pub id: String,
            pub title: String,
            pub url: String,
            pub progress: u32,
            pub duration: u32,
        }

        pub struct PlaylistPlayback {
            pub id: String,
            pub title: String,
        }

        pub struct PlaylistPlaybackResponse {
            pub playing: bool,
            pub volume: f32,
            pub muted: bool,
            pub shuffle: bool,
            pub repeat: playlist::Repeat,
            pub track: Option<TrackPlayback>,
            pub playlist: Option<PlaylistPlayback>,
        }
    }
}

mod soundcollection {

    use crate::kenkrusty_api::controller::Controller;
    use crate::kenkrusty_api::playlist::Track;
    use crate::kenkrusty_api::soundboard::Sound;
    use reqwest::Client;

    struct SoundCollection {
        soundboard: Vec<Sound>,
        playlist: Vec<Track>,
        controller: Controller,
    }

    impl SoundCollection {
        pub fn new() {
            let controller = Client::new();
        }

        pub async fn play(id: &str) {}
    }
}

mod controller {

    use reqwest::Client;

    pub struct Controller {
        pub sender: Client,
        pub address: String,
        pub port: String,
        pub path_v: String,
    }

    impl Controller {
        fn new(address: String, port: String) -> Controller {
            let sender = Client::new();
            let path_v = String::from("v1");

            Controller {
                sender,
                address,
                port,
                path_v,
            }
        }
    }
}

pub mod soundboard {

    use super::SoundType;
    use crate::kenku_remote_api::controls::soundboard::SoundboardResponse;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct Sound {
        pub local_path: String,
        pub title: String,
        pub id: String,
        pub repeat: bool,
        pub volume: f32,
        pub fadein: u32,
        pub fadeout: u32,
        pub progress: Option<u32>,
        pub duration: Option<u32>,
        pub soundtype: SoundType,
    }

    impl super::SoundPlayer for Sound {
        async fn get_duration(&self) -> Option<u32> {
            self.duration
        }

        async fn get_progress(&self) -> Option<u32> {
            self.progress
        }

        fn get_soundtype(&self) -> SoundType {
            self.soundtype
        }
    }

    impl Sound {
        async fn stop(&self) {}

        async fn get_playback(&self) {}
    }

    pub async fn get_sounds(
        url: &str,
        client: &reqwest::Client,
    ) -> Result<Vec<Sound>, reqwest::Error> {
        let soundboard_path = "/v1/soundboard";
        let url = format!("{}{}", url, soundboard_path);

        println!("{url}");

        let response = client
            .get(url)
            .send()
            .await
            .unwrap()
            .json::<SoundboardResponse>()
            .await?;

        let sounds: Vec<Sound> = response
            .sounds
            .iter()
            .map(|sound| Sound {
                local_path: sound.url.clone(),
                title: sound.title.clone(),
                id: sound.id.clone(),
                repeat: sound.loop_sound,
                volume: sound.volume,
                fadein: sound.fade_in,
                fadeout: sound.fade_out,
                progress: None,
                duration: None,
                soundtype: SoundType::Soundboard,
            })
            .collect();

        println!("sounds: {:#?}", sounds);

        Ok(sounds)
    }
}

pub mod playlist {

    use super::SoundType;

    pub enum Repeat {
        Track,
        Playlist,
        Off,
    }

    pub struct Track {
        pub local_path: String,
        pub title: String,
        pub id: String,
        pub duration: Option<u32>,
        pub progress: Option<u32>,
        pub soundtype: SoundType,
    }

    impl super::SoundPlayer for Track {
        async fn get_duration(&self) -> Option<u32> {
            self.duration
        }

        async fn get_progress(&self) -> Option<u32> {
            self.progress
        }

        fn get_soundtype(&self) -> SoundType {
            self.soundtype
        }
    }

    impl Track {
        async fn pause(&self) {}

        async fn next(&self) {}

        async fn previous(&self) {}

        async fn mute(&self) {}
    }
}

pub mod playback {

    use crate::kenkrusty_api::playlist::Track;
    use crate::kenkrusty_api::soundboard::Sound;
    use crate::kenkrusty_api::SoundType;

    struct Playback {
        soundboard: Vec<Sound>,
        playlist: Option<Track>,
    }

    impl Playback {}

    pub async fn get_sounds(
        url: &str,
        client: &reqwest::Client,
    ) -> Result<Vec<Sound>, reqwest::Error> {
        let playback_url = "/v1/soundboard/playback";
        let url = format!("{}{}", url, playback_url);

        let response = client.get(url).send().await?.json::<Vec<Sound>>().await?;

        let sounds: Vec<Sound> = response
            .iter()
            .map(|sound| Sound {
                local_path: sound.local_path.as_str().to_string(),
                title: sound.title.as_str().to_string(),
                id: sound.id.as_str().to_string(),
                repeat: sound.repeat,
                volume: sound.volume,
                fadein: sound.fadein,
                fadeout: sound.fadeout,
                progress: sound.progress,
                duration: sound.duration,
                soundtype: SoundType::Soundboard,
            })
            .collect();

        Ok(sounds)
    }
}

pub async fn test() {
    let url = format!("http://{}:{}", DEFAULT_ADDRESS, DEFAULT_PORT);
    let client = Client::new();

    kenku_remote_api::check_server_availability(DEFAULT_ADDRESS, DEFAULT_PORT)
        .await
        .unwrap();
}

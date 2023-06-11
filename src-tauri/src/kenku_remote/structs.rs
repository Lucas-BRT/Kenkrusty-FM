pub mod soundboard {
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize, Debug)]
    pub struct SoundboardResponse {
        pub soundboards: Vec<Soundboard>,
        pub sounds: Vec<Sound>,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Soundboard {
        id: String,
        background: String,
        title: String,
        sounds: Vec<String>,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Sound {
        pub url: String,
        pub title: String,
        pub id: String,
        #[serde(rename = "loop")]
        pub loop_: bool,
        pub volume: f32,
        #[serde(rename = "fadeIn")]
        pub fadein: u32,
        #[serde(rename = "fadeOut")]
        pub fadeout: u32,
    }

    pub mod playback {
        use serde::{Deserialize, Serialize};

        #[derive(Deserialize, Serialize, Debug)]
        pub struct SoundboardPlayback {
            sounds: Vec<Sound>,
        }

        #[derive(Deserialize, Serialize, Debug)]
        pub struct Sound {
            url: String,
            title: String,
            id: String,
            #[serde(rename = "loop")]
            loop_: bool,
            volume: f32,
            #[serde(rename = "fadeIn")]
            fadein: u32,
            #[serde(rename = "fadeOut")]
            fadeout: u32,
            progress: f32,
            duration: u32,
        }
    }
}

pub mod playlist {
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize, Debug)]
    pub struct PlaylistResponse {
        playlists: Vec<Playlist>,
        tracks: Vec<Track>,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Playlist {
        id: String,
        background: String,
        title: String,
        tracks: Vec<String>,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Track {
        url: String,
        title: String,
        id: String,
    }

    pub mod playback {
        use serde::{Deserialize, Serialize};

        #[derive(Deserialize, Serialize, Debug)]
        pub struct PlaylistPlayback {
            playing: bool,
            volume: f32,
            muted: bool,
            shuffle: bool,
            repeat: Repeat,
            track: Option<Track>,
            playlist: Option<Playlist>,
        }

        #[derive(Deserialize, Serialize, Debug)]
        #[serde(rename_all = "lowercase")]
        pub enum Repeat {
            Track,
            Playlist,
            Off,
        }

        #[derive(Deserialize, Serialize, Debug)]
        pub struct Playlist {
            id: String,
            title: String,
        }

        #[derive(Deserialize, Serialize, Debug)]
        pub struct Track {
            url: String,
            title: String,
            id: String,
            progress: u32,
            duration: u32,
        }
    }
}

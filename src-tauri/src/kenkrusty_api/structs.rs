use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Deserialize, Serialize, Debug)]
pub enum SoundType {
    Track,
    Sound,
    Custom,
}

impl FromStr for SoundType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "track" => Ok(Self::Track),
            "sound" => Ok(Self::Sound),
            "custom" => Ok(Self::Custom),
            _ => Err(()),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub enum TypeCommand {
    Play,
    Pause,
    Stop,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ActionControl {
    pub id: String,
    pub title: String,
    pub sound_type: SoundType,
    pub type_commands: Vec<TypeCommand>,
}

pub struct Controller {
    pub ip: String,
    pub port: String,
    pub client: Client,
}

impl Controller {
    pub fn new(ip: String, port: String) -> Controller {
        let client = Client::new();

        Controller {
            ip: ip,
            port: port,
            client: client,
        }
    }
}

/*
    playlists
        dar play na faixa
        dar play no playback
        dar pause no playback

    soundboards
        dar play na faixa
        dar stop na faixa

    mediaboard
        dar play
        dar stop
*/

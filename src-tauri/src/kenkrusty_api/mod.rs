use crate::kenku_remote::commands::soundboard::get_soundboard;
use reqwest::Client;
use rocket::State;
use serde::{Deserialize, Serialize};
pub use std::str::FromStr;

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

#[derive(Clone)]
pub struct Controller {
    pub ip: String,
    pub port: String,
    pub client: Client,
}

impl Controller {
    pub fn new(ip: String, port: String) -> Controller {
        let client = Client::new();

        Controller { ip, port, client }
    }
}

pub async fn get_sounds(
    controller: &State<Controller>,
) -> Result<Vec<ActionControl>, reqwest::Error> {
    let response = get_soundboard(&controller.client, &controller.ip, &controller.port).await?;

    let sounds: Vec<ActionControl> = response
        .sounds
        .iter()
        .map(|sound| ActionControl {
            id: sound.id.clone(),
            title: sound.title.clone(),
            sound_type: SoundType::Sound,
            type_commands: vec![TypeCommand::Play, TypeCommand::Stop],
        })
        .collect();

    Ok(sounds)
}

use crate::kenkrusty_api::structs::*;
use crate::kenku_remote::commands;
use rocket::State;

pub async fn get_sounds(controller: &State<Controller>) -> Result<Vec<ActionControl>, reqwest::Error> {
    let response =
        commands::soundboard::get_soundboard(&controller.client, &controller.ip, &controller.port)
            .await?;

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

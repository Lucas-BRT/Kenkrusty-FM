use std::str::FromStr;

use serde_json::json;

use crate::kenkrusty_api::commands::*;
use crate::kenkrusty_api::structs::{Controller, SoundType};
use crate::kenku_remote::commands::soundboard::play_soundbaord;

#[get("/media/<media_type>")]
pub async fn get_media(controller: &rocket::State<Controller>, media_type: String) -> String {
    let media_type = SoundType::from_str(&media_type.as_str()).unwrap();
    println!("{:#?}", media_type);

    let content = match media_type {
        SoundType::Track => {
            let response = get_sounds(controller).await.unwrap();

            json!(response)
        }
        SoundType::Sound => {
            let response = get_sounds(controller).await.unwrap();

            json!(response)
        }
        SoundType::Custom => {
            let response = get_sounds(controller).await.unwrap();

            json!(response)
        }
    };

    content.to_string()
}

#[get("/play/<id>/<media_type>")]
pub async fn play_media(controller: &rocket::State<Controller>, id: String, media_type: String) {
    let media_type = SoundType::from_str(&media_type.as_str()).unwrap();
    println!("{:#?}", media_type);

    match media_type {
        SoundType::Track => {
            let response = get_sounds(controller).await.unwrap();

            json!(response)
        }
        SoundType::Sound => {
            let response = play_soundbaord(
                &controller.client,
                &controller.ip.as_str(),
                &controller.port,
                &id,
            )
            .await
            .unwrap();
            println!("{}", response);
            json!(response.as_str())
        }
        SoundType::Custom => {
            let response = get_sounds(controller).await.unwrap();

            json!(response)
        }
    };
}

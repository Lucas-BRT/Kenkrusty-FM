pub mod base;

use self::base::prepare_base;
use crate::kenkrusty_api::Controller;

use rocket::fs::FileServer;

use serde_json::json;

use crate::kenkrusty_api::*;

use crate::kenku_remote::commands::soundboard::play_soundbaord;
use rocket::State;

#[get("/media/<media_type>")]
pub async fn get_media(media_type: String, controller: &State<Controller>) -> String {
    let controller_ref = controller;

    let media_type = SoundType::from_str(media_type.as_str()).unwrap();
    println!("{:#?}", media_type);

    let content = match media_type {
        SoundType::Track => {
            let response = get_sounds(controller_ref).await.unwrap();
            json!(response)
        }
        SoundType::Sound => {
            let response = get_sounds(controller_ref).await.unwrap();
            json!(response)
        }
        SoundType::Custom => {
            let response = get_sounds(controller_ref).await.unwrap();
            json!(response)
        }
    };

    content.to_string()
}

#[get("/play/<id>/<media_type>")]
pub async fn play_media(id: String, media_type: String, controller: &State<Controller>) {
    let media_type = SoundType::from_str(media_type.as_str()).unwrap();
    println!("{:#?}", media_type);

    match media_type {
        SoundType::Track => {
            let response = get_sounds(controller).await.unwrap();

            json!(response)
        }
        SoundType::Sound => {
            let response = play_soundbaord(
                &controller.client,
                controller.ip.as_str(),
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

pub fn launch(controller: Controller) {
    let rocket_base = prepare_base();

    let handle = tokio::spawn(async {
        let rocket_config = rocket::Config::figment()
            .merge(("address", "0.0.0.0"))
            .merge(("port", 8000))
            .merge(("workers", 4));

        rocket::custom(rocket_config)
            .manage(controller)
            .mount("/", FileServer::from(rocket_base))
            .mount("/", routes![get_media, play_media])
            .launch()
            .await
            .unwrap();
    });

    tokio::runtime::Runtime::new().unwrap().block_on(async {
        handle.await.unwrap();
    });
}

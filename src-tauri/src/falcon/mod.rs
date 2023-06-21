pub mod base;
pub mod commands;

use self::base::prepare_base;
use crate::kenkrusty_api::structs::Controller;
use commands::*;
use rocket::fs::FileServer;

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

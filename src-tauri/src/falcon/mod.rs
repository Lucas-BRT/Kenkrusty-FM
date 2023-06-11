pub mod base;
pub mod commands;
use crate::kenkrusty_api::structs::Controller;
use rocket::fs::FileServer;

use include_dir::Dir;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use self::base::prepare_base;

pub fn launch(controller: &Controller) {
    let rocket_base = prepare_base();

    let handle = tokio::spawn(async {
        let rocket_config = rocket::Config::figment()
            .merge(("address", "0.0.0.0"))
            .merge(("port", 8000))
            .merge(("workers", 4));

        rocket::custom(rocket_config)
            .mount("/", FileServer::from(rocket_base))
            .launch()
            .await
            .unwrap();
    });

    tokio::runtime::Runtime::new().unwrap().block_on(async {
        handle.await.unwrap();
    });
}

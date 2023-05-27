// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Adds suport for async traits
#![feature(async_fn_in_trait)]

mod falcon;
mod kenkrusty_api;
mod kenku_remote_api;

use std::thread;

use reqwest::Client;

#[tauri::command]
async fn conection_test() {}

#[tokio::main]
async fn main() {
    kenkrusty_api::test().await;

    /*
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![conection_test])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    */
}

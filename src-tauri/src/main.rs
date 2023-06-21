// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate rocket;
pub mod falcon;
pub mod kenkrusty_api;
pub mod kenku_remote;
pub mod local_ip;
pub mod tauron;
use tauron::*;

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![connect, get_ip_address])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

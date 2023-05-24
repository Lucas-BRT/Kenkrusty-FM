// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[macro_use]
extern crate rocket;

mod Falcon;
mod lib;
use std::thread;

#[tauri::command]
async fn conection_test() {}

#[tokio::main]
async fn main() {
    let rocket_thread = thread::spawn(|| Falcon::launch());

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![conection_test])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

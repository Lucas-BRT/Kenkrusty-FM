// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod lib;
use reqwest::{Client, Response};


#[tauri::command]
async fn __play_playlist(ip: String, port: String) {
    let client = Client::new();
    
    let url = format!("http://{ip}:{port}");
    let id = "129a92a1-8881-4395-bcb5-2bd2f7f6b091";


    match lib::play_soundboard(url.as_str(), &client, id).await {
        Ok(response) => return response,
        Err(error) => panic!("{:?}",error)
    };
}


#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![__play_playlist])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

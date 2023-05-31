// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Adds suport for async traits
#![feature(async_fn_in_trait)]

use kenkrusty_api::{Sound, Track};
mod kenkrusty_api;
mod Falcon;
 

use std::thread;


#[tauri::command]
async fn is_kenku_remote_avaliable(ip: String, port: String) -> bool {
    kenkrusty_api::is_kenku_remote_avaliable(&ip, &port).await
}

#[tauri::command]
async fn get_sounds(ip: String, port: String) -> Vec<Sound> {
    let board = kenkrusty_api::MediaBoard::new(ip, port).await;
    board.get_sounds()
}

#[tauri::command]
async fn get_tracks(ip: String, port: String) -> Vec<Track> {
    let board = kenkrusty_api::MediaBoard::new(ip, port).await;
    board.get_tracks()
}


#[tokio::main]
async fn main() {

    let macaco = thread::spawn(|| { 
        Falcon::launch();
    });
    
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![is_kenku_remote_avaliable,get_sounds,get_tracks])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

}

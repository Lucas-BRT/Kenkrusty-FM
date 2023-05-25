// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod falcon;
mod kenku_remote_api;
mod kenkrusty_api;

use std::thread;


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

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod lib;

#[tauri::command]
async fn conection_test(ip: String, port: String) {

}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![conection_test])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

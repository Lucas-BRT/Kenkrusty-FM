use crate::falcon;
use crate::kenkrusty_api::Controller;
use crate::local_ip::get_local_ips;

#[tauri::command]
pub async fn connect(ip: String, port: String) {
    let controller = Controller::new(ip, port);

    falcon::launch(controller);
}

#[tauri::command]
pub fn get_ip_address() -> String {
    let ips: Vec<(String, String)> = get_local_ips()
        .iter()
        .map(|ip| (ip.0.clone(), ip.1.to_string()))
        .collect();

    serde_json::to_string(&ips).unwrap()
}

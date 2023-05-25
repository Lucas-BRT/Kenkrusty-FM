use crate::kenku_remote_api;


pub async fn test() {
    let a = kenku_remote_api::check_server_availability("127.0.0.1","3333").await.unwrap();

}




















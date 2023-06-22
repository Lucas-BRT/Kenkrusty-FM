pub mod soundboard {

    use crate::kenku_remote::structs::*;
    use reqwest::Client;
    use reqwest::StatusCode;
    use serde_json::json;

    const ROUTE: &str = "soundboard";
    const V: &str = "v1";

    pub async fn get_soundboard(
        client: &Client,
        ip: &str,
        port: &str,
    ) -> Result<soundboard::SoundboardResponse, reqwest::Error> {
        let get_soundboard_url = format!("http://{ip}:{port}/{V}/{ROUTE}");
        let response = client
            .get(get_soundboard_url)
            .send()
            .await?
            .json::<soundboard::SoundboardResponse>()
            .await?;

        Ok(response)
    }

    pub async fn play_soundbaord(
        client: &Client,
        ip: &str,
        port: &str,
        id: &str,
    ) -> Result<StatusCode, reqwest::Error> {
        let play_soundboard_url = format!("http://{ip}:{port}/{V}/{ROUTE}/play");
        let json = json!({ "id" : id });

        let response = client
            .put(play_soundboard_url)
            .header("Content-Type", "application/json")
            .json(&json)
            .send()
            .await?
            .status();

        Ok(response)
    }

    pub async fn get_soundboard_playback(
        client: &Client,
        ip: &str,
        port: &str,
    ) -> Result<soundboard::playback::SoundboardPlayback, reqwest::Error> {
        let get_soundboard_playback_url = format!("http://{ip}:{port}/{V}/{ROUTE}/playback");
        let response = client
            .get(get_soundboard_playback_url)
            .send()
            .await?
            .json::<soundboard::playback::SoundboardPlayback>()
            .await?;

        Ok(response)
    }
}

pub mod playlist {
    use crate::kenku_remote::structs::*;
    use reqwest::{Client, StatusCode};
    use serde_json::json;

    const ROUTE: &str = "playlist";
    const V: &str = "v1";

    pub async fn get_playlist(
        client: &Client,
        ip: &str,
        port: &str,
    ) -> Result<playlist::PlaylistResponse, reqwest::Error> {
        let get_playlist_url = format!("http://{ip}:{port}/{V}/{ROUTE}");
        let response = client
            .get(get_playlist_url)
            .send()
            .await?
            .json::<playlist::PlaylistResponse>()
            .await?;

        Ok(response)
    }

    pub async fn play_playlist(
        client: &Client,
        ip: &str,
        port: &str,
        id: &str,
    ) -> Result<StatusCode, reqwest::Error> {
        let play_playlist_url = format!("http://{ip}:{port}/{V}/{ROUTE}/play");
        let json = json!({ "id": id});

        let response = client
            .put(play_playlist_url)
            .header("Content-Type", "application/json")
            .json(&json)
            .send()
            .await?
            .status();

        Ok(response)
    }

    pub async fn get_playlist_playback(
        client: &Client,
        ip: &str,
        port: &str,
    ) -> Result<playlist::playback::PlaylistPlayback, reqwest::Error> {
        let get_playlist_url = format!("http://{ip}:{port}/{V}/{ROUTE}/playback");
        let response = client
            .get(get_playlist_url)
            .send()
            .await?
            .json::<playlist::playback::PlaylistPlayback>()
            .await?;

        Ok(response)
    }

    pub async fn play_playlist_playback(
        client: &Client,
        ip: &str,
        port: &str,
    ) -> Result<StatusCode, reqwest::Error> {
        let play_playlist_playback_url = format!("http://{ip}:{port}/{V}/{ROUTE}/playback/play");

        let response = client
            .put(play_playlist_playback_url)
            .send()
            .await?
            .status();

        Ok(response)
    }

    pub async fn pause_playlist_playback(
        client: &Client,
        ip: &str,
        port: &str,
    ) -> Result<StatusCode, reqwest::Error> {
        let play_playlist_playback_url = format!("http://{ip}:{port}/{V}/{ROUTE}/playback/pause");

        let response = client
            .put(play_playlist_playback_url)
            .send()
            .await?
            .status();

        Ok(response)
    }

    pub async fn next_playlist_playback(
        client: &Client,
        ip: &str,
        port: &str,
    ) -> Result<StatusCode, reqwest::Error> {
        let play_playlist_playback_url = format!("http://{ip}:{port}/{V}/{ROUTE}/playback/next");

        let response = client
            .post(play_playlist_playback_url)
            .send()
            .await?
            .status();

        Ok(response)
    }

    pub async fn previous_playlist_playback(
        client: &Client,
        ip: &str,
        port: &str,
    ) -> Result<StatusCode, reqwest::Error> {
        let play_playlist_playback_url =
            format!("http://{ip}:{port}/{V}/{ROUTE}/playback/previous");

        let response = client
            .post(play_playlist_playback_url)
            .send()
            .await?
            .status();

        Ok(response)
    }

    pub async fn mute_playlist_playback(
        client: &Client,
        ip: &str,
        port: &str,
        mute: bool,
    ) -> Result<StatusCode, reqwest::Error> {
        let play_playlist_playback_url = format!("http://{ip}:{port}/{V}/{ROUTE}/playback/mute");
        let json = json!({"mute": mute});

        let response = client
            .put(play_playlist_playback_url)
            .header("Content-Type", "application/json")
            .json(&json)
            .send()
            .await?
            .status();

        Ok(response)
    }

    pub async fn volume_playlist_playback(
        client: &Client,
        ip: &str,
        port: &str,
        volume: f32,
    ) -> Result<StatusCode, reqwest::Error> {
        let play_playlist_playback_url = format!("http://{ip}:{port}/{V}/{ROUTE}/playback/volume");
        let json = json!({"volume": volume});

        let response = client
            .put(play_playlist_playback_url)
            .header("Content-Type", "application/json")
            .json(&json)
            .send()
            .await?
            .status();

        Ok(response)
    }

    pub async fn shuffle_playlist_playback(
        client: &Client,
        ip: &str,
        port: &str,
        shuffle: bool,
    ) -> Result<StatusCode, reqwest::Error> {
        let play_playlist_playback_url = format!("http://{ip}:{port}/{V}/{ROUTE}/playback/shuffle");
        let json = json!({"shuffle": shuffle});

        let response = client
            .put(play_playlist_playback_url)
            .header("Content-Type", "application/json")
            .json(&json)
            .send()
            .await?
            .status();

        Ok(response)
    }

    pub async fn repeat_playlist_playback(
        client: &Client,
        ip: &str,
        port: &str,
        repeat: playlist::playback::Repeat,
    ) -> Result<StatusCode, reqwest::Error> {
        let play_playlist_playback_url = format!("http://{ip}:{port}/{V}/{ROUTE}/playback/repeat");
        let json = json!({"repeat": repeat});

        let response = client
            .put(play_playlist_playback_url)
            .header("Content-Type", "application/json")
            .json(&json)
            .send()
            .await?
            .status();

        Ok(response)
    }
}

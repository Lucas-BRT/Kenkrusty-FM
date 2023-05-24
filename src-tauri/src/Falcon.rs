use include_dir::include_dir;
use rocket::fs::FileServer;
use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::sync::Mutex;
use tempfile;

const DIR: include_dir::Dir = include_dir!("../dist");

fn create_temp_dir() -> Result<tempfile::TempDir, std::io::Error> {
    tempfile::Builder::new().prefix("temp-dir").tempdir()
}

fn copy_dir_to_temp(dir: &include_dir::Dir, temp_dir: &Path) -> Result<(), std::io::Error> {
    for file in dir.files() {
        let file_path = file.path();
        let target_path = temp_dir.join(file_path.to_str().unwrap().trim_start_matches('/'));

        if let Some(parent_dir) = target_path.parent() {
            fs::create_dir_all(parent_dir)?;
        }

        let mut file_handle = fs::File::create(&target_path)?;
        file_handle.write_all(file.contents())?;
    }
    Ok(())
}

#[rocket::main]
pub async fn launch() {
    let temp_dir = create_temp_dir().expect("Failed to create temporary directory");
    let temp_dir_path = temp_dir.path();

    copy_dir_to_temp(&DIR, temp_dir_path).expect("Failed to copy directory to temporary directory");

    let dist_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../dist");

    let rocket_config = rocket::Config::figment()
        .merge(("address", "0.0.0.0"))
        .merge(("port", 8000))
        .merge(("workers", 4));

    rocket::custom(rocket_config)
        .mount("/", FileServer::from(dist_path))
        .manage(Mutex::new(0u32))
        .launch()
        .await
        .expect("Rocket server failed to launch");
}

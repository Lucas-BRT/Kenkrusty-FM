use include_dir::include_dir;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use tempfile;

static DIR: include_dir::Dir = include_dir!("../kenkrusty_control");

fn create_temp_dir() -> Result<tempfile::TempDir, std::io::Error> {
    tempfile::Builder::new().prefix("temp-dir").tempdir()
}

fn copy_dir_to_temp(dir: &include_dir::Dir, temp_dir: &Path) -> Result<(), std::io::Error> {
    for entry in dir.files() {
        let entry_path = PathBuf::from(temp_dir).join(entry.path());
        let entry_dir = entry_path.parent().unwrap();

        if !entry_dir.exists() {
            fs::create_dir_all(entry_dir).unwrap();
        }

        let mut file = File::create(&entry_path).unwrap();
        file.write_all(entry.contents()).unwrap();
    }
    Ok(())
}

pub fn prepare_base() -> PathBuf {
    let temp_dir = create_temp_dir().expect("Failed to create temporary directory");
    let temp_dir_path = temp_dir.path();
    copy_dir_to_temp(&DIR, temp_dir_path).expect("Failed to copy directory to temporary directory");
    println!("{:#?}", DIR);
    println!("{:#?}", temp_dir);
    temp_dir.into_path()
}

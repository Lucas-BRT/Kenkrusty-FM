use include_dir::include_dir;
use include_dir::Dir;
use std::path::Path;
use std::path::PathBuf;
use tempfile;

use std::io;

static DIR: include_dir::Dir = include_dir!("../mobile_UI");

fn create_temp_dir() -> Result<tempfile::TempDir, std::io::Error> {
    tempfile::Builder::new().prefix("temp-dir").tempdir()
}

fn copy_dir_content(include_dir: &Dir, target_dir: &Path) -> io::Result<()> {
    include_dir.extract(target_dir).unwrap();
    Ok(())
}

pub fn prepare_base() -> PathBuf {
    let temp_dir = create_temp_dir().expect("Failed to create temporary directory");
    let temp_dir_path = temp_dir.path();
    copy_dir_content(&DIR, temp_dir_path).expect("Failed to copy directory to temporary directory");
    temp_dir.into_path()
}

use std::process::Command;
use std::path::Path;
use crate::config::{PUBLIC_PATH, ASSETS_PATH, IMAGE_PATH};

pub fn if_path_exists(path: &str) -> bool {
    Path::new(path).exists()
}

pub fn copy_files(source: &str, destination: &str) {
    let mut command = Command::new("cp");
    command.arg("-r");
    command.arg("-f");
    command.arg(source).arg(destination);
    command.status()
        .expect(format!("Could not copy file from: {} to {}", source, destination).as_str());
}
pub fn copy_asset_files() {
    copy_files(ASSETS_PATH, PUBLIC_PATH);
    if if_path_exists(IMAGE_PATH) {
        copy_files(IMAGE_PATH, PUBLIC_PATH);
    }
}
use std::process::Command;
use crate::config::{PUBLIC_PATH, ASSETS_PATH};

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
}
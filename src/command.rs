use std::process::Command;
use std::path::Path;
use std::env;
use crate::config::{PUBLIC_PATH, ASSETS_PATH, IMAGE_PATH};

pub fn if_path_exists(path: &str) -> bool {
    Path::new(path).exists()
}
// for unix-like
pub fn copy_files_with_shell(source: &str, destination: &str) {
    let mut command = Command::new("cp");
    command.arg("-r");
    command.arg("-f");
    command.arg(source).arg(destination);
    command.status()
        .expect(format!("Could not copy file from: {} to {}", source, destination).as_str());
}
// for windows
pub fn copy_files_with_cmd() {
    let mut command = Command::new("XCOPY");
    command.arg("/E");
    command.arg("/Y");
    command.arg(ASSETS_PATH).arg(PUBLIC_PATH);
    command.status()
        .expect(format!("Could not copy file from: {} to {}", ASSETS_PATH, PUBLIC_PATH).as_str());

}
pub fn copy_asset_files_with_shell() {
    copy_files_with_shell(ASSETS_PATH, PUBLIC_PATH);
    if if_path_exists(IMAGE_PATH) {
        copy_files_with_shell(IMAGE_PATH, PUBLIC_PATH);
    }
}

pub fn try_copy_files() {
    let os = env::consts::OS;
    println!("current os: {}", os);
    match os {
        "linux" | "macos" | "freebsd" | "openbsd" | "solaris" => copy_asset_files_with_shell(),
        "windows" => copy_files_with_cmd(),
        _ => eprintln!("auto copy assets files in current os: {} is not supported, plz do it yourslef", os)
    }
}
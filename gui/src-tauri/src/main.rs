#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::Manager;
mod blog;
mod utils;
mod img;
mod state;
fn main() {
    let app = tauri::Builder::default();
    app.manage(state::DB::init())
    .setup(|app| {
        let c = app.state::<state::DB>();
        c.setup();
        Ok(())
    })
    .invoke_handler(tauri::generate_handler![utils::get_dir_imgs,
        blog::list, blog::add_blog])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

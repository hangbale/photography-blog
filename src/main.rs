extern crate serde_json;
extern crate serde;
extern crate tera;
extern crate urlencoding;
mod config;
mod parser;
mod frame;
mod render;
mod command;
fn main() {
    let website = parser::parse();
    frame::create_path_from_config(website);
    command::copy_asset_files();
}

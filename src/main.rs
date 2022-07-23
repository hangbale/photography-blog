extern crate serde_json;
extern crate serde;
extern crate tera;
extern crate urlencoding;
extern crate exif;

mod config;
mod parser;
mod frame;
mod render;
mod command;
mod image;
fn main() {
    let website = parser::parse();
    frame::create_path_from_config(website);
    command::try_copy_files();
}

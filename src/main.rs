
mod config;
mod parser;
// mod frame;
// mod render;
// mod command;
mod image;
fn main() {
    let mut config = config::read_config();
    parser::parse(&mut config, "", None);
    println!("{:#?}", config);
}


mod config;
mod parser;
mod render;
mod command;
mod image;
fn main() {
    let mut config = config::read_config();
    parser::parse(&mut config, "", None);
    println!("{:#?}", config);
    let mut render_inc = render::Renderer::init();
    render_inc.transfrom_config_to_list(&config);
    render_inc.render_all();
    command::try_copy_files();
}

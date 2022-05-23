// create directories and files
use crate::parser::{Website, create_breadcrumbs};
use crate::render::{Renderer};
use urlencoding::encode;
use crate::config::{PUBLIC_PATH};

pub fn walk_list(
    item: &mut Website,
    pathprefix: &str,
    renderer: &mut Renderer,
    depth: u8
) {
    
    let mut path = pathprefix.to_string();

    let mut depth: u8 = depth;

    if depth != 0 {
        if let Some(item_title) = &item.title {
            path.push_str(format!("/{}", item_title).as_str());
        }
        
    }

    if let Some(children) = &mut item.children {
        depth = depth + 1;

        let mut item_path = path.split("/").map(|x| encode(x).to_string()).collect::<Vec<String>>().join("/");
        item_path = item_path.replace(PUBLIC_PATH, "");
        
        let index_file_path = format!("{}/index.html", path);

        for child in children {
            walk_list(child, &path, renderer, depth);
        }

        item.breadcrumbs = create_breadcrumbs(&item_path);
        item.path = Some(item_path);
        item.category = Some("album".to_string());
        
        renderer.cache(index_file_path, item.clone());
    }
    
}
pub fn create_path_from_config(config: Website) {
    let mut renderer = Renderer::init();
    let mut config = config;
    walk_list(&mut config, PUBLIC_PATH, &mut renderer, 0);
    renderer.render_all();
}
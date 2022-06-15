// create directories and files
use crate::parser::{Website, create_breadcrumbs, Extra};
use crate::render::{Renderer};
use urlencoding::encode;
use crate::config::{PUBLIC_PATH};
use crate::image::{get_exif_from_image, is_local_image};

pub fn walk_list(
    item: &mut Website,
    pathprefix: &str,
    renderer: &mut Renderer,
    depth: u8,
    mut extra: Extra,
) {
    
    let mut path = pathprefix.to_string();

    let mut depth: u8 = depth;

    if depth != 0 {
        if let Some(item_title) = &item.title {
            path.push_str(format!("/{}", item_title).as_str());
        }
    }
    if let Some(item_extra) = &item.extra {
        extra = item_extra.clone();
    } else {
        item.extra = Some(extra.clone());
    }
    if let Some(ex) = &mut item.exif {
        ex.parsed = Some(true);
    } else {
          if let Some(image_url) = &item.url {
            if  is_local_image(image_url) {
                let exif = get_exif_from_image(image_url);
                item.exif = Some(exif);
            }
        }
    }

    if let Some(children) = &mut item.children {
        depth = depth + 1;

        let mut item_path = path.split("/").map(|x| encode(x).to_string()).collect::<Vec<String>>().join("/");
        item_path = item_path.replace(PUBLIC_PATH, "");
        
        let index_file_path = format!("{}/index.html", path);

        for child in children {
            walk_list(child, &path, renderer, depth, extra.clone());
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
    let extra = Extra {
        image_exif_query_suffix: "".to_string(),
        image_style_suffix: "".to_string()
    };
    walk_list(&mut config, PUBLIC_PATH, &mut renderer, 0, extra);
    renderer.render_all();
}
use crate::image::{try_get_img_exif};
use crate::config::{Website, Breadcrumb, WebsiteCategory};
use urlencoding::encode;

pub fn parse(config: &mut Website, pre_path: &str, pre_breadcrumbs: Option<Vec<Breadcrumb>>) {
    if let Some(pics) = &config.pics {
        config.pics_parsed = try_get_img_exif(pics);
    }
    let title = if let Some(t) = &config.title {
        t
    } else {
        ""
    };

    
    let curent = format!("{}/{}", pre_path, title);
    let current_path = encode(&curent);
    config.path = Some(current_path.to_string());

    let current_breadcrumbs = Breadcrumb {
        title: title.to_string(),
        path: current_path.to_string(),
        current: true
    };
    let mut pre_breadcrumbs = match pre_breadcrumbs {
        Some(p) => p,
        None => Vec::new()
    };
    pre_breadcrumbs.push(current_breadcrumbs);
    let length = pre_breadcrumbs.len();
    for breadcrumb in pre_breadcrumbs.iter_mut().take(length - 1) {
        breadcrumb.current = false;
    }

  

    config.breadcrumbs = Some(pre_breadcrumbs.clone());

    if let Some(children) = &mut config.children {
        config.category = Some(WebsiteCategory::Album);
        

        for child in children {
            parse(child, &curent, Some(pre_breadcrumbs.clone()));
        }
    }
}
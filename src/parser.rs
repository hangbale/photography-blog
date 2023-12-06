use crate::image::{parse_pics};
use crate::config::{Website, Breadcrumb, WebsiteCategory};
use urlencoding::encode;
use std::path::Path;

fn is_dir(st: &str) -> bool {
    let path = Path::new(st);
    path.is_dir()
}

fn get_pics_from_dir(dir: &str) -> Vec<String> {
    let mut pics = vec![];
    for entry in std::fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let path = path.to_str().unwrap();
        pics.push(path.to_string());
    }
    pics
}

fn get_all_pic(pics: &Vec<String>) -> Vec<String> {
    let mut all_pic = vec![];
    for pic in pics {
        if is_dir(pic) {
            let dirs_pics = get_pics_from_dir(pic);
            all_pic.extend(get_all_pic(&dirs_pics));
        } else {
            all_pic.push(pic.to_string());
        }
    }
    all_pic
}



pub fn parse(config: &mut Website, pre_path: &str, pre_breadcrumbs: Option<Vec<Breadcrumb>>) {
    if let Some(pics) = &config.pics {
        let pic_list = get_all_pic(pics);
        let parsed_pics = parse_pics(&pic_list);
        config.pics_parsed = Some(parsed_pics);

    }

    let title = &config.title;

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

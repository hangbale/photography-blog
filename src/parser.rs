use crate::image::{parse_pic};
use crate::config::{Website, Breadcrumb, WebsiteCategory};
use urlencoding::encode;
// use std::path::Path;

// fn is_dir(st: &str) -> bool {
//     let path = Path::new(st);
//     path.is_dir()
// }

// fn get_pics_from_dir(dir: &str) -> Vec<String> {
//     let mut pics = vec![];
//     for entry in std::fs::read_dir(dir).unwrap() {
//         let entry = entry.unwrap();
//         let path = entry.path();
//         let path = path.to_str().unwrap();
//         pics.push(path.to_string());
//     }
//     pics
// }

// fn get_all_pic(pics: &Vec<Website>) -> Vec<String> {
//     let mut all_pic = vec![];
//     for pic in pics {
//         if is_dir(pic) {
//             let dirs_pics = get_pics_from_dir(pic);
//             all_pic.extend(get_all_pic(&dirs_pics));
//         } else {
//             all_pic.push(pic.to_string());
//         }
//     }
//     all_pic
// }



pub fn parse(config: &mut Website, pre_path: &str, pre_breadcrumbs: Option<Vec<Breadcrumb>>) {
    if let Some(pic_url) = &config.url {
        config.parsed_pic = Some(parse_pic(&pic_url));
    }

    match config.about {
        Some(_) => (),
        None => {
            config.about = Some(Default::default());
        }
    }

    let title = &config.title;

    let curent = format!("{}/{}", pre_path, title);

    let mut split_pre_path = curent.splitn(3, '/');
    let path_without_first = match split_pre_path.nth(2) {
        Some(n) => n,
        None => ""
    };

    let current_path = encode(&path_without_first);
  
    config.path = Some(path_without_first.to_string());

    let mut current_breadcrumb = Breadcrumb {
        title: title.to_string(),
        path: current_path.to_string(),
        current: true
    };
    let mut pre_breadcrumbs = match pre_breadcrumbs {
        Some(p) => p,
        None => {
            current_breadcrumb.path = "/".to_string();
            Vec::new()
        }
    };


    

    if let Some(children) = &mut config.children {
        config.category = Some(WebsiteCategory::Album);
        pre_breadcrumbs.push(current_breadcrumb);
        let length = pre_breadcrumbs.len();
        if length > 0 {
            for breadcrumb in pre_breadcrumbs.iter_mut().take(length - 1) {
                breadcrumb.current = false;
            }
        }
        config.breadcrumbs = Some(pre_breadcrumbs.clone());
        for child in children {
            parse(child, &curent, Some(pre_breadcrumbs.clone()));
        }
    } else {
        config.category = Some(WebsiteCategory::Photo);
    }
}

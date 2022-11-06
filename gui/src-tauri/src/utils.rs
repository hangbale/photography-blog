use super::img::{DirectoryImgs, Img, ImgType, get_exif_from_image};
use serde::{Serialize, Deserialize};
use std::{fs, path::PathBuf, vec};
use serde_json;
const CONFIG_FILE: &str = "config.json";

fn is_img(name: &str) -> bool {
    let suffix = name.split(".").last();
    return match suffix {
        Some("JPG" | "jpg" | "PNG" | "png" | "gif" | "GIF" | "bmp" | "BMP" | "JPEG" | "jpeg") => {
            true
        }
        _ => false
    }
}
fn default_str(st: Option<&str>) -> String {
    if let Some(s) = st {
        return s.to_string();
    }
    return "".to_string();
}


#[derive(Debug, Serialize, Deserialize)]
struct Config {
    default_pic_lib: String
}
#[derive(Debug, Serialize)]
pub enum PError {
    IOerror(String)
}

pub fn get_config_file(app_handle: tauri::AppHandle) -> Result<String, PError> {
    let app_dir = app_handle.path_resolver().app_dir();
    // let dir = data_dir();
    if let Some(mut appdir) = app_dir {
        appdir.push(CONFIG_FILE);
        let s = fs::read_to_string(appdir)
        .map_err(|e| PError::IOerror("read file failed".into()))?;

        let st: Config = serde_json::from_str(&s)
        .map_err(|e| PError::IOerror("read file failed".into()))?;

        return Ok(st.default_pic_lib);
    } else {
        return Err(PError::IOerror("read file failed".into()));
    }
}

fn walk_dir(dir: String) -> Result<DirectoryImgs, PError> {
    let red = fs::read_dir(PathBuf::from(&dir))
    .map_err(|_| PError::IOerror("read dir failed".into()))?;
    let mut dir_imgs = DirectoryImgs {
        name: dir.clone(),
        path: dir.clone(),
        imgs: Some(vec![]),
        sub_dir: None
    };
    for entry in red {
        if let Ok(ety) = entry {
       
            if let Ok(meta) = ety.metadata() {
                let name = default_str(ety.file_name().to_str());
                let p = default_str(ety.path().to_str());
                
                let mut img = Img {
                    path: p.to_owned(),
                    name: name.to_owned(),
                    exif: None,
                    img_type: ImgType::Dir
                };
                let mut should_add = false;
                if meta.is_file() && is_img(&name) {
                    let exf = get_exif_from_image(&p);
                    img.exif = Some(exf);
                    img.img_type = ImgType::Img;
                    should_add = true
                }
                if meta.is_dir() {
                    should_add = true
                }
                if let Some(ref mut imgs) = dir_imgs.imgs {
                    if should_add {
                        imgs.push(img);
                    }
                }
            }
        }
    }
    println!("{:?}", dir_imgs);
    Ok(dir_imgs)
}

#[tauri::command]
pub fn get_dir_imgs(app_handle: tauri::AppHandle, path: Option<String>) -> Result<DirectoryImgs, PError> {
    let lib_path = match path {
        Some(entry) => entry,
        None => {
            get_config_file(app_handle).unwrap()
        }
    };
    return walk_dir(lib_path);
}
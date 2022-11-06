use serde::{Deserialize, Serialize};
use std::fs::{File};
use exif::{self, Tag, In, Exif};



#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ExifInfo {
    pub shutter: Option<String>, // 快门
    pub model: Option<String>, // 相机型号
    pub aperture: Option<String>, // 光圈
    pub focal: Option<String>, // 焦距
    pub iso: Option<String>, // 感光度
    pub date: Option<String>, // 日期
    pub parsed: bool, // 是否解析成功
}

pub fn is_local_image(path: &str) -> bool {
    !path.starts_with("http")
}
pub fn get_exif_field(ex: &Exif, tag: Tag, ifd: In) -> String {
    let ret = ex.get_field(tag, ifd);
    if let Some(f) = ret {
        let v = f.display_value();
        v.to_string()
    } else {
        "".to_string()
    }
}
pub fn get_exif_from_image(image_path: &str) -> ExifInfo {
    let file_ret = File::open(image_path);
    let mut ret = ExifInfo {
        shutter: Some("".to_string()),
        model: Some("".to_string()),
        aperture: Some("".to_string()),
        focal: Some("".to_string()),
        iso: Some("".to_string()),
        date: Some("".to_string()),
        parsed: false
    };
    if let Ok(file) = file_ret {
        let mut bufreader = std::io::BufReader::new(&file);
        let exifreader = exif::Reader::new();
        let exif_ret = exifreader.read_from_container(&mut bufreader);
        match exif_ret {
            Ok(ef) => {
                ret.shutter = Some(get_exif_field(&ef, Tag::ExposureTime, In::PRIMARY));
                ret.model = Some(get_exif_field(&ef, Tag::Model, In::PRIMARY));
                ret.aperture = Some(get_exif_field(&ef, Tag::FNumber, In::PRIMARY));
                ret.iso = Some(get_exif_field(&ef, Tag::PhotographicSensitivity, In::PRIMARY));
                ret.focal = Some(get_exif_field(&ef, Tag::FocalLengthIn35mmFilm, In::PRIMARY));
                ret.date = Some(get_exif_field(&ef, Tag::DateTimeDigitized, In::PRIMARY));
                ret.parsed = true;
                ret
            }
            Err(e) => {
                eprint!("{}", e);
                ret
            }
        }
        
    } else {
        eprint!("Could not open file: {}", image_path);
        ret
    }
}

    
#[derive(Debug, Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub enum ImgType {
    Img,
    Dir
}
#[derive(Debug, Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct Img {
    pub path: String,
    pub name: String,
    pub exif: Option<ExifInfo>,
    pub img_type: ImgType
}

#[derive(Debug, Serialize)]
pub struct DirectoryImgs {
    pub name: String,
    pub path: String,
    pub imgs: Option<Vec<Img>>,
    pub sub_dir: Option<Box<DirectoryImgs>>
}
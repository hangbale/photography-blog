// get image exif data
use serde::{Deserialize, Serialize};
use std::fs::{File};
use exif::{self, Tag, In, Exif};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ExifInfo {
    pub shutter: Option<String>,
    pub model: Option<String>,
    pub aperture: Option<String>,
    pub focal: Option<String>,
    pub iso: Option<String>,
    pub date: Option<String>,
    pub parsed: Option<bool>,
}

pub fn is_local_image(path: &str) -> bool {
    !path.starts_with("http")
}
pub fn get_exif_fild(ex: &Exif, tag: Tag, ifd: In) -> String {
    let ret = ex.get_field(tag, ifd);
    if let Some(f) = ret {
        let v = f.display_value();
        v.to_string()
    } else {
        "".to_string()
    }
}
pub fn get_exif_from_image(image_path: &str) -> ExifInfo {
    let path = image_path.replacen("/", "", 1);
    let file_ret = File::open(path);
    let mut ret = ExifInfo {
        shutter: Some("".to_string()),
        model: Some("".to_string()),
        aperture: Some("".to_string()),
        focal: Some("".to_string()),
        iso: Some("".to_string()),
        date: Some("".to_string()),
        parsed: Some(false),
    };
    if let Ok(file) = file_ret {
        let mut bufreader = std::io::BufReader::new(&file);
        let exifreader = exif::Reader::new();
        let exif_ret = exifreader.read_from_container(&mut bufreader);
        match exif_ret {
            Ok(ef) => {
                ret.shutter = Some(get_exif_fild(&ef, Tag::ExposureTime, In::PRIMARY));
                ret.model = Some(get_exif_fild(&ef, Tag::Model, In::PRIMARY));
                ret.aperture = Some(get_exif_fild(&ef, Tag::FNumber, In::PRIMARY));
                ret.iso = Some(get_exif_fild(&ef, Tag::PhotographicSensitivity, In::PRIMARY));
                ret.focal = Some(get_exif_fild(&ef, Tag::FocalLengthIn35mmFilm, In::PRIMARY));
                ret.date = Some(get_exif_fild(&ef, Tag::DateTimeDigitized, In::PRIMARY));
                ret.parsed = Some(true);
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

    
// get image exif data
use serde::{Deserialize, Serialize};
use std::fs::{File};
use std::path::Path;
use exif::{self, Tag, In, Exif, Rational};
use std::default::Default;
use crate::config::{IMAGE};
// GPS Coord
#[derive(Debug, Deserialize, Serialize)]
pub struct Coord {
    lat: f64,
    lng: f64,
}

impl From<(&Vec<Rational>, &Vec<Rational>)> for Coord {
    fn from(r: (&Vec<Rational>, &Vec<Rational>)) -> Self {
        let lat = r.0;
        let lng = r.1;
        let l1 = lat[0];
        let l2 = lat[1];
        let l3 = lat[2];
        let latv = l1.num as f64 / l1.denom as f64
            + l2.num as f64 / l2.denom as f64 / 60.0
            + l3.num as f64 / l3.denom as f64 / 3600.0;
        let ln1 = lng[0];
        let ln2 = lng[1];
        let ln3 = lng[2];
        let lngv = ln1.num as f64 / ln1.denom as f64
            + ln2.num as f64 / ln2.denom as f64 / 60.0
            + ln3.num as f64 / ln3.denom as f64 / 3600.0;
        Coord {
            lat: latv,
            lng: lngv,
        }
    }
}



#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ExifInfo {
    pub shutter: Option<String>, // 快门
    pub model: Option<String>,   // 相机型号
    pub aperture: Option<String>, // 光圈
    pub focal: Option<String>,   // 焦距
    pub iso: Option<String>,     // iso
    pub date: Option<String>,    // 拍摄时间
    pub coord: Option<Coord>,    // GPS坐标
    pub parsed: bool,            // 是否解析成功
}

fn get_gps_coord(ef: &Exif) -> Option<Coord> {
    let lat = ef.get_field(Tag::GPSLatitude, In::PRIMARY);
    let lng = ef.get_field(Tag::GPSLongitude, In::PRIMARY);
    if let (Some(lat), Some(lng)) = (lat, lng) {
        let latv = &lat.value;
        let lngv = &lng.value;
        match (latv, lngv) {
            (exif::Value::Rational(lat), exif::Value::Rational(lng)) => {
                let coord = Coord::from((lat, lng));
                Some(coord)
            }
            _ => None,
        }
    } else {
        None
    }
}

// pub fn is_online_image(path: &str) -> bool {
//     !path.starts_with("http") || !path.starts_with("//")
// }
pub fn is_local_image(path: &str) -> bool {
    println!("path: {}", path);
    let r = Path::new(path).exists();
    println!("r: {}", r);
    r
}
pub fn get_exif_field(ex: &Exif, tag: Tag, ifd: In) -> Option<String> {
    let ret = ex.get_field(tag, ifd);
    if let Some(f) = ret {
        let v = f.display_value();
        Some(v.to_string())
    } else {
        None
    }
}
pub fn get_exif_from_image(image_path: &str) -> ExifInfo {
    // let path = image_path.replacen("/", "", 1);
    let file_ret = File::open(image_path);
    let mut ret: ExifInfo = Default::default();

    if let Ok(file) = file_ret {
        let mut bufreader = std::io::BufReader::new(&file);
        let exifreader = exif::Reader::new();
        let exif_ret = exifreader.read_from_container(&mut bufreader);
        match exif_ret {
            Ok(ef) => {
                ret.shutter = get_exif_field(&ef, Tag::ExposureTime, In::PRIMARY);
                ret.model = get_exif_field(&ef, Tag::Model, In::PRIMARY);
                ret.aperture = get_exif_field(&ef, Tag::FNumber, In::PRIMARY);
                ret.iso = get_exif_field(&ef, Tag::PhotographicSensitivity, In::PRIMARY);
                ret.focal = get_exif_field(&ef, Tag::FocalLengthIn35mmFilm, In::PRIMARY);
                ret.date = get_exif_field(&ef, Tag::DateTimeDigitized, In::PRIMARY);
                ret.coord = get_gps_coord(&ef);
                ret.parsed = true; // todo: check all fields
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

pub fn try_get_img_exif(url: &str) -> Option<ExifInfo> {
    if is_local_image(url) {
        Some(get_exif_from_image(url))
    } else {
        None
    }
}
pub fn parse_pic(pic_url: &str) -> IMAGE {
    let exif = try_get_img_exif(pic_url);
    let img_name = Path::new(pic_url).file_name().unwrap();
    let img_name = img_name.to_str().unwrap();
    let mut img = IMAGE {
        title: img_name.to_string(),
        exif: None,
        url: pic_url.to_string()
    };
    if let Some(exif) = exif {
        img.exif = Some(exif);
    }
    img
}

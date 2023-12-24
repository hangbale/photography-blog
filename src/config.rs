use crate::image::ExifInfo;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::Path;
use colored::*;

pub static TEMPLATE_FILE_PATH: &str = "template/*.html";
pub static PUBLIC_PATH: &str = "public/";
pub static ASSETS_PATH: &str = "template/assets";
pub static IMAGE_PATH: &str = "image";
pub static DEFAULT_ABOUT_TEXT: &str = "ABOUT ME";
pub static DEFAULT_ABOUT_URL: &str = "http://idinr.com";
static CONFIG_FILE_PATH: &str = "config.yml";

// for windows xcopy
pub const WIN_PUBLIC_ASSET_PATH: &str = r"public\assets\";
pub const WIN_ASSETS_PATH: &str = r"template\assets";

#[derive(Debug, Deserialize, Serialize)]
pub enum WebsiteCategory {
    Album,
    Photo
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Breadcrumb {
  pub title: String,
  pub path: String,
  pub current: bool
}
#[derive(Debug, Deserialize, Serialize)]
pub struct IMAGE {
  pub title: String,
  pub exif: Option<ExifInfo>,
  pub url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Extra {
    pub image_exif_query_suffix: String, // supported cdn: 七牛云
    pub image_style_suffix: String // supported cdn: 七牛云
}

#[derive(Debug, Deserialize, Serialize)]
pub struct About {
    pub text: String,
    pub url: String
}
impl Default for About {
    fn default() -> Self {
        About {
            text: DEFAULT_ABOUT_TEXT.to_string(),
            url: DEFAULT_ABOUT_URL.to_string()
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Website {
  pub title: String, // 必填
  pub description: Option<String>,
  pub children: Option<Vec<Website>>,
  pub parsed_pic: Option<IMAGE>,
  pub category: Option<WebsiteCategory>,
//   pub author: Option<String>,
  pub path: Option<String>, // 页面路径 根据title而来
  pub breadcrumbs: Option<Vec<Breadcrumb>>,
  pub exif: Option<ExifInfo>,
  pub extra: Option<Extra>,
  pub cover: Option<String>,
  pub url: Option<String>,
  pub about: Option<About>
}

pub fn read_config() -> Website {
    if !Path::new(CONFIG_FILE_PATH).exists() {
        eprintln!("{}", "❌ 配置文件config.yml不存在，请先创建".red());
        std::process::exit(0);
    } else {
        let config_file = File::open(CONFIG_FILE_PATH).unwrap();
        match serde_yaml::from_reader(config_file) {
            Ok(config @ Website { .. }) => {
                config
            }
            Err(e) => {
                eprintln!("{}", "❌ 解析配置文件config.yml出错".red());
                eprintln!("{}", e);
                std::process::exit(0);
            }
        }
    }
}

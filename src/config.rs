use crate::image::ExifInfo;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::Path;
use colored::*;

static TEMPLATE_FILE_PATH: &str = "template/*.html";
static PUBLIC_PATH: &str = "public";
static ASSETS_PATH: &str = "template/assets";
static IMAGE_PATH: &str = "image";
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
  pub exif: Option<ExifInfo>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Extra {
    pub image_exif_query_suffix: String, // supported cdn: 七牛云
    pub image_style_suffix: String // supported cdn: 七牛云
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Website {
  pub title: String, // 必填
  pub description: Option<String>,
  pub children: Option<Vec<Website>>,
  pub pics: Option<String>,
  pub pics_parsed: Option<Vec<IMAGE>>,
  pub category: Option<WebsiteCategory>,
  pub author: Option<String>,
  pub path: Option<String>, // 页面路径 根据title而来
  pub breadcrumbs: Option<Vec<Breadcrumb>>,
  pub exif: Option<ExifInfo>,
  pub extra: Option<Extra>,
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

pub fn get_category(url: &str) -> WebsiteCategory {
    if url.ends_with(".html") {
        WebsiteCategory::Photo
    } else {
        WebsiteCategory::Album
    }
}
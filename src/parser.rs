use std::fs;
use std::path::PathBuf;
use serde_json;
use serde::{Deserialize, Serialize};
use urlencoding::{decode};


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Website {
  pub title: Option<String>,
  pub description: Option<String>,
  pub children: Option<Vec<Website>>,
  pub url: Option<String>,
  pub category: Option<String>,
  pub author: Option<String>,
  pub path: Option<String>,
  pub breadcrumbs: Option<Vec<Breadcrumb>>,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Breadcrumb {
  pub title: String,
  pub path: String,
  pub current: bool
}

pub fn create_breadcrumbs(item_path: &str) -> Option<Vec<Breadcrumb>> {
    let decoded = decode(item_path).expect("Could not decode path");
    let title_list = decoded.split("/").collect::<Vec<&str>>();
    let path_list = item_path.split("/").collect::<Vec<&str>>();
    let mut breadcrumbs = Vec::new();

    for (i, title) in title_list.iter().enumerate() {
        if title.to_string() != "" {
            let path = path_list[0..i+1].join("/");
            breadcrumbs.push(Breadcrumb {
                title: title.to_string(),
                path: path.to_string(),
                current: false
            });
        }
    }
    if breadcrumbs.len() > 0 {
        let len = breadcrumbs.len();
        breadcrumbs[len - 1].current = true;
        Some(breadcrumbs)
    } else {
        None
    }
}

pub fn get_current_path() -> PathBuf {
  std::env::current_dir().expect("Could not get current directory")
}

pub fn get_config_path() -> PathBuf {
  let mut path = get_current_path();
  path.push("config.json");
  path
}

pub fn read_config() -> String {
  let path = get_config_path();
  let config_file = fs::read_to_string(path).expect("Could not read config file");
  config_file
}
pub fn parse() -> Website {
  let config_str = read_config();
  let config: Website = serde_json::from_str(&config_str).expect("Could not parse config file");
  config
}
// use std::net::{SocketAddrV4, Ipv4Addr};
use serde::Serialize;
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Blog {
    name: String,
    ip: String,
    domain: String,
    last_update: String
}

#[tauri::command]
pub fn greet() -> Vec<Blog> {
    let b = Blog {
    name: String::from("idinr"),
    ip: String::from("121.1.1.1"),
    domain: String::from("i.idinr.com"),
    last_update: String::from("2012.32.33")
   };
   vec!(b)
}
// use std::net::{SocketAddrV4, Ipv4Addr};
use serde::{Serialize, Deserialize};
use super::state::DB;
use tauri::State;
const SQL_LIST: &str = "SELECT * FROM blog";
const SQL_INSERT: &str = "INSERT INTO blog (name, ip, domain) VALUES (?1, ?2, ?3)";
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Blog {
    name: String,
    ip: String,
    domain: String,
    last_update: Option<String>
}


#[tauri::command]
pub fn list(db: State<DB>) {
    if let Some(dbl) = db.get() {
        if let Some(ref dbi) = *dbl {
            let smt = dbi.prepare(SQL_LIST);
            if let Ok(ret) = smt {
                let mut rows = ret.query(());
                

                if let Ok(row) = rows {
                    let ret = row.map(|r| r.get(0)).collect();
                }
            }
        }
    }
}

#[tauri::command]
pub fn add_blog(db: State<DB>, item: Blog) -> u8 {
    println!("add blog");
    println!("{:?}", item);
    let dbt = db.get();
    if let Some(dbr) = dbt {
        if let Some(ref dbi) = *dbr {
            let r = dbi.execute(SQL_INSERT, (&item.name, &item.ip, &item.domain));
            println!("{:?}", r);
            let r2 = dbi.execute(SQL_LIST, ());
            println!("{:?}", r2);
            return 0
        }

    }
    1
}
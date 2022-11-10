use std::vec;

use serde::{Serialize, Deserialize};
use super::state::DB;
use tauri::State;
use super::common::Respone;


const SQL_LIST: &str = "SELECT * FROM blog";
const SQL_INSERT: &str = "INSERT INTO blog (name, ip, domain) VALUES (?1, ?2, ?3)";
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Blog {
    pub name: String,
    pub ip: String,
    pub domain: String,
    pub last_update: Option<String>
}

fn blog_list_sql(db: State<DB>) -> Result<Vec<Blog>, Box<dyn std::error::Error>> {
    let mut blogs: Vec<Blog> = vec![];
    if let Some(dbl) = db.get() {
        if let Some(ref dbi) = *dbl {
            let mut smt = dbi.prepare(SQL_LIST)?;
            let mut rows = smt.query(())?;
            
            while let Some(row) = rows.next()? {
                blogs.push(Blog {
                    name: row.get("name")?,
                    ip: row.get("ip")?,
                    domain: row.get("domain")?,
                    last_update: row.get("last_update")?,
                });
            }
        }
    }
    Ok(blogs)
}

fn blog_add_sql(db: State<DB>, item: Blog) -> Result<String, Box<dyn std::error::Error>> {
    let dbt = db.get();
    if let Some(dbr) = dbt {
        if let Some(ref dbi) = *dbr {
            let t = dbi.execute(SQL_INSERT, (&item.name, &item.ip, &item.domain)).map(|_| "操作成功".to_owned())?;
            return Ok(t);
        }
    }
    Err("操作失败".to_owned())?
}
#[tauri::command]
pub fn list(db: State<DB>) -> Respone<Vec<Blog>> {
    let t = blog_list_sql(db);
    return Respone::<Vec<Blog>>::from(t);
}

#[tauri::command]
pub fn add_blog(db: State<DB>, item: Blog) -> Respone<String> {
    let t = blog_add_sql(db, item);
    return Respone::<String>::from(t);
}
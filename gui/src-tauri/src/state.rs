use std::{sync::{Mutex, MutexGuard}};
use rusqlite::{Connection};
use std::path::PathBuf;

 const SQL_INIT: &str = "CREATE TABLE blog (
                        id     INTEGER PRIMARY KEY,
                        name   TEXT NOT NULL,
                        ip     TEXT NOT NULL,
                        domain TEXT NOT NULL,
                        last_update   INTEGER
                    )";


pub struct DB {
    pub db: Mutex<Option<Connection>>
}

impl DB {
    pub fn init() -> Self {
        Self {
            db: Mutex::new(None)
        }
    }
    pub fn setup(&self, apppath: Option<PathBuf>) {
        if let Some(mut appdir) = apppath {
            appdir.push("main.db3");
            let c = Connection::open(appdir).ok();
            let d = self.db.lock();
            if let Ok(mut db)  = d {
                *db = c
            }
        }
        self.exec_noparam(SQL_INIT);
    }
    pub fn exec_noparam(&self, sql: &str) {
        let dc = self.db.lock().ok();
        if let Some(dbm) = dc {
            if let Some(ref db) = *dbm {
                let rt = db.execute(sql, ());
                println!("sql exec result: {:?}", rt);
            } else {
                println!("none");
            }
        } else {
            println!("none");
        }
    }
    pub fn get(&self) -> Option<MutexGuard<Option<Connection>>> {
        self.db.lock().ok()
    }
}
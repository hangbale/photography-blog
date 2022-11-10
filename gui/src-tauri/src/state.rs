use std::{sync::{Mutex, MutexGuard}};
use tauri::State;
use rusqlite::{Connection, Result, Params,params, ParamsFromIter, params_from_iter};

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
        let c = Connection::open_in_memory().ok();
        Self {
            db: Mutex::new(c)
        }
    }
    pub fn setup(&self) {
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
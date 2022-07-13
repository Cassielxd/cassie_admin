use crate::{config::ApplicationConfig, APPLICATION_CONTEXT, db::db::SqliteMap};
use log::info;
use rusqlite::Connection;
pub async fn init_config() {
    let content = include_str!("../application.yml");
    let config = ApplicationConfig::new(content);
    APPLICATION_CONTEXT.set::<ApplicationConfig>(config);
}



pub fn init_db() {
    APPLICATION_CONTEXT.set::<SqliteMap>(SqliteMap::default());
    init_default();
}

pub fn init_default(){
    let connection = Connection::open(&"../cassie.db3").unwrap();
   let resout =  connection.execute("
    CREATE TABLE storage (
        id              INTEGER PRIMARY KEY,
        key            TEXT NOT NULL,
        value            TEXT NOT NULL
        )
      ",[],);
    match resout {
        Ok(_) => {
            info!("cassie 实例化成功");
        },
        Err(e) => {
            info!("cassie db 已经存在 {}",e.to_string());
        },
    }
    let map =APPLICATION_CONTEXT.get::<SqliteMap>();
    map.conn_map.lock().unwrap().insert("cassie".to_string(), connection);
}
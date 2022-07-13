use crate::{config::ApplicationConfig, APPLICATION_CONTEXT, db::db::SqliteMap};
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
    connection.execute("
    CREATE TABLE storage (
        id              INTEGER PRIMARY KEY,
        key            TEXT NOT NULL,
        value            TEXT NOT NULL
        )
      ",[],);

    let map =APPLICATION_CONTEXT.get::<SqliteMap>();
    map.conn_map.lock().unwrap().insert("cassie".to_string(), connection);
}
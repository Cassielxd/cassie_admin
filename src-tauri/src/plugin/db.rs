use rusqlite::{named_params, params, Connection};
use serde::{ser::Serializer, Serialize};
use std::{collections::HashMap, sync::Mutex};
use tauri::{
    command,
    plugin::{Builder, TauriPlugin},
    GlobalShortcutManager, Manager, Runtime,
};

use crate::APPLICATION_CONTEXT;

pub type Result<T> = std::result::Result<T, Error>;

/// A generic error that represents all the ways a method can fail inside of rexpr::core.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Sqlite(#[from] rusqlite::Error),
    #[error("database {0} not opened")]
    DatabaseNotOpened(String),
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

#[derive(Default)]
pub struct SqliteMap {
    pub conn_map: Mutex<HashMap<String, Connection>>,
}

//创建一个新的 db
#[command]
async fn open(path: String) -> Result<bool> {
    let connection = Connection::open(&path).unwrap();
    let map = APPLICATION_CONTEXT.get::<SqliteMap>();
    map.conn_map.lock().unwrap().insert(path.clone(), connection);
    Ok(true)
}

//关闭连接
#[command]
async fn close(path: String) -> Result<bool> {
    let map = APPLICATION_CONTEXT.get::<SqliteMap>();
    let mut map = map.conn_map.lock().unwrap();
    let connection = map.get_mut(&path).ok_or(Error::DatabaseNotOpened(path.clone()))?;
    drop(connection);
    map.remove(&path);
    Ok(true)
}

//执行path 下的 sql
#[command]
async fn execute(path: String, sql: String) -> Result<bool> {
    let map = APPLICATION_CONTEXT.get::<SqliteMap>();
    let mut map = map.conn_map.lock().unwrap();
    let connection = map.get_mut(&path).ok_or(Error::DatabaseNotOpened(path))?;
    connection.execute(&sql, [])?;
    Ok(true)
}

//保存默认path下的值
#[command]
async fn save(key: String, value: String) -> Result<bool> {
    let map = APPLICATION_CONTEXT.get::<SqliteMap>();
    let mut map = map.conn_map.lock().unwrap();
    let connection = map.get_mut(&"cassie".to_string()).ok_or(Error::DatabaseNotOpened("cassie".to_string()))?;
    connection.execute("INSERT INTO storage (key, value) VALUES (?1, ?2)", params![key, value])?;
    Ok(true)
}

//删除默认path下的值
#[command]
async fn del(key: String) -> Result<bool> {
    let map = APPLICATION_CONTEXT.get::<SqliteMap>();
    let mut map = map.conn_map.lock().unwrap();
    let connection = map.get_mut(&"cassie".to_string()).ok_or(Error::DatabaseNotOpened("cassie".to_string()))?;
    let mut stmt = connection.prepare(" DELETE FROM  storage where key = :key")?;
    stmt.execute([key])?;
    Ok(true)
}
//获取默认path下的 value值
#[command]
async fn get(key: String) -> Result<Vec<String>> {
    let map = APPLICATION_CONTEXT.get::<SqliteMap>();
    let mut map = map.conn_map.lock().unwrap();
    let connection = map.get_mut(&"cassie".to_string()).ok_or(Error::DatabaseNotOpened("cassie".to_string()))?;
    let mut stmt = connection.prepare("SELECT value FROM  storage where key = :key")?;
    let rows = stmt.query_map(named_params! { ":key": key }, |row| row.get(0))?;

    let mut values = Vec::new();
    for value in rows {
        values.push(value?);
    }
    Ok(values)
}

//初始化sqlite   Plugin  名称是  sqlite
pub fn init_sqlite<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("sqlite")
        .invoke_handler(tauri::generate_handler![save, open, close, execute, get, del])
        .setup(|app| Ok(()))
        .build()
}

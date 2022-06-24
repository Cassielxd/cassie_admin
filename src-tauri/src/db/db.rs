use rusqlite::Connection;



pub fn init_db(){
  let conn =  Connection::open_in_memory().unwrap();
  conn.execute(
    "CREATE TABLE token (access_token            TEXT NOT NULL)",
    [],
).unwrap();
  
}
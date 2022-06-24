use db::db::init_db;
use state::Container;

pub mod db;
pub mod meuns;
pub mod utils;
pub mod vo;

pub static APPLICATION_CONTEXT: Container![Send + Sync] = <Container![Send + Sync]>::new();

pub  fn init_context() {
    init_db();
}
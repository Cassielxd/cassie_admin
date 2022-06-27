#[macro_use]
extern crate getset;
use db::db::init_db;
use event::init_event_bus;
use initalize::init_config;
use state::Container;

pub mod config;
pub mod db;
pub mod event;
pub mod initalize;
pub mod meuns;
pub mod utils;
pub mod vo;

pub static APPLICATION_CONTEXT: Container![Send + Sync] = <Container![Send + Sync]>::new();

pub async fn init_context() {
    init_db();
    init_event_bus();
    init_config().await;
}

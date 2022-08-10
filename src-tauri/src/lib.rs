#[macro_use]
extern crate getset;

use event::init_event_bus;
use fast_log::config::Config;
use fast_log::consts::LogSize;
use fast_log::plugin::file_split::RollingType;
use fast_log::plugin::packer::ZipPacker;
use initalize::{init_config, init_db};
use log::{LevelFilter};
use state::Container;
use std::time::Duration;

pub mod config;
pub mod event;
pub mod initalize;
pub mod meuns;
pub mod plugin;
pub mod server;
pub mod utils;
pub mod vo;

pub static APPLICATION_CONTEXT: Container![Send + Sync] = <Container![Send + Sync]>::new();

pub async fn init_context() {
    init_log();
    init_db();
    init_event_bus();
    init_config().await;
}

pub fn init_log() {
    //create log dir
    let _r =std::fs::create_dir_all(&"logs/");
    //initialize fast log
    fast_log::init(
        Config::new()
            .console()
            .file_split("logs/", str_to_temp_size("100MB"), str_to_rolling("KeepNum(20)"), ZipPacker {})
            .level(LevelFilter::Info),
    )
    .unwrap();
}

fn str_to_temp_size(arg: &str) -> LogSize {
    match arg {
        arg if arg.ends_with("MB") => {
            let end = arg.find("MB").unwrap();
            let num = arg[0..end].to_string();
            LogSize::MB(num.parse::<usize>().unwrap())
        }
        arg if arg.ends_with("KB") => {
            let end = arg.find("KB").unwrap();
            let num = arg[0..end].to_string();
            LogSize::KB(num.parse::<usize>().unwrap())
        }
        arg if arg.ends_with("GB") => {
            let end = arg.find("GB").unwrap();
            let num = arg[0..end].to_string();
            LogSize::GB(num.parse::<usize>().unwrap())
        }
        _ => LogSize::MB(100),
    }
}

fn str_to_rolling(arg: &str) -> RollingType {
    match arg {
        arg if arg.starts_with("KeepNum(") => {
            let end = arg.find(")").unwrap();
            let num = arg["KeepNum(".len()..end].to_string();
            RollingType::KeepNum(num.parse::<i64>().unwrap())
        }
        arg if arg.starts_with("KeepTime(") => {
            let end = arg.find(")").unwrap();
            let num = arg["KeepTime(".len()..end].to_string();
            RollingType::KeepTime(Duration::from_secs(num.parse::<u64>().unwrap()))
        }
        _ => RollingType::All,
    }
}


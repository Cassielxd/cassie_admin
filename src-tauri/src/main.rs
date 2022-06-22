#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]
use app::{
    meuns::menu::{menu_event, init_menu},
    utils::c_create_window,
    __cmd__c_create_window
    };
use tauri::Manager;

fn main() {
   let context = tauri::generate_context!();
    tauri::Builder::default()
        //初始化菜单
        .menu(init_menu())
        //系统设置
        .setup(|_app| { Ok(())})
        //菜单点击事件
        .on_menu_event(menu_event)
        //为js提供调用方法
        .invoke_handler(tauri::generate_handler![c_create_window,close_splashscreen])
        .run(context)
        .expect("创建程序出错");
}

#[tauri::command]
async fn close_splashscreen(window: tauri::Window) {
  if let Some(splashscreen) = window.get_window("splashscreen") {
    splashscreen.close().unwrap();
  }
  window.get_window("main").unwrap().show().unwrap();
}
#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]
use app::{
    __cmd__c_create_window,
    meuns::menu::{init_menu, init_system_tray, menu_event, system_tray_menu_event, windows_event},
    utils::c_create_window,
};
use tauri::Manager;

fn main() {
    let context = tauri::generate_context!();
    tauri::Builder::default()
        //初始化菜单
        .menu(init_menu())
        .system_tray(init_system_tray())
        //系统设置
        .setup(|_app| Ok(()))
        .on_window_event(windows_event)
        //菜单点击事件
        .on_menu_event(menu_event)
        .on_system_tray_event(system_tray_menu_event)
        //为js提供调用方法
        .invoke_handler(tauri::generate_handler![c_create_window, close_splashscreen])
        //初始化db
        .plugin(tauri_plugin_sqlite::init())
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

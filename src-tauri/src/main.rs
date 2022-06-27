#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]
use app::{
    init_context,
    meuns::menu::{init_menu, init_system_tray, menu_event, system_tray_menu_event, windows_event},
    server::init_server, APPLICATION_CONTEXT,
};
use tauri::{Manager, Window};
#[tokio::main]
async fn main() {
    init_context().await;
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .plugin(tauri_plugin_sqlite::init())
        .menu(init_menu())
        .system_tray(init_system_tray())
        //系统设置
        .setup(|_app| {
           let main_window = _app.get_window("main").unwrap();
            APPLICATION_CONTEXT.set::<Window>(main_window);
            init_server();
            Ok(())
        })
        .on_window_event(windows_event)
        //菜单点击事件
        .on_menu_event(menu_event)
        .on_system_tray_event(system_tray_menu_event)
        //为js提供调用方法
        .invoke_handler(tauri::generate_handler![close_splashscreen])
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

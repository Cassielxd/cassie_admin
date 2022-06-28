#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]
use app::{
    init_context,
    meuns::menu::{init_menu, init_system_tray, menu_event, system_tray_menu_event, windows_event},
    server::init_server,
    APPLICATION_CONTEXT,
};
use tauri::{Manager, Window,Builder};
#[tokio::main]
async fn main() {
    init_context().await;
    Builder::default()
        .menu(init_menu())
        .system_tray(init_system_tray())
        //系统设置
        .setup(|_app| {
            let main_window = _app.get_window("main").unwrap();
            APPLICATION_CONTEXT.set::<Window>(main_window);
            init_server();
            Ok(())
        })
        //系统事件
        .on_window_event(windows_event)
        //菜单点击事件
        .on_menu_event(menu_event)
        //托盘事件
        .on_system_tray_event(system_tray_menu_event)
        //为js提供调用方法
        .invoke_handler(tauri::generate_handler![close_splashscreen])
        .run(tauri::generate_context!())
        .expect("创建程序出错");
}

//用于启动时 loading加载等待 主窗口加载完成 关闭 loading窗口
#[tauri::command]
async fn close_splashscreen(window: tauri::Window) {
    if let Some(splashscreen) = window.get_window("splashscreen") {
        //关闭loading窗口
        splashscreen.close().unwrap();
    }
    //显示主窗口
    window.get_window("main").unwrap().show().unwrap();
}

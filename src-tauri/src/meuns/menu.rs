use crate::utils::{create_window, open_for_browser};
use tauri::{AppHandle, CustomMenuItem, Manager, Menu, SystemTray, SystemTrayEvent, WindowMenuEvent};
use tauri::{SystemTrayMenu, SystemTrayMenuItem};
//初始化菜单
pub fn init_menu() -> Menu {
    let play_ground = CustomMenuItem::new("js_play_ground".to_string(), "PlayGround");
    let socure_code = CustomMenuItem::new("socure_code".to_string(), "socure code");
    Menu::new().add_item(play_ground).add_item(socure_code)
}

//菜单点击事件处理逻辑
pub fn menu_event(event: WindowMenuEvent) {
    match event.menu_item_id() {
        "js_play_ground" => {
            create_window(event.window().app_handle(), "js_play_ground", "PlayGround", "/#/jsruntime", Menu::default());
        }
        "socure_code" => {
            open_for_browser(&event.window(), "https://gitee.com/stringlxd");
        }
        _ => {}
    }
}

pub fn init_system_tray() -> SystemTray {
    let quit = CustomMenuItem::new("quit".to_string(), "关闭");
    let hide = CustomMenuItem::new("hide".to_string(), "隐藏");
    let tray_menu = SystemTrayMenu::new().add_item(quit).add_native_item(SystemTrayMenuItem::Separator).add_item(hide);
    SystemTray::new().with_menu(tray_menu)
}

pub fn system_tray_menu_event(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "quit" => {
                std::process::exit(0);
            }
            "hide" => {
                let window = app.get_window("main").unwrap();
                window.hide().unwrap();
            }
            _ => {}
        },
        _ => {}
    }
}

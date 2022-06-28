use crate::config::ApplicationConfig;
use crate::utils::{create_window, open_for_browser};
use crate::APPLICATION_CONTEXT;
use tauri::api::dialog::confirm;
use tauri::{AppHandle, CustomMenuItem, GlobalWindowEvent, Manager, Menu, Submenu, SystemTray, SystemTrayEvent, WindowMenuEvent};
use tauri::{SystemTrayMenu, SystemTrayMenuItem};

const QUIT: &str = "quit";
const HIDE: &str = "hide";

const JS_PLAY_GROUND: &str = "js_play_ground";
const SOCURE_CODE: &str = "socure_code";
/*
初始化菜单
*/
pub fn init_menu() -> Menu {
    let config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
    //debug 环境开启开发工具
    if *config.debug() {
        let submenu = Submenu::new(
            "工具",
            Menu::new()
                .add_item(CustomMenuItem::new(JS_PLAY_GROUND.to_string(), "Js沙箱环境"))
                .add_item(CustomMenuItem::new(SOCURE_CODE.to_string(), "源码链接")),
        );
        Menu::new().add_submenu(submenu)
    } else {
        Menu::default()
    }
}

//菜单点击事件处理逻辑
pub fn menu_event(event: WindowMenuEvent) {
    match event.menu_item_id() {
        JS_PLAY_GROUND => {
            //开启jsPlayGround 窗口
            create_window(event.window().app_handle(), JS_PLAY_GROUND, "PlayGround", "/#/jsruntime", Menu::default());
        }
        SOCURE_CODE => {
            open_for_browser(&event.window(), "https://gitee.com/stringlxd");
        }
        _ => {}
    }
}
//托盘菜单
pub fn init_system_tray() -> SystemTray {
    let q = CustomMenuItem::new(QUIT.to_string(), "关闭");
    let hide = CustomMenuItem::new(HIDE.to_string(), "隐藏");
    let tray_menu = SystemTrayMenu::new().add_item(q).add_native_item(SystemTrayMenuItem::Separator).add_item(hide);
    SystemTray::new().with_menu(tray_menu)
}

//托盘事件处理
pub fn system_tray_menu_event(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::LeftClick { .. } => {
            let window = app.get_window("main").unwrap();
            window.show().unwrap();
            window.set_focus().unwrap();
        }
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            QUIT => {
                std::process::exit(0);
            }
            HIDE => {
                let window = app.get_window("main").unwrap();
                window.hide().unwrap();
            }
            _ => {}
        },
        _ => {}
    }
}

//系统事件处理
pub fn windows_event(event: GlobalWindowEvent) {
    match event.event() {
        //窗口关闭事件
        tauri::WindowEvent::CloseRequested { api, .. } => {
            //阻止窗口默认关闭动作
            api.prevent_close();
            let window = event.window().clone();
            confirm(Some(&event.window()), "关闭窗口", "确定要关闭当前窗口?", move |e| {
                if e {
                    let _r = window.close();
                }
            });
        }
        _ => {}
    }
}

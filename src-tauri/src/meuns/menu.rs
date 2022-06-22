use tauri::{CustomMenuItem, Menu, WindowMenuEvent, Manager};

use crate::utils::{create_window, open_for_browser};

//初始化菜单
pub fn init_menu() -> Menu {
    let play_ground = CustomMenuItem::new("js_play_ground".to_string(), "PlayGround");
    let socure_code = CustomMenuItem::new("socure_code".to_string(), "socure code");
    Menu::new().add_item(play_ground).add_item(socure_code)
}

//菜单点击事件处理逻辑
pub fn menu_event(event:WindowMenuEvent){
    match event.menu_item_id() {
        "js_play_ground" => {
            create_window(event.window().app_handle(),"js_play_ground","PlayGround","/#/jsruntime",Menu::default());   
        }
        "socure_code" => {
            open_for_browser(&event.window(),"https://gitee.com/stringlxd");
        }
        _=>{}
    }
}


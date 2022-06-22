use tauri::{CustomMenuItem, Menu, WindowMenuEvent, Manager, WindowBuilder, api, Window};

pub fn init_menu() -> Menu {
    let play_ground = CustomMenuItem::new("js_play_ground".to_string(), "PlayGround");
    let socure_code = CustomMenuItem::new("socure_code".to_string(), "socure code");
    Menu::new().add_item(play_ground).add_item(socure_code)
}

pub fn menu_event(event:WindowMenuEvent){
    
    match event.menu_item_id() {
        "js_play_ground" => {
             WindowBuilder::new(&event.window().app_handle(), "JS_playground", tauri::WindowUrl::App("/#/jsruntime".into()))
            .center()
            .menu(Menu::default())
            .title("PlayGround")
            .build().unwrap();
        }
        "socure_code" => {
            api::shell::open(&event.window().shell_scope(), "https://gitee.com/stringlxd".to_string(), None).unwrap();
        }
        _=>{}
    }
    println!("{}",event.menu_item_id());
}
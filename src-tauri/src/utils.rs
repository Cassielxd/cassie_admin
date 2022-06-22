use tauri::{Window, Manager, WindowBuilder, Menu, api};


//创建新的窗口 lable唯一标识
pub fn  create_window(app: tauri::AppHandle,lable:&str,title:&str,router:&str,menu:Menu){
    match app.get_window(lable){
        Some(js) =>{
               let _r =js.set_focus();
        },
        None =>{
        WindowBuilder::new(&app, lable, tauri::WindowUrl::App(router.into()))
            .center()
            .menu(menu)
            .title(title)
            .build().unwrap();
        },
    }
    
}

 #[tauri::command]
 pub fn c_create_window(lable:&str,title:&str,router:&str,app: tauri::AppHandle) {
    create_window(app,lable,title,router,Menu::default());
 }

//url 打开浏览器 
pub fn open_for_browser(window:&Window,path:&str){
    api::shell::open(&window.shell_scope(), path.to_string(), None).unwrap();
}
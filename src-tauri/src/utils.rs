use tauri::{Window, Manager, WindowBuilder, Menu, api};


//创建新的窗口 lable唯一标识
pub fn  create_window(window:&Window,lable:&str,title:&str,router:&str,menu:Menu){
    match window.app_handle().get_window(lable){
        Some(js) =>{
               let _r =js.set_focus();
        },
        None =>{
            WindowBuilder::new(&window.app_handle(), lable, tauri::WindowUrl::App(router.into()))
            .center()
            .menu(menu)
            .title(title)
            .build().unwrap();
        },
    }
    
}
//url 打开浏览器 
pub fn open_for_browser(window:&Window,path:&str){
    api::shell::open(&window.shell_scope(), path.to_string(), None).unwrap();
}
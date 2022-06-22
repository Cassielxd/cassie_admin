#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use app::meuns::menu::{menu_event, init_menu};
use tauri::Manager;
use tauri::{GlobalWindowEvent, WindowEvent, Wry};

fn main() {
   let context = tauri::generate_context!();
    tauri::Builder::default()
        .menu(init_menu())
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();

            Ok(())
        }).on_menu_event(menu_event)
        .run(context)
        .expect("error while running tauri application");
}

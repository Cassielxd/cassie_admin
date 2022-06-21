#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{GlobalWindowEvent, WindowEvent, Wry};

fn main() {
  let context = tauri::generate_context!();
  tauri::Builder::default()
      .on_window_event(|event|{
        match event.event() {
            WindowEvent::Resized(_) => {}
            WindowEvent::Moved(_) => {}
            WindowEvent::CloseRequested { .. } => {}
            WindowEvent::Destroyed => {}
            WindowEvent::Focused(_) => {}
            WindowEvent::ScaleFactorChanged { .. } => {}
            WindowEvent::FileDrop(_) => {}
            WindowEvent::ThemeChanged(_) => {}
            _=>{}
        }
      })
    .run(context)
    .expect("error while running tauri application");
}

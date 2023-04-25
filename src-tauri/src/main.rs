// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::LogicalPosition;
use tauri::Manager;
use tauri::WindowEvent;
use tauri_plugin_positioner::{WindowExt, Position};

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      let win = app.get_window("main").unwrap();
      let _ = win.set_position(LogicalPosition::new(200, 200));
      Ok(())
    })
    .on_window_event(|e| {
      if let WindowEvent::Resized(_) = e.event() {
        std::thread::sleep(std::time::Duration::from_millis(1));
      }
    })
    .plugin(tauri_plugin_window_state::Builder::default().build())
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

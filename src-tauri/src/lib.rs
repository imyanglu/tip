// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::sync::Arc;

use std::{fs, thread};

use serde::de::value::Error;
use serde::Serialize;
use tauri::{Emitter, EventTarget, LogicalPosition, Manager, WebviewWindow, Window};
pub mod info;
pub mod model;
pub mod win;
pub struct AppState {
    pub config: Arc<model::Config>,
}

#[tauri::command]
fn get_system_info() -> info::SysInfo {
    info::get_sys_info()
}

#[tauri::command]
fn get_process_info() -> Result<Vec<win::ProcessInfo>, String> {
    let res = win::get_poc();
    if res.is_none() {
        return Err(String::from("读取错误"));
    } else {
        return Ok(res.unwrap());
    }
}
#[tauri::command]
fn kill_process(pid: u32) -> bool {
    win::kill_process(pid)
}
#[derive(Clone, Serialize)]
struct Payload {
    message: String,
}
#[tauri::command]
fn create_window(label: String) {
    println!("创建窗口{label}");
    let is_create = tauri::Builder::default().setup(|app| {
        let handle = app.handle().clone();
        std::thread::spawn(move || {
            let webview_window = tauri::WebviewWindowBuilder::new(
                &handle,
                &label,
                tauri::WebviewUrl::App("index.html".into()),
            )
            .build()
            .unwrap();
        });
        Ok(())
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let temp_dir = std::env::temp_dir();
    tauri::Builder::default()
        .setup(|app| {
            // 获取主窗口
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        // .manage(AppState {
        //     config: config.clone(),
        // })
        .invoke_handler(tauri::generate_handler![
            get_system_info,
            get_process_info,
            kill_process,
            create_window
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

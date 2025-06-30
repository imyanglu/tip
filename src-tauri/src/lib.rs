// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::sync::Arc;
use std::time::Duration;
use std::{fs, thread};

use chrono::Local;
use serde::Serialize;
use tauri::{Emitter, EventTarget, LogicalPosition, Manager, WebviewWindow, Window};
pub mod info;
pub mod model;
pub struct AppState {
    pub config: Arc<model::Config>,
}

fn load_config(path: &str) -> Result<model::Config, Box<dyn std::error::Error>> {
    let fs_res = fs::read_to_string(path)?;
    let config: model::Config = serde_json::from_str(&fs_res)?;
    Ok(config)
}

#[tauri::command]
fn get_earned_day(state: tauri::State<AppState>) -> f32 {
    state.config.get_earned_day()
}

#[tauri::command]
fn get_system_info() -> info::SysInfo {
    info::get_sys_info()
}

#[derive(Clone, Serialize)]
struct Payload {
    message: String,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let temp_dir = std::env::temp_dir();
    println!("Scanning temp dir: {:?}", temp_dir);
    tauri::Builder::default()
        .setup(|app| {
            // 获取主窗口
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        // .manage(AppState {
        //     config: config.clone(),
        // })
        .invoke_handler(tauri::generate_handler![get_system_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
